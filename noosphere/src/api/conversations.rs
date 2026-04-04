use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Conversation {
    pub id: i32,
    pub agent_id: Option<String>,
    pub content: Option<String>,
    pub timestamp: Option<chrono::NaiveDateTime>,
}

pub async fn list_conversations(
    State(state): State<AppState>,
) -> Result<Json<Vec<Conversation>>, StatusCode> {
    let conversations = sqlx::query_as::<_, Conversation>(
        r#"
        SELECT
            id,
            agent_id,
            content,
            created_at as timestamp
        FROM conversations
        ORDER BY created_at DESC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(conversations))
}
