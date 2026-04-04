//! Document endpoints

use axum::{
    extract::{Path, Query, State},
    Json,
};
use dpn_core::DbPool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct ListParams {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
}

fn default_limit() -> i64 { 50 }

#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
    #[serde(default = "default_limit")]
    limit: i64,
}

#[derive(Deserialize)]
pub struct GetByPathParams {
    path: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateDocumentRequest {
    pub path: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub frontmatter: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateDocumentRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub frontmatter: Option<String>,
}

pub async fn list_documents(
    State(pool): State<DbPool>,
    Query(params): Query<ListParams>,
) -> Result<Json<Value>, ApiError> {
    use std::collections::HashMap;

    // M2.2: Query BOTH memories (daily notes, weekly notes) AND legacy documents
    // memories is the primary source, documents table is legacy/secondary
    let memories = dpn_core::db::memories::list_light(&pool, params.limit, params.offset)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    let canonical_docs = dpn_core::db::documents::list_canonical(&pool, params.limit, params.offset)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    // Merge both sources using HashMap (memories override canonical_docs on path collision)
    let mut docs_by_path: HashMap<String, Value> = HashMap::new();

    // Add canonical docs first (lower priority)
    for doc in canonical_docs {
        let path = doc.path.clone();
        let value = serde_json::to_value(&doc)
            .map_err(|e| ApiError::Internal(format!("Serialization error: {}", e)))?;
        docs_by_path.insert(path, value);
    }

    // Add memories (higher priority - overrides legacy)
    for note in memories {
        let path = note.path.clone();
        let value = serde_json::to_value(&note)
            .map_err(|e| ApiError::Internal(format!("Serialization error: {}", e)))?;
        docs_by_path.insert(path, value);
    }

    // Convert to vec and return
    let documents: Vec<Value> = docs_by_path.into_values().collect();

    Ok(Json(serde_json::json!({ "documents": documents })))
}

pub async fn get_document(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    // M2.2 FIX: Check memories first (primary source for daily notes), then documents table
    if let Ok(note) = dpn_core::db::memories::get_by_id(&pool, id).await {
        return Ok(Json(serde_json::json!({
            "id": note.id,
            "path": note.path,
            "title": note.title,
            "content": note.content,
            "frontmatter": note.frontmatter,
            "modified_at": note.modified_at
        })));
    }

    // Fall back to documents table
    let doc = dpn_core::get_document_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Document {} not found", id)))?;

    Ok(Json(serde_json::json!(doc)))
}

pub async fn search_documents(
    State(pool): State<DbPool>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Value>, ApiError> {
    use std::collections::HashMap;

    // M2.2 FIX: Search BOTH memories and documents tables
    // Try memories first (primary source for daily notes)
    let memory_results = dpn_core::db::memories::search(&pool, &params.q, params.limit)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    // Then search canonical documents (legacy)
    let doc_results = dpn_core::db::documents::search_canonical(&pool, &params.q, params.limit)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    // Merge results using HashMap (memories override documents on path collision)
    let mut docs_by_path: HashMap<String, Value> = HashMap::new();

    // Add legacy documents first (lower priority)
    for doc in doc_results {
        let path = doc.path.clone();
        let value = serde_json::to_value(&doc)
            .map_err(|e| ApiError::Internal(format!("Serialization error: {}", e)))?;
        docs_by_path.insert(path, value);
    }

    // Add memories (higher priority - overrides duplicates)
    for note in memory_results {
        let path = note.path.clone();
        let value = serde_json::to_value(&note)
            .map_err(|e| ApiError::Internal(format!("Serialization error: {}", e)))?;
        docs_by_path.insert(path, value);
    }

    // Convert to vec
    let documents: Vec<Value> = docs_by_path.into_values().collect();

    Ok(Json(serde_json::json!({ "documents": documents })))
}

pub async fn create_document(
    State(pool): State<DbPool>,
    Json(req): Json<CreateDocumentRequest>,
) -> Result<Json<Value>, ApiError> {
    // M2.2 FIX: Write to memories (primary storage) instead of legacy documents table
    let content = req.content.as_deref().unwrap_or("");
    let id = dpn_core::db::memories::create(
        &pool,
        &req.path,
        req.title.as_deref(),
        content,
        req.frontmatter.as_deref(),
    )
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "status": "created",
        "id": id,
        "path": req.path
    })))
}

pub async fn update_document(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateDocumentRequest>,
) -> Result<Json<Value>, ApiError> {
    // Try memories first (primary storage for daily notes, weekly notes)
    // Fall back to documents table (canonical storage for world docs, agent profiles)
    let in_memories = dpn_core::db::memories::get_by_id(&pool, id).await.is_ok();

    if in_memories {
        if let Some(content) = req.content.as_deref() {
            dpn_core::db::memories::update_content(
                &pool,
                id,
                content,
                req.frontmatter.as_deref(),
            )
            .await
            .map_err(|e| ApiError::Database(e.to_string()))?;
        } else if req.frontmatter.is_some() {
            let note = dpn_core::db::memories::get_by_id(&pool, id)
                .await
                .map_err(|e| ApiError::Database(e.to_string()))?;
            let existing_content = note.content.as_deref().unwrap_or("");
            dpn_core::db::memories::update_content(
                &pool,
                id,
                existing_content,
                req.frontmatter.as_deref(),
            )
            .await
            .map_err(|e| ApiError::Database(e.to_string()))?;
        }
    } else {
        // Fall back to documents table
        dpn_core::db::documents::update_document(
            &pool,
            id,
            req.title.as_deref(),
            req.content.as_deref(),
            req.frontmatter.as_deref(),
        )
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;
    }

    Ok(Json(serde_json::json!({
        "status": "updated",
        "id": id
    })))
}

/// Get document by path (needed for event propagation and other path-based lookups)
/// Returns memories entry first, falls back to documents table
pub async fn get_document_by_path(
    State(pool): State<DbPool>,
    Query(params): Query<GetByPathParams>,
) -> Result<Json<Value>, ApiError> {
    // Try memories first (primary storage for daily notes, weekly notes, etc.)
    if let Ok(note) = dpn_core::db::memories::get_by_path(&pool, &params.path).await {
        return Ok(Json(serde_json::json!(note)));
    }

    // Fall back to documents table (legacy/canonical storage for projects, etc.)
    match dpn_core::db::documents::get_by_path(&pool, &params.path).await {
        Ok(doc) => Ok(Json(serde_json::json!(doc))),
        Err(_) => Err(ApiError::NotFound(format!("Document not found at path: {}", params.path))),
    }
}

#[derive(Deserialize)]
pub struct UpdateDocumentWithSyncRequest {
    pub content: String,
    pub title: Option<String>,
    pub frontmatter: Option<String>,
    #[serde(default)]
    pub sync_tasks: bool,
    #[serde(default)]
    pub sync_events: bool,
}

/// Atomic update: update document and sync tasks/events in single transaction
pub async fn update_document_with_sync(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateDocumentWithSyncRequest>,
) -> Result<Json<Value>, ApiError> {
    // Get document info
    let doc = dpn_core::get_document_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Document not found".to_string()))?;

    // Update document
    dpn_core::update_document(
        &pool,
        id,
        req.title.as_deref(),
        Some(&req.content),
        req.frontmatter.as_deref(),
    )
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    let mut tasks_synced = 0;
    let mut events_synced = 0;

    // Sync tasks if requested
    if req.sync_tasks {
        let title = doc.title.as_deref().unwrap_or("");
        let tasks = dpn_core::parse_tasks(&req.content, id, &doc.path, title);

        // Delete existing tasks
        sqlx::query("DELETE FROM tasks WHERE doc_id = $1")
            .bind(id)
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
        tasks_synced = tasks.len();
    }

    // Sync events if requested and is Weekly Note
    if req.sync_events && dpn_core::is_weekly_note(&doc.path) {
        let parsed_events = dpn_core::parse_events(&req.content);

        for parsed_event in &parsed_events {
            let event = dpn_core::Event {
                id: parsed_event.id.clone(),
                icon: parsed_event.icon.clone(),
                event_type: format!("{:?}", parsed_event.event_type).to_lowercase(),
                title: parsed_event.title.clone(),
                date: parsed_event.date.clone(),
                time: parsed_event.time.clone(),
                duration: parsed_event.duration.clone(),
                daily_note: parsed_event.daily_note.clone(),
                tags: parsed_event.tags.as_ref().map(|t| serde_json::to_string(t).unwrap_or_default()),
            };

            dpn_core::upsert_event(&pool, &event)
                .await
                .map_err(|e| ApiError::Database(e.to_string()))?;
        }
        events_synced = parsed_events.len();
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "id": id,
        "tasks_synced": tasks_synced,
        "events_synced": events_synced
    })))
}

/// Get or create daily note for a specific date
/// GET /api/documents/daily/:date
pub async fn get_daily_note(
    State(pool): State<DbPool>,
    Path(date_str): Path<String>,
) -> Result<Json<Value>, ApiError> {
    use chrono::NaiveDate;
    
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid date format. Use YYYY-MM-DD".to_string()))?;
    
    // Try to get existing daily note
    if let Some(note) = dpn_core::db::memories::get_daily_note(&pool, date)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
    {
        return Ok(Json(serde_json::json!({
            "id": note.id,
            "path": note.path,
            "title": note.title,
            "content": note.content,
            "note_type": note.note_type,
            "note_date": note.note_date,
            "frontmatter": note.frontmatter,
            "created": false
        })));
    }
    
    // Create new daily note
    let path = format!("Areas/N8K99Notes/Daily Notes/{}.md", date.format("%Y-%m-%d"));
    let title = format!("📅 {} Daily Log", date.format("%Y-%m-%d"));
    let content = format!(r#"# 📅 {} Daily Log

## 🎯 What I Did Today

## 📋 Tasks Completed Today

## 📋 Tasks Due Today

## 🚀 New Tasks Created Today

## 📦 Deliverables Created Today

## 🔗 Files Created/Updated Today

## 🚧 Projects Left Midstream

## 💡 Insights & Learnings

## 🔄 Git Commits Today

## 📊 Performance Metrics

- **Energy Level**: 🔋🔋🔋🔋🔋 (1-5 batteries)
- **Focus Quality**: 🎯🎯🎯🎯🎯 (1-5 targets)
"#, date.format("%Y-%m-%d"));
    
    let frontmatter = format!(r#"---
title: "{}"
date: {}
type: daily
tags: [daily-note]
---"#, title, date.format("%Y-%m-%d"));
    
    // Insert into memories
    let result = sqlx::query_scalar::<_, i32>(
        r#"
        INSERT INTO memories (path, title, content, frontmatter, note_type, note_date, created_at, modified_at)
        VALUES ($1, $2, $3, $4, 'daily', $5, NOW(), NOW())
        RETURNING id
        "#
    )
    .bind(&path)
    .bind(&title)
    .bind(&content)
    .bind(&frontmatter)
    .bind(date)
    .fetch_one(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "id": result,
        "path": path,
        "title": title,
        "content": content,
        "note_type": "daily",
        "frontmatter": frontmatter,
        "note_date": date.to_string(),
        "created": true
    })))
}

/// Get or create weekly note for a specific date (uses the Monday of that week)
/// GET /api/documents/weekly/:date
pub async fn get_weekly_note(
    State(pool): State<DbPool>,
    Path(date_str): Path<String>,
) -> Result<Json<Value>, ApiError> {
    use chrono::{NaiveDate, Datelike, Weekday};
    
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid date format. Use YYYY-MM-DD".to_string()))?;
    
    // Saturday-based week calculation (Nathan's system)
    // Week 1 starts on the first Saturday of the year
    let (year, week_num, week_saturday) = get_saturday_week(date);
    let week_friday = week_saturday + chrono::Duration::days(6);
    
    // Try to find existing weekly note by path (e.g., 2026-W09.md)
    let note_path = format!("Areas/N8K99Notes/Weekly Notes/{}-W{:02}.md", year, week_num);
    
    let existing = sqlx::query_as::<_, dpn_core::db::memories::Memory>(
        r#"
        SELECT id, path, title, content, frontmatter, size_bytes,
               note_type, note_date,
               modified_at::timestamp as modified_at,
               created_at::timestamp as created_at,
               compression_tier,
               compressed_from
        FROM memories
        WHERE path = $1 OR (note_type = 'weekly' AND note_date = $2)
        LIMIT 1
        "#
    )
    .bind(&note_path)
    .bind(week_saturday)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;
    
    if let Some(note) = existing {
        return Ok(Json(serde_json::json!({
            "id": note.id,
            "path": note.path,
            "title": note.title,
            "content": note.content,
            "note_type": note.note_type,
            "note_date": note.note_date,
            "frontmatter": note.frontmatter,
            "week_start": week_saturday.to_string(),
            "week_end": week_friday.to_string(),
            "created": false
        })));
    }
    
    // Create new weekly note using Saturday-based week format (YYYY-Www.md)
    let path = format!("Areas/N8K99Notes/Weekly Notes/{}-W{:02}.md", year, week_num);
    let title = format!("📅 {}-W{:02}", year, week_num);
    let content = format!(r#"# 📅 {}-W{:02} (Sat {} to Fri {})

## 🎯 Week Goals

## 📋 Events This Week

## 🚀 Projects Focus

## 📊 Weekly Review

### What Went Well

### What Could Improve

### Key Learnings

## 📈 Metrics

- **Productivity**: ⭐⭐⭐⭐⭐ (1-5 stars)
- **Energy**: 🔋🔋🔋🔋🔋 (1-5 batteries)
- **Focus**: 🎯🎯🎯🎯🎯 (1-5 targets)
"#, year, week_num, week_saturday.format("%Y-%m-%d"), week_friday.format("%Y-%m-%d"));
    
    let frontmatter = format!(r#"---
title: "{}"
date: {}
type: weekly
week_start: {}
week_end: {}
tags: [weekly-note]
---"#, title, week_saturday.format("%Y-%m-%d"), week_saturday.format("%Y-%m-%d"), week_friday.format("%Y-%m-%d"));
    
    // Insert into memories
    let result = sqlx::query_scalar::<_, i32>(
        r#"
        INSERT INTO memories (path, title, content, frontmatter, note_type, note_date, created_at, modified_at)
        VALUES ($1, $2, $3, $4, 'weekly', $5, NOW(), NOW())
        RETURNING id
        "#
    )
    .bind(&path)
    .bind(&title)
    .bind(&content)
    .bind(&frontmatter)
    .bind(week_saturday)
    .fetch_one(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "id": result,
        "path": path,
        "title": title,
        "content": content,
        "note_type": "weekly",
        "frontmatter": frontmatter,
        "note_date": week_saturday.to_string(),
        "week_start": week_saturday.to_string(),
        "week_end": week_friday.to_string(),
        "created": true
    })))
}

/// Calculate Saturday-based week number
/// Week 1 starts on the first Saturday of the year
/// Returns (year, week_number, week_saturday)
fn get_saturday_week(date: chrono::NaiveDate) -> (i32, u32, chrono::NaiveDate) {
    use chrono::{Datelike, Weekday};
    
    // Map weekday to Saturday-based offset (Sat=0, Sun=1, Mon=2, etc.)
    let weekday_offset = match date.weekday() {
        Weekday::Sat => 0,
        Weekday::Sun => 1,
        Weekday::Mon => 2,
        Weekday::Tue => 3,
        Weekday::Wed => 4,
        Weekday::Thu => 5,
        Weekday::Fri => 6,
    };
    
    // Find the Saturday of this week
    let week_saturday = if weekday_offset == 0 {
        date
    } else {
        date - chrono::Duration::days(weekday_offset as i64)
    };
    
    // Find the first Saturday of the year
    let year_start = chrono::NaiveDate::from_ymd_opt(week_saturday.year(), 1, 1).unwrap();
    let year_start_offset = match year_start.weekday() {
        Weekday::Sat => 0,
        Weekday::Sun => 1,
        Weekday::Mon => 2,
        Weekday::Tue => 3,
        Weekday::Wed => 4,
        Weekday::Thu => 5,
        Weekday::Fri => 6,
    };
    
    let first_saturday = if year_start_offset == 0 {
        year_start
    } else {
        year_start + chrono::Duration::days((7 - year_start_offset) as i64)
    };
    
    // Calculate week number
    if week_saturday < first_saturday {
        // This date is in the last week of the previous year
        (week_saturday.year() - 1, 52, week_saturday)
    } else {
        let days_diff = week_saturday.signed_duration_since(first_saturday).num_days();
        let week_num = ((days_diff / 7) + 1) as u32;
        (week_saturday.year(), week_num, week_saturday)
    }
}
