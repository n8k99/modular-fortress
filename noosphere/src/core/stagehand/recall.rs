//! Stagehand recall - semantic search for past show memories
//!
//! TODO: Implement embedding-based semantic search when vector support is ready.
//! For now, provides text-based recall using ILIKE queries.

use anyhow::Result;
use chrono::NaiveDate;

use crate::db::DbPool;
use crate::db::stagehand::StagehandNote;

/// Recall context from past shows
pub struct RecallResult {
    /// Matching note
    pub note: StagehandNote,
    /// Relevance score (0.0 - 1.0)
    pub relevance: f32,
    /// Why this was recalled
    pub reason: String,
}

/// Recall memories for a specific show
pub async fn recall_for_show(pool: &DbPool, show_name: &str) -> Result<Vec<RecallResult>> {
    let notes = crate::db::stagehand::search_by_show(pool, show_name).await?;
    
    Ok(notes
        .into_iter()
        .map(|note| RecallResult {
            reason: format!("Previous {} show", show_name),
            relevance: 1.0,
            note,
        })
        .collect())
}

/// Recall memories for a specific venue
pub async fn recall_for_venue(pool: &DbPool, venue: &str) -> Result<Vec<RecallResult>> {
    let notes = crate::db::stagehand::search_by_venue(pool, venue).await?;
    
    Ok(notes
        .into_iter()
        .map(|note| RecallResult {
            reason: format!("Previous show at {}", venue),
            relevance: 0.9,
            note,
        })
        .collect())
}

/// Recall memories for an upcoming show (combines show + venue history)
pub async fn recall_for_upcoming(
    pool: &DbPool,
    show_name: &str,
    venue: Option<&str>,
    _date: NaiveDate,
) -> Result<Vec<RecallResult>> {
    let mut results = Vec::new();
    
    // Get show history
    let show_notes = recall_for_show(pool, show_name).await?;
    results.extend(show_notes);
    
    // Get venue history if provided
    if let Some(v) = venue {
        let venue_notes = recall_for_venue(pool, v).await?;
        results.extend(venue_notes);
    }
    
    // Sort by relevance descending
    results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
    
    // Dedupe by note ID
    let mut seen_ids = std::collections::HashSet::new();
    results.retain(|r| seen_ids.insert(r.note.id));
    
    Ok(results)
}

/// Search notes with text query (placeholder for semantic search)
pub async fn semantic_search(pool: &DbPool, query: &str, limit: i64) -> Result<Vec<RecallResult>> {
    // For now, use simple text matching
    // TODO: When embeddings are ready, use vector similarity search
    
    let pattern = format!("%{}%", query);
    
    let notes = sqlx::query_as::<_, StagehandNote>(
        r#"
        SELECT id, show_name, venue, event_date, call_time, notes, tags,
               created_at::timestamp as created_at,
               modified_at::timestamp as modified_at
        FROM stagehand_notes
        WHERE show_name ILIKE $1 OR venue ILIKE $1 OR notes ILIKE $1
        ORDER BY event_date DESC
        LIMIT $2
        "#,
    )
    .bind(&pattern)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(notes
        .into_iter()
        .map(|note| RecallResult {
            reason: format!("Matches '{}'", query),
            relevance: 0.8,
            note,
        })
        .collect())
}
