//! Project endpoints

use axum::{
    extract::{Path, State},
    Json,
};
use dpn_core::{DbPool, Project};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::ApiError;

#[derive(Deserialize, Serialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub slug: String,
    #[serde(default = "default_status")]
    pub status: String,
    pub description: Option<String>,
    pub owner: Option<String>,
    #[serde(default = "default_lifestage")]
    pub lifestage: String,
    pub area_id: Option<i32>,
}

fn default_status() -> String {
    "active".to_string()
}

fn default_lifestage() -> String {
    "Seed".to_string()
}

pub async fn list_projects(
    State(pool): State<DbPool>,
) -> Result<Json<Value>, ApiError> {
    let projects = dpn_core::list_projects(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "projects": projects })))
}

pub async fn get_project(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let project = dpn_core::get_project_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Project {} not found", id)))?;

    Ok(Json(serde_json::json!(project)))
}

pub async fn create_project(
    State(pool): State<DbPool>,
    Json(req): Json<CreateProjectRequest>,
) -> Result<Json<Value>, ApiError> {
    let id = dpn_core::create_project(
        &pool,
        &req.name,
        &req.slug,
        &req.status,
        req.description.as_deref(),
        req.owner.as_deref(),
        &req.lifestage,
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

#[derive(Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub schedule: Option<serde_json::Value>,
    pub lifestage: Option<String>,
    pub area_id: Option<i32>,
}

pub async fn update_project(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateProjectRequest>,
) -> Result<Json<Value>, ApiError> {
    // Verify project exists
    dpn_core::get_project_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Project {} not found", id)))?;

    dpn_core::update_project(
        &pool,
        id,
        req.name.as_deref(),
        req.description.as_deref(),
        req.owner.as_deref(),
        req.status.as_deref(),
        req.schedule.as_ref(),
        req.lifestage.as_deref(),
        req.area_id,
    )
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    // Fetch updated project
    let project = dpn_core::get_project_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Project {} not found", id)))?;

    Ok(Json(serde_json::json!(project)))
}
