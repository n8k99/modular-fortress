//! Instinct storage
//!
//! Stores learned patterns as "instincts" - automatic behaviors
//! that can be triggered by context.

use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::FromRow;

use crate::db::DbPool;
use crate::patterns::extractor::Pattern;

/// A learned instinct
#[derive(Debug, Clone, FromRow)]
pub struct Instinct {
    pub id: i64,
    pub content: String,
    pub entry_type: Option<String>,
    pub importance: Option<i16>,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub agent_id: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub confidence: f32,
    pub trigger_context: Option<String>,
}

/// Minimal instinct data (when loaded from memory_entries)
#[derive(Debug, Clone)]
pub struct InstinctData {
    pub content: String,
    pub confidence: f32,
    pub trigger_context: String,
    pub tags: Vec<String>,
}

impl From<&Pattern> for InstinctData {
    fn from(pattern: &Pattern) -> Self {
        Self {
            content: pattern.value.clone(),
            confidence: pattern.confidence,
            trigger_context: pattern.key.clone(),
            tags: vec![pattern.pattern_type.to_string()],
        }
    }
}

/// Save an instinct from a detected pattern
pub async fn save_instinct(
    pool: &DbPool, 
    pattern: &Pattern, 
    agent_id: &str,
) -> Result<i64> {
    // Store as memory_entry with type='instinct'
    // Encode pattern data in the content and tags
    
    let content = format!(
        "Instinct: {} [{}]\nTrigger: {}\nFrequency: {}\nConfidence: {:.2}",
        pattern.value,
        pattern.pattern_type,
        pattern.key,
        pattern.frequency,
        pattern.confidence,
    );
    
    let mut tags = vec![
        "instinct".to_string(),
        pattern.pattern_type.to_string(),
    ];
    tags.push(pattern.key.clone());
    
    let importance = (pattern.confidence * 10.0) as i16;
    
    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO memory_entries (content, entry_type, importance, source, tags, agent_id)
        VALUES ($1, 'instinct', $2, 'pattern_learning', $3, $4)
        RETURNING id
        "#
    )
    .bind(&content)
    .bind(importance)
    .bind(&tags)
    .bind(agent_id)
    .fetch_one(pool)
    .await?;
    
    Ok(row.0)
}

/// Get instincts relevant to a context
pub async fn get_instincts(pool: &DbPool, context: &str) -> Result<Vec<InstinctData>> {
    // Search for instincts that match the context
    // Context could be a venue name, show name, or tags
    
    let pattern = format!("%{}%", context);
    
    let rows = sqlx::query_as::<_, (String, Option<i16>, Option<Vec<String>>)>(
        r#"
        SELECT content, importance, tags
        FROM memory_entries
        WHERE entry_type = 'instinct'
          AND (content ILIKE $1 OR $1 = ANY(tags))
        ORDER BY importance DESC
        LIMIT 10
        "#
    )
    .bind(&pattern)
    .fetch_all(pool)
    .await?;
    
    let instincts = rows.into_iter()
        .filter_map(|(content, importance, tags)| {
            parse_instinct_content(&content, importance, tags)
        })
        .collect();
    
    Ok(instincts)
}

/// Get all instincts for an agent
pub async fn get_all_instincts(pool: &DbPool, agent_id: &str) -> Result<Vec<InstinctData>> {
    let rows = sqlx::query_as::<_, (String, Option<i16>, Option<Vec<String>>)>(
        r#"
        SELECT content, importance, tags
        FROM memory_entries
        WHERE entry_type = 'instinct' AND agent_id = $1
        ORDER BY importance DESC
        "#
    )
    .bind(agent_id)
    .fetch_all(pool)
    .await?;
    
    let instincts = rows.into_iter()
        .filter_map(|(content, importance, tags)| {
            parse_instinct_content(&content, importance, tags)
        })
        .collect();
    
    Ok(instincts)
}

/// Parse instinct content back into structured data
fn parse_instinct_content(
    content: &str, 
    importance: Option<i16>,
    tags: Option<Vec<String>>,
) -> Option<InstinctData> {
    // Parse the structured content we stored
    let lines: Vec<&str> = content.lines().collect();
    
    if lines.is_empty() {
        return None;
    }
    
    // First line: "Instinct: value [type]"
    let value = lines.first()?
        .strip_prefix("Instinct: ")?
        .split(" [")
        .next()?
        .to_string();
    
    // Second line: "Trigger: context"
    let trigger = lines.get(1)
        .and_then(|l| l.strip_prefix("Trigger: "))
        .unwrap_or("unknown")
        .to_string();
    
    // Get confidence from importance
    let confidence = importance.unwrap_or(5) as f32 / 10.0;
    
    Some(InstinctData {
        content: value,
        confidence,
        trigger_context: trigger,
        tags: tags.unwrap_or_default(),
    })
}

/// Delete an instinct by ID
pub async fn delete_instinct(pool: &DbPool, id: i64) -> Result<()> {
    sqlx::query(
        "DELETE FROM memory_entries WHERE id = $1 AND entry_type = 'instinct'"
    )
    .bind(id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Update instinct confidence (e.g., when feedback is received)
pub async fn update_confidence(pool: &DbPool, id: i64, delta: f32) -> Result<()> {
    // Adjust importance based on confidence delta
    let importance_delta = (delta * 10.0) as i16;
    
    sqlx::query(
        r#"
        UPDATE memory_entries 
        SET importance = GREATEST(1, LEAST(10, COALESCE(importance, 5) + $1))
        WHERE id = $2 AND entry_type = 'instinct'
        "#
    )
    .bind(importance_delta)
    .bind(id)
    .execute(pool)
    .await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::PatternType;

    #[test]
    fn test_instinct_from_pattern() {
        let pattern = Pattern::new(
            PatternType::VenueSpecific,
            "Beacon Theatre".into(),
            "bring extra stingers".into(),
            5,
        );
        
        let instinct = InstinctData::from(&pattern);
        assert_eq!(instinct.content, "bring extra stingers");
        assert_eq!(instinct.trigger_context, "Beacon Theatre");
        assert!((instinct.confidence - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_parse_instinct_content() {
        let content = "Instinct: bring extra stingers [venue_specific]\nTrigger: Beacon Theatre\nFrequency: 5\nConfidence: 0.50";
        let tags = Some(vec!["instinct".into(), "venue_specific".into()]);
        
        let instinct = parse_instinct_content(content, Some(5), tags).unwrap();
        assert_eq!(instinct.content, "bring extra stingers");
        assert_eq!(instinct.trigger_context, "Beacon Theatre");
    }
}
