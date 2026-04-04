//! AF64 Drive management endpoints

use axum::{
    extract::{Path, State},
    Json,
};
use dpn_core::DbPool;
use serde::Deserialize;
use serde_json::Value;

use crate::error::ApiError;

/// POST /api/drives/tick — decay all drives
pub async fn tick_all_drives(
    State(pool): State<DbPool>,
) -> Result<Json<Value>, ApiError> {
    let result = sqlx::query(
        r#"UPDATE agent_drives d
           SET satisfaction = GREATEST(0, d.satisfaction - d.decay_rate),
               pressure = LEAST(100, 100 - GREATEST(0, d.satisfaction - d.decay_rate)),
               frustration = CASE
                   WHEN (100 - GREATEST(0, d.satisfaction - d.decay_rate)) > 70
                        AND COALESCE((SELECT s.energy FROM agent_state s WHERE s.agent_id = d.agent_id), 50) < 20
                   THEN d.frustration + 1
                   ELSE d.frustration
               END"#
    )
    .execute(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({"ok": true, "rows_affected": result.rows_affected()})))
}

#[derive(Deserialize)]
pub struct FulfillDrive {
    pub drive_name: String,
    pub amount: f64,
}

/// POST /api/drives/:agent_id/fulfill
pub async fn fulfill_drive(
    State(pool): State<DbPool>,
    Path(agent_id): Path<String>,
    Json(body): Json<FulfillDrive>,
) -> Result<Json<Value>, ApiError> {
    sqlx::query(
        "UPDATE agent_drives SET satisfaction = LEAST(100, satisfaction + $1), pressure = GREATEST(0, 100 - LEAST(100, satisfaction + $1)), frustration = 0 WHERE agent_id = $2 AND drive_name = $3"
    )
    .bind(body.amount).bind(&agent_id).bind(&body.drive_name)
    .execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({"ok": true})))
}
