//! Content preparation: Markdown → HTML conversion and frontmatter extraction

use regex::Regex;
use once_cell::sync::Lazy;

/// Frontmatter extracted from a markdown document
#[derive(Debug, Clone, Default)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub date: Option<String>,
    pub author: Option<String>,
    pub excerpt: Option<String>,
    pub tags: Option<Vec<String>>,
    pub featured_image: Option<String>,
    pub raw: Option<String>,
}

static FRONTMATTER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?s)^---\n(.*?)\n---\n?").unwrap()
});

/// Extract frontmatter from markdown content
pub fn extract_frontmatter(content: &str) -> (Frontmatter, String) {
    if let Some(caps) = FRONTMATTER_REGEX.captures(content) {
        let yaml_block = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let body = FRONTMATTER_REGEX.replace(content, "").to_string();

        let fm = parse_frontmatter_yaml(yaml_block);
        (fm, body.trim().to_string())
    } else {
        (Frontmatter::default(), content.to_string())
    }
}

/// Parse YAML frontmatter (simple key: value parsing)
fn parse_frontmatter_yaml(yaml: &str) -> Frontmatter {
    let mut fm = Frontmatter {
        raw: Some(yaml.to_string()),
        ..Default::default()
    };

    for line in yaml.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim().trim_matches('"').trim_matches('\'');

            match key.as_str() {
                "title" => fm.title = Some(value.to_string()),
                "date" | "published" | "published_at" => fm.date = Some(value.to_string()),
                "author" => fm.author = Some(value.to_string()),
                "excerpt" | "description" | "summary" => fm.excerpt = Some(value.to_string()),
                "image" | "featured_image" | "cover" => fm.featured_image = Some(value.to_string()),
                "tags" => {
                    // Handle both "tags: [a, b, c]" and "tags: a, b, c"
                    let tags: Vec<String> = value
                        .trim_matches('[')
                        .trim_matches(']')
                        .split(',')
                        .map(|t| t.trim().trim_matches('"').trim_matches('\'').to_string())
                        .filter(|t| !t.is_empty())
                        .collect();
                    if !tags.is_empty() {
                        fm.tags = Some(tags);
                    }
                }
                _ => {}
            }
        }
    }

    fm
}

/// Convert markdown to HTML
///
/// Handles:
/// - Headers (# ## ### etc.)
/// - Bold (**text**) and italic (*text* or _text_)
/// - Links [text](url)
/// - Images ![alt](url)
/// - Code blocks (``` and inline `)
/// - Blockquotes (>)
/// - Unordered lists (- or *)
/// - Ordered lists (1. 2. etc.)
/// - Horizontal rules (--- or ***)
/// - Paragraphs
pub fn markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut code_block_content = String::new();
    let mut in_list = false;
    let mut list_type = "ul";
    let mut in_blockquote = false;
    let mut blockquote_content = String::new();

    for line in markdown.lines() {
        // Handle code blocks
        if line.starts_with("```") {
            if in_code_block {
                html.push_str(&format!("<pre><code>{}</code></pre>\n", escape_html(&code_block_content)));
                code_block_content.clear();
                in_code_block = false;
            } else {
                // Close any open list or blockquote
                if in_list {
                    html.push_str(&format!("</{}>\n", list_type));
                    in_list = false;
                }
                if in_blockquote {
                    html.push_str(&format!("<blockquote>{}</blockquote>\n", process_inline(&blockquote_content)));
                    blockquote_content.clear();
                    in_blockquote = false;
                }
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            if !code_block_content.is_empty() {
                code_block_content.push('\n');
            }
            code_block_content.push_str(line);
            continue;
        }

        // Handle empty lines
        if line.trim().is_empty() {
            if in_list {
                html.push_str(&format!("</{}>\n", list_type));
                in_list = false;
            }
            if in_blockquote {
                html.push_str(&format!("<blockquote>{}</blockquote>\n", process_inline(&blockquote_content)));
                blockquote_content.clear();
                in_blockquote = false;
            }
            continue;
        }

        let trimmed = line.trim();

        // Horizontal rule
        if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            if in_list {
                html.push_str(&format!("</{}>\n", list_type));
                in_list = false;
            }
            html.push_str("<hr />\n");
            continue;
        }

        // Headers
        if let Some(header_html) = parse_header(trimmed) {
            if in_list {
                html.push_str(&format!("</{}>\n", list_type));
                in_list = false;
            }
            html.push_str(&header_html);
            continue;
        }

        // Blockquotes
        if trimmed.starts_with('>') {
            let content = trimmed[1..].trim();
            if !blockquote_content.is_empty() {
                blockquote_content.push(' ');
            }
            blockquote_content.push_str(content);
            in_blockquote = true;
            continue;
        } else if in_blockquote {
            html.push_str(&format!("<blockquote>{}</blockquote>\n", process_inline(&blockquote_content)));
            blockquote_content.clear();
            in_blockquote = false;
        }

        // Unordered lists
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            if !in_list || list_type != "ul" {
                if in_list {
                    html.push_str(&format!("</{}>\n", list_type));
                }
                html.push_str("<ul>\n");
                in_list = true;
                list_type = "ul";
            }
            let item = trimmed[2..].trim();
            html.push_str(&format!("<li>{}</li>\n", process_inline(item)));
            continue;
        }

        // Ordered lists
        if let Some(item) = parse_ordered_list_item(trimmed) {
            if !in_list || list_type != "ol" {
                if in_list {
                    html.push_str(&format!("</{}>\n", list_type));
                }
                html.push_str("<ol>\n");
                in_list = true;
                list_type = "ol";
            }
            html.push_str(&format!("<li>{}</li>\n", process_inline(&item)));
            continue;
        }

        // Close list if we hit a non-list item
        if in_list {
            html.push_str(&format!("</{}>\n", list_type));
            in_list = false;
        }

        // Regular paragraph
        html.push_str(&format!("<p>{}</p>\n", process_inline(trimmed)));
    }

    // Close any remaining open tags
    if in_code_block {
        html.push_str(&format!("<pre><code>{}</code></pre>\n", escape_html(&code_block_content)));
    }
    if in_list {
        html.push_str(&format!("</{}>\n", list_type));
    }
    if in_blockquote {
        html.push_str(&format!("<blockquote>{}</blockquote>\n", process_inline(&blockquote_content)));
    }

    html
}

/// Parse header lines (# ## ### etc.)
fn parse_header(line: &str) -> Option<String> {
    let mut level = 0;
    for c in line.chars() {
        if c == '#' {
            level += 1;
        } else {
            break;
        }
    }

    if level > 0 && level <= 6 && line.len() > level {
        let content = line[level..].trim();
        Some(format!("<h{}>{}</h{}>\n", level, process_inline(content), level))
    } else {
        None
    }
}

/// Parse ordered list items (1. 2. etc.)
fn parse_ordered_list_item(line: &str) -> Option<String> {
    static OL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)\.\s+(.+)$").unwrap());

    OL_REGEX.captures(line).map(|caps| caps.get(2).unwrap().as_str().to_string())
}

/// Process inline markdown (bold, italic, links, images, code)
fn process_inline(text: &str) -> String {
    let mut result = escape_html(text);

    // Images: ![alt](url)
    static IMG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap());
    result = IMG_REGEX.replace_all(&result, r#"<img src="$2" alt="$1" />"#).to_string();

    // Links: [text](url)
    static LINK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap());
    result = LINK_REGEX.replace_all(&result, r#"<a href="$2">$1</a>"#).to_string();

    // Inline code: `code`
    static CODE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"`([^`]+)`").unwrap());
    result = CODE_REGEX.replace_all(&result, r"<code>$1</code>").to_string();

    // Bold: **text** or __text__
    static BOLD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\*\*([^*]+)\*\*|__([^_]+)__").unwrap());
    result = BOLD_REGEX.replace_all(&result, |caps: &regex::Captures| {
        let content = caps.get(1).or_else(|| caps.get(2)).unwrap().as_str();
        format!("<strong>{}</strong>", content)
    }).to_string();

    // Italic: *text* or _text_ (but not inside words for underscores)
    static ITALIC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\*([^*]+)\*|(?:^|[^a-zA-Z])_([^_]+)_(?:[^a-zA-Z]|$)").unwrap());
    result = ITALIC_REGEX.replace_all(&result, |caps: &regex::Captures| {
        let content = caps.get(1).or_else(|| caps.get(2)).unwrap().as_str();
        format!("<em>{}</em>", content)
    }).to_string();

    result
}

/// Escape HTML special characters
fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Extract the first paragraph as an excerpt (if no explicit excerpt)
pub fn auto_excerpt(markdown: &str, max_length: usize) -> String {
    let (_, body) = extract_frontmatter(markdown);

    // Skip headers and get first paragraph
    let mut excerpt = String::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if !excerpt.is_empty() {
                break;
            }
            continue;
        }
        if trimmed.starts_with('#') {
            continue;
        }
        if !excerpt.is_empty() {
            excerpt.push(' ');
        }
        excerpt.push_str(trimmed);
    }

    // Truncate if needed
    if excerpt.len() > max_length {
        let truncated = &excerpt[..max_length];
        // Try to break at word boundary
        if let Some(last_space) = truncated.rfind(' ') {
            format!("{}…", &truncated[..last_space])
        } else {
            format!("{}…", truncated)
        }
    } else {
        excerpt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_frontmatter() {
        let content = r#"---
title: My Post
author: Test Author
date: 2024-01-15
tags: [rust, programming]
---

# Hello World

This is the body."#;

        let (fm, body) = extract_frontmatter(content);
        assert_eq!(fm.title, Some("My Post".to_string()));
        assert_eq!(fm.author, Some("Test Author".to_string()));
        assert_eq!(fm.date, Some("2024-01-15".to_string()));
        assert!(body.starts_with("# Hello World"));
    }

    #[test]
    fn test_markdown_headers() {
        assert!(markdown_to_html("# H1").contains("<h1>H1</h1>"));
        assert!(markdown_to_html("## H2").contains("<h2>H2</h2>"));
        assert!(markdown_to_html("### H3").contains("<h3>H3</h3>"));
    }

    #[test]
    fn test_markdown_bold_italic() {
        assert!(markdown_to_html("**bold**").contains("<strong>bold</strong>"));
        assert!(markdown_to_html("*italic*").contains("<em>italic</em>"));
    }

    #[test]
    fn test_markdown_links() {
        let html = markdown_to_html("[link](https://example.com)");
        assert!(html.contains(r#"<a href="https://example.com">link</a>"#));
    }

    #[test]
    fn test_markdown_code() {
        let html = markdown_to_html("`inline code`");
        assert!(html.contains("<code>inline code</code>"));

        let html = markdown_to_html("```\ncode block\n```");
        assert!(html.contains("<pre><code>code block</code></pre>"));
    }

    #[test]
    fn test_markdown_lists() {
        let html = markdown_to_html("- item 1\n- item 2");
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>item 1</li>"));
        assert!(html.contains("<li>item 2</li>"));

        let html = markdown_to_html("1. first\n2. second");
        assert!(html.contains("<ol>"));
        assert!(html.contains("<li>first</li>"));
    }

    #[test]
    fn test_auto_excerpt() {
        let content = "# Title\n\nThis is the first paragraph which is quite long.\n\nSecond paragraph.";
        let excerpt = auto_excerpt(content, 30);
        assert!(excerpt.starts_with("This is the first"));
        assert!(excerpt.ends_with('…'));
    }
}
