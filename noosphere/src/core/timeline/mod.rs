//! Timeline view - unified chronological aggregation across data sources
//!
//! Merges memories, stagehand_notes, daily_logs, and memory_entries into
//! a single chronological view for visualization and navigation.

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;

/// Timeline entry type discriminator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimelineType {
    Memory,
    StagehandGig,
    DailyLog,
    MemoryEntry,
}

/// Single timeline entry (unified across sources)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    /// When this entry occurred
    pub timestamp: NaiveDateTime,
    /// Which data source
    pub entry_type: TimelineType,
    /// Display title
    pub title: String,
    /// Preview/summary text (max 100 chars)
    pub preview: String,
    /// Source record ID
    pub source_id: i32,
    /// Source table name
    pub source_table: String,
}

impl TimelineEntry {
    /// Create a truncated preview (max 100 UTF-8 chars)
    fn make_preview(content: Option<&str>) -> String {
        content
            .map(|c| {
                let clean = c.trim().replace('\n', " ");
                let char_count = clean.chars().count();
                if char_count > 100 {
                    format!("{}...", clean.chars().take(97).collect::<String>())
                } else {
                    clean
                }
            })
            .unwrap_or_else(|| "—".to_string())
    }
}

/// Load timeline entries for a date range
///
/// Returns entries from all sources (memories, stagehand_notes, daily_logs, memory_entries)
/// sorted newest-first.
pub async fn load_timeline_entries(
    pool: &PgPool,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<TimelineEntry>, sqlx::Error> {
    let mut entries = Vec::new();

    // Convert dates to datetime for queries
    let start_dt = start_date.and_hms_opt(0, 0, 0).unwrap();
    let end_dt = end_date.and_hms_opt(23, 59, 59).unwrap();

    // Load memories
    let memories_list = load_memories(pool, start_date, end_date).await?;
    entries.extend(memories_list);

    // Load stagehand gigs
    let gigs = load_stagehand_gigs(pool, start_date, end_date).await?;
    entries.extend(gigs);

    // Load daily logs
    let logs = load_daily_logs(pool, start_dt, end_dt).await?;
    entries.extend(logs);

    // Load memory entries
    let memories = load_memory_entries(pool, start_dt, end_dt).await?;
    entries.extend(memories);

    // Sort by timestamp (newest first)
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    Ok(entries)
}

async fn load_memories(
    pool: &PgPool,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<Vec<TimelineEntry>, sqlx::Error> {
    let notes = sqlx::query!(
        r#"
        SELECT id, path, title, content, modified_at::timestamp as "modified_at?"
        FROM memories
        WHERE note_date >= $1 AND note_date <= $2
        ORDER BY note_date DESC
        "#,
        start,
        end
    )
    .fetch_all(pool)
    .await?;

    Ok(notes
        .into_iter()
        .map(|note| {
            let timestamp = note.modified_at.unwrap_or_else(|| start.and_hms_opt(12, 0, 0).unwrap());
            let title = note.title.unwrap_or_else(|| {
                note.path
                    .split('/')
                    .last()
                    .unwrap_or(&note.path)
                    .trim_end_matches(".md")
                    .to_string()
            });
            TimelineEntry {
                timestamp,
                entry_type: TimelineType::Memory,
                title,
                preview: TimelineEntry::make_preview(note.content.as_deref()),
                source_id: note.id,
                source_table: "memories".to_string(),
            }
        })
        .collect())
}

async fn load_stagehand_gigs(
    pool: &PgPool,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<Vec<TimelineEntry>, sqlx::Error> {
    let gigs = sqlx::query!(
        r#"
        SELECT id, show_name, venue, event_date, call_time, notes
        FROM stagehand_notes
        WHERE event_date >= $1 AND event_date <= $2
        ORDER BY event_date DESC
        "#,
        start,
        end
    )
    .fetch_all(pool)
    .await?;

    Ok(gigs
        .into_iter()
        .map(|gig| {
            let time = gig
                .call_time
                .unwrap_or_else(|| chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap());
            let timestamp = gig.event_date.and_time(time);
            let title = match gig.venue {
                Some(v) => format!("{} @ {}", gig.show_name, v),
                None => gig.show_name,
            };
            TimelineEntry {
                timestamp,
                entry_type: TimelineType::StagehandGig,
                title,
                preview: TimelineEntry::make_preview(gig.notes.as_deref()),
                source_id: gig.id,
                source_table: "stagehand_notes".to_string(),
            }
        })
        .collect())
}

async fn load_daily_logs(
    pool: &PgPool,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<Vec<TimelineEntry>, sqlx::Error> {
    use chrono::{DateTime, Utc};
    let start_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(start, Utc);
    let end_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(end, Utc);

    let logs = sqlx::query!(
        r#"
        SELECT id, content, category, agent_id, entry_time as "entry_time!"
        FROM daily_logs
        WHERE entry_time >= $1 AND entry_time <= $2
        ORDER BY entry_time DESC
        "#,
        start_utc,
        end_utc
    )
    .fetch_all(pool)
    .await?;

    Ok(logs
        .into_iter()
        .map(|log| {
            let title = log
                .category
                .unwrap_or_else(|| log.agent_id.unwrap_or_else(|| "Log Entry".to_string()));
            TimelineEntry {
                timestamp: log.entry_time.naive_utc(),
                entry_type: TimelineType::DailyLog,
                title,
                preview: TimelineEntry::make_preview(Some(&log.content)),
                source_id: log.id as i32,
                source_table: "daily_logs".to_string(),
            }
        })
        .collect())
}

async fn load_memory_entries(
    pool: &PgPool,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<Vec<TimelineEntry>, sqlx::Error> {
    use chrono::{DateTime, Utc};
    let start_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(start, Utc);
    let end_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(end, Utc);

    let memories = sqlx::query!(
        r#"
        SELECT id, content, entry_type, created_at as "created_at?"
        FROM memory_entries
        WHERE created_at >= $1 AND created_at <= $2
        ORDER BY created_at DESC
        "#,
        start_utc,
        end_utc
    )
    .fetch_all(pool)
    .await?;

    Ok(memories
        .into_iter()
        .map(|mem| {
            let timestamp = mem.created_at.map(|dt| dt.naive_utc()).unwrap_or_else(|| start);
            let title = mem.entry_type.unwrap_or_else(|| "Memory".to_string());
            TimelineEntry {
                timestamp,
                entry_type: TimelineType::MemoryEntry,
                title,
                preview: TimelineEntry::make_preview(Some(&mem.content)),
                source_id: mem.id as i32,
                source_table: "memory_entries".to_string(),
            }
        })
        .collect())
}

/// Get activity counts per day (for heatmap visualization)
///
/// Returns a map of date → count of events on that day across all sources.
pub async fn get_activity_counts(
    pool: &PgPool,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<HashMap<NaiveDate, u32>, sqlx::Error> {
    let mut counts: HashMap<NaiveDate, u32> = HashMap::new();

    // Count memories by note_date
    let memory_counts = sqlx::query!(
        r#"
        SELECT note_date as "date!", COUNT(*) as "cnt!"
        FROM memories
        WHERE note_date >= $1 AND note_date <= $2 AND note_date IS NOT NULL
        GROUP BY note_date
        "#,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await?;

    for row in memory_counts {
        *counts.entry(row.date).or_insert(0) += row.cnt as u32;
    }

    // Count stagehand gigs
    let gig_counts = sqlx::query!(
        r#"
        SELECT event_date as "date!", COUNT(*) as "cnt!"
        FROM stagehand_notes
        WHERE event_date >= $1 AND event_date <= $2
        GROUP BY event_date
        "#,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await?;

    for row in gig_counts {
        *counts.entry(row.date).or_insert(0) += row.cnt as u32;
    }

    // Count daily logs (need to extract date from timestamp)
    let log_counts = sqlx::query!(
        r#"
        SELECT log_date as "date!", COUNT(*) as "cnt!"
        FROM daily_logs
        WHERE log_date >= $1 AND log_date <= $2
        GROUP BY log_date
        "#,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await?;

    for row in log_counts {
        *counts.entry(row.date).or_insert(0) += row.cnt as u32;
    }

    Ok(counts)
}
