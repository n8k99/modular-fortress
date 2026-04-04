//! Decisions CRUD endpoints (append-only)

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
pub struct DecisionQuery {
    pub project_id: Option<i32>,
    pub department: Option<String>,
    pub owner: Option<String>,
    pub limit: Option<i64>,
    pub order: Option<String>,
}

/// GET /api/decisions
pub async fn list_decisions(
    State(pool): State<DbPool>,
    Query(q): Query<DecisionQuery>,
) -> Result<Json<Value>, ApiError> {
    let limit = q.limit.unwrap_or(20).min(100);
    let order = match q.order.as_deref() {
        Some("asc") | Some("ASC") => "ASC",
        _ => "DESC",
    };

    let rows = if let Some(project_id) = q.project_id {
        let sql = format!(
            "SELECT id, project_id, decision, rationale, owner, stakeholders, department, date, created_at \
             FROM decisions WHERE project_id = $1 ORDER BY created_at {} LIMIT $2",
            order
        );
        sqlx::query(&sql)
            .bind(project_id)
            .bind(limit)
            .fetch_all(&pool)
            .await
    } else if let Some(ref department) = q.department {
        let sql = format!(
            "SELECT id, project_id, decision, rationale, owner, stakeholders, department, date, created_at \
             FROM decisions WHERE department = $1 ORDER BY created_at {} LIMIT $2",
            order
        );
        sqlx::query(&sql)
            .bind(department)
            .bind(limit)
            .fetch_all(&pool)
            .await
    } else if let Some(ref owner) = q.owner {
        let sql = format!(
            "SELECT id, project_id, decision, rationale, owner, stakeholders, department, date, created_at \
             FROM decisions WHERE owner = $1 ORDER BY created_at {} LIMIT $2",
            order
        );
        sqlx::query(&sql)
            .bind(owner)
            .bind(limit)
            .fetch_all(&pool)
            .await
    } else {
        let sql = format!(
            "SELECT id, project_id, decision, rationale, owner, stakeholders, department, date, created_at \
             FROM decisions ORDER BY created_at {} LIMIT $1",
            order
        );
        sqlx::query(&sql)
            .bind(limit)
            .fetch_all(&pool)
            .await
    };

    let rows = rows.map_err(|e| ApiError::Database(e.to_string()))?;
    let decisions: Vec<Value> = rows
        .iter()
        .map(|r| {
            serde_json::json!({
                "id": r.get::<i32, _>("id"),
                "project_id": r.get::<Option<i32>, _>("project_id"),
                "decision": r.get::<String, _>("decision"),
                "rationale": r.get::<Option<String>, _>("rationale"),
                "owner": r.get::<Option<String>, _>("owner"),
                "stakeholders": r.get::<Option<Value>, _>("stakeholders"),
                "department": r.get::<Option<String>, _>("department"),
                "date": r.get::<Option<chrono::NaiveDate>, _>("date"),
                "created_at": r.get::<Option<chrono::DateTime<chrono::Utc>>, _>("created_at"),
            })
        })
        .collect();

    Ok(Json(serde_json::json!(decisions)))
}

#[derive(Deserialize)]
pub struct NewDecision {
    pub decision: String,
    pub rationale: Option<String>,
    pub project_id: Option<i32>,
    pub department: Option<String>,
    pub owner: String,
    pub stakeholders: Option<Value>,
    pub date: Option<String>,
}

/// POST /api/decisions
pub async fn create_decision(
    State(pool): State<DbPool>,
    Json(body): Json<NewDecision>,
) -> Result<Json<Value>, ApiError> {
    let date = match &body.date {
        Some(d) => chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d")
            .map_err(|e| ApiError::BadRequest(format!("Invalid date format: {}", e)))?,
        None => chrono::Utc::now().date_naive(),
    };

    let row = sqlx::query(
        "INSERT INTO decisions (decision, rationale, project_id, department, owner, stakeholders, date) \
         VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, created_at",
    )
    .bind(&body.decision)
    .bind(&body.rationale)
    .bind(&body.project_id)
    .bind(&body.department)
    .bind(&body.owner)
    .bind(&body.stakeholders)
    .bind(date)
    .fetch_one(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "id": row.get::<i32, _>("id"),
        "created_at": row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("created_at"),
    })))
}
