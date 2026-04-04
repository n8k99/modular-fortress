//! Timeline endpoints - unified chronological view across data sources

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::NaiveDate;
use dpn_core::DbPool;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct TimelineParams {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

/// GET /api/timeline?start=YYYY-MM-DD&end=YYYY-MM-DD
/// Returns timeline entries from all sources (memories, stagehand_notes, daily_logs, memory_entries)
pub async fn get_timeline_entries(
    State(pool): State<DbPool>,
    Query(params): Query<TimelineParams>,
) -> Result<Json<Value>, ApiError> {
    let entries = dpn_core::load_timeline_entries(&pool, params.start, params.end)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "entries": entries })))
}

/// GET /api/timeline/counts?start=YYYY-MM-DD&end=YYYY-MM-DD
/// Returns activity counts per day for heatmap visualization
pub async fn get_activity_counts(
    State(pool): State<DbPool>,
    Query(params): Query<TimelineParams>,
) -> Result<Json<Value>, ApiError> {
    let counts = dpn_core::get_activity_counts(&pool, params.start, params.end)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "counts": counts })))
}
