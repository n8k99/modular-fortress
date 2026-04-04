//! Task endpoints

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDate;
use dpn_core::DbPool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct TaskFilterParams {
    status: Option<String>,
    path: Option<String>,
    assignee: Option<String>,
    priority: Option<String>,
    #[serde(default)]
    due_today: bool,
    #[serde(default)]
    due_this_week: bool,
    #[serde(default)]
    overdue: bool,
    stale: Option<i32>,
    #[serde(default)]
    recurring: bool,
    tag: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncTasksRequest {
    pub doc_id: i32,
    pub doc_path: String,
    pub doc_title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateTaskRequest {
    pub doc_id: i32,
    pub line_number: usize,
    pub new_status: String,
    pub updates: Option<TaskUpdates>,
}

#[derive(Deserialize)]
pub struct TaskUpdates {
    pub priority: Option<String>,
    pub due_date: Option<String>,
    pub scheduled_date: Option<String>,
    pub assignee: Option<String>,
    pub scheduled_at: Option<String>,
    pub deadline: Option<String>,
    pub recurrence: Option<String>,
}

pub async fn list_tasks(
    State(pool): State<DbPool>,
    Query(params): Query<TaskFilterParams>,
) -> Result<Json<Value>, ApiError> {
    // Build dynamic query based on filters
    let mut sql = "SELECT * FROM tasks WHERE 1=1".to_string();
    let mut bind_params: Vec<String> = Vec::new();

    if let Some(ref status) = params.status {
        sql.push_str(&format!(" AND status = ${}", bind_params.len() + 1));
        bind_params.push(status.clone());
    }

    if let Some(ref path) = params.path {
        sql.push_str(&format!(" AND doc_path LIKE ${}", bind_params.len() + 1));
        bind_params.push(format!("{}%", path));
    }

    if let Some(ref assignee) = params.assignee {
        sql.push_str(&format!(" AND assignee = ${}", bind_params.len() + 1));
        bind_params.push(assignee.clone());
    }

    if let Some(ref priority) = params.priority {
        sql.push_str(&format!(" AND priority = ${}", bind_params.len() + 1));
        bind_params.push(priority.clone());
    }

    if params.due_today {
        sql.push_str(" AND date(due_date) = CURRENT_DATE");
    }

    if params.due_this_week {
        sql.push_str(" AND date(due_date) BETWEEN CURRENT_DATE AND CURRENT_DATE + INTERVAL '7 days'");
    }

    if params.overdue {
        sql.push_str(" AND date(due_date) < CURRENT_DATE AND status != 'done'");
    }

    if let Some(ref stale_days) = params.stale {
        sql.push_str(&format!(" AND date(updated_at) < CURRENT_DATE - (${} || ' days')::INTERVAL AND status != 'done'", bind_params.len() + 1));
        bind_params.push(stale_days.to_string());
    }

    if params.recurring {
        sql.push_str(" AND recurrence IS NOT NULL");
    }

    if let Some(ref tag) = params.tag {
        sql.push_str(&format!(" AND tags LIKE ${}", bind_params.len() + 1));
        bind_params.push(format!("%{}%", tag));
    }

    sql.push_str(" ORDER BY CASE priority WHEN 'high' THEN 0 WHEN 'medium' THEN 1 WHEN 'low' THEN 2 ELSE 3 END, due_date, doc_path, line_number");

    // Execute query with dynamic bindings - use dpn_core::Task type
    // For now, use the basic get_all_open_tasks if no filters, otherwise fall back to direct query
    let tasks = if params.status.is_none() && params.path.is_none() && params.assignee.is_none()
        && params.priority.is_none() && !params.due_today && !params.due_this_week
        && !params.overdue && params.stale.is_none() && !params.recurring && params.tag.is_none() {
        // No filters - use dpn_core function
        dpn_core::get_all_open_tasks(&pool)
            .await
            .map_err(|e| ApiError::Database(e.to_string()))?
    } else {
        // Has filters - build query with filters
        let mut sql = "SELECT id, doc_id, doc_path, doc_title, line_number, indent, status, text, assignee, start_date, due_date, notify_date, completed_date, raw_line, priority, task_id, dependencies, created_date, scheduled_date, recurrence, tags, department, project_id FROM tasks WHERE 1=1".to_string();
        
        if let Some(ref status) = params.status {
            sql.push_str(&format!(" AND status = '{}'", status.replace('\'', "''")));
        }
        if let Some(ref path) = params.path {
            sql.push_str(&format!(" AND doc_path LIKE '{}%'", path.replace('\'', "''")));
        }
        if let Some(ref assignee) = params.assignee {
            sql.push_str(&format!(" AND assignee = '{}'", assignee.replace('\'', "''")));
        }
        if let Some(ref priority) = params.priority {
            sql.push_str(&format!(" AND priority = '{}'", priority.replace('\'', "''")));
        }
        if params.due_today {
            sql.push_str(" AND date(due_date) = CURRENT_DATE");
        }
        if params.due_this_week {
            sql.push_str(" AND date(due_date) BETWEEN CURRENT_DATE AND CURRENT_DATE + INTERVAL '7 days'");
        }
        if params.overdue {
            sql.push_str(" AND date(due_date) < CURRENT_DATE AND status != 'done'");
        }
        sql.push_str(" ORDER BY CASE priority WHEN 'high' THEN 0 WHEN 'medium' THEN 1 WHEN 'low' THEN 2 ELSE 3 END, due_date, id LIMIT 500");
        
        sqlx::query_as::<_, dpn_core::db::tasks::Task>(&sql)
            .fetch_all(&pool)
            .await
            .map_err(|e| ApiError::Database(e.to_string()))?
    };

    Ok(Json(serde_json::json!({ "tasks": tasks })))
}

pub async fn get_tasks_due(
    State(pool): State<DbPool>,
    Path(date_str): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid date format. Use YYYY-MM-DD".to_string()))?;

    let tasks = dpn_core::get_tasks_due_on(&pool, date)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "tasks": tasks, "date": date_str })))
}

pub async fn get_overdue(
    State(pool): State<DbPool>,
) -> Result<Json<Value>, ApiError> {
    let today = chrono::Local::now().date_naive();
    let tasks = dpn_core::get_overdue_tasks(&pool, today)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "tasks": tasks })))
}

/// Sync tasks from a document - parse content and update tasks table
pub async fn sync_tasks(
    State(pool): State<DbPool>,
    Json(req): Json<SyncTasksRequest>,
) -> Result<Json<Value>, ApiError> {
    // Parse tasks from content
    let tasks = dpn_core::parse_tasks(&req.content, req.doc_id, &req.doc_path, &req.doc_title);

    // Delete existing tasks for this document
    sqlx::query("DELETE FROM tasks WHERE doc_id = $1")
        .bind(req.doc_id)
        .execute(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    // Insert new tasks
    for task in &tasks {
        let params = task.to_sql_params();
        sqlx::query(
            "INSERT INTO tasks (doc_id, doc_path, doc_title, line_number, indent, status, text, assignee, priority, task_id, dependencies, created_date, scheduled_date, due_date, completed_date, recurrence, tags, raw_line)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"
        )
        .bind(params.0).bind(&params.1).bind(&params.2).bind(params.3).bind(params.4)
        .bind(&params.5).bind(&params.6).bind(&params.7).bind(&params.8).bind(&params.9)
        .bind(&params.10).bind(&params.11).bind(&params.12).bind(&params.13).bind(&params.14)
        .bind(&params.15).bind(&params.16).bind(&params.17)
        .execute(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "count": tasks.len()
    })))
}

/// Update task status and optionally other fields
pub async fn update_task(
    State(pool): State<DbPool>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<Value>, ApiError> {
    // Get document content
    let doc = dpn_core::get_document_by_id(&pool, req.doc_id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Document not found".to_string()))?;

    let content = doc.content.ok_or_else(|| ApiError::BadRequest("Document has no content".to_string()))?;
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    if req.line_number >= lines.len() {
        return Err(ApiError::BadRequest("Invalid line number".to_string()));
    }

    // Update checkbox in document
    let status_char = match req.new_status.as_str() {
        "done" => 'x',
        "in-progress" => '/',
        "needs-review" => '?',
        _ => ' ',
    };

    lines[req.line_number] = lines[req.line_number]
        .replace("- [ ]", &format!("- [{}]", status_char))
        .replace("- [x]", &format!("- [{}]", status_char))
        .replace("- [X]", &format!("- [{}]", status_char))
        .replace("- [/]", &format!("- [{}]", status_char))
        .replace("- [-]", &format!("- [{}]", status_char))
        .replace("- [?]", &format!("- [{}]", status_char));

    // If marking done, add completion date if not present
    if req.new_status == "done" && !lines[req.line_number].contains("✅") {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        lines[req.line_number].push_str(&format!(" ✅ {}", today));
    }

    // Save updated document
    let new_content = lines.join("\n");
    dpn_core::update_document(&pool, req.doc_id, None, Some(&new_content), None)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    // Update tasks table
    let mut update_sql = "UPDATE tasks SET status = $1, raw_line = $2, updated_at = NOW()".to_string();
    let mut bind_idx = 3;

    if req.new_status == "done" {
        update_sql.push_str(", completed_date = CURRENT_DATE");
    }

    if let Some(ref updates) = req.updates {
        if updates.priority.is_some() {
            update_sql.push_str(&format!(", priority = ${}", bind_idx));
            bind_idx += 1;
        }
        if updates.due_date.is_some() {
            update_sql.push_str(&format!(", due_date = ${}", bind_idx));
            bind_idx += 1;
        }
        if updates.scheduled_date.is_some() {
            update_sql.push_str(&format!(", scheduled_date = ${}", bind_idx));
            bind_idx += 1;
        }
        if updates.assignee.is_some() {
            update_sql.push_str(&format!(", assignee = ${}", bind_idx));
            bind_idx += 1;
        }
        if updates.scheduled_at.is_some() {
            update_sql.push_str(&format!(", scheduled_at = ${}::timestamptz", bind_idx));
            bind_idx += 1;
        }
        if updates.deadline.is_some() {
            update_sql.push_str(&format!(", deadline = ${}::timestamptz", bind_idx));
            bind_idx += 1;
        }
        if updates.recurrence.is_some() {
            update_sql.push_str(&format!(", recurrence = ${}", bind_idx));
            bind_idx += 1;
        }
    }

    update_sql.push_str(&format!(" WHERE doc_id = ${} AND line_number = ${}", bind_idx, bind_idx + 1));

    let mut query = sqlx::query(&update_sql)
        .bind(&req.new_status)
        .bind(&lines[req.line_number]);

    if let Some(ref updates) = req.updates {
        if let Some(ref priority) = updates.priority {
            query = query.bind(priority);
        }
        if let Some(ref due_date) = updates.due_date {
            query = query.bind(due_date);
        }
        if let Some(ref scheduled_date) = updates.scheduled_date {
            query = query.bind(scheduled_date);
        }
        if let Some(ref assignee) = updates.assignee {
            query = query.bind(assignee);
        }
        if let Some(ref scheduled_at) = updates.scheduled_at {
            query = query.bind(scheduled_at);
        }
        if let Some(ref deadline) = updates.deadline {
            query = query.bind(deadline);
        }
        if let Some(ref recurrence) = updates.recurrence {
            query = query.bind(recurrence);
        }
    }

    query
        .bind(req.doc_id)
        .bind(req.line_number as i32)
        .execute(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "success": true })))
}
