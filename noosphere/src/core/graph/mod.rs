//! Graph visualization data generation
//!
//! Generates node/edge data structures for document relationship visualization.
//! Used by dpn-kb and other frontends to render interactive graph views.

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};

use crate::wikilinks::{parse_wikilinks, resolve_wikilink};

/// A node in the document graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    /// Document ID
    pub id: i32,
    /// Document title
    pub title: String,
    /// Document path (for grouping/categorization)
    pub path: String,
    /// Node category (derived from path or frontmatter)
    pub category: Option<String>,
    /// Number of incoming links (backlinks)
    pub in_degree: usize,
    /// Number of outgoing links
    pub out_degree: usize,
    /// Combined link count for sizing
    pub weight: usize,
    /// Optional: tags from frontmatter
    pub tags: Vec<String>,
}

/// An edge in the document graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    /// Source document ID
    pub source: i32,
    /// Target document ID
    pub target: i32,
    /// Edge label (the wikilink text used)
    pub label: Option<String>,
    /// Edge weight (for multiple links between same docs)
    pub weight: usize,
}

/// Complete graph data for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    /// All nodes in the graph
    pub nodes: Vec<GraphNode>,
    /// All edges in the graph
    pub edges: Vec<GraphEdge>,
    /// Graph statistics
    pub stats: GraphStats,
}

/// Statistics about the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStats {
    /// Total number of nodes
    pub node_count: usize,
    /// Total number of edges
    pub edge_count: usize,
    /// Number of connected components
    pub components: usize,
    /// Average degree (connections per node)
    pub avg_degree: f64,
    /// Max in-degree (most backlinked document)
    pub max_in_degree: usize,
    /// Max out-degree (document with most outlinks)
    pub max_out_degree: usize,
}

/// Options for graph generation
#[derive(Debug, Clone, Default)]
pub struct GraphOptions {
    /// Only include documents matching this path prefix
    pub path_prefix: Option<String>,
    /// Maximum number of nodes to include
    pub max_nodes: Option<usize>,
    /// Minimum weight (degree) to include a node
    pub min_weight: Option<usize>,
    /// Include orphan nodes (no connections)
    pub include_orphans: bool,
    /// Categories to include (empty = all)
    pub categories: Vec<String>,
    /// Tags to filter by (empty = all)
    pub tags: Vec<String>,
}

/// Lightweight document info for graph building
struct DocInfo {
    id: i32,
    title: String,
    path: String,
    content: String,
    frontmatter: Option<String>,
}

/// Extract category from document path
fn extract_category(path: &str) -> Option<String> {
    // Extract first meaningful path segment as category
    // e.g., "Areas/Projects/Project X.md" -> "Areas"
    // e.g., "Resources/Books/Book Y.md" -> "Resources"
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() >= 2 {
        Some(parts[0].to_string())
    } else {
        None
    }
}

/// Extract tags from frontmatter YAML
fn extract_tags(frontmatter: Option<&str>) -> Vec<String> {
    let Some(fm) = frontmatter else {
        return vec![];
    };
    
    // Simple tag extraction from YAML frontmatter
    // Look for: tags: [tag1, tag2] or tags:\n  - tag1\n  - tag2
    for line in fm.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("tags:") {
            let value = trimmed.strip_prefix("tags:").unwrap_or("").trim();
            if value.starts_with('[') && value.ends_with(']') {
                // Array format: [tag1, tag2]
                let inner = &value[1..value.len()-1];
                return inner
                    .split(',')
                    .map(|t| t.trim().trim_matches('"').trim_matches('\'').to_string())
                    .filter(|t| !t.is_empty())
                    .collect();
            }
        } else if trimmed.starts_with("- ") && !trimmed.contains(':') {
            // Could be a tag in list format, but need context
            // This is a simplified approach
        }
    }
    
    vec![]
}

/// Build a complete graph from the database
pub async fn build_graph(pool: &PgPool, options: GraphOptions) -> Result<GraphData, sqlx::Error> {
    // Fetch documents
    let query = if let Some(prefix) = &options.path_prefix {
        let pattern = format!("{}%", prefix);
        sqlx::query_as!(
            DocInfo,
            r#"
            SELECT id, title as "title!", path as "path!", 
                   COALESCE(content, '') as "content!", 
                   frontmatter
            FROM documents 
            WHERE is_canonical = true AND path LIKE $1
            ORDER BY id
            "#,
            pattern
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as!(
            DocInfo,
            r#"
            SELECT id, title as "title!", path as "path!", 
                   COALESCE(content, '') as "content!", 
                   frontmatter
            FROM documents 
            WHERE is_canonical = true
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?
    };
    
    // Build title -> id lookup for link resolution
    let title_to_id: HashMap<String, i32> = query
        .iter()
        .map(|d| (d.title.to_lowercase(), d.id))
        .collect();
    
    // Track in/out degrees
    let mut in_degrees: HashMap<i32, usize> = HashMap::new();
    let mut out_degrees: HashMap<i32, usize> = HashMap::new();
    
    // Build edges from wikilinks
    let mut edge_map: HashMap<(i32, i32), GraphEdge> = HashMap::new();
    
    for doc in &query {
        let wikilinks = parse_wikilinks(&doc.content);
        let mut out_count = 0;
        
        for wl in wikilinks {
            // Try to resolve locally first (fast path)
            let target_id = title_to_id
                .get(&wl.target.to_lowercase())
                .copied()
                .or_else(|| {
                    // Fall back to partial match
                    title_to_id
                        .iter()
                        .find(|(k, _)| k.contains(&wl.target.to_lowercase()))
                        .map(|(_, v)| *v)
                });
            
            if let Some(target) = target_id {
                if target != doc.id {  // No self-links
                    out_count += 1;
                    *in_degrees.entry(target).or_insert(0) += 1;
                    
                    let key = (doc.id, target);
                    edge_map
                        .entry(key)
                        .and_modify(|e| e.weight += 1)
                        .or_insert(GraphEdge {
                            source: doc.id,
                            target,
                            label: Some(wl.target.clone()),
                            weight: 1,
                        });
                }
            }
        }
        
        out_degrees.insert(doc.id, out_count);
    }
    
    // Initialize in_degrees for nodes without any incoming links
    for doc in &query {
        in_degrees.entry(doc.id).or_insert(0);
    }
    
    // Build nodes
    let mut nodes: Vec<GraphNode> = Vec::new();
    let doc_ids: HashSet<i32> = query.iter().map(|d| d.id).collect();
    
    for doc in &query {
        let in_deg = in_degrees.get(&doc.id).copied().unwrap_or(0);
        let out_deg = out_degrees.get(&doc.id).copied().unwrap_or(0);
        let weight = in_deg + out_deg;
        
        // Apply filters
        if let Some(min_w) = options.min_weight {
            if weight < min_w {
                continue;
            }
        }
        
        if !options.include_orphans && weight == 0 {
            continue;
        }
        
        let category = extract_category(&doc.path);
        let tags = extract_tags(doc.frontmatter.as_deref());
        
        // Category filter
        if !options.categories.is_empty() {
            if let Some(ref cat) = category {
                if !options.categories.contains(cat) {
                    continue;
                }
            } else {
                continue;  // No category but filter is active
            }
        }
        
        // Tag filter
        if !options.tags.is_empty() {
            if !options.tags.iter().any(|t| tags.contains(t)) {
                continue;
            }
        }
        
        nodes.push(GraphNode {
            id: doc.id,
            title: doc.title.clone(),
            path: doc.path.clone(),
            category,
            in_degree: in_deg,
            out_degree: out_deg,
            weight,
            tags,
        });
    }
    
    // Apply max_nodes limit (keep highest weight)
    if let Some(max) = options.max_nodes {
        if nodes.len() > max {
            nodes.sort_by(|a, b| b.weight.cmp(&a.weight));
            nodes.truncate(max);
        }
    }
    
    // Filter edges to only include nodes that are in our filtered set
    let included_ids: HashSet<i32> = nodes.iter().map(|n| n.id).collect();
    let edges: Vec<GraphEdge> = edge_map
        .into_values()
        .filter(|e| included_ids.contains(&e.source) && included_ids.contains(&e.target))
        .collect();
    
    // Calculate stats
    let max_in_degree = nodes.iter().map(|n| n.in_degree).max().unwrap_or(0);
    let max_out_degree = nodes.iter().map(|n| n.out_degree).max().unwrap_or(0);
    let total_degree: usize = nodes.iter().map(|n| n.weight).sum();
    let avg_degree = if !nodes.is_empty() {
        total_degree as f64 / nodes.len() as f64
    } else {
        0.0
    };
    
    // Simple connected components calculation
    let components = count_components(&nodes, &edges);
    
    let stats = GraphStats {
        node_count: nodes.len(),
        edge_count: edges.len(),
        components,
        avg_degree,
        max_in_degree,
        max_out_degree,
    };
    
    Ok(GraphData { nodes, edges, stats })
}

/// Count connected components using union-find
fn count_components(nodes: &[GraphNode], edges: &[GraphEdge]) -> usize {
    if nodes.is_empty() {
        return 0;
    }
    
    let mut parent: HashMap<i32, i32> = nodes.iter().map(|n| (n.id, n.id)).collect();
    
    fn find(parent: &mut HashMap<i32, i32>, x: i32) -> i32 {
        let p = *parent.get(&x).unwrap_or(&x);
        if p != x {
            let root = find(parent, p);
            parent.insert(x, root);
            root
        } else {
            x
        }
    }
    
    fn union(parent: &mut HashMap<i32, i32>, x: i32, y: i32) {
        let px = find(parent, x);
        let py = find(parent, y);
        if px != py {
            parent.insert(px, py);
        }
    }
    
    for edge in edges {
        union(&mut parent, edge.source, edge.target);
    }
    
    let roots: HashSet<i32> = nodes.iter().map(|n| find(&mut parent, n.id)).collect();
    roots.len()
}

/// Build a local neighborhood graph around a specific document
pub async fn build_neighborhood_graph(
    pool: &PgPool, 
    center_id: i32, 
    depth: usize
) -> Result<GraphData, sqlx::Error> {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut frontier: Vec<i32> = vec![center_id];
    let mut all_ids: HashSet<i32> = HashSet::new();
    
    // BFS to find all nodes within depth
    for _ in 0..=depth {
        let mut next_frontier: Vec<i32> = Vec::new();
        
        for id in frontier {
            if visited.contains(&id) {
                continue;
            }
            visited.insert(id);
            all_ids.insert(id);
            
            // Get outlinks from this document
            let doc = sqlx::query_scalar::<_, Option<String>>(
                r#"SELECT content FROM documents WHERE id = $1"#
            )
            .bind(id)
            .fetch_optional(pool)
            .await?;
            
            if let Some(Some(content)) = doc {
                if !content.is_empty() {
                    let content = &content;
                    let wikilinks = parse_wikilinks(content);
                    for wl in wikilinks {
                        if let Some((target_id, _)) = resolve_wikilink(pool, &wl.target).await {
                            next_frontier.push(target_id);
                        }
                    }
                }
            }
            
            // Get backlinks to this document
            let backlinks = sqlx::query!(
                r#"
                SELECT id FROM documents 
                WHERE is_canonical = true 
                AND content LIKE $1
                "#,
                format!("%[[%{}%]]%", id)  // This is a rough approximation
            )
            .fetch_all(pool)
            .await?;
            
            for bl in backlinks {
                next_frontier.push(bl.id);
            }
        }
        
        frontier = next_frontier;
    }
    
    // Build graph with only these IDs
    let options = GraphOptions {
        include_orphans: true,
        ..Default::default()
    };
    
    let full_graph = build_graph(pool, options).await?;
    
    // Filter to only include discovered nodes
    let nodes: Vec<GraphNode> = full_graph
        .nodes
        .into_iter()
        .filter(|n| all_ids.contains(&n.id))
        .collect();
    
    let node_ids: HashSet<i32> = nodes.iter().map(|n| n.id).collect();
    let edges: Vec<GraphEdge> = full_graph
        .edges
        .into_iter()
        .filter(|e| node_ids.contains(&e.source) && node_ids.contains(&e.target))
        .collect();
    
    let stats = GraphStats {
        node_count: nodes.len(),
        edge_count: edges.len(),
        components: count_components(&nodes, &edges),
        avg_degree: if !nodes.is_empty() {
            nodes.iter().map(|n| n.weight).sum::<usize>() as f64 / nodes.len() as f64
        } else {
            0.0
        },
        max_in_degree: nodes.iter().map(|n| n.in_degree).max().unwrap_or(0),
        max_out_degree: nodes.iter().map(|n| n.out_degree).max().unwrap_or(0),
    };
    
    Ok(GraphData { nodes, edges, stats })
}

/// Get the most connected documents (hubs)
pub async fn get_hub_documents(pool: &PgPool, limit: usize) -> Result<Vec<GraphNode>, sqlx::Error> {
    let options = GraphOptions {
        include_orphans: false,
        ..Default::default()
    };
    
    let graph = build_graph(pool, options).await?;
    
    let mut nodes = graph.nodes;
    nodes.sort_by(|a, b| b.weight.cmp(&a.weight));
    nodes.truncate(limit);
    
    Ok(nodes)
}

/// Get orphan documents (no incoming or outgoing links)
pub async fn get_orphan_documents(pool: &PgPool) -> Result<Vec<GraphNode>, sqlx::Error> {
    let options = GraphOptions {
        include_orphans: true,
        ..Default::default()
    };
    
    let graph = build_graph(pool, options).await?;
    
    let orphans: Vec<GraphNode> = graph
        .nodes
        .into_iter()
        .filter(|n| n.weight == 0)
        .collect();
    
    Ok(orphans)
}
