//! Context injection - pull related memories for a document
//!
//! Given content (e.g., today's notes), find semantically similar
//! records across all memory sources.

use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::FromRow;
use tracing::debug;

use crate::db::DbPool;
use crate::embeddings::generator::EmbeddingService;
use super::relevance::RelevanceScorer;

/// Source of context result
#[derive(Debug, Clone, PartialEq)]
pub enum ContextSource {
    DailyLog,
    MemoryEntry,
    StagehandNote,
    Memory,
}

impl std::fmt::Display for ContextSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextSource::DailyLog => write!(f, "daily_log"),
            ContextSource::MemoryEntry => write!(f, "memory"),
            ContextSource::StagehandNote => write!(f, "stagehand"),
            ContextSource::Memory => write!(f, "memory"),
        }
    }
}

/// Result from context injection
#[derive(Debug, Clone)]
pub struct ContextResult {
    pub id: i64,
    pub content: String,
    pub source: ContextSource,
    pub similarity: f32,
    pub relevance_score: f32,
    pub created_at: Option<NaiveDateTime>,
    pub metadata: Option<String>,
}

impl ContextResult {
    /// Combined score (similarity * relevance weight)
    pub fn combined_score(&self) -> f32 {
        self.similarity * self.relevance_score
    }
}

/// Row from daily_logs semantic search
#[derive(Debug, FromRow)]
struct DailyLogRow {
    id: i64,
    content: String,
    category: Option<String>,
    similarity: f32,
    created_at: Option<NaiveDateTime>,
}

/// Row from memory_entries semantic search
#[derive(Debug, FromRow)]
struct MemoryRow {
    id: i64,
    content: String,
    entry_type: Option<String>,
    similarity: f32,
    created_at: Option<NaiveDateTime>,
}

/// Row from stagehand_notes semantic search
#[derive(Debug, FromRow)]
struct StagehandRow {
    id: i32,
    show_name: String,
    venue: Option<String>,
    notes: Option<String>,
    similarity: f32,
    created_at: Option<NaiveDateTime>,
}

/// Row from memories semantic search
#[derive(Debug, FromRow)]
struct VaultRow {
    id: i32,
    title: Option<String>,
    content: String,
    similarity: f32,
    created_at: Option<NaiveDateTime>,
}

/// Get related context for given content
///
/// Embeds the content, then searches across all memory sources
/// for semantically similar records. Results are scored for relevance
/// and sorted by combined score.
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `content` - The content to find related context for
/// * `limit` - Maximum number of results to return
///
/// # Returns
/// Vector of context results, sorted by relevance
pub async fn get_related_context(
    pool: &DbPool,
    content: &str,
    limit: i64,
) -> Result<Vec<ContextResult>> {
    let service = EmbeddingService::default_service();
    
    if !service.is_available().await {
        return Err(anyhow::anyhow!("No embedding service available"));
    }
    
    // Generate embedding for the query content
    let embedding = service.generate(content).await?;
    let embedding_str = embedding_to_pg_vector(&embedding);
    
    debug!("Searching for related context ({} chars -> {} dims)", content.len(), embedding.len());
    
    // Search each source (query more than limit, we'll combine and re-rank)
    let per_source_limit = (limit * 2).max(10);
    
    let mut all_results = Vec::new();
    
    // Search daily_logs
    let logs = search_daily_logs(pool, &embedding_str, per_source_limit).await?;
    all_results.extend(logs);
    
    // Search memory_entries
    let memories = search_memory_entries(pool, &embedding_str, per_source_limit).await?;
    all_results.extend(memories);
    
    // Search stagehand_notes
    let stagehand = search_stagehand_notes(pool, &embedding_str, per_source_limit).await?;
    all_results.extend(stagehand);
    
    // Search memories
    let vault = search_memories(pool, &embedding_str, per_source_limit).await?;
    all_results.extend(vault);
    
    debug!("Found {} total results across all sources", all_results.len());
    
    // Apply relevance scoring
    let scorer = RelevanceScorer::default();
    for result in &mut all_results {
        result.relevance_score = scorer.score(result);
    }
    
    // Sort by combined score
    all_results.sort_by(|a, b| {
        b.combined_score()
            .partial_cmp(&a.combined_score())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Take top results
    all_results.truncate(limit as usize);
    
    Ok(all_results)
}

/// Search daily_logs by embedding similarity
async fn search_daily_logs(
    pool: &DbPool,
    embedding_str: &str,
    limit: i64,
) -> Result<Vec<ContextResult>> {
    let rows = sqlx::query_as::<_, DailyLogRow>(
        r#"
        SELECT id, content, category,
               1 - (embedding <=> $1::vector) as similarity,
               created_at::timestamp as created_at
        FROM daily_logs
        WHERE embedding IS NOT NULL
        ORDER BY embedding <=> $1::vector
        LIMIT $2
        "#,
    )
    .bind(embedding_str)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(rows.into_iter().map(|row| ContextResult {
        id: row.id,
        content: row.content,
        source: ContextSource::DailyLog,
        similarity: row.similarity,
        relevance_score: 1.0, // Will be calculated later
        created_at: row.created_at,
        metadata: row.category,
    }).collect())
}

/// Search memory_entries by embedding similarity
async fn search_memory_entries(
    pool: &DbPool,
    embedding_str: &str,
    limit: i64,
) -> Result<Vec<ContextResult>> {
    let rows = sqlx::query_as::<_, MemoryRow>(
        r#"
        SELECT id, content, entry_type,
               1 - (embedding <=> $1::vector) as similarity,
               created_at::timestamp as created_at
        FROM memory_entries
        WHERE embedding IS NOT NULL
        ORDER BY embedding <=> $1::vector
        LIMIT $2
        "#,
    )
    .bind(embedding_str)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(rows.into_iter().map(|row| ContextResult {
        id: row.id,
        content: row.content,
        source: ContextSource::MemoryEntry,
        similarity: row.similarity,
        relevance_score: 1.0,
        created_at: row.created_at,
        metadata: row.entry_type,
    }).collect())
}

/// Search stagehand_notes by embedding similarity
async fn search_stagehand_notes(
    pool: &DbPool,
    embedding_str: &str,
    limit: i64,
) -> Result<Vec<ContextResult>> {
    let rows = sqlx::query_as::<_, StagehandRow>(
        r#"
        SELECT id, show_name, venue, notes,
               1 - (embedding <=> $1::vector) as similarity,
               created_at::timestamp as created_at
        FROM stagehand_notes
        WHERE embedding IS NOT NULL
        ORDER BY embedding <=> $1::vector
        LIMIT $2
        "#,
    )
    .bind(embedding_str)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(rows.into_iter().map(|row| {
        // Combine show info into content
        let content = if let Some(notes) = &row.notes {
            format!("{} @ {}: {}", 
                row.show_name, 
                row.venue.as_deref().unwrap_or("TBD"),
                notes
            )
        } else {
            format!("{} @ {}", 
                row.show_name, 
                row.venue.as_deref().unwrap_or("TBD")
            )
        };
        
        ContextResult {
            id: row.id as i64,
            content,
            source: ContextSource::StagehandNote,
            similarity: row.similarity,
            relevance_score: 1.0,
            created_at: row.created_at,
            metadata: Some(row.show_name),
        }
    }).collect())
}

/// Search memories by embedding similarity
async fn search_memories(
    pool: &DbPool,
    embedding_str: &str,
    limit: i64,
) -> Result<Vec<ContextResult>> {
    let rows = sqlx::query_as::<_, VaultRow>(
        r#"
        SELECT id, title, content,
               1 - (embedding <=> $1::vector) as similarity,
               created_at::timestamp as created_at
        FROM memories
        WHERE embedding IS NOT NULL
        ORDER BY embedding <=> $1::vector
        LIMIT $2
        "#,
    )
    .bind(embedding_str)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(rows.into_iter().map(|row| ContextResult {
        id: row.id as i64,
        content: row.content,
        source: ContextSource::Memory,
        similarity: row.similarity,
        relevance_score: 1.0,
        created_at: row.created_at,
        metadata: row.title,
    }).collect())
}

/// Convert embedding slice to PostgreSQL vector string format
fn embedding_to_pg_vector(embedding: &[f32]) -> String {
    let values: Vec<String> = embedding.iter().map(|v| v.to_string()).collect();
    format!("[{}]", values.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_source_display() {
        assert_eq!(ContextSource::DailyLog.to_string(), "daily_log");
        assert_eq!(ContextSource::StagehandNote.to_string(), "stagehand");
    }

    #[test]
    fn test_combined_score() {
        let result = ContextResult {
            id: 1,
            content: "test".to_string(),
            source: ContextSource::DailyLog,
            similarity: 0.9,
            relevance_score: 0.8,
            created_at: None,
            metadata: None,
        };
        assert!((result.combined_score() - 0.72).abs() < 0.001);
    }
}
