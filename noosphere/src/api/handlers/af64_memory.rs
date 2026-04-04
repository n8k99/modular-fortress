//! AF64 Agent Daily Memory endpoints

use axum::{
    extract::{Query, State},
    Json,
};
use dpn_core::DbPool;
use serde::Deserialize;
use serde_json::Value;
use sqlx::Row;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct MemoryUpdate {
    pub agent_id: String,
    pub log_date: Option<String>,
    pub actions_taken: Option<String>,
    pub decisions_made: Option<String>,
    pub knowledge_gained: Option<String>,
    pub blockers: Option<String>,
    pub handoffs: Option<String>,
    pub plan_tomorrow: Option<String>,
}

#[derive(Deserialize)]
pub struct MemoryQuery {
    pub agent_id: Option<String>,
    pub date: Option<String>,
    pub department: Option<String>,
    pub days: Option<i32>,
}

/// PUT /api/agents/memory — upsert daily memory for an agent
pub async fn upsert_memory(
    State(pool): State<DbPool>,
    Json(body): Json<MemoryUpdate>,
) -> Result<Json<Value>, ApiError> {
    let date = body.log_date.unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string());
    
    sqlx::query(
        "INSERT INTO agent_daily_memory (agent_id, log_date, actions_taken, decisions_made, knowledge_gained, blockers, handoffs, plan_tomorrow, updated_at)
         VALUES ($1, $2::date, $3, $4, $5, $6, $7, $8, NOW())
         ON CONFLICT (agent_id, log_date) DO UPDATE SET
           actions_taken = CASE WHEN EXCLUDED.actions_taken IS NOT NULL THEN
             COALESCE(agent_daily_memory.actions_taken || E'\n', '') || EXCLUDED.actions_taken
             ELSE agent_daily_memory.actions_taken END,
           decisions_made = CASE WHEN EXCLUDED.decisions_made IS NOT NULL THEN
             COALESCE(agent_daily_memory.decisions_made || E'\n', '') || EXCLUDED.decisions_made
             ELSE agent_daily_memory.decisions_made END,
           knowledge_gained = CASE WHEN EXCLUDED.knowledge_gained IS NOT NULL THEN
             COALESCE(agent_daily_memory.knowledge_gained || E'\n', '') || EXCLUDED.knowledge_gained
             ELSE agent_daily_memory.knowledge_gained END,
           blockers = CASE WHEN EXCLUDED.blockers IS NOT NULL THEN
             COALESCE(agent_daily_memory.blockers || E'\n', '') || EXCLUDED.blockers
             ELSE agent_daily_memory.blockers END,
           handoffs = CASE WHEN EXCLUDED.handoffs IS NOT NULL THEN
             COALESCE(agent_daily_memory.handoffs || E'\n', '') || EXCLUDED.handoffs
             ELSE agent_daily_memory.handoffs END,
           plan_tomorrow = COALESCE(EXCLUDED.plan_tomorrow, agent_daily_memory.plan_tomorrow),
           updated_at = NOW()"
    )
    .bind(&body.agent_id)
    .bind(&date)
    .bind(&body.actions_taken)
    .bind(&body.decisions_made)
    .bind(&body.knowledge_gained)
    .bind(&body.blockers)
    .bind(&body.handoffs)
    .bind(&body.plan_tomorrow)
    .execute(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;
    
    Ok(Json(serde_json::json!({ "ok": true, "agent_id": body.agent_id, "date": date })))
}

/// GET /api/agents/memory — query daily memory
pub async fn get_memory(
    State(pool): State<DbPool>,
    Query(q): Query<MemoryQuery>,
) -> Result<Json<Value>, ApiError> {
    let days = q.days.unwrap_or(1);
    let interval = format!("{} days", days);
    
    let rows = if let Some(ref agent_id) = q.agent_id {
        if let Some(ref date) = q.date {
            sqlx::query(
                "SELECT m.id, m.agent_id, m.log_date::text, m.actions_taken, m.decisions_made,
                        m.knowledge_gained, m.blockers, m.handoffs, m.plan_tomorrow, m.daily_summary
                 FROM agent_daily_memory m
                 WHERE m.agent_id = $1 AND m.log_date = $2::date
                 ORDER BY m.log_date DESC"
            )
            .bind(agent_id)
            .bind(date)
            .fetch_all(&pool).await
        } else {
            sqlx::query(
                "SELECT m.id, m.agent_id, m.log_date::text, m.actions_taken, m.decisions_made,
                        m.knowledge_gained, m.blockers, m.handoffs, m.plan_tomorrow, m.daily_summary
                 FROM agent_daily_memory m
                 WHERE m.agent_id = $1 AND m.log_date >= CURRENT_DATE - $2::interval
                 ORDER BY m.log_date DESC"
            )
            .bind(agent_id)
            .bind(&interval)
            .fetch_all(&pool).await
        }
    } else if let Some(ref dept) = q.department {
        sqlx::query(
            "SELECT m.id, m.agent_id, m.log_date::text, m.actions_taken, m.decisions_made,
                    m.knowledge_gained, m.blockers, m.handoffs, m.plan_tomorrow, m.daily_summary
             FROM agent_daily_memory m
             JOIN agents a ON a.id = m.agent_id
             WHERE a.department = $1 AND m.log_date >= CURRENT_DATE - $2::interval
             ORDER BY m.log_date DESC, m.agent_id"
        )
        .bind(dept)
        .bind(&interval)
        .fetch_all(&pool).await
    } else {
        sqlx::query(
            "SELECT m.id, m.agent_id, m.log_date::text, m.actions_taken, m.decisions_made,
                    m.knowledge_gained, m.blockers, m.handoffs, m.plan_tomorrow, m.daily_summary
             FROM agent_daily_memory m
             WHERE m.log_date >= CURRENT_DATE - $1::interval
             ORDER BY m.log_date DESC, m.agent_id"
        )
        .bind(&interval)
        .fetch_all(&pool).await
    }.map_err(|e| ApiError::Database(e.to_string()))?;
    
    let memories: Vec<Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.get::<i64, _>("id"),
            "agent_id": r.get::<String, _>("agent_id"),
            "log_date": r.get::<String, _>("log_date"),
            "actions_taken": r.get::<Option<String>, _>("actions_taken"),
            "decisions_made": r.get::<Option<String>, _>("decisions_made"),
            "knowledge_gained": r.get::<Option<String>, _>("knowledge_gained"),
            "blockers": r.get::<Option<String>, _>("blockers"),
            "handoffs": r.get::<Option<String>, _>("handoffs"),
            "plan_tomorrow": r.get::<Option<String>, _>("plan_tomorrow"),
            "daily_summary": r.get::<Option<String>, _>("daily_summary"),
        })
    }).collect();
    
    Ok(Json(serde_json::json!({ "memories": memories, "count": memories.len() })))
}
