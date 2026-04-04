//! Area endpoints (PARAT pillar)

use axum::{
    extract::{Path, State},
    Json,
};
use dpn_core::{DbPool, Area};
use serde::Deserialize;
use serde_json::Value;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct CreateAreaRequest {
    pub name: String,
    pub slug: String,
    #[serde(default = "default_status")]
    pub status: String,
    pub description: Option<String>,
    pub owner: Option<String>,
}

fn default_status() -> String {
    "active".to_string()
}

#[derive(Deserialize)]
pub struct UpdateAreaRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub status: Option<String>,
}

pub async fn list_areas(
    State(pool): State<DbPool>,
) -> Result<Json<Value>, ApiError> {
    let areas = dpn_core::list_areas(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "areas": areas })))
}

pub async fn get_area(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let area = dpn_core::get_area_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Area {} not found", id)))?;

    Ok(Json(serde_json::json!(area)))
}

pub async fn create_area(
    State(pool): State<DbPool>,
    Json(req): Json<CreateAreaRequest>,
) -> Result<Json<Value>, ApiError> {
    let id = dpn_core::create_area(
        &pool,
        &req.name,
        &req.slug,
        req.description.as_deref(),
        req.owner.as_deref(),
        &req.status,
    )
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "status": "created",
        "id": id,
        "name": req.name,
        "slug": req.slug
    })))
}

pub async fn update_area(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateAreaRequest>,
) -> Result<Json<Value>, ApiError> {
    // Verify area exists
    dpn_core::get_area_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Area {} not found", id)))?;

    dpn_core::update_area(
        &pool,
        id,
        req.name.as_deref(),
        req.description.as_deref(),
        req.owner.as_deref(),
        req.status.as_deref(),
    )
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    // Fetch updated area
    let area = dpn_core::get_area_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Area {} not found", id)))?;

    Ok(Json(serde_json::json!(area)))
}
