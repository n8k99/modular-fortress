//! Auto-embedding triggers for insert operations
//!
//! Wrapper functions that automatically generate embeddings on insert.

use anyhow::Result;
use tracing::{debug, warn};

use crate::db::DbPool;
use crate::memory::store::{DailyLogCreate, MemoryEntryCreate};
use crate::db::stagehand::StagehandNoteCreate;
use super::generator::EmbeddingService;

/// Write a daily log with automatic embedding generation
///
/// Creates the log entry first, then generates and stores the embedding.
/// If embedding generation fails, the log is still created (embedding remains NULL).
pub async fn write_log_with_embedding(pool: &DbPool, log: &DailyLogCreate) -> Result<i64> {
    // First, write the log
    let id = crate::memory::store::write_log(pool, log).await?;
    
    // Then try to generate and store embedding
    let service = EmbeddingService::default_service();
    
    if service.is_available().await {
        match service.generate(&log.content).await {
            Ok(embedding) => {
                if let Err(e) = crate::memory::recall::set_log_embedding(pool, id, &embedding).await {
                    warn!("Failed to store log embedding: {}", e);
                } else {
                    debug!("Generated embedding for daily_log id={}", id);
                }
            }
            Err(e) => {
                warn!("Failed to generate embedding for daily_log id={}: {}", id, e);
            }
        }
    } else {
        debug!("No embedding service available, skipping embedding for daily_log id={}", id);
    }
    
    Ok(id)
}

/// Write a memory entry with automatic embedding generation
///
/// Creates the memory entry first, then generates and stores the embedding.
/// If embedding generation fails, the entry is still created (embedding remains NULL).
pub async fn write_memory_with_embedding(pool: &DbPool, entry: &MemoryEntryCreate) -> Result<i64> {
    // First, write the memory
    let id = crate::memory::store::write_memory(pool, entry).await?;
    
    // Then try to generate and store embedding
    let service = EmbeddingService::default_service();
    
    if service.is_available().await {
        match service.generate(&entry.content).await {
            Ok(embedding) => {
                if let Err(e) = crate::memory::recall::set_memory_embedding(pool, id, &embedding).await {
                    warn!("Failed to store memory embedding: {}", e);
                } else {
                    debug!("Generated embedding for memory_entry id={}", id);
                }
            }
            Err(e) => {
                warn!("Failed to generate embedding for memory_entry id={}: {}", id, e);
            }
        }
    } else {
        debug!("No embedding service available, skipping embedding for memory_entry id={}", id);
    }
    
    Ok(id)
}

/// Create a stagehand note with automatic embedding generation
///
/// Creates the note first, then generates and stores the embedding based on
/// the combined show_name, venue, and notes content.
/// If embedding generation fails, the note is still created (embedding remains NULL).
pub async fn create_stagehand_note_with_embedding(pool: &DbPool, note: &StagehandNoteCreate) -> Result<i32> {
    // First, create the note
    let id = crate::db::stagehand::create(pool, note).await?;
    
    // Build content for embedding (combine relevant fields)
    let embed_content = build_stagehand_embed_content(note);
    
    if embed_content.is_empty() {
        debug!("No content to embed for stagehand_note id={}", id);
        return Ok(id);
    }
    
    // Try to generate and store embedding
    let service = EmbeddingService::default_service();
    
    if service.is_available().await {
        match service.generate(&embed_content).await {
            Ok(embedding) => {
                if let Err(e) = set_stagehand_embedding(pool, id, &embedding).await {
                    warn!("Failed to store stagehand embedding: {}", e);
                } else {
                    debug!("Generated embedding for stagehand_note id={}", id);
                }
            }
            Err(e) => {
                warn!("Failed to generate embedding for stagehand_note id={}: {}", id, e);
            }
        }
    } else {
        debug!("No embedding service available, skipping embedding for stagehand_note id={}", id);
    }
    
    Ok(id)
}

/// Build content string for stagehand note embedding
fn build_stagehand_embed_content(note: &StagehandNoteCreate) -> String {
    let mut parts = Vec::new();
    
    parts.push(format!("Show: {}", note.show_name));
    
    if let Some(venue) = &note.venue {
        parts.push(format!("Venue: {}", venue));
    }
    
    if let Some(notes) = &note.notes {
        if !notes.trim().is_empty() {
            parts.push(notes.clone());
        }
    }
    
    if let Some(tags) = &note.tags {
        if !tags.is_empty() {
            parts.push(format!("Tags: {}", tags.join(", ")));
        }
    }
    
    parts.join("\n")
}

/// Store embedding for a stagehand note
async fn set_stagehand_embedding(pool: &DbPool, id: i32, embedding: &[f32]) -> Result<()> {
    let embedding_str = embedding_to_pg_vector(embedding);
    
    sqlx::query(
        "UPDATE stagehand_notes SET embedding = $1::vector WHERE id = $2"
    )
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_build_stagehand_embed_content() {
        let note = StagehandNoteCreate {
            show_name: "Hamilton".to_string(),
            venue: Some("Richard Rodgers Theatre".to_string()),
            event_date: NaiveDate::from_ymd_opt(2025, 3, 15).unwrap(),
            call_time: None,
            notes: Some("Load in at 8am, bring extra stingers".to_string()),
            tags: Some(vec!["broadway".to_string(), "musical".to_string()]),
        };
        
        let content = build_stagehand_embed_content(&note);
        assert!(content.contains("Hamilton"));
        assert!(content.contains("Richard Rodgers Theatre"));
        assert!(content.contains("stingers"));
        assert!(content.contains("broadway"));
    }
}
