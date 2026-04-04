//! AF64 Task endpoints

use axum::{
    extract::{Path, Query, State},
    Json,
};
use dpn_core::DbPool;
use serde::Deserialize;
use serde_json::Value;
use sqlx::Row;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct TaskQuery {
    pub assigned_to: Option<String>,
    pub status: Option<String>,
    pub goal_id: Option<i32>,
    pub project_id: Option<i32>,
    pub limit: Option<i64>,
}

/// GET /api/af64/tasks
pub async fn list_tasks(
    State(pool): State<DbPool>,
    Query(q): Query<TaskQuery>,
) -> Result<Json<Value>, ApiError> {
    let limit = q.limit.unwrap_or(20).min(100);

    let rows = if let Some(ref goal_id) = q.goal_id {
        sqlx::query("SELECT id, task_id, text, status, assignee, due_date, completed_date, project_id, stage, pipeline_order, goal_id, stage_notes, scheduled_at, deadline, recurrence, context, parent_id, source, blocked_by FROM tasks WHERE goal_id = $1 ORDER BY pipeline_order ASC LIMIT $2")
            .bind(goal_id).bind(limit)
            .fetch_all(&pool).await
    } else if let Some(project_id) = q.project_id {
        sqlx::query("SELECT id, task_id, text, status, assignee, due_date, completed_date, project_id, stage, pipeline_order, goal_id, stage_notes, scheduled_at, deadline, recurrence, context, parent_id, source, blocked_by FROM tasks WHERE project_id = $1 ORDER BY id ASC LIMIT $2")
            .bind(project_id).bind(limit)
            .fetch_all(&pool).await
    } else if let Some(ref assigned_to) = q.assigned_to {
        let status = q.status.as_deref().unwrap_or("open");
        sqlx::query("SELECT id, task_id, text, status, assignee, due_date, completed_date, project_id, stage, pipeline_order, goal_id, stage_notes, scheduled_at, deadline, recurrence, context, parent_id, source, blocked_by FROM tasks WHERE assignee = $1 AND status = $2 ORDER BY id DESC LIMIT $3")
            .bind(assigned_to).bind(status).bind(limit)
            .fetch_all(&pool).await
    } else {
        sqlx::query("SELECT id, task_id, text, status, assignee, due_date, completed_date, project_id, stage, pipeline_order, goal_id, stage_notes, scheduled_at, deadline, recurrence, context, parent_id, source, blocked_by FROM tasks ORDER BY id DESC LIMIT $1")
            .bind(limit)
            .fetch_all(&pool).await
    };

    let rows = rows.map_err(|e| ApiError::Database(e.to_string()))?;
    let tasks: Vec<Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.get::<i32, _>("id"),
            "task_id": r.get::<Option<String>, _>("task_id"),
            "text": r.get::<String, _>("text"),
            "status": r.get::<String, _>("status"),
            "assignee": r.get::<Option<String>, _>("assignee"),
            "stage": r.get::<Option<String>, _>("stage"),
            "pipeline_order": r.get::<Option<i32>, _>("pipeline_order"),
            "goal_id": r.get::<Option<i32>, _>("goal_id"),
            "stage_notes": r.get::<Option<serde_json::Value>, _>("stage_notes"),
            "due_date": r.get::<Option<String>, _>("due_date"),
            "completed_date": r.get::<Option<String>, _>("completed_date"),
            "project_id": r.get::<Option<i32>, _>("project_id"),
            "context": r.get::<Option<String>, _>("context"),
            "parent_id": r.get::<Option<i32>, _>("parent_id"),
            "source": r.get::<Option<String>, _>("source"),
            "blocked_by": r.get::<Option<Vec<i32>>, _>("blocked_by"),
        })
    }).collect();

    Ok(Json(serde_json::json!(tasks)))
}

#[derive(Deserialize)]
pub struct TaskUpdate {
    pub status: Option<String>,
    pub completed_date: Option<String>,
    pub assignee: Option<String>,
    pub department: Option<String>,
    pub project_id: Option<i32>,
    pub stage: Option<String>,
    pub pipeline_order: Option<i32>,
    pub goal_id: Option<i32>,
    pub blocked_by: Option<Vec<i32>>,
    pub stage_notes: Option<serde_json::Value>,
    pub scheduled_at: Option<String>,
    pub deadline: Option<String>,
    pub recurrence: Option<String>,
}

/// PATCH /api/af64/tasks/:id
pub async fn update_task(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(body): Json<TaskUpdate>,
) -> Result<Json<Value>, ApiError> {
    if let Some(ref status) = body.status {
        sqlx::query("UPDATE tasks SET status = $1 WHERE id = $2")
            .bind(status).bind(id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(ref cd) = body.completed_date {
        sqlx::query("UPDATE tasks SET completed_date = $1 WHERE id = $2")
            .bind(cd).bind(id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(ref assignee) = body.assignee {
        sqlx::query("UPDATE tasks SET assignee = $1 WHERE id = $2")
            .bind(assignee).bind(id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(ref dept) = body.department {
        sqlx::query("UPDATE tasks SET department = $1 WHERE id = $2")
            .bind(dept).bind(id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(project_id) = body.project_id {
        sqlx::query("UPDATE tasks SET project_id = $1 WHERE id = $2")
            .bind(project_id).bind(id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(ref stage) = body.stage {
        sqlx::query("UPDATE tasks SET stage = $1 WHERE id = $2")
            .bind(stage).bind(id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(ref stage_notes) = body.stage_notes {
        sqlx::query("UPDATE tasks SET stage_notes = $1 WHERE id = $2")
            .bind(stage_notes).bind(id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(ref scheduled_at) = body.scheduled_at {
        sqlx::query("UPDATE tasks SET scheduled_at = $1::timestamptz WHERE id = $2")
            .bind(scheduled_at).bind(id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(ref deadline) = body.deadline {
        sqlx::query("UPDATE tasks SET deadline = $1::timestamptz WHERE id = $2")
            .bind(deadline).bind(id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(ref recurrence) = body.recurrence {
        sqlx::query("UPDATE tasks SET recurrence = $1 WHERE id = $2")
            .bind(recurrence).bind(id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }
    if let Some(ref blocked_by) = body.blocked_by {
        sqlx::query("UPDATE tasks SET blocked_by = $1 WHERE id = $2")
            .bind(blocked_by).bind(id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    }

    Ok(Json(serde_json::json!({"ok": true, "id": id})))
}

/// DELETE /api/af64/tasks/:id
pub async fn delete_task(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id).execute(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;
    Ok(Json(serde_json::json!({"ok": true, "deleted": id})))
}

#[derive(Deserialize)]
pub struct NewTask {
    pub text: String,
    pub status: Option<String>,
    pub assignee: Option<String>,
    pub department: Option<String>,
    pub doc_path: Option<String>,
    pub due_date: Option<String>,
    pub project_id: Option<i32>,
    pub task_id: Option<String>,
    pub parent_id: Option<i32>,
    pub source: Option<String>,
    pub blocked_by: Option<Vec<i32>>,
}

/// POST /api/af64/tasks
pub async fn create_task(
    State(pool): State<DbPool>,
    Json(body): Json<NewTask>,
) -> Result<Json<Value>, ApiError> {
    let status = body.status.unwrap_or_else(|| "open".into());
    let doc_path = body.doc_path.unwrap_or_else(|| "af64/generated".into());
    let task_id = body.task_id.unwrap_or_else(|| format!("ghost-{}", Uuid::new_v4()));
    let source = body.source.unwrap_or_else(|| "ghost".into());

    let row = sqlx::query("INSERT INTO tasks (task_id, text, status, assignee, department, doc_path, line_number, raw_line, due_date, project_id, parent_id, source, blocked_by) VALUES ($1, $2, $3, $4, $5, $6, 0, $2, $7, $8, $9, $10, $11) RETURNING id, task_id")
        .bind(&task_id).bind(&body.text).bind(&status).bind(&body.assignee).bind(&body.department).bind(&doc_path).bind(&body.due_date).bind(&body.project_id).bind(&body.parent_id).bind(&source).bind(&body.blocked_by)
        .fetch_one(&pool).await.map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({"id": row.get::<i32, _>("id"), "task_id": row.get::<String, _>("task_id")})))
}
