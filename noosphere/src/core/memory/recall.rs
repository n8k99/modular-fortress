//! Semantic recall using pgvector
//!
//! Search memories by embedding similarity.

use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::FromRow;

use crate::db::DbPool;

/// Result from semantic recall
#[derive(Debug, Clone)]
pub struct RecallResult {
    pub id: i64,
    pub agent_id: Option<String>,
    pub content: String,
    pub category: Option<String>,
    pub similarity: f32,
    pub source: RecallSource,
    pub created_at: Option<NaiveDateTime>,
}

/// Source of recalled memory
#[derive(Debug, Clone, PartialEq)]
pub enum RecallSource {
    DailyLog,
    MemoryEntry,
}

impl std::fmt::Display for RecallSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecallSource::DailyLog => write!(f, "daily_log"),
            RecallSource::MemoryEntry => write!(f, "memory"),
        }
    }
}

/// Row from daily_logs with similarity score
#[derive(Debug, FromRow)]
struct DailyLogWithSimilarity {
    id: i64,
    agent_id: Option<String>,
    content: String,
    category: Option<String>,
    similarity: f32,
    created_at: Option<NaiveDateTime>,
}

/// Row from memory_entries with similarity score
#[derive(Debug, FromRow)]
struct MemoryWithSimilarity {
    id: i64,
    agent_id: Option<String>,
    content: String,
    entry_type: Option<String>,
    similarity: f32,
    created_at: Option<NaiveDateTime>,
}

/// Semantic recall manager
pub struct SemanticRecall;

impl SemanticRecall {
    /// Recall memories for a specific agent using embedding similarity
    /// 
    /// Requires the query to already be embedded. For raw text queries,
    /// use `recall_text` which will search by content.
    pub async fn recall(
        pool: &DbPool,
        query_embedding: &[f32],
        agent_id: &str,
        limit: i64,
    ) -> Result<Vec<RecallResult>> {
        let mut results = Vec::new();
        
        // Search daily_logs
        let logs = Self::search_daily_logs(pool, query_embedding, Some(agent_id), limit).await?;
        results.extend(logs);
        
        // Search memory_entries
        let memories = Self::search_memories(pool, query_embedding, Some(agent_id), limit).await?;
        results.extend(memories);
        
        // Sort by similarity and take top results
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit as usize);
        
        Ok(results)
    }

    /// Recall memories across all agents (cross-agent search)
    pub async fn recall_cross_agent(
        pool: &DbPool,
        query_embedding: &[f32],
        limit: i64,
    ) -> Result<Vec<RecallResult>> {
        let mut results = Vec::new();
        
        // Search daily_logs across all agents
        let logs = Self::search_daily_logs(pool, query_embedding, None, limit).await?;
        results.extend(logs);
        
        // Search memory_entries across all agents
        let memories = Self::search_memories(pool, query_embedding, None, limit).await?;
        results.extend(memories);
        
        // Sort by similarity and take top results
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit as usize);
        
        Ok(results)
    }

    /// Search daily_logs using pgvector cosine similarity
    async fn search_daily_logs(
        pool: &DbPool,
        query_embedding: &[f32],
        agent_id: Option<&str>,
        limit: i64,
    ) -> Result<Vec<RecallResult>> {
        let embedding_str = Self::embedding_to_pg_vector(query_embedding);
        
        let rows = if let Some(aid) = agent_id {
            sqlx::query_as::<_, DailyLogWithSimilarity>(
                r#"
                SELECT id, agent_id, content, category,
                       1 - (embedding <=> $1::vector) as similarity,
                       created_at::timestamp as created_at
                FROM daily_logs
                WHERE embedding IS NOT NULL AND agent_id = $2
                ORDER BY embedding <=> $1::vector
                LIMIT $3
                "#,
            )
            .bind(&embedding_str)
            .bind(aid)
            .bind(limit)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, DailyLogWithSimilarity>(
                r#"
                SELECT id, agent_id, content, category,
                       1 - (embedding <=> $1::vector) as similarity,
                       created_at::timestamp as created_at
                FROM daily_logs
                WHERE embedding IS NOT NULL
                ORDER BY embedding <=> $1::vector
                LIMIT $2
                "#,
            )
            .bind(&embedding_str)
            .bind(limit)
            .fetch_all(pool)
            .await?
        };

        Ok(rows.into_iter().map(|row| RecallResult {
            id: row.id,
            agent_id: row.agent_id,
            content: row.content,
            category: row.category,
            similarity: row.similarity,
            source: RecallSource::DailyLog,
            created_at: row.created_at,
        }).collect())
    }

    /// Search memory_entries using pgvector cosine similarity
    async fn search_memories(
        pool: &DbPool,
        query_embedding: &[f32],
        agent_id: Option<&str>,
        limit: i64,
    ) -> Result<Vec<RecallResult>> {
        let embedding_str = Self::embedding_to_pg_vector(query_embedding);
        
        let rows = if let Some(aid) = agent_id {
            sqlx::query_as::<_, MemoryWithSimilarity>(
                r#"
                SELECT id, agent_id, content, entry_type,
                       1 - (embedding <=> $1::vector) as similarity,
                       created_at::timestamp as created_at
                FROM memory_entries
                WHERE embedding IS NOT NULL AND agent_id = $2
                ORDER BY embedding <=> $1::vector
                LIMIT $3
                "#,
            )
            .bind(&embedding_str)
            .bind(aid)
            .bind(limit)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, MemoryWithSimilarity>(
                r#"
                SELECT id, agent_id, content, entry_type,
                       1 - (embedding <=> $1::vector) as similarity,
                       created_at::timestamp as created_at
                FROM memory_entries
                WHERE embedding IS NOT NULL
                ORDER BY embedding <=> $1::vector
                LIMIT $2
                "#,
            )
            .bind(&embedding_str)
            .bind(limit)
            .fetch_all(pool)
            .await?
        };

        Ok(rows.into_iter().map(|row| RecallResult {
            id: row.id,
            agent_id: row.agent_id,
            content: row.content,
            category: row.entry_type,
            similarity: row.similarity,
            source: RecallSource::MemoryEntry,
            created_at: row.created_at,
        }).collect())
    }

    /// Convert embedding slice to PostgreSQL vector string format
    fn embedding_to_pg_vector(embedding: &[f32]) -> String {
        let values: Vec<String> = embedding.iter().map(|v| v.to_string()).collect();
        format!("[{}]", values.join(","))
    }

    /// Text-based recall using keyword matching (fallback when no embedding)
    pub async fn recall_text(
        pool: &DbPool,
        query: &str,
        agent_id: &str,
        limit: i64,
    ) -> Result<Vec<RecallResult>> {
        let pattern = format!("%{}%", query);
        let mut results = Vec::new();

        // Search daily_logs by content
        let logs = sqlx::query_as::<_, DailyLogWithSimilarity>(
            r#"
            SELECT id, agent_id, content, category,
                   0.5::real as similarity,
                   created_at::timestamp as created_at
            FROM daily_logs
            WHERE agent_id = $1 AND content ILIKE $2
            ORDER BY created_at DESC
            LIMIT $3
            "#,
        )
        .bind(agent_id)
        .bind(&pattern)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        for row in logs {
            results.push(RecallResult {
                id: row.id,
                agent_id: row.agent_id,
                content: row.content,
                category: row.category,
                similarity: row.similarity,
                source: RecallSource::DailyLog,
                created_at: row.created_at,
            });
        }

        // Search memory_entries by content
        let memories = sqlx::query_as::<_, MemoryWithSimilarity>(
            r#"
            SELECT id, agent_id, content, entry_type,
                   0.5::real as similarity,
                   created_at::timestamp as created_at
            FROM memory_entries
            WHERE agent_id = $1 AND content ILIKE $2
            ORDER BY importance DESC NULLS LAST, created_at DESC
            LIMIT $3
            "#,
        )
        .bind(agent_id)
        .bind(&pattern)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        for row in memories {
            results.push(RecallResult {
                id: row.id,
                agent_id: row.agent_id,
                content: row.content,
                category: row.entry_type,
                similarity: row.similarity,
                source: RecallSource::MemoryEntry,
                created_at: row.created_at,
            });
        }

        results.truncate(limit as usize);
        Ok(results)
    }
}

/// Store an embedding for a daily log
pub async fn set_log_embedding(pool: &DbPool, id: i64, embedding: &[f32]) -> Result<()> {
    let embedding_str = SemanticRecall::embedding_to_pg_vector(embedding);
    
    sqlx::query(
        "UPDATE daily_logs SET embedding = $1::vector WHERE id = $2"
    )
    .bind(&embedding_str)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Store an embedding for a memory entry
pub async fn set_memory_embedding(pool: &DbPool, id: i64, embedding: &[f32]) -> Result<()> {
    let embedding_str = SemanticRecall::embedding_to_pg_vector(embedding);
    
    sqlx::query(
        "UPDATE memory_entries SET embedding = $1::vector WHERE id = $2"
    )
    .bind(&embedding_str)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}
