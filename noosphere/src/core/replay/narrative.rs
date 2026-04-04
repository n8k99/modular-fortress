//! Narrative generation from day data
//!
//! Transforms collected day data into a readable markdown narrative.

use std::collections::HashMap;

use crate::replay::collector::{DayData, DayEntry, EntrySource};
use crate::replay::templates::{NarrativeTemplate, TimeOfDay};

/// Options for narrative generation
#[derive(Debug, Clone)]
pub struct ReplayOptions {
    /// Include the raw content in entries
    pub include_content: bool,
    /// Maximum content length per entry
    pub max_content_length: usize,
    /// Include connections section
    pub include_connections: bool,
    /// Include themes section
    pub include_themes: bool,
    /// Include source attribution
    pub include_sources: bool,
}

impl Default for ReplayOptions {
    fn default() -> Self {
        Self {
            include_content: true,
            max_content_length: 200,
            include_connections: true,
            include_themes: true,
            include_sources: true,
        }
    }
}

/// Generate a narrative replay from day data
pub fn generate_replay(data: &DayData) -> String {
    generate_replay_with_options(data, &ReplayOptions::default())
}

/// Generate a narrative replay with custom options
pub fn generate_replay_with_options(data: &DayData, options: &ReplayOptions) -> String {
    let mut output = String::new();
    
    // Title
    let formatted_date = data.date.format("%B %d, %Y");
    output.push_str(&format!("# {} - Day Replay\n\n", formatted_date));
    
    if data.entries.is_empty() {
        output.push_str("*No entries recorded for this day.*\n");
        return output;
    }
    
    // Group entries by time of day
    let mut by_time: HashMap<TimeOfDay, Vec<&DayEntry>> = HashMap::new();
    for entry in data.sorted_entries() {
        let time_of_day = TimeOfDay::from_time(entry.timestamp.time());
        by_time.entry(time_of_day).or_default().push(entry);
    }
    
    // Sort time periods
    let mut time_periods: Vec<_> = by_time.keys().collect();
    time_periods.sort_by_key(|t| t.order());
    
    // Generate narrative for each time period
    for (period_idx, &period) in time_periods.iter().enumerate() {
        let entries = &by_time[period];
        
        output.push_str(period.header());
        output.push_str("\n\n");
        
        for (i, entry) in entries.iter().enumerate() {
            let intro = NarrativeTemplate::random_intro(&entry.source, period_idx * 100 + i);
            let time_str = entry.timestamp.format("%H:%M").to_string();
            
            // Build the entry line
            match &entry.source {
                EntrySource::StagehandNote => {
                    // Special formatting for gigs
                    if let Some(venue) = &entry.metadata.venue {
                        output.push_str(&format!("{} {} - **{}**", intro, venue, entry.title));
                    } else {
                        output.push_str(&format!("{} **{}**", intro, entry.title));
                    }
                    if let Some(call_time) = entry.metadata.call_time {
                        output.push_str(&format!(" (call: {})", call_time.format("%H:%M")));
                    }
                }
                EntrySource::Memory => {
                    output.push_str(&format!("{} [[{}]]", intro, entry.title));
                }
                EntrySource::DailyLog => {
                    if options.include_sources {
                        if let Some(agent) = &entry.metadata.agent_id {
                            output.push_str(&format!("[{}] ", agent));
                        }
                    }
                    output.push_str(&format!("{} at {}", intro, time_str));
                }
                EntrySource::MemoryEntry => {
                    let entry_type = entry.metadata.entry_type.as_deref().unwrap_or("memory");
                    output.push_str(&format!("{} ({})", intro, entry_type));
                }
            }
            
            // Add content snippet if enabled
            if options.include_content && !entry.content.is_empty() {
                let content = truncate_content(&entry.content, options.max_content_length);
                if !content.is_empty() {
                    output.push_str(": ");
                    output.push_str(&content);
                }
            }
            
            output.push_str("\n\n");
        }
    }
    
    // Key Themes section
    if options.include_themes {
        let themes = extract_themes(data);
        if !themes.is_empty() {
            output.push_str("## Key Themes\n\n");
            for theme in themes {
                output.push_str(&format!("- {}\n", theme));
            }
            output.push('\n');
        }
    }
    
    // Connections section
    if options.include_connections {
        let connections = extract_connections(data);
        if !connections.is_empty() {
            output.push_str("## Connections Made\n\n");
            for connection in connections {
                output.push_str(&format!("- {}\n", connection));
            }
            output.push('\n');
        }
    }
    
    // Summary stats
    output.push_str("---\n\n");
    output.push_str(&format!("*{} entries across {} sources*\n", 
        data.entries.len(),
        count_unique_sources(data)
    ));
    
    output
}

/// Truncate content to a maximum length, breaking at word boundaries
fn truncate_content(content: &str, max_len: usize) -> String {
    // Clean up content: remove excessive whitespace, newlines
    let cleaned: String = content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    
    if cleaned.len() <= max_len {
        return cleaned;
    }
    
    // Find last space before max_len
    let truncated = &cleaned[..max_len];
    match truncated.rfind(' ') {
        Some(pos) if pos > max_len / 2 => format!("{}...", &truncated[..pos]),
        _ => format!("{}...", truncated),
    }
}

/// Extract themes from day data based on tags and content
fn extract_themes(data: &DayData) -> Vec<String> {
    let mut themes = Vec::new();
    let tags = data.all_tags();
    
    // Group tags into themes
    let mut tag_counts: HashMap<&str, usize> = HashMap::new();
    for tag in &tags {
        *tag_counts.entry(tag.as_str()).or_default() += 1;
    }
    
    // Look for technical work
    let tech_keywords = ["dpn", "rust", "code", "dev", "api", "db", "database"];
    let has_tech = tags.iter().any(|t| {
        let lower = t.to_lowercase();
        tech_keywords.iter().any(|k| lower.contains(k))
    });
    if has_tech {
        themes.push("Technical development".to_string());
    }
    
    // Look for stagehand work
    let has_gigs = data.entries.iter().any(|e| e.source == EntrySource::StagehandNote);
    if has_gigs {
        themes.push("Stagehand work".to_string());
    }
    
    // Look for writing/notes work
    let note_count = data.entries.iter()
        .filter(|e| e.source == EntrySource::Memory)
        .count();
    if note_count >= 3 {
        themes.push("Documentation and notes".to_string());
    }
    
    // Add most common tags as themes (if not already covered)
    for tag in tags.iter().take(3) {
        let tag_theme = format!("{}", capitalize_first(tag));
        if !themes.iter().any(|t| t.to_lowercase().contains(&tag.to_lowercase())) {
            themes.push(tag_theme);
        }
    }
    
    themes.truncate(5);
    themes
}

/// Extract connections (wiki links and relationships)
fn extract_connections(data: &DayData) -> Vec<String> {
    let mut connections = Vec::new();
    let all_tags = data.all_tags();
    
    // Look for wiki-style links
    for tag in all_tags.iter().take(5) {
        // Find entries that reference this tag
        let refs: Vec<_> = data.entries.iter()
            .filter(|e| e.tags.contains(tag))
            .map(|e| &e.title)
            .collect();
        
        if refs.len() >= 2 {
            connections.push(format!("Linked [[{}]] across {} notes", tag, refs.len()));
        } else if !refs.is_empty() {
            connections.push(format!("Referenced [[{}]]", tag));
        }
    }
    
    // Look for new stagehand notes
    for entry in &data.entries {
        if entry.source == EntrySource::StagehandNote {
            if let Some(show) = &entry.metadata.show_name {
                connections.push(format!("New stagehand note for {}", show));
            }
        }
    }
    
    connections.truncate(5);
    connections
}

/// Count unique source types
fn count_unique_sources(data: &DayData) -> usize {
    let mut sources: Vec<_> = data.entries.iter().map(|e| &e.source).collect();
    sources.sort_by_key(|s| format!("{:?}", s));
    sources.dedup();
    sources.len()
}

/// Capitalize first letter
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use crate::replay::collector::EntryMetadata;

    #[test]
    fn test_truncate_content() {
        let long = "This is a very long piece of content that should be truncated at some point";
        let truncated = truncate_content(long, 30);
        assert!(truncated.len() <= 33); // 30 + "..."
        assert!(truncated.ends_with("..."));
    }

    #[test]
    fn test_empty_day() {
        let data = DayData::new(NaiveDate::from_ymd_opt(2026, 2, 22).unwrap());
        let replay = generate_replay(&data);
        assert!(replay.contains("No entries recorded"));
    }

    #[test]
    fn test_generate_replay() {
        let mut data = DayData::new(NaiveDate::from_ymd_opt(2026, 2, 22).unwrap());
        data.entries.push(DayEntry {
            source: EntrySource::Memory,
            timestamp: NaiveDate::from_ymd_opt(2026, 2, 22).unwrap()
                .and_hms_opt(10, 30, 0).unwrap(),
            title: "dpn-core architecture".to_string(),
            content: "Notes about the core architecture".to_string(),
            tags: vec!["dpn-core".to_string(), "rust".to_string()],
            importance: 5,
            metadata: EntryMetadata::default(),
        });
        
        let replay = generate_replay(&data);
        assert!(replay.contains("February 22, 2026"));
        assert!(replay.contains("Morning"));
        assert!(replay.contains("[[dpn-core architecture]]"));
    }
}
