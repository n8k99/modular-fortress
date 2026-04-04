//! stagehand_notes table access
//!
//! New table for show/venue notes with semantic search capability.

use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::FromRow;

use super::DbPool;

/// Full stagehand note
#[derive(Debug, Clone, FromRow)]
pub struct StagehandNote {
    pub id: i32,
    pub show_name: String,
    pub venue: Option<String>,
    pub event_date: NaiveDate,
    pub call_time: Option<NaiveTime>,
    pub notes: Option<String>,
    pub tags: Option<Vec<String>>,
    // Note: embedding field (vector) handled separately for semantic search
    pub created_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
}

/// Input for creating a new stagehand note
#[derive(Debug, Clone)]
pub struct StagehandNoteCreate {
    pub show_name: String,
    pub venue: Option<String>,
    pub event_date: NaiveDate,
    pub call_time: Option<NaiveTime>,
    pub notes: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl StagehandNote {
    pub fn display_title(&self) -> String {
        match &self.venue {
            Some(v) => format!("{} @ {}", self.show_name, v),
            None => self.show_name.clone(),
        }
    }

    pub fn display_date(&self) -> String {
        self.event_date.format("%Y-%m-%d").to_string()
    }

    pub fn display_call_time(&self) -> String {
        match &self.call_time {
            Some(t) => t.format("%H:%M").to_string(),
            None => "—".to_string(),
        }
    }
}

/// Get all stagehand notes
pub async fn list_all(pool: &DbPool) -> Result<Vec<StagehandNote>> {
    let notes = sqlx::query_as::<_, StagehandNote>(
        r#"
        SELECT id, show_name, venue, event_date, call_time, notes, tags,
               created_at::timestamp as created_at,
               modified_at::timestamp as modified_at
        FROM stagehand_notes
        ORDER BY event_date DESC
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(notes)
}

/// Get stagehand notes for a specific date
pub async fn get_by_date(pool: &DbPool, date: NaiveDate) -> Result<Vec<StagehandNote>> {
    let notes = sqlx::query_as::<_, StagehandNote>(
        r#"
        SELECT id, show_name, venue, event_date, call_time, notes, tags,
               created_at::timestamp as created_at,
               modified_at::timestamp as modified_at
        FROM stagehand_notes
        WHERE event_date = $1
        ORDER BY call_time
        "#,
    )
    .bind(date)
    .fetch_all(pool)
    .await?;
    Ok(notes)
}

/// Get stagehand notes for a date range
pub async fn get_by_date_range(
    pool: &DbPool,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<Vec<StagehandNote>> {
    let notes = sqlx::query_as::<_, StagehandNote>(
        r#"
        SELECT id, show_name, venue, event_date, call_time, notes, tags,
               created_at::timestamp as created_at,
               modified_at::timestamp as modified_at
        FROM stagehand_notes
        WHERE event_date >= $1 AND event_date <= $2
        ORDER BY event_date, call_time
        "#,
    )
    .bind(start)
    .bind(end)
    .fetch_all(pool)
    .await?;
    Ok(notes)
}

/// Get stagehand notes by show name (fuzzy)
pub async fn search_by_show(pool: &DbPool, show_name: &str) -> Result<Vec<StagehandNote>> {
    let pattern = format!("%{}%", show_name);
    let notes = sqlx::query_as::<_, StagehandNote>(
        r#"
        SELECT id, show_name, venue, event_date, call_time, notes, tags,
               created_at::timestamp as created_at,
               modified_at::timestamp as modified_at
        FROM stagehand_notes
        WHERE show_name ILIKE $1
        ORDER BY event_date DESC
        "#,
    )
    .bind(&pattern)
    .fetch_all(pool)
    .await?;
    Ok(notes)
}

/// Get stagehand notes by venue (fuzzy)
pub async fn search_by_venue(pool: &DbPool, venue: &str) -> Result<Vec<StagehandNote>> {
    let pattern = format!("%{}%", venue);
    let notes = sqlx::query_as::<_, StagehandNote>(
        r#"
        SELECT id, show_name, venue, event_date, call_time, notes, tags,
               created_at::timestamp as created_at,
               modified_at::timestamp as modified_at
        FROM stagehand_notes
        WHERE venue ILIKE $1
        ORDER BY event_date DESC
        "#,
    )
    .bind(&pattern)
    .fetch_all(pool)
    .await?;
    Ok(notes)
}

/// Get a stagehand note by ID
pub async fn get_by_id(pool: &DbPool, id: i32) -> Result<StagehandNote> {
    let note = sqlx::query_as::<_, StagehandNote>(
        r#"
        SELECT id, show_name, venue, event_date, call_time, notes, tags,
               created_at::timestamp as created_at,
               modified_at::timestamp as modified_at
        FROM stagehand_notes
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(note)
}

/// Create a new stagehand note
pub async fn create(pool: &DbPool, note: &StagehandNoteCreate) -> Result<i32> {
    let row: (i32,) = sqlx::query_as(
        r#"
        INSERT INTO stagehand_notes (show_name, venue, event_date, call_time, notes, tags)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#
    )
    .bind(&note.show_name)
    .bind(&note.venue)
    .bind(note.event_date)
    .bind(note.call_time)
    .bind(&note.notes)
    .bind(&note.tags)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

/// Update a stagehand note
pub async fn update(
    pool: &DbPool,
    id: i32,
    show_name: &str,
    venue: Option<&str>,
    event_date: NaiveDate,
    call_time: Option<NaiveTime>,
    notes: Option<&str>,
    tags: Option<&[String]>,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE stagehand_notes 
        SET show_name = $1, venue = $2, event_date = $3, call_time = $4, 
            notes = $5, tags = $6, modified_at = NOW()
        WHERE id = $7
        "#
    )
    .bind(show_name)
    .bind(venue)
    .bind(event_date)
    .bind(call_time)
    .bind(notes)
    .bind(tags)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete a stagehand note
pub async fn delete(pool: &DbPool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM stagehand_notes WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get count of stagehand notes
pub async fn get_count(pool: &DbPool) -> Result<i64> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM stagehand_notes")
        .fetch_one(pool)
        .await?;
    Ok(count.0)
}
