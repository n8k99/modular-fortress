//! archives table access and CRUD operations
//!
//! Immutable archive storage for PARAT temporal records.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

use super::DbPool;

/// Archive record from database
/// Note: tsv (tsvector) is a GENERATED column and excluded from this struct
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Archive {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub source_type: String,
    pub source_id: Option<i32>,
    pub original_path: Option<String>,
    pub period_start: Option<chrono::NaiveDate>,
    pub period_end: Option<chrono::NaiveDate>,
    pub topic: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// List all archives, ordered by creation date descending
pub async fn list_archives(pool: &DbPool) -> Result<Vec<Archive>> {
    let archives = sqlx::query_as::<_, Archive>(
        r#"
        SELECT id, title, content, source_type, source_id, original_path,
               period_start, period_end, topic, tags, metadata, created_at
        FROM archives
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(archives)
}

/// Get archive by ID
pub async fn get_archive_by_id(pool: &DbPool, id: i32) -> Result<Option<Archive>> {
    let archive = sqlx::query_as::<_, Archive>(
        r#"
        SELECT id, title, content, source_type, source_id, original_path,
               period_start, period_end, topic, tags, metadata, created_at
        FROM archives
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(archive)
}

/// Create a new archive record
pub async fn create_archive(
    pool: &DbPool,
    title: Option<&str>,
    content: Option<&str>,
    source_type: &str,
    source_id: Option<i32>,
    original_path: Option<&str>,
    period_start: Option<chrono::NaiveDate>,
    period_end: Option<chrono::NaiveDate>,
    topic: Option<&str>,
    tags: Option<&serde_json::Value>,
    metadata: Option<&serde_json::Value>,
) -> Result<i32> {
    let result = sqlx::query(
        r#"
        INSERT INTO archives (title, content, source_type, source_id, original_path,
                              period_start, period_end, topic, tags, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id
        "#,
    )
    .bind(title)
    .bind(content)
    .bind(source_type)
    .bind(source_id)
    .bind(original_path)
    .bind(period_start)
    .bind(period_end)
    .bind(topic)
    .bind(tags)
    .bind(metadata)
    .fetch_one(pool)
    .await?;

    let id: i32 = result.get("id");
    Ok(id)
}

/// Update archive metadata fields only (archives are immutable per D-04)
pub async fn update_archive_metadata(
    pool: &DbPool,
    id: i32,
    topic: Option<&str>,
    tags: Option<&serde_json::Value>,
    metadata: Option<&serde_json::Value>,
) -> Result<()> {
    let mut query = String::from("UPDATE archives SET ");
    let mut updates = vec![];
    let mut param_idx = 1;

    if topic.is_some() {
        updates.push(format!("topic = ${}", param_idx));
        param_idx += 1;
    }
    if tags.is_some() {
        updates.push(format!("tags = ${}", param_idx));
        param_idx += 1;
    }
    if metadata.is_some() {
        updates.push(format!("metadata = ${}", param_idx));
        param_idx += 1;
    }

    if updates.is_empty() {
        return Ok(());
    }

    query.push_str(&updates.join(", "));
    query.push_str(&format!(" WHERE id = ${}", param_idx));

    let mut q = sqlx::query(&query);

    if let Some(t) = topic {
        q = q.bind(t);
    }
    if let Some(tg) = tags {
        q = q.bind(tg);
    }
    if let Some(m) = metadata {
        q = q.bind(m);
    }
    q = q.bind(id);

    q.execute(pool).await?;

    Ok(())
}

/// Full-text search across archives using tsv column
pub async fn search_archives(pool: &DbPool, query: &str) -> Result<Vec<Archive>> {
    let archives = sqlx::query_as::<_, Archive>(
        r#"
        SELECT id, title, content, source_type, source_id, original_path,
               period_start, period_end, topic, tags, metadata, created_at
        FROM archives
        WHERE tsv @@ plainto_tsquery('english', $1)
        ORDER BY ts_rank(tsv, plainto_tsquery('english', $1)) DESC
        "#,
    )
    .bind(query)
    .fetch_all(pool)
    .await?;

    Ok(archives)
}
