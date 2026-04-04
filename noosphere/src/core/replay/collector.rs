//! Day data collector
//!
//! Gathers all entries for a specific date from memories, daily_logs,
//! stagehand_notes, and memory_entries, sorted chronologically.

use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::db::DbPool;
use crate::db::{memories, stagehand};
use crate::memory::store::{get_logs_by_date, get_memories_by_date};

/// Source of a day entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntrySource {
    Memory,
    DailyLog,
    StagehandNote,
    MemoryEntry,
}

impl std::fmt::Display for EntrySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntrySource::Memory => write!(f, "memory"),
            EntrySource::DailyLog => write!(f, "log"),
            EntrySource::StagehandNote => write!(f, "gig"),
            EntrySource::MemoryEntry => write!(f, "memory"),
        }
    }
}

/// A unified entry from any source
#[derive(Debug, Clone)]
pub struct DayEntry {
    pub source: EntrySource,
    pub timestamp: NaiveDateTime,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub importance: i16,
    pub metadata: EntryMetadata,
}

/// Source-specific metadata
#[derive(Debug, Clone, Default)]
pub struct EntryMetadata {
    /// For memories: the path
    pub path: Option<String>,
    /// For stagehand_notes: venue
    pub venue: Option<String>,
    /// For stagehand_notes: show name
    pub show_name: Option<String>,
    /// For stagehand_notes: call time
    pub call_time: Option<NaiveTime>,
    /// For daily_logs/memory_entries: agent_id
    pub agent_id: Option<String>,
    /// For memory_entries: entry_type
    pub entry_type: Option<String>,
}

/// All data for a single day
#[derive(Debug, Clone)]
pub struct DayData {
    pub date: NaiveDate,
    pub entries: Vec<DayEntry>,
}

impl DayData {
    pub fn new(date: NaiveDate) -> Self {
        Self {
            date,
            entries: Vec::new(),
        }
    }

    /// Get entries sorted by timestamp
    pub fn sorted_entries(&self) -> Vec<&DayEntry> {
        let mut entries: Vec<_> = self.entries.iter().collect();
        entries.sort_by_key(|e| e.timestamp);
        entries
    }

    /// Get entries by source type
    pub fn entries_by_source(&self, source: EntrySource) -> Vec<&DayEntry> {
        self.entries.iter().filter(|e| e.source == source).collect()
    }

    /// Get high importance entries (importance >= 7)
    pub fn important_entries(&self) -> Vec<&DayEntry> {
        self.entries.iter().filter(|e| e.importance >= 7).collect()
    }

    /// Collect all unique tags from all entries
    pub fn all_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self.entries
            .iter()
            .flat_map(|e| e.tags.clone())
            .collect();
        tags.sort();
        tags.dedup();
        tags
    }
}

/// Collect all entries for a specific date
pub async fn collect_day(pool: &DbPool, date: NaiveDate) -> Result<DayData> {
    let mut data = DayData::new(date);
    
    // Collect memories for this date
    collect_memories(pool, date, &mut data).await?;
    
    // Collect daily logs
    collect_daily_logs(pool, date, &mut data).await?;
    
    // Collect stagehand notes
    collect_stagehand_notes(pool, date, &mut data).await?;
    
    // Collect memory entries
    collect_memory_entries(pool, date, &mut data).await?;
    
    Ok(data)
}

async fn collect_memories(pool: &DbPool, date: NaiveDate, data: &mut DayData) -> Result<()> {
    // Get daily note if it exists
    if let Some(note) = memories::get_daily_note(pool, date).await? {
        let timestamp = note.modified_at
            .unwrap_or_else(|| date.and_hms_opt(0, 0, 0).unwrap());
        
        let content = note.content.clone().unwrap_or_default();
        data.entries.push(DayEntry {
            source: EntrySource::Memory,
            timestamp,
            title: note.display_title(),
            content: content.clone(),
            tags: extract_tags_from_content(&content),
            importance: 5,
            metadata: EntryMetadata {
                path: Some(note.path),
                ..Default::default()
            },
        });
    }
    
    // Also get any notes modified on this date
    let notes = memories::list_by_date_range(pool, date, date).await?;
    for note_light in notes {
        // Skip daily notes (already included above)
        if note_light.note_type.as_deref() == Some("daily") {
            continue;
        }
        
        // Fetch full note for content
        if let Ok(note) = memories::get_by_id(pool, note_light.id).await {
            let timestamp = note.modified_at
                .unwrap_or_else(|| date.and_hms_opt(12, 0, 0).unwrap());
            
            let content = note.content.clone().unwrap_or_default();
            data.entries.push(DayEntry {
                source: EntrySource::Memory,
                timestamp,
                title: note.display_title(),
                content: content.clone(),
                tags: extract_tags_from_content(&content),
                importance: 5,
                metadata: EntryMetadata {
                    path: Some(note.path),
                    ..Default::default()
                },
            });
        }
    }
    
    Ok(())
}

async fn collect_daily_logs(pool: &DbPool, date: NaiveDate, data: &mut DayData) -> Result<()> {
    let logs = get_logs_by_date(pool, date).await?;
    
    for log in logs {
        data.entries.push(DayEntry {
            source: EntrySource::DailyLog,
            timestamp: log.entry_time,
            title: log.category.clone().unwrap_or_else(|| "Log".to_string()),
            content: log.content,
            tags: vec![],
            importance: log.importance.unwrap_or(3),
            metadata: EntryMetadata {
                agent_id: log.agent_id,
                ..Default::default()
            },
        });
    }
    
    Ok(())
}

async fn collect_stagehand_notes(pool: &DbPool, date: NaiveDate, data: &mut DayData) -> Result<()> {
    let notes = stagehand::get_by_date(pool, date).await?;
    
    for note in notes {
        let timestamp = note.call_time
            .map(|t| date.and_time(t))
            .unwrap_or_else(|| date.and_hms_opt(9, 0, 0).unwrap());
        
        data.entries.push(DayEntry {
            source: EntrySource::StagehandNote,
            timestamp,
            title: note.display_title(),
            content: note.notes.unwrap_or_default(),
            tags: note.tags.unwrap_or_default(),
            importance: 7, // Gigs are important!
            metadata: EntryMetadata {
                venue: note.venue,
                show_name: Some(note.show_name),
                call_time: note.call_time,
                ..Default::default()
            },
        });
    }
    
    Ok(())
}

async fn collect_memory_entries(pool: &DbPool, date: NaiveDate, data: &mut DayData) -> Result<()> {
    let entries = get_memories_by_date(pool, date).await?;
    
    for entry in entries {
        let timestamp = entry.created_at
            .unwrap_or_else(|| date.and_hms_opt(12, 0, 0).unwrap());
        
        data.entries.push(DayEntry {
            source: EntrySource::MemoryEntry,
            timestamp,
            title: entry.entry_type.clone().unwrap_or_else(|| "Memory".to_string()),
            content: entry.content,
            tags: entry.tags.unwrap_or_default(),
            importance: entry.importance.unwrap_or(5),
            metadata: EntryMetadata {
                agent_id: entry.agent_id,
                entry_type: entry.entry_type,
                ..Default::default()
            },
        });
    }
    
    Ok(())
}

/// Extract wiki-style [[tags]] from content
fn extract_tags_from_content(content: &str) -> Vec<String> {
    let mut tags = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = content.chars().collect();
    
    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == '[' && chars[i + 1] == '[' {
            // Found opening [[
            i += 2;
            let start = i;
            while i < chars.len() && !(chars[i] == ']' && i + 1 < chars.len() && chars[i + 1] == ']') {
                i += 1;
            }
            if i < chars.len() {
                let tag: String = chars[start..i].iter().collect();
                // Handle [[display|link]] format
                let tag = tag.split('|').next().unwrap_or(&tag).to_string();
                if !tag.is_empty() {
                    tags.push(tag);
                }
                i += 2; // Skip closing ]]
            }
        } else {
            i += 1;
        }
    }
    
    tags.sort();
    tags.dedup();
    tags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_tags() {
        let content = "Working on [[dpn-core]] and [[stagehand]] today. Also [[dpn-core|DPN Core]] again.";
        let tags = extract_tags_from_content(content);
        assert_eq!(tags, vec!["dpn-core".to_string(), "stagehand".to_string()]);
    }
    
    #[test]
    fn test_day_data_tags() {
        let mut data = DayData::new(NaiveDate::from_ymd_opt(2026, 2, 22).unwrap());
        data.entries.push(DayEntry {
            source: EntrySource::Memory,
            timestamp: NaiveDate::from_ymd_opt(2026, 2, 22).unwrap().and_hms_opt(10, 0, 0).unwrap(),
            title: "Test".to_string(),
            content: "Test content".to_string(),
            tags: vec!["rust".to_string(), "dev".to_string()],
            importance: 5,
            metadata: EntryMetadata::default(),
        });
        data.entries.push(DayEntry {
            source: EntrySource::DailyLog,
            timestamp: NaiveDate::from_ymd_opt(2026, 2, 22).unwrap().and_hms_opt(11, 0, 0).unwrap(),
            title: "Log".to_string(),
            content: "Log content".to_string(),
            tags: vec!["dev".to_string(), "testing".to_string()],
            importance: 3,
            metadata: EntryMetadata::default(),
        });
        
        let all_tags = data.all_tags();
        assert_eq!(all_tags, vec!["dev".to_string(), "rust".to_string(), "testing".to_string()]);
    }
}
