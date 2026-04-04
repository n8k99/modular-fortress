//! events table access and CRUD operations
//!
//! Structured events with date, time, duration, and type classification.
//! Format: icon TYPE Title 📅 YYYY-MM-DD 🕐 HH:MM ⏱️ duration 🆔 evt-id 📍 [[daily-note]] #tags

use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::DbPool;

/// Event record from database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Event {
    pub id: String,                       // Event ID (e.g., "evt-001")
    pub icon: String,                     // Emoji icon (📆📋🎯🔔🏥🎸🎲🔧🎭📞)
    #[sqlx(rename = "type")]
    pub event_type: String,               // Event type (calendar, deadline, goal, etc.)
    pub title: String,                    // Event description
    pub date: String,                     // Event date (YYYY-MM-DD)
    pub time: Option<String>,             // Event time (HH:MM)
    pub duration: Option<String>,         // Duration (Xh, Xm, Xh Xm)
    pub daily_note: Option<String>,       // Link to daily note
    pub tags: Option<String>,             // JSON array of tags
}

impl Event {
    /// Get display text for the event (one-line summary)
    pub fn display_text(&self) -> String {
        let mut parts = vec![self.icon.clone(), self.title.clone()];

        if let Some(ref time) = self.time {
            parts.push(format!("🕐 {}", time));
        }

        if let Some(ref duration) = self.duration {
            parts.push(format!("⏱️ {}", duration));
        }

        parts.join(" ")
    }

    /// Get tags as Vec<String>
    pub fn get_tags(&self) -> Vec<String> {
        if let Some(ref tags) = self.tags {
            serde_json::from_str::<Vec<String>>(tags).unwrap_or_default()
        } else {
            vec![]
        }
    }
}

/// Get events on a specific date
pub async fn get_events_on_date(pool: &DbPool, date: NaiveDate) -> Result<Vec<Event>> {
    let date_str = date.format("%Y-%m-%d").to_string();

    let events = sqlx::query_as::<_, Event>(
        r#"
        SELECT id, icon, type, title, date::text as date, time::text as time,
               duration, daily_note, tags::text as tags
        FROM events
        WHERE date = $1::date
        ORDER BY time, title
        "#,
    )
    .bind(&date_str)
    .fetch_all(pool)
    .await?;

    Ok(events)
}

/// Get events in a date range
pub async fn get_events_in_range(
    pool: &DbPool,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<Vec<Event>> {
    let start_str = start.format("%Y-%m-%d").to_string();
    let end_str = end.format("%Y-%m-%d").to_string();

    let events = sqlx::query_as::<_, Event>(
        r#"
        SELECT id, icon, type, title, date::text as date, time::text as time,
               duration, daily_note, tags::text as tags
        FROM events
        WHERE date >= $1::date AND date <= $2::date
        ORDER BY date, time, title
        "#,
    )
    .bind(&start_str)
    .bind(&end_str)
    .fetch_all(pool)
    .await?;

    Ok(events)
}

/// Get upcoming events (today and future)
pub async fn get_upcoming_events(pool: &DbPool, days: i32) -> Result<Vec<Event>> {
    let today = chrono::Local::now().date_naive();
    let end_date = today + chrono::Duration::days(days as i64);

    get_events_in_range(pool, today, end_date).await
}

/// Get event by ID
pub async fn get_event_by_id(pool: &DbPool, id: &str) -> Result<Option<Event>> {
    let event = sqlx::query_as::<_, Event>(
        r#"
        SELECT id, icon, type, title, date::text as date, time::text as time,
               duration, daily_note, tags::text as tags
        FROM events
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(event)
}

/// Create or update an event
pub async fn upsert_event(pool: &DbPool, event: &Event) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO events (id, icon, type, title, date, time, duration, daily_note, tags)
        VALUES ($1, $2, $3, $4, $5::date, $6::time, $7, $8, $9::jsonb)
        ON CONFLICT (id) DO UPDATE SET
            icon = EXCLUDED.icon,
            type = EXCLUDED.type,
            title = EXCLUDED.title,
            date = EXCLUDED.date,
            time = EXCLUDED.time,
            duration = EXCLUDED.duration,
            daily_note = EXCLUDED.daily_note,
            tags = EXCLUDED.tags
        "#,
    )
    .bind(&event.id)
    .bind(&event.icon)
    .bind(&event.event_type)
    .bind(&event.title)
    .bind(&event.date)
    .bind(&event.time)
    .bind(&event.duration)
    .bind(&event.daily_note)
    .bind(&event.tags)
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete an event by ID
pub async fn delete_event(pool: &DbPool, id: &str) -> Result<()> {
    sqlx::query("DELETE FROM events WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Count events on a specific date (for heatmap)
pub async fn count_events_on_date(pool: &DbPool, date: NaiveDate) -> Result<i64> {
    let date_str = date.format("%Y-%m-%d").to_string();

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events WHERE date = $1")
        .bind(&date_str)
        .fetch_one(pool)
        .await?;

    Ok(count.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_display_text() {
        let event = Event {
            id: "evt-001".to_string(),
            icon: "📆".to_string(),
            event_type: "calendar".to_string(),
            title: "Team meeting".to_string(),
            date: "2026-02-26".to_string(),
            time: Some("14:00".to_string()),
            duration: Some("1h".to_string()),
            daily_note: None,
            tags: Some(r#"["work","meeting"]"#.to_string()),
        };

        let display = event.display_text();
        assert!(display.contains("📆"));
        assert!(display.contains("Team meeting"));
        assert!(display.contains("🕐 14:00"));
        assert!(display.contains("⏱️ 1h"));
    }

    #[test]
    fn test_event_get_tags() {
        let event = Event {
            id: "evt-002".to_string(),
            icon: "🎸".to_string(),
            event_type: "music".to_string(),
            title: "Studio session".to_string(),
            date: "2026-02-27".to_string(),
            time: None,
            duration: None,
            daily_note: None,
            tags: Some(r#"["music","recording"]"#.to_string()),
        };

        let tags = event.get_tags();
        assert_eq!(tags, vec!["music", "recording"]);
    }

    #[test]
    fn test_event_get_tags_empty() {
        let event = Event {
            id: "evt-003".to_string(),
            icon: "📆".to_string(),
            event_type: "calendar".to_string(),
            title: "Test event".to_string(),
            date: "2026-02-26".to_string(),
            time: None,
            duration: None,
            daily_note: None,
            tags: None,
        };

        let tags = event.get_tags();
        assert!(tags.is_empty());
    }
}
