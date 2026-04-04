//! Wikilink parser and resolver
//!
//! Parses `[[Note Title]]` syntax and resolves to document IDs.
//!
//! Supports:
//! - `[[Note Title]]` - basic wikilink
//! - `[[Note Title|display text]]` - aliased wikilink
//! - `[[Note Title#heading]]` - wikilink with heading anchor
//! - `[[Note Title#heading|display text]]` - full form

use regex::Regex;
use sqlx::PgPool;
use std::collections::HashMap;

/// A parsed wikilink from document content
#[derive(Debug, Clone, PartialEq)]
pub struct Wikilink {
    /// The full match including brackets: [[Note Title]]
    pub raw: String,
    /// The target note title
    pub target: String,
    /// Optional heading anchor (after #)
    pub heading: Option<String>,
    /// Optional display text (after |)
    pub display: Option<String>,
    /// Start position in source text
    pub start: usize,
    /// End position in source text
    pub end: usize,
}

/// A resolved wikilink with document ID
#[derive(Debug, Clone)]
pub struct ResolvedWikilink {
    pub wikilink: Wikilink,
    pub doc_id: Option<i32>,
    pub doc_path: Option<String>,
    /// True if resolved, false if broken link
    pub resolved: bool,
}

/// Resolution result for a document's wikilinks
#[derive(Debug)]
pub struct WikilinkResolution {
    /// All wikilinks found in the document
    pub links: Vec<ResolvedWikilink>,
    /// Count of resolved links
    pub resolved_count: usize,
    /// Count of broken links
    pub broken_count: usize,
}

/// Parse all wikilinks from markdown content
pub fn parse_wikilinks(content: &str) -> Vec<Wikilink> {
    // Regex: [[target#heading|display]] - heading and display are optional
    let re = Regex::new(r"\[\[([^\]#|]+)(?:#([^\]|]+))?(?:\|([^\]]+))?\]\]").unwrap();
    
    let mut links = Vec::new();
    
    for cap in re.captures_iter(content) {
        let full_match = cap.get(0).unwrap();
        let target = cap.get(1).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
        let heading = cap.get(2).map(|m| m.as_str().trim().to_string());
        let display = cap.get(3).map(|m| m.as_str().trim().to_string());
        
        links.push(Wikilink {
            raw: full_match.as_str().to_string(),
            target,
            heading,
            display,
            start: full_match.start(),
            end: full_match.end(),
        });
    }
    
    links
}

/// Resolve a single wikilink target to a document ID
pub async fn resolve_wikilink(pool: &PgPool, target: &str) -> Option<(i32, String)> {
    // First try exact title match
    let result = sqlx::query_as::<_, (i32, String)>(
        r#"
        SELECT id, path 
        FROM documents 
        WHERE title = $1 AND is_canonical = true
        LIMIT 1
        "#
    )
    .bind(target)
    .fetch_optional(pool)
    .await
    .ok()?;
    
    if result.is_some() {
        return result;
    }
    
    // Try case-insensitive match
    let result = sqlx::query_as::<_, (i32, String)>(
        r#"
        SELECT id, path 
        FROM documents 
        WHERE LOWER(title) = LOWER($1) AND is_canonical = true
        LIMIT 1
        "#
    )
    .bind(target)
    .fetch_optional(pool)
    .await
    .ok()?;
    
    if result.is_some() {
        return result;
    }
    
    // Try title pattern match (for partial matches)
    let pattern = format!("%{}%", target);
    sqlx::query_as::<_, (i32, String)>(
        r#"
        SELECT id, path 
        FROM documents 
        WHERE title ILIKE $1 AND is_canonical = true
        ORDER BY 
            CASE WHEN LOWER(title) = LOWER($2) THEN 0
                 WHEN LOWER(title) LIKE LOWER($2) || '%' THEN 1
                 ELSE 2
            END,
            LENGTH(title)
        LIMIT 1
        "#
    )
    .bind(&pattern)
    .bind(target)
    .fetch_optional(pool)
    .await
    .ok()?
}

/// Resolve all wikilinks in content
pub async fn resolve_all_wikilinks(pool: &PgPool, content: &str) -> WikilinkResolution {
    let parsed = parse_wikilinks(content);
    let mut links = Vec::new();
    let mut resolved_count = 0;
    let mut broken_count = 0;
    
    // Build a cache of unique targets to avoid duplicate queries
    let mut resolution_cache: HashMap<String, Option<(i32, String)>> = HashMap::new();
    
    for wikilink in parsed {
        let target_lower = wikilink.target.to_lowercase();
        
        let resolution = if let Some(cached) = resolution_cache.get(&target_lower) {
            cached.clone()
        } else {
            let result = resolve_wikilink(pool, &wikilink.target).await;
            resolution_cache.insert(target_lower.clone(), result.clone());
            result
        };
        
        let (doc_id, doc_path, resolved) = match resolution {
            Some((id, path)) => {
                resolved_count += 1;
                (Some(id), Some(path), true)
            }
            None => {
                broken_count += 1;
                (None, None, false)
            }
        };
        
        links.push(ResolvedWikilink {
            wikilink,
            doc_id,
            doc_path,
            resolved,
        });
    }
    
    WikilinkResolution {
        links,
        resolved_count,
        broken_count,
    }
}

/// Get all unique wikilink targets from content (for batch resolution)
pub fn extract_unique_targets(content: &str) -> Vec<String> {
    let links = parse_wikilinks(content);
    let mut targets: Vec<String> = links.into_iter().map(|l| l.target).collect();
    targets.sort();
    targets.dedup();
    targets
}

/// Convert wikilinks to HTML links (for rendering)
pub fn wikilinks_to_html(content: &str, resolutions: &[ResolvedWikilink]) -> String {
    let mut result = content.to_string();
    
    // Process in reverse order to maintain position accuracy
    for resolved in resolutions.iter().rev() {
        let wl = &resolved.wikilink;
        let display = wl.display.as_ref().unwrap_or(&wl.target);
        
        let html = if resolved.resolved {
            let _path = resolved.doc_path.as_ref().unwrap();
            let anchor = wl.heading.as_ref().map(|h| format!("#{}", h)).unwrap_or_default();
            format!(
                r#"<a href="/doc/{}{}" class="wikilink">{}</a>"#,
                resolved.doc_id.unwrap(),
                anchor,
                display
            )
        } else {
            format!(r#"<span class="wikilink broken">{}</span>"#, display)
        };
        
        result.replace_range(wl.start..wl.end, &html);
    }
    
    result
}

/// Build a graph of document connections via wikilinks
pub async fn build_link_graph(pool: &PgPool, doc_ids: &[i32]) -> HashMap<i32, Vec<i32>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    
    for &doc_id in doc_ids {
        // Get document content
        let doc = sqlx::query_as::<_, (String,)>(
            "SELECT content FROM documents WHERE id = $1"
        )
        .bind(doc_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten();
        
        if let Some((content,)) = doc {
            let resolution = resolve_all_wikilinks(pool, &content).await;
            let linked_ids: Vec<i32> = resolution.links
                .into_iter()
                .filter_map(|r| r.doc_id)
                .collect();
            graph.insert(doc_id, linked_ids);
        }
    }
    
    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_wikilink() {
        let content = "Check out [[My Note]] for details.";
        let links = parse_wikilinks(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "My Note");
        assert_eq!(links[0].heading, None);
        assert_eq!(links[0].display, None);
    }

    #[test]
    fn test_parse_wikilink_with_heading() {
        let content = "See [[My Note#Section 2]] for more.";
        let links = parse_wikilinks(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "My Note");
        assert_eq!(links[0].heading, Some("Section 2".to_string()));
    }

    #[test]
    fn test_parse_wikilink_with_alias() {
        let content = "Read the [[Technical Doc|documentation]] here.";
        let links = parse_wikilinks(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "Technical Doc");
        assert_eq!(links[0].display, Some("documentation".to_string()));
    }

    #[test]
    fn test_parse_full_wikilink() {
        let content = "[[Project Plan#Timeline|see timeline]]";
        let links = parse_wikilinks(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "Project Plan");
        assert_eq!(links[0].heading, Some("Timeline".to_string()));
        assert_eq!(links[0].display, Some("see timeline".to_string()));
    }

    #[test]
    fn test_parse_multiple_wikilinks() {
        let content = "Connect [[Note A]] to [[Note B]] via [[Note C|link]].";
        let links = parse_wikilinks(content);
        assert_eq!(links.len(), 3);
        assert_eq!(links[0].target, "Note A");
        assert_eq!(links[1].target, "Note B");
        assert_eq!(links[2].target, "Note C");
    }

    #[test]
    fn test_extract_unique_targets() {
        let content = "[[A]] links to [[B]] and [[A]] again, plus [[C]].";
        let targets = extract_unique_targets(content);
        assert_eq!(targets, vec!["A", "B", "C"]);
    }
}
