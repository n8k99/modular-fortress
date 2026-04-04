//! Batch embedding processing
//!
//! Backfill embeddings for existing records with NULL embeddings.

use anyhow::Result;
use sqlx::FromRow;
use tokio::time::{sleep, Duration};
use tracing::{debug, info, warn};

use crate::db::DbPool;
use super::generator::EmbeddingService;

/// Target table for backfilling
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackfillTarget {
    DailyLogs,
    MemoryEntries,
    StagehandNotes,
    Memories,
}

impl BackfillTarget {
    pub fn table_name(&self) -> &'static str {
        match self {
            BackfillTarget::DailyLogs => "daily_logs",
            BackfillTarget::MemoryEntries => "memory_entries",
            BackfillTarget::StagehandNotes => "stagehand_notes",
            BackfillTarget::Memories => "memories",
        }
    }

    pub fn content_field(&self) -> &'static str {
        match self {
            BackfillTarget::DailyLogs => "content",
            BackfillTarget::MemoryEntries => "content",
            BackfillTarget::StagehandNotes => "notes",
            BackfillTarget::Memories => "content",
        }
    }

    pub fn id_field(&self) -> &'static str {
        match self {
            BackfillTarget::StagehandNotes => "id",
            BackfillTarget::Memories => "id",
            _ => "id",
        }
    }
}

/// Progress reporting for backfill operations
#[derive(Debug, Clone, Default)]
pub struct BackfillProgress {
    pub total: usize,
    pub processed: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub skipped: usize,
}

impl BackfillProgress {
    pub fn percent_complete(&self) -> f32 {
        if self.total == 0 {
            100.0
        } else {
            (self.processed as f32 / self.total as f32) * 100.0
        }
    }
}

/// Row with ID and content for embedding
#[derive(Debug, FromRow)]
struct EmbedRow {
    id: i64,
    content: Option<String>,
}

/// Backfill embeddings for records with NULL embedding
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `target` - Which table to backfill
/// * `batch_size` - Number of records to process per batch
/// * `rate_limit_ms` - Milliseconds to wait between requests (rate limiting)
///
/// # Returns
/// Progress report with counts
pub async fn backfill_embeddings(
    pool: &DbPool,
    target: BackfillTarget,
    batch_size: i64,
    rate_limit_ms: u64,
) -> Result<BackfillProgress> {
    let service = EmbeddingService::default_service();
    
    if !service.is_available().await {
        return Err(anyhow::anyhow!("No embedding service available"));
    }

    let table = target.table_name();
    let content_field = target.content_field();
    
    info!("Starting backfill for {} (batch_size={}, rate_limit={}ms)", 
          table, batch_size, rate_limit_ms);

    // Count total NULL embeddings
    let count_query = format!(
        "SELECT COUNT(*) as count FROM {} WHERE embedding IS NULL",
        table
    );
    let total: (i64,) = sqlx::query_as(&count_query)
        .fetch_one(pool)
        .await?;
    
    let mut progress = BackfillProgress {
        total: total.0 as usize,
        ..Default::default()
    };

    if progress.total == 0 {
        info!("No records need embedding in {}", table);
        return Ok(progress);
    }

    info!("Found {} records needing embeddings", progress.total);

    // Process in batches
    loop {
        let query = format!(
            "SELECT id, {} as content FROM {} WHERE embedding IS NULL LIMIT {}",
            content_field, table, batch_size
        );
        
        let rows: Vec<EmbedRow> = sqlx::query_as(&query)
            .fetch_all(pool)
            .await?;

        if rows.is_empty() {
            break;
        }

        for row in rows {
            progress.processed += 1;

            // Skip if no content
            let content = match &row.content {
                Some(c) if !c.trim().is_empty() => c.clone(),
                _ => {
                    debug!("Skipping {} id={} (no content)", table, row.id);
                    progress.skipped += 1;
                    continue;
                }
            };

            // Generate embedding
            match service.generate(&content).await {
                Ok(embedding) => {
                    // Store embedding
                    if let Err(e) = store_embedding(pool, target, row.id, &embedding).await {
                        warn!("Failed to store embedding for {} id={}: {}", table, row.id, e);
                        progress.failed += 1;
                    } else {
                        debug!("Embedded {} id={} ({} dims)", table, row.id, embedding.len());
                        progress.succeeded += 1;
                    }
                }
                Err(e) => {
                    warn!("Failed to generate embedding for {} id={}: {}", table, row.id, e);
                    progress.failed += 1;
                }
            }

            // Rate limiting
            if rate_limit_ms > 0 {
                sleep(Duration::from_millis(rate_limit_ms)).await;
            }

            // Progress logging every 10 records
            if progress.processed % 10 == 0 {
                info!(
                    "Progress: {}/{} ({:.1}%) - {} succeeded, {} failed, {} skipped",
                    progress.processed, progress.total, progress.percent_complete(),
                    progress.succeeded, progress.failed, progress.skipped
                );
            }
        }
    }

    info!(
        "Backfill complete for {}: {} processed, {} succeeded, {} failed, {} skipped",
        table, progress.processed, progress.succeeded, progress.failed, progress.skipped
    );

    Ok(progress)
}

/// Store embedding for a specific record
async fn store_embedding(
    pool: &DbPool,
    target: BackfillTarget,
    id: i64,
    embedding: &[f32],
) -> Result<()> {
    let embedding_str = embedding_to_pg_vector(embedding);
    let table = target.table_name();
    
    let query = format!(
        "UPDATE {} SET embedding = $1::vector WHERE id = $2",
        table
    );
    
    sqlx::query(&query)
        .bind(&embedding_str)
        .bind(id)
        .execute(pool)
        .await?;
    
    Ok(())
}

/// Convert embedding slice to PostgreSQL vector string format
fn embedding_to_pg_vector(embedding: &[f32]) -> String {
    let values: Vec<String> = embedding.iter().map(|v| v.to_string()).collect();
    format!("[{}]", values.join(","))
}

/// Backfill all tables in order
pub async fn backfill_all(
    pool: &DbPool,
    batch_size: i64,
    rate_limit_ms: u64,
) -> Result<Vec<(BackfillTarget, BackfillProgress)>> {
    let targets = [
        BackfillTarget::DailyLogs,
        BackfillTarget::MemoryEntries,
        BackfillTarget::StagehandNotes,
    ];
    
    let mut results = Vec::new();
    
    for target in targets {
        let progress = backfill_embeddings(pool, target, batch_size, rate_limit_ms).await?;
        results.push((target, progress));
    }
    
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backfill_target() {
        assert_eq!(BackfillTarget::DailyLogs.table_name(), "daily_logs");
        assert_eq!(BackfillTarget::DailyLogs.content_field(), "content");
        assert_eq!(BackfillTarget::StagehandNotes.content_field(), "notes");
    }

    #[test]
    fn test_progress_percent() {
        let mut p = BackfillProgress::default();
        assert_eq!(p.percent_complete(), 100.0);
        
        p.total = 100;
        p.processed = 50;
        assert_eq!(p.percent_complete(), 50.0);
    }
}
