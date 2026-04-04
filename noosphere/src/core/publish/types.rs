//! Types for the publish module

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A content stream (e.g., "Myths of Orbis", "Living Room Music", "Thought Police")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub site_url: Option<String>,
    pub language: Option<String>,
    pub author: Option<String>,
    pub email: Option<String>,
    /// For podcast streams
    pub is_podcast: bool,
    pub podcast_category: Option<String>,
    pub podcast_image: Option<String>,
    pub podcast_explicit: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create request for a new stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCreate {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub site_url: Option<String>,
    pub language: Option<String>,
    pub author: Option<String>,
    pub email: Option<String>,
    pub is_podcast: bool,
    pub podcast_category: Option<String>,
    pub podcast_image: Option<String>,
    pub podcast_explicit: bool,
}

impl Default for StreamCreate {
    fn default() -> Self {
        Self {
            slug: String::new(),
            title: String::new(),
            description: None,
            site_url: None,
            language: Some("en".to_string()),
            author: None,
            email: None,
            is_podcast: false,
            podcast_category: None,
            podcast_image: None,
            podcast_explicit: false,
        }
    }
}

/// A publishable piece of content (drop)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drop {
    pub id: i32,
    pub stream_id: i32,
    pub slug: String,
    pub title: String,
    pub content_markdown: String,
    pub content_html: Option<String>,
    pub excerpt: Option<String>,
    pub author: Option<String>,
    pub status: DropStatus,
    pub published_at: Option<DateTime<Utc>>,
    /// For podcast episodes
    pub enclosure_url: Option<String>,
    pub enclosure_type: Option<String>,
    pub enclosure_length: Option<i64>,
    pub duration_seconds: Option<i32>,
    /// Metadata
    pub featured_image: Option<String>,
    pub tags: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Drop status (draft, scheduled, published, archived)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DropStatus {
    #[default]
    Draft,
    Scheduled,
    Published,
    Archived,
}

impl DropStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DropStatus::Draft => "draft",
            DropStatus::Scheduled => "scheduled",
            DropStatus::Published => "published",
            DropStatus::Archived => "archived",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "draft" => DropStatus::Draft,
            "scheduled" => DropStatus::Scheduled,
            "published" => DropStatus::Published,
            "archived" => DropStatus::Archived,
            _ => DropStatus::Draft,
        }
    }
}

impl std::fmt::Display for DropStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Create request for a new drop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropCreate {
    pub stream_id: i32,
    pub slug: String,
    pub title: String,
    pub content_markdown: String,
    pub excerpt: Option<String>,
    pub author: Option<String>,
    pub status: DropStatus,
    pub published_at: Option<DateTime<Utc>>,
    /// Podcast episode fields
    pub enclosure_url: Option<String>,
    pub enclosure_type: Option<String>,
    pub enclosure_length: Option<i64>,
    pub duration_seconds: Option<i32>,
    /// Metadata
    pub featured_image: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Update request for an existing drop
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DropUpdate {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub content_markdown: Option<String>,
    pub excerpt: Option<String>,
    pub author: Option<String>,
    pub status: Option<DropStatus>,
    pub published_at: Option<DateTime<Utc>>,
    pub enclosure_url: Option<String>,
    pub enclosure_type: Option<String>,
    pub enclosure_length: Option<i64>,
    pub duration_seconds: Option<i32>,
    pub featured_image: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Drop with associated stream info (for feed generation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropWithStream {
    pub drop: Drop,
    pub stream: Stream,
}

/// Response for a Thought Police comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtPoliceResponse {
    pub id: i32,
    pub drop_id: i32,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub content: String,
    pub approved: bool,
    pub created_at: Option<DateTime<Utc>>,
}

/// Create request for a Thought Police response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseCreate {
    pub drop_id: i32,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub content: String,
}

/// Filter options for listing drops
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DropFilter {
    pub stream_id: Option<i32>,
    pub stream_slug: Option<String>,
    pub status: Option<DropStatus>,
    pub tag: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_status_roundtrip() {
        assert_eq!(DropStatus::from_str("draft"), DropStatus::Draft);
        assert_eq!(DropStatus::from_str("published"), DropStatus::Published);
        assert_eq!(DropStatus::Draft.as_str(), "draft");
        assert_eq!(DropStatus::Published.as_str(), "published");
    }

    #[test]
    fn test_stream_create_default() {
        let stream = StreamCreate::default();
        assert!(!stream.is_podcast);
        assert_eq!(stream.language, Some("en".to_string()));
    }
}
