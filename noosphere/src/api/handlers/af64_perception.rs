//! AF64 Perception endpoint

use axum::{
    extract::{Path, Query, State},
    Json,
};
use dpn_core::DbPool;
use serde::Deserialize;
use serde_json::Value;
use sqlx::Row;
use chrono;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct PerceptionQuery {
    pub tier: Option<String>,
    pub since: Option<String>,
}

/// GET /api/perception/:agent_id
pub async fn get_perception(
    State(pool): State<DbPool>,
    Path(agent_id): Path<String>,
    Query(q): Query<PerceptionQuery>,
) -> Result<Json<Value>, ApiError> {
    let tier = q.tier.as_deref().unwrap_or("base");
    let since = q.since.as_deref().unwrap_or("1970-01-01T00:00:00Z");

    // 1. Messages — look for recent messages OR handoffs (never expire)
    // Recent = after since timestamp (normal messages)
    // Handoffs = any unresponded handoff regardless of age
    let msgs = sqlx::query(
        r#"SELECT id, from_agent, message, channel, thread_id, created_at 
           FROM conversations 
           WHERE ($1 = ANY(to_agent) OR message ILIKE '%@' || $1 || '%')
             AND from_agent != $1
             AND NOT ($1 = ANY(read_by))
             AND (
               -- Recent messages since last tick
               created_at > $2::timestamptz
               OR
               -- Handoffs that haven't been responded to yet
               (metadata->>'source' = 'handoff' AND NOT EXISTS (
                 SELECT 1 FROM conversations r 
                 WHERE r.from_agent = $1 
                   AND r.metadata->>'responding_to' = conversations.id::text
               ))
             )
           ORDER BY 
             CASE WHEN metadata->>'source' = 'handoff' THEN 0 ELSE 1 END,
             created_at DESC
           LIMIT 10"#
    )
    .bind(&agent_id).bind(since)
    .fetch_all(&pool).await.unwrap_or_default();

    let messages: Vec<Value> = msgs.iter().map(|r| {
        let msg: String = r.get("message");
        let truncated: String = msg.chars().take(500).collect();
        serde_json::json!({
            "id": r.get::<i32, _>("id"),
            "from": r.get::<String, _>("from_agent"),
            "message": truncated,
            "channel": r.get::<Option<String>, _>("channel"),
            "thread_id": r.get::<Option<uuid::Uuid>, _>("thread_id").map(|u| u.to_string()),
        })
    }).collect();

    // 2. Tasks
    // 2a. Get agent's department and tier for task routing
    let agent_meta = sqlx::query(
        "SELECT department, agent_tier FROM agents WHERE id = $1"
    )
    .bind(&agent_id)
    .fetch_optional(&pool).await.unwrap_or(None);

    let agent_dept: Option<String> = agent_meta.as_ref().and_then(|r| r.get("department"));
    let agent_tier_str: Option<String> = agent_meta.as_ref().and_then(|r| r.get("agent_tier"));
    let is_exec = agent_tier_str.as_deref() == Some("executive");

    // Check if agent has tool_scope — no tools = limited perception
    let has_tools = {
        let tool_row = sqlx::query("SELECT tool_scope FROM agents WHERE id = $1")
            .bind(&agent_id)
            .fetch_optional(&pool).await.unwrap_or(None);
        tool_row
            .and_then(|r| r.get::<Option<Vec<String>>, _>("tool_scope"))
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    };

    // 2b. Tasks: only for agents with tools (toolless agents just hallucinate progress)
    // Exception: triage agents (sarah, lara) see unassigned tasks even without tool_scope
    let is_triage = agent_id == "lara" || agent_id == "sarah";
    let task_rows = if !has_tools && !is_triage {
        // No tools = no tasks. They can't do real work on them.
        vec![]
    } else if is_triage {
        // Triage agents see unclassified/unassigned tasks (excluding blocked)
        sqlx::query(
            r#"SELECT id, text, status, assignee, assigned_to, department, stage, goal_id, stage_notes,
                      project_id, source, context, parent_id, priority, scheduled_at, blocked_by
               FROM tasks
               WHERE status IN ('open', 'pending', 'in-progress') AND assigned_to IS NULL
                 AND (blocked_by IS NULL OR blocked_by = '{}' OR NOT EXISTS (
                   SELECT 1 FROM unnest(blocked_by) AS dep_id
                   JOIN tasks t ON t.id = dep_id
                   WHERE t.status NOT IN ('done', 'completed')
                 ))
               ORDER BY id DESC LIMIT 15"#
        )
        .fetch_all(&pool).await.unwrap_or_default()
    } else if is_exec {
        // Executives see personal tasks + their department's unassigned tasks (excluding blocked)
        sqlx::query(
            r#"SELECT id, text, status, assignee, assigned_to, department, stage, goal_id, stage_notes,
                      project_id, source, context, parent_id, priority, scheduled_at, blocked_by
               FROM tasks
               WHERE status IN ('open', 'pending', 'in-progress')
                 AND ($1 = ANY(assigned_to) OR (department = $2 AND assigned_to IS NULL))
                 AND (blocked_by IS NULL OR blocked_by = '{}' OR NOT EXISTS (
                   SELECT 1 FROM unnest(blocked_by) AS dep_id
                   JOIN tasks t ON t.id = dep_id
                   WHERE t.status NOT IN ('done', 'completed')
                 ))
               ORDER BY CASE WHEN $1 = ANY(assigned_to) THEN 0 ELSE 1 END, id DESC LIMIT 10"#
        )
        .bind(&agent_id).bind(&agent_dept)
        .fetch_all(&pool).await.unwrap_or_default()
    } else {
        // Staff see only personal tasks (excluding blocked)
        sqlx::query(
            r#"SELECT id, text, status, assignee, assigned_to, department, stage, goal_id, stage_notes,
                      project_id, source, context, parent_id, priority, scheduled_at, blocked_by
               FROM tasks
               WHERE $1 = ANY(assigned_to) AND status IN ('open', 'pending', 'in-progress')
                 AND (blocked_by IS NULL OR blocked_by = '{}' OR NOT EXISTS (
                   SELECT 1 FROM unnest(blocked_by) AS dep_id
                   JOIN tasks t ON t.id = dep_id
                   WHERE t.status NOT IN ('done', 'completed')
                 ))
               ORDER BY id DESC LIMIT 5"#
        )
        .bind(&agent_id)
        .fetch_all(&pool).await.unwrap_or_default()
    };

    // Executive blocked task visibility (per D-04)
    let blocked_tasks: Vec<Value> = if is_exec {
        let blocked_rows = sqlx::query(
            r#"SELECT t.id, t.text, t.status, t.blocked_by, t.assignee, t.project_id
               FROM tasks t
               WHERE t.project_id IN (
                   SELECT id FROM projects WHERE owner = $1
               )
               AND t.blocked_by IS NOT NULL AND t.blocked_by != '{}'
               AND EXISTS (
                   SELECT 1 FROM unnest(t.blocked_by) AS dep_id
                   JOIN tasks bt ON bt.id = dep_id
                   WHERE bt.status NOT IN ('done', 'completed')
               )
               ORDER BY t.id DESC LIMIT 10"#
        )
        .bind(&agent_id)
        .fetch_all(&pool).await.unwrap_or_default();

        blocked_rows.iter().map(|r| {
            let text: String = r.get("text");
            let truncated: String = text.chars().take(200).collect();
            serde_json::json!({
                "id": r.get::<i32, _>("id"),
                "text": truncated,
                "status": r.get::<String, _>("status"),
                "blocked_by": r.get::<Option<Vec<i32>>, _>("blocked_by"),
                "assignee": r.get::<Option<String>, _>("assignee"),
                "project_id": r.get::<Option<i32>, _>("project_id"),
            })
        }).collect()
    } else {
        vec![]
    };

    // Executive critical quality issues visibility (VER-02)
    let critical_issues: Vec<Value> = if is_exec {
        let critical_rows = sqlx::query(
            r#"SELECT t.id, t.text, t.stage_notes, t.project_id, t.assignee
               FROM tasks t
               WHERE t.project_id IN (
                   SELECT id FROM projects WHERE owner = $1
               )
               AND t.status IN ('done', 'completed')
               AND t.stage_notes IS NOT NULL
               AND t.stage_notes -> 'issues' IS NOT NULL
               AND EXISTS (
                   SELECT 1 FROM jsonb_array_elements(t.stage_notes -> 'issues') AS issue
                   WHERE issue->>'severity' = 'CRITICAL'
               )
               ORDER BY t.updated_at DESC LIMIT 10"#
        )
        .bind(&agent_id)
        .fetch_all(&pool).await.unwrap_or_default();

        critical_rows.iter().map(|r| {
            let text: String = r.get("text");
            let truncated: String = text.chars().take(200).collect();
            serde_json::json!({
                "id": r.get::<i32, _>("id"),
                "text": truncated,
                "project_id": r.get::<Option<i32>, _>("project_id"),
                "assignee": r.get::<Option<String>, _>("assignee"),
                "stage_notes": r.get::<Option<serde_json::Value>, _>("stage_notes"),
            })
        }).collect()
    } else {
        vec![]
    };

    let tasks: Vec<Value> = task_rows.iter().map(|r| {
        let text: String = r.get("text");
        let truncated_text: String = text.chars().take(300).collect();
        serde_json::json!({
            "id": r.get::<i32, _>("id"),
            "text": truncated_text,
            "status": r.get::<String, _>("status"),
            "assignee": r.get::<Option<String>, _>("assignee"),
            "assigned_to": r.get::<Option<Vec<String>>, _>("assigned_to"),
            "department": r.get::<Option<String>, _>("department"),
            "stage": r.get::<Option<String>, _>("stage"),
            "goal_id": r.get::<Option<i32>, _>("goal_id"),
            "stage_notes": r.get::<Option<serde_json::Value>, _>("stage_notes"),
            "project_id": r.get::<Option<i32>, _>("project_id"),
            "source": r.get::<Option<String>, _>("source"),
            "context": r.get::<Option<String>, _>("context"),
            "parent_id": r.get::<Option<i32>, _>("parent_id"),
            "priority": r.get::<Option<String>, _>("priority"),
            "scheduled_at": r.get::<Option<chrono::DateTime<chrono::Utc>>, _>("scheduled_at")
                .map(|dt| dt.to_rfc3339()),
            "blocked_by": r.get::<Option<Vec<i32>>, _>("blocked_by"),
        })
    }).collect();

    // 3. Documents
    let doc_rows = sqlx::query(
        "SELECT d.id, d.title, COALESCE(d.modified_at, d.created_at) as updated_at FROM documents d JOIN agent_document_links adl ON adl.document_id = d.id WHERE adl.agent_id = $1 AND COALESCE(d.modified_at, d.created_at) > $2::timestamptz ORDER BY COALESCE(d.modified_at, d.created_at) DESC LIMIT 5"
    )
    .bind(&agent_id).bind(since)
    .fetch_all(&pool).await.unwrap_or_default();

    let documents: Vec<Value> = doc_rows.iter().map(|r| {
        serde_json::json!({
            "id": r.get::<i32, _>("id"),
            "title": r.get::<Option<String>, _>("title"),
        })
    }).collect();

    // 4. Team activity
    let mut team_activity: Vec<Value> = vec![];
    if tier == "working" || tier == "prime" {
        let reports = sqlx::query("SELECT id FROM agents WHERE reports_to = $1 AND status = 'active'")
            .bind(&agent_id)
            .fetch_all(&pool).await.unwrap_or_default();

        let report_ids: Vec<String> = reports.iter().map(|r| r.get::<String, _>("id")).collect();
        if !report_ids.is_empty() {
            let activity = sqlx::query(
                "SELECT agent_id, action_taken, energy_after, tick_at FROM tick_log WHERE agent_id = ANY($1) AND tick_at > $2::timestamptz ORDER BY tick_at DESC LIMIT 10"
            )
            .bind(&report_ids).bind(since)
            .fetch_all(&pool).await.unwrap_or_default();

            team_activity = activity.iter().map(|r| {
                serde_json::json!({
                    "agent": r.get::<Option<String>, _>("agent_id"),
                    "action": r.get::<Option<String>, _>("action_taken"),
                    "energy": r.get::<Option<f64>, _>("energy_after"),
                })
            }).collect();
        }
    }

    // 5. Proactive eligibility — can this agent self-initiate work this tick?
    // Eligible if: energy >= 40, last action was > 5 ticks ago (or never acted)
    // Load agent tier for cooldown differentiation
    let agent_tier_row = sqlx::query("SELECT agent_tier FROM agents WHERE id = $1")
        .bind(&agent_id)
        .fetch_optional(&pool).await.unwrap_or(None);
    let agent_tier = agent_tier_row
        .and_then(|r| r.get::<Option<String>, _>("agent_tier"))
        .unwrap_or_default();

    let proactive_eligible = if !has_tools {
        // No tools = no proactive work. Only respond to messages/requests/tasks.
        false
    } else {
        let state = sqlx::query(
            "SELECT energy, ticks_alive, last_tick_at FROM agent_state WHERE agent_id = $1"
        )
        .bind(&agent_id)
        .fetch_optional(&pool).await.unwrap_or(None);

        let last_action = sqlx::query(
            "SELECT MAX(created_at) as last_acted FROM conversations WHERE from_agent = $1"
        )
        .bind(&agent_id)
        .fetch_optional(&pool).await.unwrap_or(None);

        if let Some(s) = state {
            let energy: f64 = s.get::<Option<f64>, _>("energy").unwrap_or(50.0);
            let ticks: i32 = s.get::<Option<i32>, _>("ticks_alive").unwrap_or(0);
            let last_acted: Option<chrono::DateTime<chrono::Utc>> = last_action
                .and_then(|r| r.get::<Option<chrono::DateTime<chrono::Utc>>, _>("last_acted"));
            // Executives get 5-min cooldown (delegation work is fast)
            // Staff get 30-min cooldown (production work takes time)
            let cooldown_minutes = if agent_tier == "executive" { 5 } else { 30 };
            let cooldown_passed = match last_acted {
                Some(t) => (chrono::Utc::now() - t).num_minutes() >= cooldown_minutes,
                None => ticks > 3,
            };
            energy >= 40.0 && cooldown_passed && messages.is_empty() && tasks.is_empty()
        } else {
            false
        }
    };

    // 6. If proactive, load agent's responsibilities + goals from frontmatter
    let responsibilities = if proactive_eligible {
        let doc = sqlx::query(
            "SELECT frontmatter FROM documents WHERE frontmatter LIKE '%AF64-Ghost%' AND frontmatter::jsonb->>'title' = $1"
        )
        .bind(&agent_id)
        .fetch_optional(&pool).await.unwrap_or(None);

        // Try matching on agents.document_id too
        let doc = if doc.is_none() {
            sqlx::query(
                "SELECT d.frontmatter FROM agents a JOIN documents d ON d.id = a.document_id WHERE a.id = $1"
            )
            .bind(&agent_id)
            .fetch_optional(&pool).await.unwrap_or(None)
        } else { doc };

        doc.and_then(|r| {
            let fm: Option<String> = r.get("frontmatter");
            fm.and_then(|s| serde_json::from_str::<Value>(&s).ok())
        }).map(|fm| {
            serde_json::json!({
                "responsibilities": fm.get("responsibilities").cloned().unwrap_or(Value::Null),
                "goals": fm.get("goals").cloned().unwrap_or(Value::Null),
                "content_focus": fm.get("content_focus").cloned().unwrap_or(Value::Null),
                "department": fm.get("earth_department").cloned().unwrap_or(Value::Null),
                "role": fm.get("earth_role").cloned().unwrap_or(Value::Null),
            })
        }).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    // 7. Relationships — collaborators, mentor, reports_to + their recent activity
    let relationships = {
        // Load from agent's document frontmatter
        let doc = sqlx::query(
            "SELECT d.frontmatter FROM agents a JOIN documents d ON d.id = a.document_id WHERE a.id = $1"
        )
        .bind(&agent_id)
        .fetch_optional(&pool).await.unwrap_or(None);

        let fm: Option<Value> = doc.and_then(|r| {
            let s: Option<String> = r.get("frontmatter");
            s.and_then(|s| serde_json::from_str(&s).ok())
        });

        let mentor = fm.as_ref().and_then(|f| f.get("mentor")).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let reports_to = fm.as_ref().and_then(|f| f.get("reports_to")).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let team_collabs = fm.as_ref().and_then(|f| f.get("team_collaborators")).and_then(|v| v.as_str()).unwrap_or("").to_string();

        // Parse collaborator names from bullet list (• Name\n• Name)
        let collab_names: Vec<String> = team_collabs.lines()
            .map(|l| l.trim().trim_start_matches('•').trim().to_string())
            .filter(|n| !n.is_empty())
            .collect();

        // Resolve names to agent IDs
        let mut collab_ids: Vec<String> = vec![];
        for name in &collab_names {
            let clean = name.replace("[[", "").replace("]]", "");
            if let Ok(Some(row)) = sqlx::query(
                "SELECT id FROM agents WHERE full_name = $1 OR regexp_replace(full_name, ' ', '', 'g') = $1"
            ).bind(&clean).fetch_optional(&pool).await {
                collab_ids.push(row.get::<String, _>("id"));
            }
        }

        // Get recent activity from collaborators
        let collab_activity = if !collab_ids.is_empty() {
            sqlx::query(
                r#"SELECT from_agent, LEFT(message, 200) as msg, metadata->>'source' as source, created_at
                   FROM conversations 
                   WHERE from_agent = ANY($1) AND created_at > NOW() - INTERVAL '2 hours'
                   ORDER BY created_at DESC LIMIT 5"#
            )
            .bind(&collab_ids)
            .fetch_all(&pool).await.unwrap_or_default()
        } else { vec![] };

        let activity: Vec<Value> = collab_activity.iter().map(|r| {
            serde_json::json!({
                "agent": r.get::<String, _>("from_agent"),
                "message": r.get::<String, _>("msg"),
                "source": r.get::<Option<String>, _>("source"),
            })
        }).collect();

        serde_json::json!({
            "mentor": mentor,
            "reports_to": reports_to,
            "collaborators": collab_names,
            "collaborator_ids": collab_ids,
            "collaborator_activity": activity,
        })
    };

    // 8. Projects — owned active projects give executives portfolio awareness
    let project_rows = sqlx::query(
        r#"SELECT p.id, p.name, p.status, p.description, p.goals, p.blockers, p.current_context, p.schedule,
                  p.lifestage,
                  a.name as area_name,
                  (SELECT COUNT(*) FROM tasks t WHERE t.project_id = p.id AND t.status IN ('open', 'pending', 'in-progress')) as open_tasks,
                  (SELECT COUNT(*) FROM tasks t WHERE t.project_id = p.id AND t.status IN ('done', 'completed')) as completed_tasks
           FROM projects p
           LEFT JOIN areas a ON p.area_id = a.id
           WHERE p.owner = $1 AND p.status = 'active'
           ORDER BY p.updated_at DESC"#
    )
    .bind(&agent_id)
    .fetch_all(&pool).await.unwrap_or_default();

    let projects: Vec<Value> = project_rows.iter().map(|r| {
        let desc: Option<String> = r.get("description");
        let truncated_desc: Option<String> = desc.map(|d| d.chars().take(500).collect());
        serde_json::json!({
            "id": r.get::<i32, _>("id"),
            "name": r.get::<String, _>("name"),
            "status": r.get::<Option<String>, _>("status"),
            "description": truncated_desc,
            "goals": r.get::<Option<String>, _>("goals"),
            "blockers": r.get::<Option<String>, _>("blockers"),
            "current_context": r.get::<Option<String>, _>("current_context"),
            "open_tasks": r.get::<Option<i64>, _>("open_tasks"),
            "completed_tasks": r.get::<Option<i64>, _>("completed_tasks"),
            "schedule": r.get::<Option<serde_json::Value>, _>("schedule"),
            "lifestage": r.get::<String, _>("lifestage"),
            "area_name": r.get::<Option<String>, _>("area_name"),
        })
    }).collect();

    // 9. Agent requests — DISABLED: old dispatch system replaced by artificial life evolution
    //    Agents now act based on tools, fitness, and direct messages only
    let request_rows: Vec<sqlx::postgres::PgRow> = vec![];

    let requests: Vec<Value> = request_rows.iter().map(|r| {
        let subject: String = r.get("subject");
        let truncated_subject: String = subject.chars().take(300).collect();
        serde_json::json!({
            "id": r.get::<i32, _>("id"),
            "from": r.get::<String, _>("from_agent"),
            "type": r.get::<String, _>("request_type"),
            "subject": truncated_subject,
            "status": r.get::<String, _>("status"),
            "response": r.get::<Option<String>, _>("response"),
            "priority": r.get::<Option<i32>, _>("priority"),
        })
    }).collect();

    // 9. Recent memories from memories daily note columns
    let mem_column = format!("{}_memories", agent_id.replace("-", "_"));
    // Safety: only allow alphanumeric + underscore in column name
    let safe_column = mem_column.chars().all(|c| c.is_alphanumeric() || c == '_');
    let recent_memories: Vec<String> = if safe_column {
        let mem_query = format!(
            "SELECT note_date::text, {} FROM memories \
             WHERE note_type = 'daily' AND {} IS NOT NULL AND {} != '' \
             AND note_date >= CURRENT_DATE - INTERVAL '2 days' \
             ORDER BY note_date DESC LIMIT 2",
            mem_column, mem_column, mem_column
        );
        match sqlx::query(&mem_query).fetch_all(&pool).await {
            Ok(rows) => rows.iter().map(|r| {
                use sqlx::Row;
                let date: String = r.try_get::<String, _>("note_date").unwrap_or_default();
                let mem: String = r.try_get::<String, _>(mem_column.as_str()).unwrap_or_default();
                format!("[{}] {}", date, mem.chars().take(1000).collect::<String>())
            }).collect(),
            Err(_) => vec![],
        }
    } else {
        vec![]
    };

    Ok(Json(serde_json::json!({
        "messages": messages,
        "tasks": tasks,
        "projects": projects,
        "documents": documents,
        "team_activity": team_activity,
        "proactive_eligible": proactive_eligible,
        "responsibilities": responsibilities,
        "relationships": relationships,
        "requests": requests,
        "recent_memories": recent_memories,
        "blocked_tasks": blocked_tasks,
        "critical_issues": critical_issues,
    })))
}
