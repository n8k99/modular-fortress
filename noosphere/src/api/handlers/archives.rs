//! Archive endpoints (PARAT pillar)
//!
//! Archives are immutable records -- only metadata (topic, tags, metadata) can be updated (D-04).

use axum::{
    extract::{Path, Query, State},
    Json,
};
use dpn_core::{DbPool, Archive};
use serde::Deserialize;
use serde_json::Value;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct CreateArchiveRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub source_type: String,
    pub source_id: Option<i32>,
    pub original_path: Option<String>,
    pub period_start: Option<chrono::NaiveDate>,
    pub period_end: Option<chrono::NaiveDate>,
    pub topic: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct UpdateArchiveMetadataRequest {
    pub topic: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct SearchArchivesQuery {
    pub q: String,
}

pub async fn list_archives(
    State(pool): State<DbPool>,
) -> Result<Json<Value>, ApiError> {
    let archives = dpn_core::list_archives(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "archives": archives })))
}

pub async fn get_archive(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let archive = dpn_core::get_archive_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Archive {} not found", id)))?;

    Ok(Json(serde_json::json!(archive)))
}

pub async fn create_archive(
    State(pool): State<DbPool>,
    Json(req): Json<CreateArchiveRequest>,
) -> Result<Json<Value>, ApiError> {
    let id = dpn_core::create_archive(
        &pool,
        req.title.as_deref(),
        req.content.as_deref(),
        &req.source_type,
        req.source_id,
        req.original_path.as_deref(),
        req.period_start,
        req.period_end,
        req.topic.as_deref(),
        req.tags.as_ref(),
        req.metadata.as_ref(),
    )
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "status": "created",
        "id": id
    })))
}

pub async fn update_archive_metadata(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateArchiveMetadataRequest>,
) -> Result<Json<Value>, ApiError> {
    // Verify archive exists
    dpn_core::get_archive_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Archive {} not found", id)))?;

    dpn_core::update_archive_metadata(
        &pool,
        id,
        req.topic.as_deref(),
        req.tags.as_ref(),
        req.metadata.as_ref(),
    )
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    // Fetch updated archive
    let archive = dpn_core::get_archive_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Archive {} not found", id)))?;

    Ok(Json(serde_json::json!(archive)))
}

pub async fn search_archives(
    State(pool): State<DbPool>,
    Query(params): Query<SearchArchivesQuery>,
) -> Result<Json<Value>, ApiError> {
    let archives = dpn_core::search_archives(&pool, &params.q)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "archives": archives })))
}
