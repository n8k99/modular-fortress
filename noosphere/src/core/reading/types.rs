//! Types for RSS reader functionality

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Feed subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub site_url: Option<String>,
    pub description: Option<String>,
    pub last_fetched: Option<DateTime<Utc>>,
    pub fetch_error: Option<String>,
    pub created_at: Option<DateTime<Utc>>, // Nullable due to DEFAULT
    pub updated_at: Option<DateTime<Utc>>, // Nullable due to DEFAULT
    pub tags: Option<Vec<String>>,
}

/// Article from a feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub feed_id: i32,
    pub guid: String,
    pub title: String,
    pub url: String,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub author: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>, // Nullable due to DEFAULT
}

/// Firehose view: Article with feed info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirehoseArticle {
    pub id: Option<i32>, // Nullable from view
    pub guid: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub author: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub feed_id: Option<i32>,
    pub feed_title: Option<String>,
    pub feed_site: Option<String>,
    pub feed_tags: Option<Vec<String>>,
}

/// Reading comment (stored as virtual document)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingComment {
    pub article_url: String,
    pub article_title: String,
    pub comment: String,
    pub created_at: DateTime<Utc>,
}

impl ReadingComment {
    /// Generate document path for Thought Police
    /// Format: Areas/Eckenrode Muziekopname/Executive/Thought Police/YYYY-MM-DD-slug.md
    pub fn document_path(&self) -> String {
        let date = self.created_at.format("%Y-%m-%d");
        let slug = self.article_title
            .chars()
            .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { '-' })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
            .to_lowercase();

        format!("Areas/Eckenrode Muziekopname/Executive/Thought Police/{}-{}.md", date, slug)
    }

    /// Generate markdown content for the comment document
    pub fn to_markdown(&self) -> String {
        format!(
            r#"---
title: {}
source: {}
date: {}
type: reading-comment
---

# Reading: {}

**Source:** [{}]({})

---

{}
"#,
            self.article_title,
            self.article_url,
            self.created_at.to_rfc3339(),
            self.article_title,
            self.article_title,
            self.article_url,
            self.comment
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_path_generation() {
        let comment = ReadingComment {
            article_url: "https://example.com/article".to_string(),
            article_title: "Why RSS is Great!".to_string(),
            comment: "I agree.".to_string(),
            created_at: Utc::now(),
        };

        let path = comment.document_path();
        assert!(path.starts_with("Areas/Eckenrode Muziekopname/Executive/Thought Police/"));
        assert!(path.contains("why-rss-is-great"));
        assert!(path.ends_with(".md"));
    }

    #[test]
    fn test_comment_markdown() {
        let comment = ReadingComment {
            article_url: "https://example.com/test".to_string(),
            article_title: "Test Article".to_string(),
            comment: "This is my comment.".to_string(),
            created_at: Utc::now(),
        };

        let md = comment.to_markdown();
        assert!(md.contains("# Reading: Test Article"));
        assert!(md.contains("This is my comment."));
        assert!(md.contains("https://example.com/test"));
    }
}
