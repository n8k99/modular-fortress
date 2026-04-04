//! Event endpoints

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDate;
use dpn_core::{DbPool, Event};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct ListEventsParams {
    #[serde(default = "default_days")]
    days: i32,
}

fn default_days() -> i32 { 14 }

pub async fn list_events(
    State(pool): State<DbPool>,
    Query(params): Query<ListEventsParams>,
) -> Result<Json<Value>, ApiError> {
    let events = dpn_core::get_upcoming_events(&pool, params.days)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "events": events })))
}

pub async fn get_event(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let event = dpn_core::get_event_by_id(&pool, &id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Event {} not found", id)))?;

    Ok(Json(serde_json::json!(event)))
}

pub async fn create_event(
    State(pool): State<DbPool>,
    Json(event): Json<Event>,
) -> Result<Json<Value>, ApiError> {
    dpn_core::upsert_event(&pool, &event)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "status": "created", "id": event.id })))
}

pub async fn update_event(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
    Json(mut event): Json<Event>,
) -> Result<Json<Value>, ApiError> {
    event.id = id.clone();
    dpn_core::upsert_event(&pool, &event)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "status": "updated", "id": id })))
}

pub async fn delete_event(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    dpn_core::delete_event(&pool, &id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "status": "deleted", "id": id })))
}

#[derive(Deserialize)]
pub struct SyncEventsRequest {
    pub doc_path: String,
    pub content: String,
}

/// Sync events from a Weekly Note - parse content and upsert events
pub async fn sync_events(
    State(pool): State<DbPool>,
    Json(req): Json<SyncEventsRequest>,
) -> Result<Json<Value>, ApiError> {
    // Check if this is a Weekly Note
    if !dpn_core::is_weekly_note(&req.doc_path) {
        return Err(ApiError::BadRequest("Not a Weekly Note document".to_string()));
    }

    // Parse events from content
    let parsed_events = dpn_core::parse_events(&req.content);

    // Upsert each event (replace by id)
    for parsed_event in &parsed_events {
        // Convert ParsedEvent to Event for database
        let event = Event {
            id: parsed_event.id.clone(),
            icon: parsed_event.icon.clone(),
            event_type: format!("{:?}", parsed_event.event_type).to_lowercase(),
            title: parsed_event.title.clone(),
            date: parsed_event.date.clone(),
            time: parsed_event.time.clone(),
            duration: parsed_event.duration.clone(),
            daily_note: parsed_event.daily_note.clone(),
            tags: parsed_event.tags.as_ref().map(|t| serde_json::to_string(t).unwrap_or_default()),
        };

        dpn_core::upsert_event(&pool, &event)
            .await
            .map_err(|e| ApiError::Database(e.to_string()))?;
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "count": parsed_events.len()
    })))
}
