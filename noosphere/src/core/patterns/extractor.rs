//! Pattern extraction from historical data
//!
//! Finds recurring patterns in stagehand notes, common tags,
//! venue-specific notes, and time-of-day patterns.

use std::collections::HashMap;
use anyhow::Result;
use chrono::{NaiveTime, Timelike};

use crate::db::DbPool;
use crate::db::stagehand::{self, StagehandNote};
use crate::memory::store::{self, MemoryEntry};

/// Type of pattern detected
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternType {
    /// Recurring phrase in notes
    RecurringPhrase,
    /// Common tag usage
    CommonTag,
    /// Venue-specific note
    VenueSpecific,
    /// Time-of-day pattern
    TimeOfDay,
    /// Show-specific pattern
    ShowSpecific,
}

impl std::fmt::Display for PatternType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternType::RecurringPhrase => write!(f, "recurring_phrase"),
            PatternType::CommonTag => write!(f, "common_tag"),
            PatternType::VenueSpecific => write!(f, "venue_specific"),
            PatternType::TimeOfDay => write!(f, "time_of_day"),
            PatternType::ShowSpecific => write!(f, "show_specific"),
        }
    }
}

/// A detected pattern
#[derive(Debug, Clone)]
pub struct Pattern {
    pub pattern_type: PatternType,
    pub key: String,           // What the pattern is about (venue name, tag, phrase, etc.)
    pub value: String,         // The pattern content
    pub frequency: usize,      // How often it appears
    pub confidence: f32,       // 0.0 to 1.0
    pub source_ids: Vec<i32>,  // IDs of source notes
}

impl Pattern {
    pub fn new(pattern_type: PatternType, key: String, value: String, frequency: usize) -> Self {
        // Confidence is based on frequency
        let confidence = (frequency as f32 / 10.0).min(1.0);
        Self {
            pattern_type,
            key,
            value,
            frequency,
            confidence,
            source_ids: Vec::new(),
        }
    }
    
    pub fn with_sources(mut self, ids: Vec<i32>) -> Self {
        self.source_ids = ids;
        self
    }
}

/// Extract patterns from all available data for an agent
pub async fn extract_patterns(pool: &DbPool, agent_id: Option<&str>) -> Result<Vec<Pattern>> {
    let mut patterns = Vec::new();
    
    // Extract from stagehand notes
    let stagehand_patterns = extract_stagehand_patterns(pool).await?;
    patterns.extend(stagehand_patterns);
    
    // Extract from memory entries
    let memory_patterns = extract_memory_patterns(pool, agent_id).await?;
    patterns.extend(memory_patterns);
    
    // Sort by confidence
    patterns.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
    
    Ok(patterns)
}

/// Extract patterns from stagehand notes
async fn extract_stagehand_patterns(pool: &DbPool) -> Result<Vec<Pattern>> {
    let notes = stagehand::list_all(pool).await?;
    let mut patterns = Vec::new();
    
    // Venue-specific patterns
    patterns.extend(extract_venue_patterns(&notes));
    
    // Show-specific patterns
    patterns.extend(extract_show_patterns(&notes));
    
    // Tag frequency patterns
    patterns.extend(extract_tag_patterns(&notes));
    
    // Time-of-day patterns
    patterns.extend(extract_time_patterns(&notes));
    
    // Recurring phrases
    patterns.extend(extract_phrase_patterns(&notes));
    
    Ok(patterns)
}

/// Extract venue-specific patterns (things noted at specific venues)
fn extract_venue_patterns(notes: &[StagehandNote]) -> Vec<Pattern> {
    let mut venue_notes: HashMap<String, Vec<(&StagehandNote, String)>> = HashMap::new();
    
    for note in notes {
        if let Some(venue) = &note.venue {
            if let Some(content) = &note.notes {
                venue_notes.entry(venue.clone())
                    .or_default()
                    .push((note, content.clone()));
            }
        }
    }
    
    let mut patterns = Vec::new();
    
    for (venue, notes_list) in venue_notes {
        if notes_list.len() >= 2 {
            // Find recurring phrases at this venue
            let phrases = find_recurring_phrases(&notes_list.iter().map(|(_, c)| c.as_str()).collect::<Vec<_>>());
            for (phrase, count) in phrases {
                if count >= 2 {
                    let ids: Vec<_> = notes_list.iter().map(|(n, _)| n.id).collect();
                    patterns.push(
                        Pattern::new(
                            PatternType::VenueSpecific,
                            venue.clone(),
                            phrase,
                            count,
                        ).with_sources(ids)
                    );
                }
            }
        }
    }
    
    patterns
}

/// Extract show-specific patterns
fn extract_show_patterns(notes: &[StagehandNote]) -> Vec<Pattern> {
    let mut show_notes: HashMap<String, Vec<&StagehandNote>> = HashMap::new();
    
    for note in notes {
        show_notes.entry(note.show_name.clone())
            .or_default()
            .push(note);
    }
    
    let mut patterns = Vec::new();
    
    for (show_name, show_note_list) in show_notes {
        if show_note_list.len() >= 2 {
            // This show has multiple entries - it's recurring
            let ids: Vec<_> = show_note_list.iter().map(|n| n.id).collect();
            patterns.push(
                Pattern::new(
                    PatternType::ShowSpecific,
                    show_name.clone(),
                    format!("{} entries", show_note_list.len()),
                    show_note_list.len(),
                ).with_sources(ids)
            );
        }
    }
    
    patterns
}

/// Extract tag frequency patterns
fn extract_tag_patterns(notes: &[StagehandNote]) -> Vec<Pattern> {
    let mut tag_counts: HashMap<String, Vec<i32>> = HashMap::new();
    
    for note in notes {
        if let Some(tags) = &note.tags {
            for tag in tags {
                tag_counts.entry(tag.clone())
                    .or_default()
                    .push(note.id);
            }
        }
    }
    
    let mut patterns = Vec::new();
    
    for (tag, ids) in tag_counts {
        if ids.len() >= 3 {
            patterns.push(
                Pattern::new(
                    PatternType::CommonTag,
                    tag.clone(),
                    format!("Used {} times", ids.len()),
                    ids.len(),
                ).with_sources(ids)
            );
        }
    }
    
    patterns
}

/// Extract time-of-day patterns (common call times)
fn extract_time_patterns(notes: &[StagehandNote]) -> Vec<Pattern> {
    let mut time_slots: HashMap<u32, Vec<i32>> = HashMap::new();
    
    for note in notes {
        if let Some(call_time) = note.call_time {
            // Group by hour
            let hour = call_time.hour();
            time_slots.entry(hour).or_default().push(note.id);
        }
    }
    
    let mut patterns = Vec::new();
    
    for (hour, ids) in time_slots {
        if ids.len() >= 3 {
            let time_str = NaiveTime::from_hms_opt(hour, 0, 0)
                .map(|t| t.format("%H:00").to_string())
                .unwrap_or_else(|| format!("{}:00", hour));
            
            patterns.push(
                Pattern::new(
                    PatternType::TimeOfDay,
                    time_str,
                    format!("{} gigs at this hour", ids.len()),
                    ids.len(),
                ).with_sources(ids)
            );
        }
    }
    
    patterns
}

/// Find recurring phrases in a set of content strings
fn find_recurring_phrases(contents: &[&str]) -> Vec<(String, usize)> {
    let mut phrase_counts: HashMap<String, usize> = HashMap::new();
    
    for content in contents {
        // Extract meaningful phrases (3-5 word sequences)
        let words: Vec<&str> = content.split_whitespace().collect();
        
        // Look for 3-word phrases
        for window in words.windows(3) {
            let phrase = window.join(" ").to_lowercase();
            if phrase.len() > 10 && !is_common_phrase(&phrase) {
                *phrase_counts.entry(phrase).or_default() += 1;
            }
        }
    }
    
    let mut phrases: Vec<_> = phrase_counts.into_iter()
        .filter(|(_, count)| *count >= 2)
        .collect();
    
    phrases.sort_by(|a, b| b.1.cmp(&a.1));
    phrases.truncate(10);
    phrases
}

/// Extract recurring phrases from notes
fn extract_phrase_patterns(notes: &[StagehandNote]) -> Vec<Pattern> {
    let contents: Vec<&str> = notes.iter()
        .filter_map(|n| n.notes.as_deref())
        .collect();
    
    let phrases = find_recurring_phrases(&contents);
    
    phrases.into_iter()
        .map(|(phrase, count)| {
            Pattern::new(
                PatternType::RecurringPhrase,
                "notes".to_string(),
                phrase,
                count,
            )
        })
        .collect()
}

/// Check if a phrase is too common to be meaningful
fn is_common_phrase(phrase: &str) -> bool {
    let common = [
        "the", "and", "but", "for", "with", "this", "that",
        "was", "were", "are", "been", "being", "have", "has",
    ];
    
    let words: Vec<&str> = phrase.split_whitespace().collect();
    words.iter().all(|w| common.contains(w))
}

/// Extract patterns from memory entries
async fn extract_memory_patterns(pool: &DbPool, agent_id: Option<&str>) -> Result<Vec<Pattern>> {
    let entries = if let Some(id) = agent_id {
        store::get_memories_by_agent(pool, id, 1000).await?
    } else {
        store::get_memories_by_type(pool, "fact", 1000).await?
    };
    
    let mut patterns = Vec::new();
    
    // Tag frequency from memories
    patterns.extend(extract_memory_tag_patterns(&entries));
    
    Ok(patterns)
}

fn extract_memory_tag_patterns(entries: &[MemoryEntry]) -> Vec<Pattern> {
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    
    for entry in entries {
        if let Some(tags) = &entry.tags {
            for tag in tags {
                *tag_counts.entry(tag.clone()).or_default() += 1;
            }
        }
    }
    
    tag_counts.into_iter()
        .filter(|(_, count)| *count >= 3)
        .map(|(tag, count)| {
            Pattern::new(
                PatternType::CommonTag,
                tag,
                format!("Memory tag used {} times", count),
                count,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_confidence() {
        let p1 = Pattern::new(PatternType::CommonTag, "test".into(), "value".into(), 5);
        assert!((p1.confidence - 0.5).abs() < 0.01);
        
        let p2 = Pattern::new(PatternType::CommonTag, "test".into(), "value".into(), 15);
        assert!((p2.confidence - 1.0).abs() < 0.01); // Capped at 1.0
    }

    #[test]
    fn test_is_common_phrase() {
        assert!(is_common_phrase("the and with"));
        assert!(!is_common_phrase("bring extra stingers"));
    }
}
