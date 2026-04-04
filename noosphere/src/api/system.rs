use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStats {
    pub active_ghosts: i32,
    pub idle_ghosts: i32,
    pub dormant_ghosts: i32,
    pub total_tasks: i32,
    pub total_conversations: i32,
    pub tick_number: i32,
    pub uptime_days: i32,
}

pub async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<SystemStats>, StatusCode> {
    // Query real stats from database
    let agent_counts: (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE state = 'active') as active,
            COUNT(*) FILTER (WHERE state = 'idle') as idle,
            COUNT(*) FILTER (WHERE state = 'dormant') as dormant
        FROM agents
        "#,
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or((0, 0, 0));

    let total_tasks: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks")
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));

    let total_conversations: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM conversations")
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));

    let stats = SystemStats {
        active_ghosts: agent_counts.0 as i32,
        idle_ghosts: agent_counts.1 as i32,
        dormant_ghosts: agent_counts.2 as i32,
        total_tasks: total_tasks.0 as i32,
        total_conversations: total_conversations.0 as i32,
        tick_number: 4847, // TODO: get from tick_log
        uptime_days: 31,   // TODO: calculate from system start
    };

    Ok(Json(stats))
}
