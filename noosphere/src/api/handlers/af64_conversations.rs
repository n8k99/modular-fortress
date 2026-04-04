//! AF64 Conversation endpoints

use axum::{
    extract::{Query, State},
    Json,
};
use dpn_core::DbPool;
use serde::Deserialize;
use serde_json::Value;
use sqlx::Row;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct ConversationQuery {
    pub to_agent: Option<String>,
    pub since: Option<String>,
    pub limit: Option<i64>,
    pub channel: Option<String>,
    pub thread_id: Option<String>,
}

/// GET /api/conversations
pub async fn list_conversations(
    State(pool): State<DbPool>,
    Query(q): Query<ConversationQuery>,
) -> Result<Json<Value>, ApiError> {
    let limit = q.limit.unwrap_or(20).min(100);
    let since = q.since.as_deref().unwrap_or("1970-01-01T00:00:00Z");

    let rows = if let Some(ref to_agent) = q.to_agent {
        sqlx::query(
            "SELECT id, from_agent, to_agent, message, channel, message_type, thread_id, metadata, created_at FROM conversations WHERE $1 = ANY(to_agent) AND created_at > $2::timestamptz ORDER BY created_at DESC LIMIT $3"
        )
        .bind(to_agent).bind(since).bind(limit)
        .fetch_all(&pool).await
    } else if let Some(ref channel) = q.channel {
        if let Some(ref thread_id) = q.thread_id {
            let tid: uuid::Uuid = thread_id.parse().map_err(|_| ApiError::BadRequest("Invalid UUID".into()))?;
            sqlx::query(
                "SELECT id, from_agent, to_agent, message, channel, message_type, thread_id, metadata, created_at FROM conversations WHERE channel = $1 AND thread_id = $2 AND created_at > $3::timestamptz ORDER BY created_at DESC LIMIT $4"
            )
            .bind(channel).bind(tid).bind(since).bind(limit)
            .fetch_all(&pool).await
        } else {
            sqlx::query(
                "SELECT id, from_agent, to_agent, message, channel, message_type, thread_id, metadata, created_at FROM conversations WHERE channel = $1 AND created_at > $2::timestamptz ORDER BY created_at DESC LIMIT $3"
            )
            .bind(channel).bind(since).bind(limit)
            .fetch_all(&pool).await
        }
    } else {
        sqlx::query(
            "SELECT id, from_agent, to_agent, message, channel, message_type, thread_id, metadata, created_at FROM conversations WHERE created_at > $1::timestamptz ORDER BY created_at DESC LIMIT $2"
        )
        .bind(since).bind(limit)
        .fetch_all(&pool).await
    };

    let rows = rows.map_err(|e| ApiError::Database(e.to_string()))?;
    let convos: Vec<Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.get::<i32, _>("id"),
            "from_agent": r.get::<String, _>("from_agent"),
            "to_agent": r.get::<Option<Vec<String>>, _>("to_agent"),
            "message": r.get::<String, _>("message"),
            "channel": r.get::<Option<String>, _>("channel"),
            "message_type": r.get::<Option<String>, _>("message_type"),
            "thread_id": r.get::<Option<uuid::Uuid>, _>("thread_id").map(|u| u.to_string()),
            "metadata": r.get::<Option<Value>, _>("metadata"),
        })
    }).collect();

    Ok(Json(serde_json::json!(convos)))
}

#[derive(Deserialize)]
pub struct NewConversation {
    pub from_agent: String,
    pub to_agent: Vec<String>,
    pub message: String,
    pub channel: Option<String>,
    pub thread_id: Option<String>,
    pub metadata: Option<Value>,
}

/// POST /api/conversations
pub async fn create_conversation(
    State(pool): State<DbPool>,
    Json(body): Json<NewConversation>,
) -> Result<Json<Value>, ApiError> {
    let channel = body.channel.unwrap_or_else(|| "noosphere".into());
    let thread_id: Option<uuid::Uuid> = body.thread_id.as_deref().and_then(|t| t.parse().ok());
    let metadata_str = body.metadata.map(|m| m.to_string());

    let row = sqlx::query(
        "INSERT INTO conversations (from_agent, to_agent, message, channel, message_type, thread_id, metadata) VALUES ($1, $2, $3, $4, 'chat', $5, $6::jsonb) RETURNING id"
    )
    .bind(&body.from_agent)
    .bind(&body.to_agent)
    .bind(&body.message)
    .bind(&channel)
    .bind(thread_id)
    .bind(metadata_str)
    .fetch_one(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({"id": row.get::<i32, _>("id")})))
}

#[derive(Deserialize)]
pub struct MarkReadRequest {
    pub agent_id: String,
    pub message_ids: Vec<i32>,
}

/// POST /api/conversations/mark-read
/// Batch mark messages as read by appending agent_id to read_by array
pub async fn mark_read(
    State(pool): State<DbPool>,
    Json(body): Json<MarkReadRequest>,
) -> Result<Json<Value>, ApiError> {
    if body.agent_id.is_empty() {
        return Err(ApiError::BadRequest("agent_id is required".into()));
    }
    if body.message_ids.is_empty() {
        return Ok(Json(serde_json::json!({"updated": 0})));
    }

    let result = sqlx::query(
        "UPDATE conversations SET read_by = array_append(read_by, $1) WHERE id = ANY($2) AND NOT ($1 = ANY(read_by))"
    )
    .bind(&body.agent_id)
    .bind(&body.message_ids)
    .execute(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({"updated": result.rows_affected()})))
}
