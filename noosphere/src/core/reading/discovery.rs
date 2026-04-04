//! RSS feed discovery and parsing

use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use url::Url;

/// Discover RSS/Atom feeds from an HTML page
/// Looks for <link rel="alternate" type="application/rss+xml"> tags
pub async fn discover_feeds(url: &str) -> Result<Vec<String>> {
    // Fetch the HTML
    let response = reqwest::get(url).await?;
    let html = response.text().await?;

    // Parse HTML
    let document = Html::parse_document(&html);

    // Look for RSS/Atom feed links
    let mut feeds = Vec::new();

    // RSS 2.0
    if let Ok(selector) = Selector::parse("link[rel='alternate'][type='application/rss+xml']") {
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                feeds.push(resolve_url(url, href)?);
            }
        }
    }

    // Atom
    if let Ok(selector) = Selector::parse("link[rel='alternate'][type='application/atom+xml']") {
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                feeds.push(resolve_url(url, href)?);
            }
        }
    }

    if feeds.is_empty() {
        // Try common feed URLs as fallback
        let base = Url::parse(url)?;
        let common_paths = vec![
            "/feed",
            "/rss",
            "/feed.xml",
            "/rss.xml",
            "/atom.xml",
            "/feed/",
            "/rss/",
        ];

        for path in common_paths {
            if let Ok(feed_url) = base.join(path) {
                feeds.push(feed_url.to_string());
            }
        }
    }

    Ok(feeds)
}

/// Resolve relative URL to absolute
fn resolve_url(base: &str, href: &str) -> Result<String> {
    if href.starts_with("http://") || href.starts_with("https://") {
        Ok(href.to_string())
    } else {
        let base_url = Url::parse(base)?;
        let resolved = base_url.join(href)?;
        Ok(resolved.to_string())
    }
}

/// Check if a URL is likely a feed URL (not an HTML page)
pub fn is_feed_url(url: &str) -> bool {
    url.ends_with(".xml")
        || url.ends_with("/feed")
        || url.ends_with("/rss")
        || url.ends_with("/atom")
        || url.contains("/feed/")
        || url.contains("/rss/")
        || url.contains("/atom/")
}

/// Try to subscribe to a URL (auto-discovers if needed)
pub async fn subscribe_to_url(url: &str) -> Result<String> {
    if is_feed_url(url) {
        // Likely already a feed URL, try it directly
        match fetch_feed(url).await {
            Ok(_) => return Ok(url.to_string()),
            Err(_) => {
                // Fall through to discovery
            }
        }
    }

    // Not a feed URL or direct fetch failed - try discovery
    let discovered = discover_feeds(url).await?;

    if discovered.is_empty() {
        return Err(anyhow!("No RSS/Atom feeds found at {}", url));
    }

    // Return the first discovered feed
    Ok(discovered[0].clone())
}

/// Fetch and parse a feed
pub async fn fetch_feed(feed_url: &str) -> Result<feed_rs::model::Feed> {
    let response = reqwest::get(feed_url).await?;
    let content = response.bytes().await?;
    let feed = feed_rs::parser::parse(&content[..])?;
    Ok(feed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_feed_url() {
        assert!(is_feed_url("https://example.com/feed.xml"));
        assert!(is_feed_url("https://example.com/rss.xml"));
        assert!(is_feed_url("https://example.com/feed"));
        assert!(is_feed_url("https://example.com/feed/"));
        assert!(!is_feed_url("https://example.com/article"));
        assert!(!is_feed_url("https://example.com/"));
    }

    #[test]
    fn test_resolve_url() {
        let base = "https://example.com/articles/foo";

        assert_eq!(
            resolve_url(base, "/feed.xml").unwrap(),
            "https://example.com/feed.xml"
        );

        assert_eq!(
            resolve_url(base, "https://other.com/rss").unwrap(),
            "https://other.com/rss"
        );
    }
}
