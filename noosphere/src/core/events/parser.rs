//! Event Parser for Weekly Notes
//!
//! Parses events in the format:
//! TYPE Event description 📅 YYYY-MM-DD 🕐 HH:MM ⏱️ duration 🆔 evt-id 📍 [[daily-note]] 🔁 recurrence #tags

use regex::Regex;
use once_cell::sync::Lazy;

use super::types::{ParsedEvent, EventType};

// Event icons
static EVENT_ICONS: &[&str] = &["📆", "📋", "🎯", "🔔", "🏥", "🎸", "🎲", "🔧", "🎭", "📞"];

// Compiled regex patterns
static DATE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"📅\s*(\d{4}-\d{2}-\d{2})").unwrap());
static TIME_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"🕐\s*(\d{1,2}:\d{2})").unwrap());
static DURATION_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"⏱️\s*(\d+[hm]|\d+h\s*\d+m)").unwrap());
static ID_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"🆔\s*([\w-]+)").unwrap());
static DAILY_NOTE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"📍\s*\[\[([\d-]+)\]\]").unwrap());
static RECURRENCE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"🔁\s*(every\s+\w+|daily|weekly|monthly|yearly)").unwrap()
});
static TAG_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"#(\w+)").unwrap());

/// Parse a single event line
///
/// Returns `Some(ParsedEvent)` if the line is a valid event, `None` otherwise.
pub fn parse_event_line(line: &str) -> Option<ParsedEvent> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Check if line starts with a known event icon
    let icon = EVENT_ICONS.iter().find(|&&i| trimmed.starts_with(i))?;

    // Must have at least a date to be a valid event
    let date_match = DATE_PATTERN.captures(trimmed)?;
    let date = date_match.get(1)?.as_str().to_string();

    // Extract other fields
    let time = TIME_PATTERN.captures(trimmed)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());

    let duration = DURATION_PATTERN.captures(trimmed)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());

    let id = ID_PATTERN.captures(trimmed)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());

    let daily_note = DAILY_NOTE_PATTERN.captures(trimmed)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());

    let recurrence = RECURRENCE_PATTERN.captures(trimmed)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());

    // Extract tags
    let tags: Vec<String> = TAG_PATTERN.captures_iter(trimmed)
        .filter_map(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .collect();

    let tags_opt = if tags.is_empty() {
        None
    } else {
        Some(tags)
    };

    // Extract title: everything between icon and first marker
    let icon_len = icon.chars().count();
    let title_part = trimmed.chars().skip(icon_len).collect::<String>().trim().to_string();

    // Find the first marker position
    let markers = ["📅", "🕐", "⏱️", "🆔", "📍", "🔁"];
    let mut first_marker_pos = title_part.len();
    for marker in &markers {
        if let Some(pos) = title_part.find(marker) {
            if pos < first_marker_pos {
                first_marker_pos = pos;
            }
        }
    }

    let title = title_part[..first_marker_pos].trim().to_string();

    if title.is_empty() {
        return None;
    }

    // Generate ID if not provided
    let event_id = id.unwrap_or_else(|| {
        format!("evt-{}-{:x}", chrono::Utc::now().timestamp(), rand::random::<u16>())
    });

    let event_type = EventType::from_icon(icon).unwrap_or(EventType::Calendar);

    Some(ParsedEvent {
        id: event_id,
        icon: icon.to_string(),
        event_type,
        title,
        date,
        time,
        duration,
        daily_note,
        recurrence,
        tags: tags_opt,
    })
}

/// Parse all events from document content
///
/// Looks for `## 📅 Events` section and parses event lines within it.
pub fn parse_events(content: &str) -> Vec<ParsedEvent> {
    let mut events = Vec::new();
    let mut in_events_section = false;

    for line in content.lines() {
        // Check if we're entering the Events section
        if line.contains("## 📅") && line.to_lowercase().contains("event") {
            in_events_section = true;
            continue;
        }

        // Check if we're leaving the Events section (next ## header without 📅)
        if in_events_section && line.starts_with("##") && !line.contains("📅") {
            in_events_section = false;
            continue;
        }

        // Parse event lines (only in Events section for Weekly Notes)
        if in_events_section {
            if let Some(event) = parse_event_line(line) {
                events.push(event);
            }
        }
    }

    events
}

/// Check if a document is a Weekly Note (should have events parsed)
///
/// Weekly Notes match the pattern: `Weekly Notes/YYYY-WNN.md`
pub fn is_weekly_note(path: &str) -> bool {
    path.contains("Weekly Notes/") && Regex::new(r"\d{4}-W\d{2}\.md$")
        .map(|re| re.is_match(path))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_event() {
        let line = "📆 Team meeting 📅 2025-01-15";
        let event = parse_event_line(line).unwrap();
        assert_eq!(event.title, "Team meeting");
        assert_eq!(event.date, "2025-01-15");
    }

    #[test]
    fn test_parse_event_with_time() {
        let line = "📆 Doctor appointment 📅 2025-01-20 🕐 14:30";
        let event = parse_event_line(line).unwrap();
        assert_eq!(event.title, "Doctor appointment");
        assert_eq!(event.time, Some("14:30".to_string()));
    }

    #[test]
    fn test_is_weekly_note() {
        assert!(is_weekly_note("Weekly Notes/2025-W03.md"));
        assert!(!is_weekly_note("Daily Notes/2025-01-15.md"));
    }
}
