//! documents table access (LEGACY)
//!
//! The documents table has ~47K records. Use sparingly - prefer memories.
//! This module provides light query support and canonical/dedup queries.

use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::DbPool;

/// Document from legacy documents table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Document {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub frontmatter: Option<String>,
    pub size_bytes: Option<i32>,
    pub modified_at: Option<NaiveDateTime>,
}

/// Lightweight document metadata (no content)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DocumentMeta {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    pub size_bytes: Option<i32>,
    pub modified_at: Option<NaiveDateTime>,
}

/// Document version info for version history
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DocumentVersion {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    pub modified_at: Option<NaiveDateTime>,
    pub size_bytes: Option<i32>,
}

impl Document {
    pub fn display_title(&self) -> String {
        self.title.clone().unwrap_or_else(|| self.path.clone())
    }

    pub fn display_size(&self) -> String {
        match self.size_bytes {
            Some(bytes) if bytes >= 1024 => format!("{} KB", bytes / 1024),
            Some(bytes) => format!("{} B", bytes),
            None => "—".to_string(),
        }
    }

    pub fn display_date(&self) -> String {
        match &self.modified_at {
            Some(dt) => dt.format("%Y-%m-%d").to_string(),
            None => "—".to_string(),
        }
    }
}

impl DocumentMeta {
    pub fn display_title(&self) -> String {
        self.title.clone().unwrap_or_else(|| self.path.clone())
    }
}

/// Get document count (WARNING: slow on 47K records)
pub async fn get_count(pool: &DbPool) -> Result<i64> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM documents")
        .fetch_one(pool)
        .await?;
    Ok(count.0)
}

/// List documents (metadata only, paginated)
pub async fn list_light(pool: &DbPool, limit: i64, offset: i64) -> Result<Vec<DocumentMeta>> {
    let docs = sqlx::query_as::<_, DocumentMeta>(
        r#"
        SELECT id, path, title, size_bytes, modified_at::timestamp as modified_at
        FROM documents
        ORDER BY path
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(docs)
}

/// Get document by ID (with content)
pub async fn get_by_id(pool: &DbPool, id: i32) -> Result<Option<Document>> {
    let doc = sqlx::query_as::<_, Document>(
        r#"
        SELECT id, path, title, content, frontmatter, size_bytes,
               modified_at::timestamp as modified_at
        FROM documents
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(doc)
}

/// Get document by path (with content)
pub async fn get_by_path(pool: &DbPool, path: &str) -> Result<Document> {
    let doc = sqlx::query_as::<_, Document>(
        r#"
        SELECT id, path, title, content, frontmatter, size_bytes,
               modified_at::timestamp as modified_at
        FROM documents
        WHERE path = $1
        LIMIT 1
        "#,
    )
    .bind(path)
    .fetch_one(pool)
    .await?;
    Ok(doc)
}

/// Search documents (limited, use memories for better perf)
pub async fn search(pool: &DbPool, query: &str, limit: i64) -> Result<Vec<Document>> {
    let fuzzy_pattern = format!("%{}%", query);
    let docs = sqlx::query_as::<_, Document>(
        r#"
        SELECT id, path, title, content, frontmatter, size_bytes,
               modified_at::timestamp as modified_at
        FROM documents
        WHERE path ILIKE $1 OR title ILIKE $1
        ORDER BY path
        LIMIT $2
        "#,
    )
    .bind(&fuzzy_pattern)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(docs)
}

// ============================================================================
// Canonical/Dedup Queries
// ============================================================================

/// List canonical documents only (excludes duplicate versions)
/// Falls back to all documents if is_canonical column doesn't exist yet
pub async fn list_canonical(pool: &DbPool, limit: i64, offset: i64) -> Result<Vec<Document>> {
    // Try canonical-only query first
    let canonical_result = sqlx::query_as::<_, DocumentMeta>(
        r#"
        SELECT id, path, title, size_bytes, modified_at::timestamp as modified_at
        FROM documents
        WHERE is_canonical = true OR is_canonical IS NULL
        ORDER BY path
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await;

    match canonical_result {
        Ok(metas) => {
            Ok(metas.into_iter().map(|m| Document {
                id: m.id,
                path: m.path,
                title: m.title,
                content: None,
                frontmatter: None,
                size_bytes: m.size_bytes,
                modified_at: m.modified_at,
            }).collect())
        }
        Err(_) => {
            // Column doesn't exist yet - fall back to all documents
            let docs = list_light(pool, limit, offset).await?;
            Ok(docs.into_iter().map(|m| Document {
                id: m.id,
                path: m.path,
                title: m.title,
                content: None,
                frontmatter: None,
                size_bytes: m.size_bytes,
                modified_at: m.modified_at,
            }).collect())
        }
    }
}

/// Search canonical documents only (full-text on path/title)
pub async fn search_canonical(pool: &DbPool, query: &str, limit: i64) -> Result<Vec<Document>> {
    let fuzzy_pattern = format!("%{}%", query);
    
    // Try canonical-only search first
    let canonical_result = sqlx::query_as::<_, Document>(
        r#"
        SELECT id, path, title, content, frontmatter, size_bytes,
               modified_at::timestamp as modified_at
        FROM documents
        WHERE (is_canonical = true OR is_canonical IS NULL)
          AND (path ILIKE $1 OR title ILIKE $1)
        ORDER BY path
        LIMIT $2
        "#,
    )
    .bind(&fuzzy_pattern)
    .bind(limit)
    .fetch_all(pool)
    .await;

    match canonical_result {
        Ok(docs) => Ok(docs),
        Err(_) => {
            // Fall back to regular search
            search(pool, query, limit).await
        }
    }
}

/// Get all canonical document titles (lightweight for autocomplete)
/// Returns (id, title) tuples
pub async fn get_all_titles_canonical(pool: &DbPool) -> Result<Vec<(i32, String)>> {
    #[derive(FromRow)]
    struct TitleRow {
        id: i32,
        title: Option<String>,
        path: String,
    }
    
    // Try canonical-only query first
    let canonical_result = sqlx::query_as::<_, TitleRow>(
        r#"
        SELECT id, title, path
        FROM documents
        WHERE is_canonical = true OR is_canonical IS NULL
        ORDER BY COALESCE(title, path)
        "#,
    )
    .fetch_all(pool)
    .await;

    let rows = match canonical_result {
        Ok(rows) => rows,
        Err(_) => {
            // Fall back to all documents
            sqlx::query_as::<_, TitleRow>(
                r#"
                SELECT id, title, path
                FROM documents
                ORDER BY COALESCE(title, path)
                "#,
            )
            .fetch_all(pool)
            .await?
        }
    };

    Ok(rows.into_iter()
        .map(|r| (r.id, r.title.unwrap_or(r.path)))
        .collect())
}

/// Get all versions of a document given its canonical_id
/// Returns empty vec if canonical_id column doesn't exist yet
pub async fn get_versions(pool: &DbPool, canonical_id: i32) -> Result<Vec<DocumentVersion>> {
    let result = sqlx::query_as::<_, DocumentVersion>(
        r#"
        SELECT id, path, title, modified_at::timestamp as modified_at, size_bytes
        FROM documents
        WHERE canonical_id = $1
        ORDER BY modified_at DESC
        "#,
    )
    .bind(canonical_id)
    .fetch_all(pool)
    .await;

    match result {
        Ok(versions) => Ok(versions),
        Err(_) => {
            // canonical_id column doesn't exist yet
            Ok(vec![])
        }
    }
}

// ============================================================================
// Write Operations (Create/Update/Delete)
// ============================================================================

/// Create a new document
pub async fn create_document(
    pool: &DbPool,
    path: &str,
    title: Option<&str>,
    content: Option<&str>,
    frontmatter: Option<&str>,
) -> Result<i32> {
    use sqlx::Row;

    let size_bytes = content.map(|c| c.len() as i32);

    let result = sqlx::query(
        r#"
        INSERT INTO documents (path, title, content, frontmatter, size_bytes, modified_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        RETURNING id
        "#,
    )
    .bind(path)
    .bind(title)
    .bind(content)
    .bind(frontmatter)
    .bind(size_bytes)
    .fetch_one(pool)
    .await?;

    let id: i32 = result.get("id");
    Ok(id)
}

/// Update an existing document
pub async fn update_document(
    pool: &DbPool,
    id: i32,
    title: Option<&str>,
    content: Option<&str>,
    frontmatter: Option<&str>,
) -> Result<()> {
    let size_bytes = content.map(|c| c.len() as i32);

    sqlx::query(
        r#"
        UPDATE documents
        SET title = COALESCE($2, title),
            content = COALESCE($3, content),
            frontmatter = COALESCE($4, frontmatter),
            size_bytes = COALESCE($5, size_bytes),
            modified_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(title)
    .bind(content)
    .bind(frontmatter)
    .bind(size_bytes)
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete a document by ID
pub async fn delete_document(pool: &DbPool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM documents WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
