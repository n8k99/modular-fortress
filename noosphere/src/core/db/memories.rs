//! memories table access (renamed from vault_notes)
//!
//! Primary memory storage with ~2,678 records. Much faster than documents table.
//! Structure mirrors Obsidian vault with path-based organization.

use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::DbPool;

/// Full memory with content
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Memory {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub frontmatter: Option<String>,
    pub size_bytes: Option<i32>,
    pub note_type: Option<String>,
    pub note_date: Option<NaiveDate>,
    pub modified_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub compression_tier: String,
    pub compressed_from: Option<Vec<i32>>,
}

/// Lightweight memory metadata (no content)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MemoryLight {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    pub size_bytes: Option<i32>,
    pub note_type: Option<String>,
    pub note_date: Option<NaiveDate>,
    pub modified_at: Option<NaiveDateTime>,
    pub compression_tier: String,
}

impl Memory {
    pub fn display_title(&self) -> String {
        self.title.clone().unwrap_or_else(|| self.path.clone())
    }

    pub fn display_size(&self) -> String {
        match self.size_bytes {
            Some(bytes) if bytes >= 1024 => format!("{} KB", bytes / 1024),
            Some(bytes) => format!("{} B", bytes),
            None => "---".to_string(),
        }
    }

    pub fn display_date(&self) -> String {
        match &self.modified_at {
            Some(dt) => dt.format("%Y-%m-%d").to_string(),
            None => "---".to_string(),
        }
    }
}

impl MemoryLight {
    pub fn display_title(&self) -> String {
        self.title.clone().unwrap_or_else(|| self.path.clone())
    }
}

/// Get total count of memories
pub async fn get_count(pool: &DbPool) -> Result<i64> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM memories")
        .fetch_one(pool)
        .await?;
    Ok(count.0)
}

/// List memories (metadata only, fast)
pub async fn list_light(pool: &DbPool, limit: i64, offset: i64) -> Result<Vec<MemoryLight>> {
    let notes = sqlx::query_as::<_, MemoryLight>(
        r#"
        SELECT id, path, title, size_bytes, note_type, note_date,
               modified_at::timestamp as modified_at,
               compression_tier
        FROM memories
        ORDER BY path
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(notes)
}

/// List all memories (metadata only)
pub async fn list_all_light(pool: &DbPool) -> Result<Vec<MemoryLight>> {
    let notes = sqlx::query_as::<_, MemoryLight>(
        r#"
        SELECT id, path, title, size_bytes, note_type, note_date,
               modified_at::timestamp as modified_at,
               compression_tier
        FROM memories
        ORDER BY path
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(notes)
}

/// List memories with content (for graph building)
pub async fn list_with_content(pool: &DbPool, limit: i64, offset: i64) -> Result<Vec<Memory>> {
    let notes = sqlx::query_as::<_, Memory>(
        r#"
        SELECT id, path, title, content, frontmatter, size_bytes,
               note_type, note_date,
               modified_at::timestamp as modified_at,
               created_at::timestamp as created_at,
               compression_tier, compressed_from
        FROM memories
        ORDER BY path
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(notes)
}

/// Get a memory by path (with content)
pub async fn get_by_path(pool: &DbPool, path: &str) -> Result<Memory> {
    let note = sqlx::query_as::<_, Memory>(
        r#"
        SELECT id, path, title, content, frontmatter, size_bytes,
               note_type, note_date,
               modified_at::timestamp as modified_at,
               created_at::timestamp as created_at,
               compression_tier, compressed_from
        FROM memories
        WHERE path = $1
        LIMIT 1
        "#,
    )
    .bind(path)
    .fetch_one(pool)
    .await?;
    Ok(note)
}

/// Get a memory by ID (with content)
pub async fn get_by_id(pool: &DbPool, id: i32) -> Result<Memory> {
    let note = sqlx::query_as::<_, Memory>(
        r#"
        SELECT id, path, title, content, frontmatter, size_bytes,
               note_type, note_date,
               modified_at::timestamp as modified_at,
               created_at::timestamp as created_at,
               compression_tier, compressed_from
        FROM memories
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(note)
}

/// Search memories by path, title, or content
pub async fn search(pool: &DbPool, query: &str, limit: i64) -> Result<Vec<Memory>> {
    let notes = if query.starts_with('/') {
        // Path prefix search
        let path_pattern = format!("{}%", &query[1..]);
        sqlx::query_as::<_, Memory>(
            r#"
            SELECT id, path, title, content, frontmatter, size_bytes,
                   note_type, note_date,
                   modified_at::timestamp as modified_at,
                   created_at::timestamp as created_at,
                   compression_tier, compressed_from
            FROM memories
            WHERE path LIKE $1
            ORDER BY path
            LIMIT $2
            "#,
        )
        .bind(&path_pattern)
        .bind(limit)
        .fetch_all(pool)
        .await?
    } else {
        // Fuzzy search
        let fuzzy_pattern = format!("%{}%", query);
        sqlx::query_as::<_, Memory>(
            r#"
            SELECT id, path, title, content, frontmatter, size_bytes,
                   note_type, note_date,
                   modified_at::timestamp as modified_at,
                   created_at::timestamp as created_at,
                   compression_tier, compressed_from
            FROM memories
            WHERE path ILIKE $1 OR title ILIKE $1 OR content ILIKE $1
            ORDER BY path
            LIMIT $2
            "#,
        )
        .bind(&fuzzy_pattern)
        .bind(limit)
        .fetch_all(pool)
        .await?
    };
    Ok(notes)
}

/// Get daily note by date (uses note_date field first, falls back to path patterns)
pub async fn get_daily_note(pool: &DbPool, date: NaiveDate) -> Result<Option<Memory>> {
    // First try using the note_date field
    let note = sqlx::query_as::<_, Memory>(
        r#"
        SELECT id, path, title, content, frontmatter, size_bytes,
               note_type, note_date,
               modified_at::timestamp as modified_at,
               created_at::timestamp as created_at,
               compression_tier, compressed_from
        FROM memories
        WHERE note_date = $1 AND note_type = 'daily'
        LIMIT 1
        "#,
    )
    .bind(date)
    .fetch_optional(pool)
    .await?;

    if note.is_some() {
        return Ok(note);
    }

    // Fall back to path-based lookup
    let date_str = date.format("%Y-%m-%d").to_string();
    let patterns = vec![
        format!("journal/daily/{}.md", date_str),
        format!("daily/{}.md", date_str),
        format!("{}.md", date_str),
    ];

    for pattern in patterns {
        if let Ok(note) = get_by_path(pool, &pattern).await {
            return Ok(Some(note));
        }
    }
    Ok(None)
}

/// Get notes by type (daily, weekly, project, etc.)
pub async fn list_by_type(pool: &DbPool, note_type: &str) -> Result<Vec<MemoryLight>> {
    let notes = sqlx::query_as::<_, MemoryLight>(
        r#"
        SELECT id, path, title, size_bytes, note_type, note_date,
               modified_at::timestamp as modified_at,
               compression_tier
        FROM memories
        WHERE note_type = $1
        ORDER BY note_date DESC NULLS LAST, path
        "#,
    )
    .bind(note_type)
    .fetch_all(pool)
    .await?;
    Ok(notes)
}

/// Get notes for a date range
pub async fn list_by_date_range(
    pool: &DbPool,
    start: NaiveDate,
    end: NaiveDate
) -> Result<Vec<MemoryLight>> {
    let notes = sqlx::query_as::<_, MemoryLight>(
        r#"
        SELECT id, path, title, size_bytes, note_type, note_date,
               modified_at::timestamp as modified_at,
               compression_tier
        FROM memories
        WHERE note_date >= $1 AND note_date <= $2
        ORDER BY note_date, path
        "#,
    )
    .bind(start)
    .bind(end)
    .fetch_all(pool)
    .await?;
    Ok(notes)
}

/// Get notes modified since a given timestamp
pub async fn list_modified_since(pool: &DbPool, since: NaiveDateTime) -> Result<Vec<Memory>> {
    let notes = sqlx::query_as::<_, Memory>(
        r#"
        SELECT id, path, title, content, frontmatter, size_bytes,
               note_type, note_date,
               modified_at::timestamp as modified_at,
               created_at::timestamp as created_at,
               compression_tier, compressed_from
        FROM memories
        WHERE modified_at > $1
        ORDER BY modified_at DESC
        "#,
    )
    .bind(since)
    .fetch_all(pool)
    .await?;
    Ok(notes)
}

/// Update a memory's content
pub async fn update_content(
    pool: &DbPool,
    id: i32,
    content: &str,
    frontmatter: Option<&str>,
) -> Result<()> {
    tracing::info!("Updating memory id={}, content_len={} bytes", id, content.len());

    let result = sqlx::query(
        "UPDATE memories SET content = $1, frontmatter = $2, modified_at = NOW() WHERE id = $3"
    )
    .bind(content)
    .bind(frontmatter)
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        tracing::warn!("UPDATE affected 0 rows - memory id={} may not exist", id);
    }
    Ok(())
}

/// Create a new memory
pub async fn create(
    pool: &DbPool,
    path: &str,
    title: Option<&str>,
    content: &str,
    frontmatter: Option<&str>,
) -> Result<i32> {
    let row: (i32,) = sqlx::query_as(
        r#"
        INSERT INTO memories (path, title, content, frontmatter, compression_tier, created_at, modified_at)
        VALUES ($1, $2, $3, $4, 'daily', NOW(), NOW())
        RETURNING id
        "#
    )
    .bind(path)
    .bind(title)
    .bind(content)
    .bind(frontmatter)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}
