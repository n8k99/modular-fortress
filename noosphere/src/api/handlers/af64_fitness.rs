//! AF64 Fitness endpoints

use axum::{
    extract::{Path, Query, State},
    Json,
};
use dpn_core::DbPool;
use serde::Deserialize;
use serde_json::Value;
use sqlx::Row;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct FitnessQuery {
    pub days: Option<i32>,
}

/// GET /api/fitness/:agent_id
pub async fn get_fitness(
    State(pool): State<DbPool>,
    Path(agent_id): Path<String>,
    Query(q): Query<FitnessQuery>,
) -> Result<Json<Value>, ApiError> {
    let days = q.days.unwrap_or(30);
    let interval = format!("{} days", days);

    let row = sqlx::query("SELECT COALESCE(SUM(score), 0) as total FROM agent_fitness WHERE agent_id = $1 AND created_at > now() - $2::interval")
        .bind(&agent_id).bind(&interval)
        .fetch_one(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;

    let score: i64 = row.get("total");
    Ok(Json(serde_json::json!({"agent_id": agent_id, "fitness": score, "days": days})))
}

#[derive(Deserialize)]
pub struct NewFitness {
    pub agent_id: String,
    pub outcome: String,
    pub score: i32,
    pub context: Option<String>,
}

/// POST /api/fitness
pub async fn create_fitness(
    State(pool): State<DbPool>,
    Json(body): Json<NewFitness>,
) -> Result<Json<Value>, ApiError> {
    let row = sqlx::query("INSERT INTO agent_fitness (agent_id, outcome, score, context) VALUES ($1, $2, $3, $4) RETURNING id")
        .bind(&body.agent_id).bind(&body.outcome).bind(body.score).bind(&body.context)
        .fetch_one(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({"id": row.get::<i32, _>("id")})))
}
