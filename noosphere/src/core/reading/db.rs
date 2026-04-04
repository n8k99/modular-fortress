//! Database operations for RSS feeds and articles

use anyhow::Result;
use chrono::Utc;
use sqlx::PgPool;

use super::discovery::{fetch_feed, subscribe_to_url};
use super::types::{Feed, FirehoseArticle, ReadingComment};

/// Subscribe to a feed (with auto-discovery if needed)
pub async fn subscribe_feed(pool: &PgPool, url: &str) -> Result<Feed> {
    // Auto-discover the actual feed URL if needed
    let feed_url = subscribe_to_url(url).await?;

    // Fetch and parse the feed
    let parsed = fetch_feed(&feed_url).await?;

    // Extract feed metadata
    let title = parsed.title.map(|t| t.content).unwrap_or_else(|| "Untitled Feed".to_string());
    let site_url = parsed.links.first().map(|l| l.href.clone());
    let description = parsed.description.map(|d| d.content);

    // Insert into database
    let feed = sqlx::query_as!(
        Feed,
        r#"
        INSERT INTO feeds (url, title, site_url, description, last_fetched)
        VALUES ($1, $2, $3, $4, NOW())
        ON CONFLICT (url) DO UPDATE
        SET title = EXCLUDED.title,
            site_url = EXCLUDED.site_url,
            description = EXCLUDED.description,
            last_fetched = NOW()
        RETURNING id, url, title, site_url, description, last_fetched, fetch_error,
                  created_at, updated_at, tags
        "#,
        feed_url,
        title,
        site_url,
        description
    )
    .fetch_one(pool)
    .await?;

    Ok(feed)
}

/// List all feeds
pub async fn list_feeds(pool: &PgPool) -> Result<Vec<Feed>> {
    let feeds = sqlx::query_as!(
        Feed,
        r#"
        SELECT id, url, title, site_url, description, last_fetched, fetch_error,
               created_at, updated_at, tags
        FROM feeds
        ORDER BY title
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(feeds)
}

/// Get a single feed by ID
pub async fn get_feed(pool: &PgPool, feed_id: i32) -> Result<Option<Feed>> {
    let feed = sqlx::query_as!(
        Feed,
        r#"
        SELECT id, url, title, site_url, description, last_fetched, fetch_error,
               created_at, updated_at, tags
        FROM feeds
        WHERE id = $1
        "#,
        feed_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(feed)
}

/// Refresh a feed (fetch new articles)
pub async fn refresh_feed(pool: &PgPool, feed_id: i32) -> Result<usize> {
    let feed = get_feed(pool, feed_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Feed not found"))?;

    // Fetch and parse feed
    let parsed = match fetch_feed(&feed.url).await {
        Ok(f) => f,
        Err(e) => {
            // Record error
            sqlx::query!(
                "UPDATE feeds SET fetch_error = $1, last_fetched = NOW() WHERE id = $2",
                e.to_string(),
                feed_id
            )
            .execute(pool)
            .await?;
            return Err(e);
        }
    };

    // Clear any previous error
    sqlx::query!(
        "UPDATE feeds SET fetch_error = NULL, last_fetched = NOW() WHERE id = $1",
        feed_id
    )
    .execute(pool)
    .await?;

    let mut inserted = 0;

    // Insert articles
    for entry in parsed.entries {
        let guid = entry.id.clone();
        let title = entry.title.map(|t| t.content).unwrap_or_else(|| "Untitled".to_string());
        let url = entry.links.first().map(|l| l.href.clone()).unwrap_or_default();

        let content = entry.content.and_then(|c| c.body);
        let summary = entry.summary.map(|s| s.content);
        let author = entry.authors.first().map(|a| a.name.clone());
        let published_at = entry.published.or(entry.updated);

        // Insert (ignore duplicates)
        let result = sqlx::query!(
            r#"
            INSERT INTO articles (feed_id, guid, title, url, content, summary, author, published_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (feed_id, guid) DO NOTHING
            "#,
            feed_id,
            guid,
            title,
            url,
            content,
            summary,
            author,
            published_at
        )
        .execute(pool)
        .await?;

        if result.rows_affected() > 0 {
            inserted += 1;
        }
    }

    Ok(inserted)
}

/// Get firehose (all articles, newest first)
pub async fn get_firehose(pool: &PgPool, limit: i64) -> Result<Vec<FirehoseArticle>> {
    let articles = sqlx::query_as!(
        FirehoseArticle,
        r#"
        SELECT id, guid, title, url, content, summary, author, published_at, read_at,
               feed_id, feed_title, feed_site, feed_tags
        FROM firehose
        LIMIT $1
        "#,
        limit
    )
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

/// Mark article as read
pub async fn mark_read(pool: &PgPool, article_id: i32) -> Result<()> {
    sqlx::query!(
        "UPDATE articles SET read_at = NOW() WHERE id = $1",
        article_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Create a reading comment (stores as virtual document)
pub async fn create_comment(
    pool: &PgPool,
    article_url: &str,
    article_title: &str,
    comment_text: &str,
) -> Result<i32> {
    let comment = ReadingComment {
        article_url: article_url.to_string(),
        article_title: article_title.to_string(),
        comment: comment_text.to_string(),
        created_at: Utc::now(),
    };

    let path = comment.document_path();
    let title = format!("Reading: {}", article_title);
    let content = comment.to_markdown();

    // Insert into documents table
    let result = sqlx::query!(
        r#"
        INSERT INTO documents (path, title, content, frontmatter)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        path,
        title,
        content,
        format!(
            "---\ntitle: {}\nsource: {}\ndate: {}\ntype: reading-comment\nstatus: draft\n---",
            article_title,
            article_url,
            comment.created_at.to_rfc3339()
        )
    )
    .fetch_one(pool)
    .await?;

    Ok(result.id)
}

/// Import OPML feed list
pub async fn import_opml(pool: &PgPool, opml_content: &str) -> Result<usize> {
    // Parse OPML (basic XML parsing)
    use scraper::{Html, Selector};

    let document = Html::parse_document(opml_content);
    let outline_selector = Selector::parse("outline[xmlUrl]").unwrap();

    let mut imported = 0;

    for outline in document.select(&outline_selector) {
        if let Some(xml_url) = outline.value().attr("xmlUrl") {
            // Try to subscribe (ignore errors for individual feeds)
            if subscribe_feed(pool, xml_url).await.is_ok() {
                imported += 1;
            }
        }
    }

    Ok(imported)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_creation() {
        let comment = ReadingComment {
            article_url: "https://example.com/article".to_string(),
            article_title: "Test Article".to_string(),
            comment: "Great read!".to_string(),
            created_at: Utc::now(),
        };

        let path = comment.document_path();
        assert!(path.contains("Thought Police"));
        assert!(path.ends_with(".md"));

        let md = comment.to_markdown();
        assert!(md.contains("# Reading: Test Article"));
        assert!(md.contains("Great read!"));
    }
}
