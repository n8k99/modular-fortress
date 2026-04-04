//! AF64 Tick Log endpoints

use axum::{
    extract::State,
    Json,
};
use dpn_core::DbPool;
use serde::Deserialize;
use serde_json::Value;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct TickLogEntry {
    pub tick_number: i32,
    pub agent_id: String,
    pub action_taken: String,
    pub action_detail: Option<Value>,
    pub energy_before: f64,
    pub energy_after: f64,
    pub tier: Option<String>,
    pub model_used: Option<String>,
    pub llm_called: Option<bool>,
}

/// POST /api/tick-log
pub async fn create_tick_log(
    State(pool): State<DbPool>,
    Json(body): Json<TickLogEntry>,
) -> Result<Json<Value>, ApiError> {
    let detail = body.action_detail.map(|d| d.to_string()).unwrap_or_else(|| "{}".into());
    let llm = body.llm_called.unwrap_or(false);

    sqlx::query(
        "INSERT INTO tick_log (tick_number, agent_id, action_taken, action_detail, energy_before, energy_after, tier, model_used, llm_called) VALUES ($1, $2, $3, $4::jsonb, $5, $6, $7, $8, $9)"
    )
    .bind(body.tick_number).bind(&body.agent_id).bind(&body.action_taken).bind(&detail)
    .bind(body.energy_before).bind(body.energy_after).bind(&body.tier).bind(&body.model_used).bind(llm)
    .execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({"ok": true})))
}

#[derive(Deserialize)]
pub struct BatchTickLog {
    pub entries: Vec<TickLogEntry>,
}

/// POST /api/tick-log/batch
pub async fn create_tick_log_batch(
    State(pool): State<DbPool>,
    Json(body): Json<BatchTickLog>,
) -> Result<Json<Value>, ApiError> {
    let mut count = 0;
    for entry in &body.entries {
        let detail = entry.action_detail.as_ref().map(|d| d.to_string()).unwrap_or_else(|| "{}".into());
        let llm = entry.llm_called.unwrap_or(false);

        sqlx::query(
            "INSERT INTO tick_log (tick_number, agent_id, action_taken, action_detail, energy_before, energy_after, tier, model_used, llm_called) VALUES ($1, $2, $3, $4::jsonb, $5, $6, $7, $8, $9)"
        )
        .bind(entry.tick_number).bind(&entry.agent_id).bind(&entry.action_taken).bind(&detail)
        .bind(entry.energy_before).bind(entry.energy_after).bind(&entry.tier).bind(&entry.model_used).bind(llm)
        .execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
        count += 1;
    }

    Ok(Json(serde_json::json!({"ok": true, "count": count})))
}
