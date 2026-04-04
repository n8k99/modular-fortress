//! Document migration for deduplication
//!
//! Moves non-canonical duplicate documents to a versions table,
//! preserving the content while cleaning up the main documents table.
//!
//! The versions table stores:
//! - Original document metadata
//! - Link to the canonical document
//! - Migration timestamp
//!
//! This is a one-way migration — run with caution!

use anyhow::{Context, Result};
use chrono::Utc;
use tracing::{debug, info, warn};

use crate::db::DbPool;
use super::analyzer::find_duplicates_by_title;
use super::canonicalizer::select_canonical;
use super::types::{DuplicateCluster, MigrationResult, MigrationSummary};

/// SQL to create the document_versions table if it doesn't exist
const CREATE_VERSIONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS document_versions (
    id SERIAL PRIMARY KEY,
    original_doc_id INTEGER NOT NULL,
    canonical_doc_id INTEGER NOT NULL,
    path TEXT NOT NULL,
    title TEXT,
    content TEXT,
    frontmatter TEXT,
    size_bytes INTEGER,
    original_modified_at TIMESTAMP,
    migrated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    -- Index for lookups
    CONSTRAINT fk_canonical 
        FOREIGN KEY (canonical_doc_id) 
        REFERENCES documents(id)
        ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_versions_canonical ON document_versions(canonical_doc_id);
CREATE INDEX IF NOT EXISTS idx_versions_original ON document_versions(original_doc_id);
"#;

/// Ensure the versions table exists
pub async fn ensure_versions_table(pool: &DbPool) -> Result<()> {
    info!("Ensuring document_versions table exists...");
    sqlx::query(CREATE_VERSIONS_TABLE)
        .execute(pool)
        .await
        .context("Failed to create document_versions table")?;
    info!("document_versions table ready");
    Ok(())
}

/// Migrate a single cluster's non-canonical documents to versions table
///
/// This function:
/// 1. Copies non-canonical documents to document_versions
/// 2. Deletes the originals from documents table
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `cluster` - The duplicate cluster to process
/// * `canonical_id` - ID of the canonical document (will NOT be migrated)
///
/// # Returns
/// `MigrationResult` with details of the operation
///
/// # Note
/// This operation is NOT atomic across the entire cluster.
/// Each document is migrated in its own transaction for safety.
pub async fn migrate_cluster(
    pool: &DbPool,
    cluster: &DuplicateCluster,
    canonical_id: i32,
) -> MigrationResult {
    let non_canonical = cluster.non_canonical(canonical_id);
    
    if non_canonical.is_empty() {
        return MigrationResult::success(cluster.title.clone(), canonical_id, vec![]);
    }

    info!(
        "Migrating cluster '{}': keeping {}, migrating {} documents",
        cluster.title,
        canonical_id,
        non_canonical.len()
    );

    let mut migrated_ids = Vec::new();

    for doc in non_canonical {
        match migrate_single_document(pool, doc.id, canonical_id).await {
            Ok(()) => {
                debug!("Migrated document {} to versions", doc.id);
                migrated_ids.push(doc.id);
            }
            Err(e) => {
                warn!("Failed to migrate document {}: {}", doc.id, e);
                return MigrationResult::failure(
                    cluster.title.clone(),
                    canonical_id,
                    format!("Failed on document {}: {}", doc.id, e),
                );
            }
        }
    }

    MigrationResult::success(cluster.title.clone(), canonical_id, migrated_ids)
}

/// Migrate a single document to versions table
async fn migrate_single_document(
    pool: &DbPool,
    doc_id: i32,
    canonical_id: i32,
) -> Result<()> {
    // Start a transaction
    let mut tx = pool.begin().await?;

    // Copy to versions table
    sqlx::query(
        r#"
        INSERT INTO document_versions 
            (original_doc_id, canonical_doc_id, path, title, content, frontmatter, 
             size_bytes, original_modified_at, migrated_at)
        SELECT 
            id, $2, path, title, content, frontmatter,
            size_bytes, modified_at, $3
        FROM documents
        WHERE id = $1
        "#
    )
    .bind(doc_id)
    .bind(canonical_id)
    .bind(Utc::now().naive_utc())
    .execute(&mut *tx)
    .await
    .context("Failed to copy document to versions table")?;

    // Delete from documents table
    sqlx::query("DELETE FROM documents WHERE id = $1")
        .bind(doc_id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete document from documents table")?;

    // Commit transaction
    tx.commit().await?;

    Ok(())
}

/// Migrate all duplicate clusters in the database
///
/// This is the main entry point for batch deduplication.
///
/// # Arguments
/// * `pool` - Database connection pool
///
/// # Returns
/// `MigrationSummary` with statistics and per-cluster results
///
/// # Warning
/// This modifies the database! Run with caution.
/// Consider using `dry_run_all` first to preview changes.
pub async fn migrate_all(pool: &DbPool) -> Result<MigrationSummary> {
    // Ensure versions table exists
    ensure_versions_table(pool).await?;

    // Find all duplicates
    let clusters = find_duplicates_by_title(pool).await?;
    
    info!("Starting migration of {} duplicate clusters", clusters.len());

    let mut summary = MigrationSummary::new();

    for cluster in clusters {
        let canonical_id = select_canonical(&cluster);
        let result = migrate_cluster(pool, &cluster, canonical_id).await;
        summary.add_result(result);
    }

    info!(
        "Migration complete: {} clusters processed, {} succeeded, {} failed, {} documents migrated",
        summary.clusters_processed,
        summary.clusters_succeeded,
        summary.clusters_failed,
        summary.documents_migrated
    );

    Ok(summary)
}

/// Dry run: preview what would be migrated without making changes
pub async fn dry_run_all(pool: &DbPool) -> Result<MigrationSummary> {
    let clusters = find_duplicates_by_title(pool).await?;
    
    info!("Dry run: analyzing {} duplicate clusters", clusters.len());

    let mut summary = MigrationSummary::new();

    for cluster in clusters {
        let canonical_id = select_canonical(&cluster);
        let non_canonical_ids: Vec<i32> = cluster
            .non_canonical(canonical_id)
            .iter()
            .map(|d| d.id)
            .collect();
        
        // Create a "would succeed" result for dry run
        let result = MigrationResult::success(
            cluster.title.clone(),
            canonical_id,
            non_canonical_ids,
        );
        summary.add_result(result);
    }

    info!(
        "Dry run complete: would process {} clusters, migrate {} documents",
        summary.clusters_processed,
        summary.documents_migrated
    );

    Ok(summary)
}

/// Migrate specific clusters by title
pub async fn migrate_by_titles(pool: &DbPool, titles: &[&str]) -> Result<MigrationSummary> {
    ensure_versions_table(pool).await?;

    let mut summary = MigrationSummary::new();

    for title in titles {
        let docs: Vec<super::types::DocInfo> = sqlx::query_as(
            r#"
            SELECT id, path, title, size_bytes, modified_at::timestamp as modified_at
            FROM documents
            WHERE title = $1
            ORDER BY path
            "#
        )
        .bind(*title)
        .fetch_all(pool)
        .await?;

        if docs.len() <= 1 {
            info!("Skipping '{}': not a duplicate", title);
            continue;
        }

        let cluster = DuplicateCluster::new(title.to_string(), docs);
        let canonical_id = select_canonical(&cluster);
        let result = migrate_cluster(pool, &cluster, canonical_id).await;
        summary.add_result(result);
    }

    Ok(summary)
}

/// Restore a document from versions back to documents table
///
/// Use this to undo a migration for a specific document.
pub async fn restore_from_version(pool: &DbPool, version_id: i32) -> Result<i32> {
    let mut tx = pool.begin().await?;

    // Get the version record
    let (original_id, path, title, content, frontmatter, size_bytes, modified_at): (
        i32, String, Option<String>, Option<String>, Option<String>, Option<i32>, Option<chrono::NaiveDateTime>
    ) = sqlx::query_as(
        r#"
        SELECT original_doc_id, path, title, content, frontmatter, size_bytes, original_modified_at
        FROM document_versions
        WHERE id = $1
        "#
    )
    .bind(version_id)
    .fetch_one(&mut *tx)
    .await
    .context("Version record not found")?;

    // Re-insert into documents (with new ID)
    let (new_id,): (i32,) = sqlx::query_as(
        r#"
        INSERT INTO documents (path, title, content, frontmatter, size_bytes, modified_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#
    )
    .bind(&path)
    .bind(&title)
    .bind(&content)
    .bind(&frontmatter)
    .bind(size_bytes)
    .bind(modified_at)
    .fetch_one(&mut *tx)
    .await?;

    // Delete from versions
    sqlx::query("DELETE FROM document_versions WHERE id = $1")
        .bind(version_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    info!(
        "Restored version {} (original doc {}) as new document {}",
        version_id, original_id, new_id
    );

    Ok(new_id)
}

#[cfg(test)]
mod tests {
    // Integration tests would require a test database
    // Consider using testcontainers or a separate test DB
}
