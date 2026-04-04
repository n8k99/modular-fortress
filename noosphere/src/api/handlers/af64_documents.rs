//! AF64 Documents endpoint

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
pub struct DocQuery {
    pub path_prefix: Option<String>,
    pub since: Option<String>,
    pub limit: Option<i64>,
}

/// GET /api/af64/documents
pub async fn list_documents(
    State(pool): State<DbPool>,
    Query(q): Query<DocQuery>,
) -> Result<Json<Value>, ApiError> {
    let limit = q.limit.unwrap_or(20).min(100);
    let since = q.since.as_deref().unwrap_or("1970-01-01T00:00:00Z");

    let rows = if let Some(ref prefix) = q.path_prefix {
        let pattern = format!("{}%", prefix);
        sqlx::query("SELECT id, title, path, status, COALESCE(modified_at, created_at) as updated_at FROM documents WHERE path LIKE $1 AND COALESCE(modified_at, created_at) > $2::timestamptz ORDER BY COALESCE(modified_at, created_at) DESC LIMIT $3")
            .bind(&pattern).bind(since).bind(limit)
            .fetch_all(&pool).await
    } else {
        sqlx::query("SELECT id, title, path, status, COALESCE(modified_at, created_at) as updated_at FROM documents WHERE COALESCE(modified_at, created_at) > $1::timestamptz ORDER BY COALESCE(modified_at, created_at) DESC LIMIT $2")
            .bind(since).bind(limit)
            .fetch_all(&pool).await
    };

    let rows = rows.map_err(|e| ApiError::Database(e.to_string()))?;
    let docs: Vec<Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.get::<i32, _>("id"),
            "title": r.get::<Option<String>, _>("title"),
            "path": r.get::<Option<String>, _>("path"),
            "status": r.get::<Option<String>, _>("status"),
        })
    }).collect();

    Ok(Json(serde_json::json!(docs)))
}
