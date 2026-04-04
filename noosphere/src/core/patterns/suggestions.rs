//! Contextual suggestions based on patterns and instincts
//!
//! Provides relevant suggestions based on the current context
//! (venue, show, time of day, tags, etc.)

use anyhow::Result;

use crate::db::DbPool;
use crate::patterns::instincts::get_instincts;
use crate::db::stagehand;

/// Context for generating suggestions
#[derive(Debug, Clone, Default)]
pub struct SuggestionContext {
    pub venue: Option<String>,
    pub show_name: Option<String>,
    pub tags: Vec<String>,
    pub hour: Option<u32>,
}

impl SuggestionContext {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_venue(mut self, venue: impl Into<String>) -> Self {
        self.venue = Some(venue.into());
        self
    }
    
    pub fn with_show(mut self, show: impl Into<String>) -> Self {
        self.show_name = Some(show.into());
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
    
    pub fn with_hour(mut self, hour: u32) -> Self {
        self.hour = Some(hour);
        self
    }
}

/// A suggestion based on past patterns
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub content: String,
    pub source: SuggestionSource,
    pub confidence: f32,
    pub reason: String,
}

/// Source of the suggestion
#[derive(Debug, Clone)]
pub enum SuggestionSource {
    Instinct,
    PastNote,
    VenueHistory,
    ShowHistory,
}

impl std::fmt::Display for SuggestionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuggestionSource::Instinct => write!(f, "learned pattern"),
            SuggestionSource::PastNote => write!(f, "past note"),
            SuggestionSource::VenueHistory => write!(f, "venue history"),
            SuggestionSource::ShowHistory => write!(f, "show history"),
        }
    }
}

/// Get suggestions based on context
pub async fn get_suggestions(pool: &DbPool, context: &SuggestionContext) -> Result<Vec<Suggestion>> {
    let mut suggestions = Vec::new();
    
    // Get instinct-based suggestions
    if let Some(venue) = &context.venue {
        let instincts = get_instincts(pool, venue).await?;
        for instinct in instincts {
            suggestions.push(Suggestion {
                content: instinct.content.clone(),
                source: SuggestionSource::Instinct,
                confidence: instinct.confidence,
                reason: format!("Based on pattern at {}", instinct.trigger_context),
            });
        }
    }
    
    if let Some(show) = &context.show_name {
        let instincts = get_instincts(pool, show).await?;
        for instinct in instincts {
            suggestions.push(Suggestion {
                content: instinct.content.clone(),
                source: SuggestionSource::Instinct,
                confidence: instinct.confidence,
                reason: format!("Based on pattern for {}", instinct.trigger_context),
            });
        }
    }
    
    // Get venue-history based suggestions
    if let Some(venue) = &context.venue {
        let venue_suggestions = get_venue_suggestions(pool, venue).await?;
        suggestions.extend(venue_suggestions);
    }
    
    // Get show-history based suggestions
    if let Some(show) = &context.show_name {
        let show_suggestions = get_show_suggestions(pool, show).await?;
        suggestions.extend(show_suggestions);
    }
    
    // Deduplicate similar suggestions
    dedup_suggestions(&mut suggestions);
    
    // Sort by confidence
    suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
    
    // Limit results
    suggestions.truncate(5);
    
    Ok(suggestions)
}

/// Get suggestions based on venue history
async fn get_venue_suggestions(pool: &DbPool, venue: &str) -> Result<Vec<Suggestion>> {
    let notes = stagehand::search_by_venue(pool, venue).await?;
    
    let mut suggestions = Vec::new();
    
    // Look for recurring content in notes for this venue
    let contents: Vec<_> = notes.iter()
        .filter_map(|n| n.notes.as_ref())
        .collect();
    
    if contents.len() >= 2 {
        // Find common themes
        let common = find_common_content(&contents);
        for (content, count) in common {
            let confidence = (count as f32 / contents.len() as f32).min(1.0);
            if confidence >= 0.3 {
                suggestions.push(Suggestion {
                    content,
                    source: SuggestionSource::VenueHistory,
                    confidence,
                    reason: format!("Noted {} times at {}", count, venue),
                });
            }
        }
    }
    
    Ok(suggestions)
}

/// Get suggestions based on show history
async fn get_show_suggestions(pool: &DbPool, show_name: &str) -> Result<Vec<Suggestion>> {
    let notes = stagehand::search_by_show(pool, show_name).await?;
    
    let mut suggestions = Vec::new();
    
    // Look for common patterns across shows
    let contents: Vec<_> = notes.iter()
        .filter_map(|n| n.notes.as_ref())
        .collect();
    
    if contents.len() >= 2 {
        let common = find_common_content(&contents);
        for (content, count) in common {
            let confidence = (count as f32 / contents.len() as f32).min(1.0);
            if confidence >= 0.3 {
                suggestions.push(Suggestion {
                    content,
                    source: SuggestionSource::ShowHistory,
                    confidence,
                    reason: format!("Noted {} times for {}", count, show_name),
                });
            }
        }
    }
    
    // Also suggest based on tags
    let all_tags: Vec<_> = notes.iter()
        .filter_map(|n| n.tags.as_ref())
        .flatten()
        .collect();
    
    let mut tag_counts: std::collections::HashMap<&String, usize> = std::collections::HashMap::new();
    for tag in &all_tags {
        *tag_counts.entry(tag).or_default() += 1;
    }
    
    for (tag, count) in tag_counts {
        if count >= 2 {
            suggestions.push(Suggestion {
                content: format!("Common tag: {}", tag),
                source: SuggestionSource::ShowHistory,
                confidence: (count as f32 / notes.len() as f32).min(0.8),
                reason: format!("Tag appears {} times for {}", count, show_name),
            });
        }
    }
    
    Ok(suggestions)
}

/// Find common content snippets across multiple texts
fn find_common_content(contents: &[&String]) -> Vec<(String, usize)> {
    let mut sentence_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    
    for content in contents {
        // Split by sentence-like boundaries
        let sentences: Vec<_> = content
            .split(|c| c == '.' || c == '\n' || c == ';')
            .map(|s| s.trim())
            .filter(|s| s.len() > 10 && s.len() < 100)
            .collect();
        
        for sentence in sentences {
            let normalized = sentence.to_lowercase();
            *sentence_counts.entry(normalized).or_default() += 1;
        }
    }
    
    let mut common: Vec<_> = sentence_counts.into_iter()
        .filter(|(_, count)| *count >= 2)
        .collect();
    
    common.sort_by(|a, b| b.1.cmp(&a.1));
    common.truncate(5);
    common
}

/// Remove duplicate/similar suggestions
fn dedup_suggestions(suggestions: &mut Vec<Suggestion>) {
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    
    suggestions.retain(|s| {
        let key = s.content.to_lowercase();
        if seen.contains(&key) {
            false
        } else {
            seen.insert(key);
            true
        }
    });
}

/// Format suggestions for display
pub fn format_suggestions(suggestions: &[Suggestion]) -> String {
    if suggestions.is_empty() {
        return "No suggestions available.".to_string();
    }
    
    let mut output = String::from("## Suggestions\n\n");
    
    for (i, suggestion) in suggestions.iter().enumerate() {
        output.push_str(&format!(
            "{}. **{}**\n   *{}* (confidence: {:.0}%)\n\n",
            i + 1,
            suggestion.content,
            suggestion.reason,
            suggestion.confidence * 100.0
        ));
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggestion_context() {
        let ctx = SuggestionContext::new()
            .with_venue("Beacon Theatre")
            .with_show("Hamilton")
            .with_hour(9);
        
        assert_eq!(ctx.venue, Some("Beacon Theatre".to_string()));
        assert_eq!(ctx.show_name, Some("Hamilton".to_string()));
        assert_eq!(ctx.hour, Some(9));
    }

    #[test]
    fn test_format_suggestions() {
        let suggestions = vec![
            Suggestion {
                content: "Bring extra stingers".to_string(),
                source: SuggestionSource::VenueHistory,
                confidence: 0.8,
                reason: "Noted 4 times at Beacon Theatre".to_string(),
            },
        ];
        
        let formatted = format_suggestions(&suggestions);
        assert!(formatted.contains("Bring extra stingers"));
        assert!(formatted.contains("80%"));
    }

    #[test]
    fn test_dedup() {
        let mut suggestions = vec![
            Suggestion {
                content: "Test suggestion".to_string(),
                source: SuggestionSource::Instinct,
                confidence: 0.8,
                reason: "test".to_string(),
            },
            Suggestion {
                content: "test suggestion".to_string(), // Same, different case
                source: SuggestionSource::VenueHistory,
                confidence: 0.7,
                reason: "test 2".to_string(),
            },
        ];
        
        dedup_suggestions(&mut suggestions);
        assert_eq!(suggestions.len(), 1);
    }
}
