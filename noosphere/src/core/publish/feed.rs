//! RSS/Atom feed generation

use chrono::{DateTime, Utc};

use super::types::{Drop, Stream};

/// Feed format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedFormat {
    Rss,
    Atom,
}

/// Generate an RSS 2.0 feed for a stream
pub fn generate_rss(stream: &Stream, drops: &[Drop], base_url: &str) -> String {
    let channel_link = stream.site_url.as_deref().unwrap_or(base_url);
    let feed_url = format!("{}/feeds/{}.xml", base_url, stream.slug);

    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push_str("\n<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\"");

    // Add content namespace for full HTML content
    xml.push_str(" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\"");

    xml.push_str(">\n<channel>\n");

    // Channel metadata
    xml.push_str(&format!("  <title>{}</title>\n", escape_xml(&stream.title)));
    xml.push_str(&format!("  <link>{}</link>\n", escape_xml(channel_link)));

    if let Some(desc) = &stream.description {
        xml.push_str(&format!("  <description>{}</description>\n", escape_xml(desc)));
    } else {
        xml.push_str(&format!("  <description>{}</description>\n", escape_xml(&stream.title)));
    }

    // Self-referential atom link
    xml.push_str(&format!(
        "  <atom:link href=\"{}\" rel=\"self\" type=\"application/rss+xml\"/>\n",
        escape_xml(&feed_url)
    ));

    if let Some(lang) = &stream.language {
        xml.push_str(&format!("  <language>{}</language>\n", escape_xml(lang)));
    }

    // Last build date
    if let Some(most_recent) = drops.iter().filter_map(|d| d.published_at).max() {
        xml.push_str(&format!("  <lastBuildDate>{}</lastBuildDate>\n", rfc2822_date(&most_recent)));
    }

    xml.push_str("  <generator>dpn-core/publish</generator>\n");

    // Items
    for drop in drops {
        xml.push_str(&generate_rss_item(drop, stream, base_url));
    }

    xml.push_str("</channel>\n</rss>\n");
    xml
}

/// Generate a single RSS item
fn generate_rss_item(drop: &Drop, stream: &Stream, base_url: &str) -> String {
    let mut item = String::from("  <item>\n");

    let item_url = format!("{}/{}/{}", base_url, stream.slug, drop.slug);

    item.push_str(&format!("    <title>{}</title>\n", escape_xml(&drop.title)));
    item.push_str(&format!("    <link>{}</link>\n", escape_xml(&item_url)));
    item.push_str(&format!("    <guid isPermaLink=\"true\">{}</guid>\n", escape_xml(&item_url)));

    // Description (excerpt or first part of content)
    if let Some(excerpt) = &drop.excerpt {
        item.push_str(&format!("    <description>{}</description>\n", escape_xml(excerpt)));
    } else if let Some(html) = &drop.content_html {
        // Use first 300 chars as description
        let stripped = strip_html(html);
        let desc = if stripped.len() > 300 {
            format!("{}…", &stripped[..300])
        } else {
            stripped
        };
        item.push_str(&format!("    <description>{}</description>\n", escape_xml(&desc)));
    }

    // Full content
    if let Some(html) = &drop.content_html {
        item.push_str(&format!(
            "    <content:encoded><![CDATA[{}]]></content:encoded>\n",
            html
        ));
    }

    // Author
    if let Some(author) = &drop.author {
        item.push_str(&format!("    <author>{}</author>\n", escape_xml(author)));
    }

    // Published date
    if let Some(pub_date) = drop.published_at {
        item.push_str(&format!("    <pubDate>{}</pubDate>\n", rfc2822_date(&pub_date)));
    }

    // Tags as categories
    if let Some(tags) = &drop.tags {
        for tag in tags {
            item.push_str(&format!("    <category>{}</category>\n", escape_xml(tag)));
        }
    }

    item.push_str("  </item>\n");
    item
}

/// Generate an Atom feed for a stream
pub fn generate_atom(stream: &Stream, drops: &[Drop], base_url: &str) -> String {
    let feed_url = format!("{}/feeds/{}.atom", base_url, stream.slug);
    let channel_link = stream.site_url.as_deref().unwrap_or(base_url);

    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push_str("\n<feed xmlns=\"http://www.w3.org/2005/Atom\">\n");

    // Feed metadata
    xml.push_str(&format!("  <title>{}</title>\n", escape_xml(&stream.title)));
    xml.push_str(&format!("  <id>{}</id>\n", escape_xml(&feed_url)));

    xml.push_str(&format!(
        "  <link href=\"{}\" rel=\"alternate\" type=\"text/html\"/>\n",
        escape_xml(channel_link)
    ));
    xml.push_str(&format!(
        "  <link href=\"{}\" rel=\"self\" type=\"application/atom+xml\"/>\n",
        escape_xml(&feed_url)
    ));

    if let Some(desc) = &stream.description {
        xml.push_str(&format!("  <subtitle>{}</subtitle>\n", escape_xml(desc)));
    }

    // Updated timestamp
    let updated = drops
        .iter()
        .filter_map(|d| d.published_at.or(d.updated_at))
        .max()
        .unwrap_or_else(Utc::now);
    xml.push_str(&format!("  <updated>{}</updated>\n", rfc3339_date(&updated)));

    // Author
    if let Some(author) = &stream.author {
        xml.push_str("  <author>\n");
        xml.push_str(&format!("    <name>{}</name>\n", escape_xml(author)));
        if let Some(email) = &stream.email {
            xml.push_str(&format!("    <email>{}</email>\n", escape_xml(email)));
        }
        xml.push_str("  </author>\n");
    }

    xml.push_str("  <generator uri=\"https://github.com/n8k99/dpn-core\" version=\"0.1.0\">dpn-core</generator>\n");

    // Entries
    for drop in drops {
        xml.push_str(&generate_atom_entry(drop, stream, base_url));
    }

    xml.push_str("</feed>\n");
    xml
}

/// Generate a single Atom entry
fn generate_atom_entry(drop: &Drop, stream: &Stream, base_url: &str) -> String {
    let mut entry = String::from("  <entry>\n");

    let entry_url = format!("{}/{}/{}", base_url, stream.slug, drop.slug);
    let entry_id = format!("urn:uuid:{}-{}", stream.slug, drop.id);

    entry.push_str(&format!("    <title>{}</title>\n", escape_xml(&drop.title)));
    entry.push_str(&format!("    <id>{}</id>\n", escape_xml(&entry_id)));
    entry.push_str(&format!(
        "    <link href=\"{}\" rel=\"alternate\" type=\"text/html\"/>\n",
        escape_xml(&entry_url)
    ));

    // Published and updated
    if let Some(pub_date) = drop.published_at {
        entry.push_str(&format!("    <published>{}</published>\n", rfc3339_date(&pub_date)));
    }

    let updated = drop.updated_at.or(drop.published_at).unwrap_or_else(Utc::now);
    entry.push_str(&format!("    <updated>{}</updated>\n", rfc3339_date(&updated)));

    // Author
    if let Some(author) = &drop.author {
        entry.push_str("    <author>\n");
        entry.push_str(&format!("      <name>{}</name>\n", escape_xml(author)));
        entry.push_str("    </author>\n");
    }

    // Summary
    if let Some(excerpt) = &drop.excerpt {
        entry.push_str(&format!("    <summary>{}</summary>\n", escape_xml(excerpt)));
    }

    // Content
    if let Some(html) = &drop.content_html {
        entry.push_str(&format!(
            "    <content type=\"html\"><![CDATA[{}]]></content>\n",
            html
        ));
    }

    // Categories
    if let Some(tags) = &drop.tags {
        for tag in tags {
            entry.push_str(&format!("    <category term=\"{}\"/>\n", escape_xml(tag)));
        }
    }

    entry.push_str("  </entry>\n");
    entry
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Strip HTML tags for plain text description
fn strip_html(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;

    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }

    // Decode common HTML entities
    result
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&nbsp;", " ")
}

/// Format date as RFC 2822 (for RSS)
fn rfc2822_date(dt: &DateTime<Utc>) -> String {
    dt.format("%a, %d %b %Y %H:%M:%S +0000").to_string()
}

/// Format date as RFC 3339 (for Atom)
fn rfc3339_date(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::publish::DropStatus;

    fn test_stream() -> Stream {
        Stream {
            id: 1,
            slug: "myths-of-orbis".to_string(),
            title: "Myths of Orbis".to_string(),
            description: Some("A fantasy worldbuilding blog".to_string()),
            site_url: Some("https://n8k99.com/myths".to_string()),
            language: Some("en".to_string()),
            author: Some("Nate".to_string()),
            email: Some("nate@n8k99.com".to_string()),
            is_podcast: false,
            podcast_category: None,
            podcast_image: None,
            podcast_explicit: false,
            created_at: None,
            updated_at: None,
        }
    }

    fn test_drop() -> Drop {
        Drop {
            id: 1,
            stream_id: 1,
            slug: "first-post".to_string(),
            title: "First Post".to_string(),
            content_markdown: "# Hello\n\nWorld".to_string(),
            content_html: Some("<h1>Hello</h1>\n<p>World</p>".to_string()),
            excerpt: Some("A short intro".to_string()),
            author: Some("Nate".to_string()),
            status: DropStatus::Published,
            published_at: Some(Utc::now()),
            enclosure_url: None,
            enclosure_type: None,
            enclosure_length: None,
            duration_seconds: None,
            featured_image: None,
            tags: Some(vec!["fantasy".to_string(), "worldbuilding".to_string()]),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }

    #[test]
    fn test_generate_rss() {
        let stream = test_stream();
        let drops = vec![test_drop()];
        let rss = generate_rss(&stream, &drops, "https://n8k99.com");

        assert!(rss.contains("<?xml version="));
        assert!(rss.contains("<rss version=\"2.0\""));
        assert!(rss.contains("<title>Myths of Orbis</title>"));
        assert!(rss.contains("<title>First Post</title>"));
        assert!(rss.contains("<category>fantasy</category>"));
        assert!(rss.contains("content:encoded"));
    }

    #[test]
    fn test_generate_atom() {
        let stream = test_stream();
        let drops = vec![test_drop()];
        let atom = generate_atom(&stream, &drops, "https://n8k99.com");

        assert!(atom.contains("<?xml version="));
        assert!(atom.contains("<feed xmlns=\"http://www.w3.org/2005/Atom\""));
        assert!(atom.contains("<title>Myths of Orbis</title>"));
        assert!(atom.contains("<title>First Post</title>"));
        assert!(atom.contains("<category term=\"fantasy\""));
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("a < b & c > d"), "a &lt; b &amp; c &gt; d");
        assert_eq!(escape_xml("\"quotes\" and 'apostrophes'"), "&quot;quotes&quot; and &apos;apostrophes&apos;");
    }

    #[test]
    fn test_strip_html() {
        assert_eq!(strip_html("<p>Hello <strong>world</strong></p>"), "Hello world");
        assert_eq!(strip_html("<h1>Title</h1><p>Text</p>"), "TitleText");
    }
}
