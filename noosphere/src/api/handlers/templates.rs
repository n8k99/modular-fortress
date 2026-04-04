//! Template endpoints (PARAT pillar)
//!
//! Templates store .dpn expressions with version history.
//! DB trigger handles version history automatically on body changes (D-08).

use axum::{
    extract::{Path, State},
    Json,
};
use dpn_core::{DbPool, Template, TemplateHistory};
use serde::Deserialize;
use serde_json::Value;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub slug: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub body: String,
    pub parameters: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub parameters: Option<serde_json::Value>,
}

pub async fn list_templates(
    State(pool): State<DbPool>,
) -> Result<Json<Value>, ApiError> {
    let templates = dpn_core::list_templates(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "templates": templates })))
}

pub async fn get_template(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let template = dpn_core::get_template_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Template {} not found", id)))?;

    Ok(Json(serde_json::json!(template)))
}

pub async fn create_template(
    State(pool): State<DbPool>,
    Json(req): Json<CreateTemplateRequest>,
) -> Result<Json<Value>, ApiError> {
    let id = dpn_core::create_template(
        &pool,
        &req.name,
        &req.slug,
        req.category.as_deref(),
        req.description.as_deref(),
        &req.body,
        req.parameters.as_ref(),
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

pub async fn update_template(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateTemplateRequest>,
) -> Result<Json<Value>, ApiError> {
    // Verify template exists
    dpn_core::get_template_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Template {} not found", id)))?;

    dpn_core::update_template(
        &pool,
        id,
        req.name.as_deref(),
        req.category.as_deref(),
        req.description.as_deref(),
        req.body.as_deref(),
        req.parameters.as_ref(),
    )
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    // Fetch updated template
    let template = dpn_core::get_template_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Template {} not found", id)))?;

    Ok(Json(serde_json::json!(template)))
}

pub async fn get_template_history(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let history = dpn_core::get_template_history(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "history": history })))
}
