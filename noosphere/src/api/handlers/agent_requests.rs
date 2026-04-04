//! Agent requests endpoints for inter-agent coordination

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use dpn_core::DbPool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx;

use crate::error::ApiError;

#[derive(Serialize, sqlx::FromRow)]
pub struct AgentRequest {
    pub id: i32,
    pub from_agent: String,
    pub to_agent: String,
    pub request_type: Option<String>,
    pub subject: Option<String>,
    pub context: Option<String>,
    pub status: Option<String>,
    pub priority: Option<i32>,
    pub response: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct ListQuery {
    /// Filter by target agent
    pub agent: Option<String>,
    /// Filter by request type
    #[serde(rename = "type")]
    pub request_type: Option<String>,
    /// Include non-pending statuses (default: only pending)
    #[serde(default)]
    pub all: bool,
}

#[derive(Deserialize)]
pub struct CreateRequest {
    pub from_agent: String,
    pub to_agent: String,
    pub request_type: Option<String>,
    pub subject: Option<String>,
    pub context: Option<String>,
    #[serde(default = "default_priority")]
    pub priority: i32,
}

fn default_priority() -> i32 {
    3
}

#[derive(Deserialize)]
pub struct UpdateRequest {
    pub status: Option<String>,
    pub response: Option<String>,
}

/// GET /api/agent-requests - List requests (default: pending only)
pub async fn list_requests(
    State(pool): State<DbPool>,
    Query(params): Query<ListQuery>,
) -> Result<Json<Value>, ApiError> {
    let mut query = String::from(
        "SELECT id, from_agent, to_agent, request_type, subject, context, status, priority, response, created_at, resolved_at 
         FROM agent_requests WHERE 1=1"
    );
    
    if !params.all {
        query.push_str(" AND (status = 'pending' OR status IS NULL)");
    }
    
    if params.agent.is_some() {
        query.push_str(" AND to_agent = $1");
    }
    
    if params.request_type.is_some() {
        if params.agent.is_some() {
            query.push_str(" AND request_type = $2");
        } else {
            query.push_str(" AND request_type = $1");
        }
    }
    
    query.push_str(" ORDER BY priority ASC, created_at ASC");

    // Build query based on parameters
    let requests: Vec<AgentRequest> = match (&params.agent, &params.request_type) {
        (Some(agent), Some(req_type)) => {
            sqlx::query_as(&query)
                .bind(agent)
                .bind(req_type)
                .fetch_all(&pool)
                .await
        }
        (Some(agent), None) => {
            sqlx::query_as(&query)
                .bind(agent)
                .fetch_all(&pool)
                .await
        }
        (None, Some(req_type)) => {
            sqlx::query_as(&query)
                .bind(req_type)
                .fetch_all(&pool)
                .await
        }
        (None, None) => {
            sqlx::query_as(&query)
                .fetch_all(&pool)
                .await
        }
    }.map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "requests": requests,
        "count": requests.len()
    })))
}

/// GET /api/agent-requests/:id - Get single request
pub async fn get_request(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<AgentRequest>, ApiError> {
    let request: AgentRequest = sqlx::query_as(
        "SELECT id, from_agent, to_agent, request_type, subject, context, status, priority, response, created_at, resolved_at 
         FROM agent_requests WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?
    .ok_or_else(|| ApiError::NotFound(format!("Request {} not found", id)))?;

    Ok(Json(request))
}

/// POST /api/agent-requests - Create new request
pub async fn create_request(
    State(pool): State<DbPool>,
    Json(req): Json<CreateRequest>,
) -> Result<Json<Value>, ApiError> {
    let result: AgentRequest = sqlx::query_as(
        "INSERT INTO agent_requests (from_agent, to_agent, request_type, subject, context, priority)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, from_agent, to_agent, request_type, subject, context, status, priority, response, created_at, resolved_at"
    )
    .bind(&req.from_agent)
    .bind(&req.to_agent)
    .bind(&req.request_type)
    .bind(&req.subject)
    .bind(&req.context)
    .bind(req.priority)
    .fetch_one(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "request": result
    })))
}

/// PUT /api/agent-requests/:id - Update request (status, response)
pub async fn update_request(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateRequest>,
) -> Result<Json<Value>, ApiError> {
    // Check request exists
    let existing: Option<(i32,)> = sqlx::query_as("SELECT id FROM agent_requests WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    if existing.is_none() {
        return Err(ApiError::NotFound(format!("Request {} not found", id)));
    }

    // Build update - always update both fields, set resolved_at when completing
    let result: AgentRequest = match (&req.status, &req.response) {
        (Some(status), Some(response)) => {
            let resolved = if status == "completed" || status == "cancelled" {
                "resolved_at = NOW(),"
            } else { "" };
            
            let query = format!(
                "UPDATE agent_requests SET status = $2, response = $3, {} updated_at = NOW() 
                 WHERE id = $1
                 RETURNING id, from_agent, to_agent, request_type, subject, context, status, priority, response, created_at, resolved_at",
                resolved
            );
            // Can't use dynamic query with resolved_at, do it simpler
            if status == "completed" || status == "cancelled" {
                sqlx::query_as(
                    "UPDATE agent_requests SET status = $2, response = $3, resolved_at = NOW()
                     WHERE id = $1
                     RETURNING id, from_agent, to_agent, request_type, subject, context, status, priority, response, created_at, resolved_at"
                )
                .bind(id)
                .bind(status)
                .bind(response)
                .fetch_one(&pool)
                .await
            } else {
                sqlx::query_as(
                    "UPDATE agent_requests SET status = $2, response = $3
                     WHERE id = $1
                     RETURNING id, from_agent, to_agent, request_type, subject, context, status, priority, response, created_at, resolved_at"
                )
                .bind(id)
                .bind(status)
                .bind(response)
                .fetch_one(&pool)
                .await
            }
        }
        (Some(status), None) => {
            if status == "completed" || status == "cancelled" {
                sqlx::query_as(
                    "UPDATE agent_requests SET status = $2, resolved_at = NOW()
                     WHERE id = $1
                     RETURNING id, from_agent, to_agent, request_type, subject, context, status, priority, response, created_at, resolved_at"
                )
                .bind(id)
                .bind(status)
                .fetch_one(&pool)
                .await
            } else {
                sqlx::query_as(
                    "UPDATE agent_requests SET status = $2
                     WHERE id = $1
                     RETURNING id, from_agent, to_agent, request_type, subject, context, status, priority, response, created_at, resolved_at"
                )
                .bind(id)
                .bind(status)
                .fetch_one(&pool)
                .await
            }
        }
        (None, Some(response)) => {
            sqlx::query_as(
                "UPDATE agent_requests SET response = $2
                 WHERE id = $1
                 RETURNING id, from_agent, to_agent, request_type, subject, context, status, priority, response, created_at, resolved_at"
            )
            .bind(id)
            .bind(response)
            .fetch_one(&pool)
            .await
        }
        (None, None) => {
            // No updates provided, just return the current state
            sqlx::query_as(
                "SELECT id, from_agent, to_agent, request_type, subject, context, status, priority, response, created_at, resolved_at
                 FROM agent_requests WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&pool)
            .await
        }
    }.map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "request": result
    })))
}

/// DELETE /api/agent-requests/:id - Delete/cancel request
pub async fn delete_request(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let result = sqlx::query("DELETE FROM agent_requests WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("Request {} not found", id)));
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "deleted_id": id
    })))
}
