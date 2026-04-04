//! Podcast RSS feed generation (iTunes/Apple Podcasts compatible)

use chrono::{DateTime, Utc};

use super::types::{Drop, Stream};

/// Generate a podcast RSS feed (iTunes compatible)
///
/// Creates an RSS 2.0 feed with iTunes namespace extensions for podcast distribution.
/// Suitable for Apple Podcasts, Spotify, and other podcast platforms.
pub fn generate_podcast_rss(stream: &Stream, episodes: &[Drop], base_url: &str) -> String {
    if !stream.is_podcast {
        // Fall back to regular RSS for non-podcast streams
        return super::feed::generate_rss(stream, episodes, base_url);
    }

    let channel_link = stream.site_url.as_deref().unwrap_or(base_url);
    let feed_url = format!("{}/feeds/{}.xml", base_url, stream.slug);

    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push('\n');

    // RSS with all necessary namespaces
    xml.push_str("<rss version=\"2.0\"");
    xml.push_str(" xmlns:atom=\"http://www.w3.org/2005/Atom\"");
    xml.push_str(" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\"");
    xml.push_str(" xmlns:itunes=\"http://www.itunes.com/dtds/podcast-1.0.dtd\"");
    xml.push_str(" xmlns:podcast=\"https://podcastindex.org/namespace/1.0\"");
    xml.push_str(">\n<channel>\n");

    // Basic channel info
    xml.push_str(&format!("  <title>{}</title>\n", escape_xml(&stream.title)));
    xml.push_str(&format!("  <link>{}</link>\n", escape_xml(channel_link)));

    if let Some(desc) = &stream.description {
        xml.push_str(&format!("  <description>{}</description>\n", escape_xml(desc)));
        xml.push_str(&format!("  <itunes:summary>{}</itunes:summary>\n", escape_xml(desc)));
    }

    // Self-referential atom link
    xml.push_str(&format!(
        "  <atom:link href=\"{}\" rel=\"self\" type=\"application/rss+xml\"/>\n",
        escape_xml(&feed_url)
    ));

    if let Some(lang) = &stream.language {
        xml.push_str(&format!("  <language>{}</language>\n", escape_xml(lang)));
    }

    // iTunes-specific tags
    if let Some(author) = &stream.author {
        xml.push_str(&format!("  <itunes:author>{}</itunes:author>\n", escape_xml(author)));
    }

    if let Some(email) = &stream.email {
        xml.push_str(&format!(
            "  <itunes:owner>\n    <itunes:name>{}</itunes:name>\n    <itunes:email>{}</itunes:email>\n  </itunes:owner>\n",
            escape_xml(stream.author.as_deref().unwrap_or("Unknown")),
            escape_xml(email)
        ));
    }

    // Podcast image (required by Apple)
    if let Some(image) = &stream.podcast_image {
        xml.push_str(&format!("  <itunes:image href=\"{}\"/>\n", escape_xml(image)));
        xml.push_str(&format!(
            "  <image>\n    <url>{}</url>\n    <title>{}</title>\n    <link>{}</link>\n  </image>\n",
            escape_xml(image),
            escape_xml(&stream.title),
            escape_xml(channel_link)
        ));
    }

    // Category
    if let Some(category) = &stream.podcast_category {
        // iTunes categories can be hierarchical: "Music" or "Society &amp; Culture/History"
        if let Some((main, sub)) = category.split_once('/') {
            xml.push_str(&format!(
                "  <itunes:category text=\"{}\">\n    <itunes:category text=\"{}\"/>\n  </itunes:category>\n",
                escape_xml(main.trim()),
                escape_xml(sub.trim())
            ));
        } else {
            xml.push_str(&format!("  <itunes:category text=\"{}\"/>\n", escape_xml(category)));
        }
    }

    // Explicit content flag
    xml.push_str(&format!(
        "  <itunes:explicit>{}</itunes:explicit>\n",
        if stream.podcast_explicit { "true" } else { "false" }
    ));

    // Type (episodic or serial)
    xml.push_str("  <itunes:type>episodic</itunes:type>\n");

    // Last build date
    if let Some(most_recent) = episodes.iter().filter_map(|e| e.published_at).max() {
        xml.push_str(&format!("  <lastBuildDate>{}</lastBuildDate>\n", rfc2822_date(&most_recent)));
    }

    xml.push_str("  <generator>dpn-core/publish</generator>\n");

    // Episodes
    for episode in episodes {
        xml.push_str(&generate_podcast_episode(episode, stream, base_url));
    }

    xml.push_str("</channel>\n</rss>\n");
    xml
}

/// Generate a single podcast episode item
fn generate_podcast_episode(episode: &Drop, stream: &Stream, base_url: &str) -> String {
    let mut item = String::from("  <item>\n");

    let episode_url = format!("{}/{}/{}", base_url, stream.slug, episode.slug);

    item.push_str(&format!("    <title>{}</title>\n", escape_xml(&episode.title)));
    item.push_str(&format!("    <link>{}</link>\n", escape_xml(&episode_url)));
    item.push_str(&format!("    <guid isPermaLink=\"true\">{}</guid>\n", escape_xml(&episode_url)));

    // Description (required)
    if let Some(excerpt) = &episode.excerpt {
        item.push_str(&format!("    <description>{}</description>\n", escape_xml(excerpt)));
        item.push_str(&format!("    <itunes:summary>{}</itunes:summary>\n", escape_xml(excerpt)));
    } else if let Some(html) = &episode.content_html {
        let stripped = strip_html(html);
        let desc = if stripped.len() > 4000 {
            format!("{}…", &stripped[..4000])
        } else {
            stripped
        };
        item.push_str(&format!("    <description>{}</description>\n", escape_xml(&desc)));
        item.push_str(&format!("    <itunes:summary>{}</itunes:summary>\n", escape_xml(&desc)));
    }

    // Enclosure (required for podcasts)
    if let Some(url) = &episode.enclosure_url {
        let mime_type = episode.enclosure_type.as_deref().unwrap_or("audio/mpeg");
        let length = episode.enclosure_length.unwrap_or(0);
        item.push_str(&format!(
            "    <enclosure url=\"{}\" length=\"{}\" type=\"{}\"/>\n",
            escape_xml(url),
            length,
            escape_xml(mime_type)
        ));
    }

    // Duration (HH:MM:SS format)
    if let Some(duration) = episode.duration_seconds {
        let formatted = format_duration(duration);
        item.push_str(&format!("    <itunes:duration>{}</itunes:duration>\n", formatted));
    }

    // Episode artwork (optional, falls back to channel image)
    if let Some(image) = &episode.featured_image {
        item.push_str(&format!("    <itunes:image href=\"{}\"/>\n", escape_xml(image)));
    }

    // Author
    if let Some(author) = &episode.author {
        item.push_str(&format!("    <itunes:author>{}</itunes:author>\n", escape_xml(author)));
        item.push_str(&format!("    <author>{}</author>\n", escape_xml(author)));
    }

    // Published date
    if let Some(pub_date) = episode.published_at {
        item.push_str(&format!("    <pubDate>{}</pubDate>\n", rfc2822_date(&pub_date)));
    }

    // Show notes (full HTML content)
    if let Some(html) = &episode.content_html {
        item.push_str(&format!(
            "    <content:encoded><![CDATA[{}]]></content:encoded>\n",
            html
        ));
    }

    // Episode type (full, trailer, or bonus)
    item.push_str("    <itunes:episodeType>full</itunes:episodeType>\n");

    // Explicit per-episode
    item.push_str("    <itunes:explicit>false</itunes:explicit>\n");

    item.push_str("  </item>\n");
    item
}

/// Format duration in seconds to HH:MM:SS or MM:SS
fn format_duration(seconds: i32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Strip HTML tags for plain text
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

    result
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&nbsp;", " ")
}

/// Format date as RFC 2822
fn rfc2822_date(dt: &DateTime<Utc>) -> String {
    dt.format("%a, %d %b %Y %H:%M:%S +0000").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::publish::DropStatus;

    fn podcast_stream() -> Stream {
        Stream {
            id: 1,
            slug: "living-room-music".to_string(),
            title: "Living Room Music".to_string(),
            description: Some("A music podcast".to_string()),
            site_url: Some("https://n8k99.com/podcast".to_string()),
            language: Some("en".to_string()),
            author: Some("Nate".to_string()),
            email: Some("podcast@n8k99.com".to_string()),
            is_podcast: true,
            podcast_category: Some("Music".to_string()),
            podcast_image: Some("https://n8k99.com/images/podcast-cover.jpg".to_string()),
            podcast_explicit: false,
            created_at: None,
            updated_at: None,
        }
    }

    fn podcast_episode() -> Drop {
        Drop {
            id: 1,
            stream_id: 1,
            slug: "episode-1".to_string(),
            title: "Episode 1: Welcome".to_string(),
            content_markdown: "Show notes...".to_string(),
            content_html: Some("<p>Show notes...</p>".to_string()),
            excerpt: Some("Welcome to the first episode!".to_string()),
            author: Some("Nate".to_string()),
            status: DropStatus::Published,
            published_at: Some(Utc::now()),
            enclosure_url: Some("https://n8k99.com/audio/ep1.mp3".to_string()),
            enclosure_type: Some("audio/mpeg".to_string()),
            enclosure_length: Some(45_000_000), // ~45MB
            duration_seconds: Some(2700), // 45 minutes
            featured_image: Some("https://n8k99.com/images/ep1.jpg".to_string()),
            tags: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }

    #[test]
    fn test_generate_podcast_rss() {
        let stream = podcast_stream();
        let episodes = vec![podcast_episode()];
        let rss = generate_podcast_rss(&stream, &episodes, "https://n8k99.com");

        // Check namespaces
        assert!(rss.contains("xmlns:itunes=\"http://www.itunes.com/dtds/podcast-1.0.dtd\""));
        assert!(rss.contains("xmlns:podcast=\"https://podcastindex.org/namespace/1.0\""));

        // Check channel info
        assert!(rss.contains("<itunes:author>Nate</itunes:author>"));
        assert!(rss.contains("<itunes:category text=\"Music\""));
        assert!(rss.contains("<itunes:explicit>false</itunes:explicit>"));
        assert!(rss.contains("itunes:image href=\"https://n8k99.com/images/podcast-cover.jpg\""));

        // Check episode
        assert!(rss.contains("<title>Episode 1: Welcome</title>"));
        assert!(rss.contains("enclosure url=\"https://n8k99.com/audio/ep1.mp3\""));
        assert!(rss.contains("<itunes:duration>45:00</itunes:duration>"));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(65), "01:05");
        assert_eq!(format_duration(3665), "01:01:05");
        assert_eq!(format_duration(2700), "45:00");
        assert_eq!(format_duration(0), "00:00");
    }
}
