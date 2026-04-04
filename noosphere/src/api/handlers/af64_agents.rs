//! AF64 Agent endpoints

use axum::{
    extract::{Path, Query, State},
    Json,
};
use dpn_core::DbPool;
use serde::Deserialize;
use serde_json::Value;
use sqlx::Row;

use crate::error::ApiError;

/// GET /api/agents
pub async fn list_agents(State(pool): State<DbPool>) -> Result<Json<Value>, ApiError> {
    let rows = sqlx::query(
        r#"SELECT a.id, a.full_name, a.role, a.department, a.reports_to, a.mentor, a.collaborators, a.liaises_with, a.avatar, a.agent_tier, a.status,
                  s.energy, s.tier, s.last_tick_at, s.ticks_at_current_tier, s.ticks_alive, s.dormant_since, s.metadata
           FROM agents a
           LEFT JOIN agent_state s ON s.agent_id = a.id
           ORDER BY a.id"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    let agents: Vec<Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.get::<String, _>("id"),
            "full_name": r.get::<Option<String>, _>("full_name"),
            "role": r.get::<Option<String>, _>("role"),
            "department": r.get::<Option<String>, _>("department"),
            "reports_to": r.get::<Option<Vec<String>>, _>("reports_to"),
            "mentor": r.get::<Option<String>, _>("mentor"),
            "collaborators": r.get::<Option<Vec<String>>, _>("collaborators"),
            "liaises_with": r.get::<Option<Vec<String>>, _>("liaises_with"),
            "avatar": r.get::<Option<String>, _>("avatar"),
            "agent_tier": r.get::<Option<String>, _>("agent_tier"),
            "status": r.get::<Option<String>, _>("status"),
            "energy": r.get::<Option<f64>, _>("energy"),
            "tier": r.get::<Option<String>, _>("tier"),
            "last_tick_at": r.get::<Option<chrono::DateTime<chrono::Utc>>, _>("last_tick_at"),
            "ticks_at_current_tier": r.get::<Option<i32>, _>("ticks_at_current_tier"),
            "ticks_alive": r.get::<Option<i32>, _>("ticks_alive"),
            "metadata": r.get::<Option<Value>, _>("metadata"),
        })
    }).collect();

    Ok(Json(serde_json::json!(agents)))
}

/// GET /api/agents/:id
pub async fn get_agent(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let agent = sqlx::query(
        r#"SELECT a.id, a.full_name, a.role, a.department, a.reports_to, a.mentor, a.collaborators, a.liaises_with, a.avatar, a.agent_tier, a.status, a.document_id, a.tool_scope,
                  s.energy, s.tier, s.last_tick_at, s.ticks_at_current_tier, s.ticks_alive, s.dormant_since, s.metadata
           FROM agents a
           LEFT JOIN agent_state s ON s.agent_id = a.id
           WHERE a.id = $1"#
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?
    .ok_or_else(|| ApiError::NotFound(format!("Agent {} not found", id)))?;

    let agent_json = serde_json::json!({
        "id": agent.get::<String, _>("id"),
        "full_name": agent.get::<Option<String>, _>("full_name"),
        "role": agent.get::<Option<String>, _>("role"),
        "department": agent.get::<Option<String>, _>("department"),
        "reports_to": agent.get::<Option<Vec<String>>, _>("reports_to"),
        "mentor": agent.get::<Option<String>, _>("mentor"),
        "collaborators": agent.get::<Option<Vec<String>>, _>("collaborators"),
        "liaises_with": agent.get::<Option<Vec<String>>, _>("liaises_with"),
        "avatar": agent.get::<Option<String>, _>("avatar"),
        "agent_tier": agent.get::<Option<String>, _>("agent_tier"),
        "status": agent.get::<Option<String>, _>("status"),
        "document_id": agent.get::<Option<i32>, _>("document_id"),
        "tool_scope": agent.get::<Option<Vec<String>>, _>("tool_scope"),
        "energy": agent.get::<Option<f64>, _>("energy"),
        "tier": agent.get::<Option<String>, _>("tier"),
        "ticks_at_current_tier": agent.get::<Option<i32>, _>("ticks_at_current_tier"),
        "ticks_alive": agent.get::<Option<i32>, _>("ticks_alive"),
        "metadata": agent.get::<Option<Value>, _>("metadata"),
    });

    let drives = sqlx::query(
        "SELECT id, agent_id, drive_name, description, satisfaction, pressure, frustration, decay_rate FROM agent_drives WHERE agent_id = $1 ORDER BY pressure DESC"
    )
    .bind(&id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    let drives_json: Vec<Value> = drives.iter().map(|r| {
        serde_json::json!({
            "id": r.get::<i32, _>("id"),
            "agent_id": r.get::<Option<String>, _>("agent_id"),
            "drive_name": r.get::<String, _>("drive_name"),
            "description": r.get::<Option<String>, _>("description"),
            "satisfaction": r.get::<Option<f64>, _>("satisfaction"),
            "pressure": r.get::<Option<f64>, _>("pressure"),
            "frustration": r.get::<Option<f64>, _>("frustration"),
            "decay_rate": r.get::<Option<f64>, _>("decay_rate"),
        })
    }).collect();

    Ok(Json(serde_json::json!({"agent": agent_json, "drives": drives_json})))
}

#[derive(Deserialize)]
pub struct StateUpdate {
    pub energy: Option<f64>,
    pub tier: Option<String>,
    pub last_tick_at: Option<String>,
    pub ticks_at_current_tier: Option<i32>,
    pub ticks_alive: Option<i32>,
    pub dormant_since: Option<String>,
    pub metadata: Option<Value>,
    pub energy_delta: Option<f64>,
}

/// PATCH /api/agents/:id/state
pub async fn update_state(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
    Json(body): Json<StateUpdate>,
) -> Result<Json<Value>, ApiError> {
    if let Some(delta) = body.energy_delta {
        let row = sqlx::query(
            "UPDATE agent_state SET energy = GREATEST(0, LEAST(100, energy + $1)) WHERE agent_id = $2 RETURNING energy"
        )
        .bind(delta)
        .bind(&id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

        if let Some(r) = row {
            return Ok(Json(serde_json::json!({"energy": r.get::<Option<f64>, _>("energy")})));
        }
    }

    if let Some(energy) = body.energy {
        sqlx::query("UPDATE agent_state SET energy = GREATEST(0, LEAST(100, $1)) WHERE agent_id = $2")
            .bind(energy).bind(&id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(ref tier) = body.tier {
        sqlx::query("UPDATE agent_state SET tier = $1 WHERE agent_id = $2")
            .bind(tier).bind(&id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if body.last_tick_at.is_some() {
        sqlx::query("UPDATE agent_state SET last_tick_at = now() WHERE agent_id = $1")
            .bind(&id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(val) = body.ticks_at_current_tier {
        sqlx::query("UPDATE agent_state SET ticks_at_current_tier = $1 WHERE agent_id = $2")
            .bind(val).bind(&id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(val) = body.ticks_alive {
        sqlx::query("UPDATE agent_state SET ticks_alive = $1 WHERE agent_id = $2")
            .bind(val).bind(&id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(ref metadata) = body.metadata {
        sqlx::query(
            "UPDATE agent_state SET metadata = COALESCE(metadata, '{}'::jsonb) || $1::jsonb WHERE agent_id = $2"
        )
        .bind(metadata)
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;
    }

    let energy = sqlx::query("SELECT energy FROM agent_state WHERE agent_id = $1")
        .bind(&id).fetch_optional(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({"ok": true, "energy": energy.map(|r| r.get::<Option<f64>, _>("energy"))})))
}

/// GET /api/agents/:id/drives
pub async fn get_drives(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let drives = sqlx::query(
        "SELECT id, agent_id, drive_name, description, satisfaction, pressure, frustration, decay_rate FROM agent_drives WHERE agent_id = $1 ORDER BY pressure DESC"
    )
    .bind(&id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    let drives_json: Vec<Value> = drives.iter().map(|r| {
        serde_json::json!({
            "id": r.get::<i32, _>("id"),
            "drive_name": r.get::<String, _>("drive_name"),
            "description": r.get::<Option<String>, _>("description"),
            "satisfaction": r.get::<Option<f64>, _>("satisfaction"),
            "pressure": r.get::<Option<f64>, _>("pressure"),
            "frustration": r.get::<Option<f64>, _>("frustration"),
            "decay_rate": r.get::<Option<f64>, _>("decay_rate"),
        })
    }).collect();

    Ok(Json(serde_json::json!(drives_json)))
}

#[derive(Deserialize)]
pub struct DriveUpdate {
    pub satisfaction: Option<f64>,
    pub pressure: Option<f64>,
    pub frustration: Option<f64>,
}

/// PATCH /api/agents/:id/drives/:drive_name
pub async fn update_drive(
    State(pool): State<DbPool>,
    Path((id, drive_name)): Path<(String, String)>,
    Json(body): Json<DriveUpdate>,
) -> Result<Json<Value>, ApiError> {
    if let Some(sat) = body.satisfaction {
        sqlx::query("UPDATE agent_drives SET satisfaction = $1 WHERE agent_id = $2 AND drive_name = $3")
            .bind(sat).bind(&id).bind(&drive_name).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(pres) = body.pressure {
        sqlx::query("UPDATE agent_drives SET pressure = $1 WHERE agent_id = $2 AND drive_name = $3")
            .bind(pres).bind(&id).bind(&drive_name).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(frust) = body.frustration {
        sqlx::query("UPDATE agent_drives SET frustration = $1 WHERE agent_id = $2 AND drive_name = $3")
            .bind(frust).bind(&id).bind(&drive_name).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }

    Ok(Json(serde_json::json!({"ok": true})))
}

/// PUT /api/agents/:id/relationships - Update agent relationships and write back to document
pub async fn update_relationships(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    // Get current agent
    let agent = sqlx::query("SELECT id, full_name, document_path FROM agents WHERE id = $1")
        .bind(&id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Agent {} not found", id)))?;
    
    let full_name: String = agent.get("full_name");
    let doc_name = full_name.replace(" ", "").replace(".", "").replace(".", "");
    let doc_path = agent.get::<Option<String>, _>("document_path")
        .unwrap_or_else(|| format!("Areas/Eckenrode Muziekopname/EM Staff/{}.md", doc_name));
    
    // Get the document
    // Try to get the document (optional - some agents like Nova don't have one)
    let doc = sqlx::query("SELECT id, content FROM documents WHERE path = $1 AND is_canonical = true")
        .bind(&doc_path)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;
    
    // Update document if it exists
    if let Some(doc_row) = doc {
        let doc_id: i32 = doc_row.get("id");
        let mut content: String = doc_row.get("content");
        
        // Update fields in content
        // Handle reports_to as array
        if let Some(reports) = payload.get("reports_to").and_then(|v| v.as_array()) {
            let names: Vec<String> = reports.iter().filter_map(|v| v.as_str().map(String::from)).collect();
            let wikilinks: Vec<String> = names.iter().map(|n| format!("[[{}]]", n)).collect();
            let reports_str = wikilinks.join(", ");
            
            let re = regex::Regex::new(r"\*\*Reports To:\*\* .*").unwrap();
            if re.is_match(&content) {
                let replacement = format!("**Reports To:** {}", reports_str);
                content = re.replace(&content, replacement.as_str()).to_string();
            }
        }
        
        if let Some(mentor) = payload.get("mentor").and_then(|v| v.as_str()) {
            let re = regex::Regex::new(r"\*\*Mentor:\*\* \[\[[^\]]*\]\]").unwrap();
            let replacement = format!("**Mentor:** [[{}]]", mentor);
            content = re.replace(&content, replacement.as_str()).to_string();
        }
        
        // Handle liaises_with in document
        if let Some(liaises) = payload.get("liaises_with").and_then(|v| v.as_array()) {
            let names: Vec<String> = liaises.iter().filter_map(|v| v.as_str().map(String::from)).collect();
            let wikilinks: Vec<String> = names.iter().map(|n| format!("[[{}]]", n)).collect();
            let liaises_str = wikilinks.join(", ");
            
            // Try to update existing Liaises With line
            let re = regex::Regex::new(r"\*\*Liaises With:\*\* .*").unwrap();
            if re.is_match(&content) {
                let replacement = format!("**Liaises With:** {}", liaises_str);
                content = re.replace(&content, replacement.as_str()).to_string();
            } else if !names.is_empty() {
                // Add after Mentor line if it exists
                let insert_re = regex::Regex::new(r"(\*\*Mentor:\*\* \[\[[^\]]*\]\])").unwrap();
                if insert_re.is_match(&content) {
                    let replacement = format!("$1\n**Liaises With:** {}", liaises_str);
                    content = insert_re.replace(&content, replacement.as_str()).to_string();
                }
            }
        }
        
        // Write updated content back
        sqlx::query("UPDATE documents SET content = $1 WHERE id = $2")
            .bind(&content)
            .bind(doc_id)
            .execute(&pool)
            .await
            .map_err(|e| ApiError::Database(e.to_string()))?;
    }
    // Handle reports_to (array)
    if let Some(reports) = payload.get("reports_to").and_then(|v| v.as_array()) {
        let names: Vec<String> = reports.iter().filter_map(|v| v.as_str().map(String::from)).collect();
        // Convert names to IDs
        let mut ids: Vec<String> = vec![];
        for name in names {
            let target = sqlx::query("SELECT id FROM agents WHERE regexp_replace(full_name, ' ', '', 'g') = $1 OR id = $1")
                .bind(&name)
                .fetch_optional(&pool)
                .await
                .map_err(|e| ApiError::Database(e.to_string()))?;
            if let Some(t) = target {
                ids.push(t.get("id"));
            }
        }
        sqlx::query("UPDATE agents SET reports_to = $1 WHERE id = $2")
            .bind(&ids)
            .bind(&id)
            .execute(&pool)
            .await
            .map_err(|e| ApiError::Database(e.to_string()))?;
    }
    
    if let Some(mentor) = payload.get("mentor").and_then(|v| v.as_str()) {
        let target = sqlx::query("SELECT id FROM agents WHERE regexp_replace(full_name, ' ', '', 'g') = $1")
            .bind(mentor)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ApiError::Database(e.to_string()))?;
        if let Some(t) = target {
            let target_id: String = t.get("id");
            sqlx::query("UPDATE agents SET mentor = $1 WHERE id = $2")
                .bind(&target_id)
                .bind(&id)
                .execute(&pool)
                .await
                .map_err(|e| ApiError::Database(e.to_string()))?;
        }
    }
    
    // Handle liaises_with (array)
    if let Some(liaises) = payload.get("liaises_with").and_then(|v| v.as_array()) {
        let names: Vec<String> = liaises.iter().filter_map(|v| v.as_str().map(String::from)).collect();
        // Convert names to IDs
        let mut ids: Vec<String> = vec![];
        for name in names {
            let target = sqlx::query("SELECT id FROM agents WHERE regexp_replace(full_name, ' ', '', 'g') = $1")
                .bind(&name)
                .fetch_optional(&pool)
                .await
                .map_err(|e| ApiError::Database(e.to_string()))?;
            if let Some(t) = target {
                ids.push(t.get("id"));
            }
        }
        sqlx::query("UPDATE agents SET liaises_with = $1 WHERE id = $2")
            .bind(&ids)
            .bind(&id)
            .execute(&pool)
            .await
            .map_err(|e| ApiError::Database(e.to_string()))?;
        
        // Bidirectional: also add this agent to each liaison target
        for target_id in &ids {
            sqlx::query("UPDATE agents SET liaises_with = array_append(COALESCE(liaises_with, ARRAY[]::text[]), $1) WHERE id = $2 AND NOT ($1 = ANY(COALESCE(liaises_with, ARRAY[]::text[])))")
                .bind(&id)
                .bind(target_id)
                .execute(&pool)
                .await
                .map_err(|e| ApiError::Database(e.to_string()))?;
        }
    }

    Ok(Json(serde_json::json!({"success": true, "updated": id, "document": doc_path})))
}


/// GET /api/agents/requests/pending - Get pending request counts per agent
pub async fn pending_requests(State(pool): State<DbPool>) -> Result<Json<Value>, ApiError> {
    let rows = sqlx::query(
        r#"SELECT unnest(to_agent) as agent_id, COUNT(*) as count
           FROM agent_requests ar LEFT JOIN extracted_tasks et ON (et.agent_request_id = ar.id OR lower(et.title) = lower(regexp_replace(ar.subject, '^[^:]+: ', ''))) LEFT JOIN documents d ON d.id = et.source_note_id 
           WHERE ar.status IN ('in_progress', 'decision_needed')
           GROUP BY agent_id"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    let counts: std::collections::HashMap<String, i64> = rows.iter()
        .filter_map(|r| {
            let aid: Option<String> = r.get("agent_id");
            let cnt: i64 = r.get("count");
            aid.map(|a| (a, cnt))
        })
        .collect();

    Ok(Json(serde_json::json!(counts)))
}

/// GET /api/agents/:id/requests - Get requests for specific agent
pub async fn agent_requests(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let rows = sqlx::query(
        r#"SELECT ar.id, ar.from_agent, ar.request_type, ar.subject, ar.context, ar.status, ar.priority, ar.created_at, et.project, d.path as source_doc
           FROM agent_requests ar LEFT JOIN extracted_tasks et ON (et.agent_request_id = ar.id OR lower(et.title) = lower(regexp_replace(ar.subject, '^[^:]+: ', ''))) LEFT JOIN documents d ON d.id = et.source_note_id 
           WHERE $1 = ANY(ar.to_agent) AND ar.status IN ('in_progress', 'decision_needed')
           ORDER BY ar.priority ASC, created_at DESC"#
    )
    .bind(&id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    let requests: Vec<Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.get::<i32, _>("id"),
            "from_agent": r.get::<Option<String>, _>("from_agent"),
            "request_type": r.get::<Option<String>, _>("request_type"),
            "subject": r.get::<Option<String>, _>("subject"),
            "context": r.get::<Option<String>, _>("context"),
            "status": r.get::<Option<String>, _>("status"),
            "priority": r.get::<Option<i32>, _>("priority"),
            "created_at": r.get::<Option<chrono::DateTime<chrono::Utc>>, _>("created_at"),
            "project": r.get::<Option<String>, _>("project"),
            "source_doc": r.get::<Option<String>, _>("source_doc"),
        })
    }).collect();

    Ok(Json(serde_json::json!(requests)))
}


/// PUT /api/agents/requests/:id - Update request status/response
pub async fn update_request(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    let status = payload.get("status").and_then(|v| v.as_str());
    let response = payload.get("response").and_then(|v| v.as_str());
    let to_agent = payload.get("to_agent").and_then(|v| v.as_str());
    
    // Build dynamic update query
    let mut updates = vec![];
    let mut param_idx = 1;
    
    if status.is_some() {
        param_idx += 1;
        updates.push(format!("status = ${}", param_idx));
        if status == Some("resolved") {
            updates.push("resolved_at = NOW()".to_string());
        }
    }
    if response.is_some() {
        param_idx += 1;
        updates.push(format!("response = COALESCE(response, '') || E'\\n---\\n' || ${}", param_idx));
    }
    if to_agent.is_some() {
        param_idx += 1;
        updates.push(format!("to_agent = ARRAY[${0}]", param_idx));
    }
    
    if updates.is_empty() {
        return Err(ApiError::BadRequest("No valid fields to update".to_string()));
    }
    
    let query = format!("UPDATE agent_requests SET {} WHERE id = $1 RETURNING id, status", updates.join(", "));
    
    let mut q = sqlx::query(&query).bind(id);
    if let Some(s) = status {
        q = q.bind(s);
    }
    if let Some(r) = response {
        q = q.bind(r);
    }
    if let Some(t) = to_agent {
        q = q.bind(t);
    }
    
    let row = q.fetch_one(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;
    
    let new_status: String = row.get("status");
    
    Ok(Json(serde_json::json!({
        "success": true,
        "id": id,
        "status": new_status
    })))
}
