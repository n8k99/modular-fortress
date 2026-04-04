//! XMPP message endpoints

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use dpn_core::DbPool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Serialize, sqlx::FromRow)]
pub struct XmppRoom {
    pub room_jid: String,
    pub message_count: i64,
    pub last_message_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct XmppMessage {
    pub id: i32,
    pub room_jid: String,
    pub sender: String,
    pub body: String,
    pub timestamp: DateTime<Utc>,
    pub message_id: String,
}

#[derive(Deserialize)]
pub struct MessagesQuery {
    pub room: String,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub before: Option<DateTime<Utc>>,
}

fn default_limit() -> i64 {
    50
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub room: String,
    pub sender: String,
    pub body: String,
}

/// GET /api/xmpp/rooms - List distinct rooms with message counts
pub async fn list_rooms(
    State(pool): State<DbPool>,
) -> Result<Json<Value>, ApiError> {
    let rooms: Vec<XmppRoom> = sqlx::query_as(
        r#"
        SELECT 
            room_jid,
            COUNT(*)::bigint as message_count,
            MAX(timestamp) as last_message_at
        FROM xmpp_messages
        GROUP BY room_jid
        ORDER BY last_message_at DESC NULLS LAST
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "rooms": rooms })))
}

/// GET /api/xmpp/messages?room=X&limit=50&before=timestamp - Fetch messages
pub async fn list_messages(
    State(pool): State<DbPool>,
    Query(params): Query<MessagesQuery>,
) -> Result<Json<Value>, ApiError> {
    let limit = params.limit.min(100); // Cap at 100

    let messages: Vec<XmppMessage> = if let Some(before) = params.before {
        sqlx::query_as(
            r#"
            SELECT 
                id,
                room_jid,
                sender,
                body,
                timestamp,
                message_id
            FROM xmpp_messages
            WHERE room_jid = $1 AND timestamp < $2
            ORDER BY timestamp DESC
            LIMIT $3
            "#
        )
        .bind(&params.room)
        .bind(before)
        .bind(limit)
        .fetch_all(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
    } else {
        sqlx::query_as(
            r#"
            SELECT 
                id,
                room_jid,
                sender,
                body,
                timestamp,
                message_id
            FROM xmpp_messages
            WHERE room_jid = $1
            ORDER BY timestamp DESC
            LIMIT $2
            "#
        )
        .bind(&params.room)
        .bind(limit)
        .fetch_all(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
    };

    Ok(Json(serde_json::json!({ 
        "messages": messages,
        "room": params.room,
        "count": messages.len()
    })))
}

/// POST /api/xmpp/send - Insert a new message
pub async fn send_message(
    State(pool): State<DbPool>,
    Json(req): Json<SendMessageRequest>,
) -> Result<Json<Value>, ApiError> {
    let message_id = format!("api-{}", Uuid::new_v4());
    let timestamp = Utc::now();

    let result: (i32,) = sqlx::query_as(
        r#"
        INSERT INTO xmpp_messages (room_jid, sender, body, timestamp, message_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#
    )
    .bind(&req.room)
    .bind(&req.sender)
    .bind(&req.body)
    .bind(timestamp)
    .bind(&message_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "id": result.0,
        "message_id": message_id,
        "timestamp": timestamp
    })))
}
