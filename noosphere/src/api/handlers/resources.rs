//! Resource endpoints (PARAT pillar)
//!
//! Resources are curated references. Frozen resources return 409 on update attempts (D-05).

use axum::{
    extract::{Path, State},
    Json,
};
use dpn_core::{DbPool, Resource};
use serde::Deserialize;
use serde_json::Value;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct CreateResourceRequest {
    pub name: String,
    pub slug: String,
    pub resource_type: String,
    pub source_type: String,
    pub source_id: i32,
    pub description: Option<String>,
    pub tags: Option<serde_json::Value>,
    #[serde(default)]
    pub frozen: bool,
    pub area_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateResourceRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub resource_type: Option<String>,
    pub area_id: Option<i32>,
}

pub async fn list_resources(
    State(pool): State<DbPool>,
) -> Result<Json<Value>, ApiError> {
    let resources = dpn_core::list_resources(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "resources": resources })))
}

pub async fn get_resource(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let resource = dpn_core::get_resource_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Resource {} not found", id)))?;

    Ok(Json(serde_json::json!(resource)))
}

pub async fn create_resource(
    State(pool): State<DbPool>,
    Json(req): Json<CreateResourceRequest>,
) -> Result<Json<Value>, ApiError> {
    let id = dpn_core::create_resource(
        &pool,
        &req.name,
        &req.slug,
        &req.resource_type,
        &req.source_type,
        req.source_id,
        req.description.as_deref(),
        req.tags.as_ref(),
        req.frozen,
        req.area_id,
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

pub async fn update_resource(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateResourceRequest>,
) -> Result<Json<Value>, ApiError> {
    // Fetch resource and check frozen status (D-05)
    let resource = dpn_core::get_resource_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Resource {} not found", id)))?;

    if resource.frozen {
        return Err(ApiError::Conflict("Resource is frozen and cannot be updated".to_string()));
    }

    dpn_core::update_resource(
        &pool,
        id,
        req.name.as_deref(),
        req.description.as_deref(),
        req.tags.as_ref(),
        req.resource_type.as_deref(),
        req.area_id,
    )
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    // Fetch updated resource
    let resource = dpn_core::get_resource_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Resource {} not found", id)))?;

    Ok(Json(serde_json::json!(resource)))
}
