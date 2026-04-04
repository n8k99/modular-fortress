use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub status: Option<String>,
    pub assigned_to: Option<String>,
    pub domain: Option<String>,
    pub pipeline: Option<String>,
}

pub async fn list_tasks(
    State(state): State<AppState>,
) -> Result<Json<Vec<Task>>, StatusCode> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT
            id,
            title,
            status,
            assigned_agent as assigned_to,
            '' as domain,
            '' as pipeline
        FROM tasks
        WHERE status IN ('pending', 'in_progress')
        ORDER BY created_at DESC
        LIMIT 50
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(tasks))
}
