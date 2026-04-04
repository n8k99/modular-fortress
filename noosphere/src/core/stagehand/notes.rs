//! Stagehand notes management
//!
//! CRUD operations and ICS import for show notes.

use anyhow::Result;
use chrono::NaiveDate;

use crate::db::{self, DbPool};
use crate::db::stagehand::{StagehandNote, StagehandNoteCreate};
use crate::ics::parser::IcsEvent;

/// High-level stagehand note manager
pub struct StagehandManager {
    pool: DbPool,
}

impl StagehandManager {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// List all stagehand notes
    pub async fn list_all(&self) -> Result<Vec<StagehandNote>> {
        db::stagehand::list_all(&self.pool).await
    }

    /// Get notes for a specific date
    pub async fn get_by_date(&self, date: NaiveDate) -> Result<Vec<StagehandNote>> {
        db::stagehand::get_by_date(&self.pool, date).await
    }

    /// Get notes for a date range (e.g., upcoming week)
    pub async fn get_upcoming(&self, days: i64) -> Result<Vec<StagehandNote>> {
        let start = chrono::Local::now().date_naive();
        let end = start + chrono::Duration::days(days);
        db::stagehand::get_by_date_range(&self.pool, start, end).await
    }

    /// Search by show name
    pub async fn search_show(&self, query: &str) -> Result<Vec<StagehandNote>> {
        db::stagehand::search_by_show(&self.pool, query).await
    }

    /// Search by venue
    pub async fn search_venue(&self, query: &str) -> Result<Vec<StagehandNote>> {
        db::stagehand::search_by_venue(&self.pool, query).await
    }

    /// Create a new note
    pub async fn create(&self, note: StagehandNoteCreate) -> Result<i32> {
        db::stagehand::create(&self.pool, &note).await
    }

    /// Get a note by ID
    pub async fn get(&self, id: i32) -> Result<StagehandNote> {
        db::stagehand::get_by_id(&self.pool, id).await
    }

    /// Delete a note
    pub async fn delete(&self, id: i32) -> Result<()> {
        db::stagehand::delete(&self.pool, id).await
    }

    /// Import events from ICS
    pub async fn import_ics_events(&self, events: &[IcsEvent]) -> Result<Vec<i32>> {
        let mut ids = Vec::new();
        
        for event in events {
            let note = StagehandNoteCreate {
                show_name: event.summary.clone(),
                venue: event.location.clone(),
                event_date: event.date,
                call_time: event.start_time,
                notes: event.description.clone(),
                tags: None,
            };
            
            let id = self.create(note).await?;
            ids.push(id);
        }
        
        Ok(ids)
    }
}

/// Import events from ICS content directly
pub async fn import_from_ics(pool: &DbPool, ics_content: &str) -> Result<Vec<i32>> {
    let events = crate::ics::parse_ics(ics_content)?;
    let manager = StagehandManager::new(pool.clone());
    manager.import_ics_events(&events).await
}
