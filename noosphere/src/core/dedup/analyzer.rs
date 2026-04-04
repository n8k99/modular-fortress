//! Duplicate document analyzer
//!
//! Finds clusters of documents that share the same title, indicating potential duplicates.
//! Uses efficient SQL grouping to identify candidates without loading full content.

use anyhow::Result;
use std::collections::HashMap;
use tracing::{debug, info};

use crate::db::DbPool;
use super::types::{DocInfo, DuplicateCluster};

/// Find all documents that share titles with other documents
///
/// This function identifies "duplicate clusters" - groups of documents that have
/// the same title. These are candidates for deduplication.
///
/// # Arguments
/// * `pool` - Database connection pool
///
/// # Returns
/// A vector of `DuplicateCluster`s, each containing 2+ documents with the same title.
/// Clusters are sorted by document count (most duplicates first).
///
/// # Example
/// ```ignore
/// let clusters = find_duplicates_by_title(&pool).await?;
/// for cluster in clusters {
///     println!("{}: {} copies", cluster.title, cluster.count());
/// }
/// ```
pub async fn find_duplicates_by_title(pool: &DbPool) -> Result<Vec<DuplicateCluster>> {
    info!("Scanning for duplicate documents by title...");

    // Step 1: Find titles that appear more than once
    let duplicate_titles: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT title, COUNT(*) as cnt
        FROM documents
        WHERE title IS NOT NULL AND title != ''
        GROUP BY title
        HAVING COUNT(*) > 1
        ORDER BY COUNT(*) DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    info!("Found {} titles with duplicates", duplicate_titles.len());

    if duplicate_titles.is_empty() {
        return Ok(vec![]);
    }

    // Step 2: Fetch all documents for these titles
    // We batch this to avoid massive IN clauses
    let titles: Vec<String> = duplicate_titles.iter().map(|(t, _)| t.clone()).collect();
    let mut clusters: HashMap<String, Vec<DocInfo>> = HashMap::new();

    // Process in batches of 100 titles
    for chunk in titles.chunks(100) {
        let placeholders: String = chunk.iter()
            .enumerate()
            .map(|(i, _)| format!("${}", i + 1))
            .collect::<Vec<_>>()
            .join(", ");

        let query = format!(
            r#"
            SELECT id, path, title, size_bytes, modified_at::timestamp as modified_at
            FROM documents
            WHERE title IN ({})
            ORDER BY title, path
            "#,
            placeholders
        );

        // Build the query dynamically
        let mut q = sqlx::query_as::<_, DocInfo>(&query);
        for title in chunk {
            q = q.bind(title);
        }

        let docs: Vec<DocInfo> = q.fetch_all(pool).await?;

        // Group documents by title
        for doc in docs {
            if let Some(ref title) = doc.title {
                clusters.entry(title.clone()).or_default().push(doc);
            }
        }
    }

    // Step 3: Convert to DuplicateCluster vec
    let mut result: Vec<DuplicateCluster> = clusters
        .into_iter()
        .filter(|(_, docs)| docs.len() > 1) // Safety check
        .map(|(title, docs)| DuplicateCluster::new(title, docs))
        .collect();

    // Sort by document count (most duplicates first)
    result.sort_by(|a, b| b.count().cmp(&a.count()));

    info!("Built {} duplicate clusters", result.len());
    debug!("Top clusters: {:?}", result.iter().take(5).map(|c| (&c.title, c.count())).collect::<Vec<_>>());

    Ok(result)
}

/// Find duplicates for a specific title
///
/// Useful for investigating a single title without scanning the entire database.
pub async fn find_duplicates_for_title(pool: &DbPool, title: &str) -> Result<Option<DuplicateCluster>> {
    let docs: Vec<DocInfo> = sqlx::query_as(
        r#"
        SELECT id, path, title, size_bytes, modified_at::timestamp as modified_at
        FROM documents
        WHERE title = $1
        ORDER BY path
        "#
    )
    .bind(title)
    .fetch_all(pool)
    .await?;

    if docs.len() > 1 {
        Ok(Some(DuplicateCluster::new(title.to_string(), docs)))
    } else {
        Ok(None)
    }
}

/// Get statistics about duplicates in the database
#[derive(Debug, Clone)]
pub struct DuplicateStats {
    /// Total documents in database
    pub total_documents: i64,
    /// Documents with duplicate titles
    pub documents_with_duplicates: i64,
    /// Number of unique titles that have duplicates
    pub duplicate_title_count: i64,
    /// Maximum number of copies for any single title
    pub max_copies: i64,
    /// Title with most copies
    pub most_duplicated_title: Option<String>,
}

/// Get statistics about document duplication
pub async fn get_duplicate_stats(pool: &DbPool) -> Result<DuplicateStats> {
    // Total documents
    let (total_documents,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM documents")
        .fetch_one(pool)
        .await?;

    // Documents with duplicate titles and stats
    let stats: Option<(i64, i64, i64, String)> = sqlx::query_as(
        r#"
        WITH dup_titles AS (
            SELECT title, COUNT(*) as cnt
            FROM documents
            WHERE title IS NOT NULL AND title != ''
            GROUP BY title
            HAVING COUNT(*) > 1
        )
        SELECT 
            COUNT(*) as title_count,
            SUM(cnt) as doc_count,
            MAX(cnt) as max_copies,
            (SELECT title FROM dup_titles ORDER BY cnt DESC LIMIT 1) as top_title
        FROM dup_titles
        "#
    )
    .fetch_optional(pool)
    .await?;

    match stats {
        Some((duplicate_title_count, documents_with_duplicates, max_copies, most_duplicated_title)) => {
            Ok(DuplicateStats {
                total_documents,
                documents_with_duplicates,
                duplicate_title_count,
                max_copies,
                most_duplicated_title: Some(most_duplicated_title),
            })
        }
        None => {
            Ok(DuplicateStats {
                total_documents,
                documents_with_duplicates: 0,
                duplicate_title_count: 0,
                max_copies: 0,
                most_duplicated_title: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests would require a test database connection
    // For unit testing, we could mock the pool or use a test database
}
