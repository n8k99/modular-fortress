//! Canonical document selection
//!
//! Determines which document in a duplicate cluster should be considered
//! the "canonical" (authoritative) version based on location priority.
//!
//! Priority order (highest to lowest):
//! 1. Areas/Eckenrode Muziekopname — active project, highest priority
//! 2. Areas/Master Chronicle — knowledge base, high priority
//! 3. Areas/* — other active areas
//! 4. Archive/* — archived content, lowest priority
//!
//! Within the same priority tier, we prefer:
//! - Larger file size (more content)
//! - More recent modification date

use tracing::debug;

use super::types::{DocInfo, DuplicateCluster};

/// Priority tiers for document locations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocationPriority {
    /// Archive — lowest priority (0)
    Archive = 0,
    /// Generic Areas folder (1)
    AreasOther = 1,
    /// Areas/Master Chronicle (2)
    AreasMasterChronicle = 2,
    /// Areas/Eckenrode Muziekopname — highest priority (3)
    AreasEckenrodeMuziekopname = 3,
}

impl LocationPriority {
    /// Determine priority from a document path
    pub fn from_path(path: &str) -> Self {
        if path.starts_with("Areas/Eckenrode Muziekopname") {
            LocationPriority::AreasEckenrodeMuziekopname
        } else if path.starts_with("Areas/Master Chronicle") {
            LocationPriority::AreasMasterChronicle
        } else if path.starts_with("Areas/") {
            LocationPriority::AreasOther
        } else {
            // Archive, Resources, or anything else
            LocationPriority::Archive
        }
    }

    /// Human-readable name for this priority
    pub fn name(&self) -> &'static str {
        match self {
            LocationPriority::AreasEckenrodeMuziekopname => "Eckenrode Muziekopname",
            LocationPriority::AreasMasterChronicle => "Master Chronicle",
            LocationPriority::AreasOther => "Other Areas",
            LocationPriority::Archive => "Archive/Other",
        }
    }
}

/// Score a document for canonical selection
///
/// Higher score = more likely to be canonical.
/// Score components:
/// - Priority tier (0-3) × 1000
/// - Size bonus (0-500) — larger files preferred
/// - Recency bonus (0-100) — more recent preferred
fn score_document(doc: &DocInfo) -> i64 {
    let priority = LocationPriority::from_path(&doc.path);
    let priority_score = (priority as i64) * 1000;

    // Size bonus: up to 500 points for larger files
    // Cap at 500KB to avoid outlier influence
    let size_score = doc.size_bytes
        .map(|s| std::cmp::min(s as i64, 500_000) / 1000)
        .unwrap_or(0);

    // Recency bonus: up to 100 points for recent files
    // We don't have a good baseline, so just use timestamp value mod 100
    let recency_score = doc.modified_at
        .map(|dt| (dt.and_utc().timestamp() % 100).abs())
        .unwrap_or(0);

    priority_score + size_score + recency_score
}

/// Select the canonical document from a cluster
///
/// Returns the ID of the document that should be considered the canonical version.
/// Selection is based on location priority, file size, and modification date.
///
/// # Arguments
/// * `cluster` - A cluster of duplicate documents
///
/// # Returns
/// The ID of the canonical document
///
/// # Panics
/// Panics if the cluster is empty (should never happen for valid clusters)
///
/// # Example
/// ```ignore
/// let canonical_id = select_canonical(&cluster);
/// println!("Canonical version: document #{}", canonical_id);
/// ```
pub fn select_canonical(cluster: &DuplicateCluster) -> i32 {
    assert!(!cluster.documents.is_empty(), "Cannot select canonical from empty cluster");

    let mut best_doc: Option<&DocInfo> = None;
    let mut best_score: i64 = i64::MIN;

    for doc in &cluster.documents {
        let score = score_document(doc);
        debug!(
            "Document {} '{}': score {} (priority: {:?})",
            doc.id,
            doc.path,
            score,
            LocationPriority::from_path(&doc.path)
        );

        if score > best_score {
            best_score = score;
            best_doc = Some(doc);
        }
    }

    let canonical = best_doc.expect("Cluster was non-empty but no best doc found");
    debug!(
        "Selected canonical: {} '{}' with score {}",
        canonical.id, canonical.path, best_score
    );

    canonical.id
}

/// Detailed selection result with reasoning
#[derive(Debug, Clone)]
pub struct CanonicalSelection {
    /// ID of the selected canonical document
    pub canonical_id: i32,
    /// Path of the canonical document
    pub canonical_path: String,
    /// Priority tier of the canonical document
    pub priority: LocationPriority,
    /// Score of the canonical document
    pub score: i64,
    /// Documents that will be migrated (non-canonical)
    pub to_migrate: Vec<DocInfo>,
}

/// Select canonical with full details
pub fn select_canonical_detailed(cluster: &DuplicateCluster) -> CanonicalSelection {
    let canonical_id = select_canonical(cluster);
    
    let canonical = cluster.documents
        .iter()
        .find(|d| d.id == canonical_id)
        .expect("Canonical ID not found in cluster");

    let to_migrate: Vec<DocInfo> = cluster.documents
        .iter()
        .filter(|d| d.id != canonical_id)
        .cloned()
        .collect();

    CanonicalSelection {
        canonical_id,
        canonical_path: canonical.path.clone(),
        priority: LocationPriority::from_path(&canonical.path),
        score: score_document(canonical),
        to_migrate,
    }
}

/// Preview canonical selection for a cluster without making changes
pub fn preview_selection(cluster: &DuplicateCluster) -> String {
    let selection = select_canonical_detailed(cluster);
    
    let mut output = format!(
        "Cluster: '{}' ({} documents)\n",
        cluster.title,
        cluster.count()
    );
    output.push_str(&format!(
        "  ✓ CANONICAL: {} ({})\n    Path: {}\n",
        selection.canonical_id,
        selection.priority.name(),
        selection.canonical_path
    ));
    
    for doc in &selection.to_migrate {
        let priority = LocationPriority::from_path(&doc.path);
        output.push_str(&format!(
            "  → MIGRATE: {} ({})\n    Path: {}\n",
            doc.id,
            priority.name(),
            doc.path
        ));
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dedup::types::DocInfo;

    fn make_doc(id: i32, path: &str) -> DocInfo {
        DocInfo {
            id,
            path: path.to_string(),
            title: Some("Test".to_string()),
            size_bytes: Some(100),
            modified_at: None,
        }
    }

    #[test]
    fn test_location_priority() {
        assert_eq!(
            LocationPriority::from_path("Areas/Eckenrode Muziekopname/Notes/test.md"),
            LocationPriority::AreasEckenrodeMuziekopname
        );
        assert_eq!(
            LocationPriority::from_path("Areas/Master Chronicle/test.md"),
            LocationPriority::AreasMasterChronicle
        );
        assert_eq!(
            LocationPriority::from_path("Areas/Other Project/test.md"),
            LocationPriority::AreasOther
        );
        assert_eq!(
            LocationPriority::from_path("Archive/2020/test.md"),
            LocationPriority::Archive
        );
    }

    #[test]
    fn test_priority_ordering() {
        assert!(LocationPriority::AreasEckenrodeMuziekopname > LocationPriority::AreasMasterChronicle);
        assert!(LocationPriority::AreasMasterChronicle > LocationPriority::AreasOther);
        assert!(LocationPriority::AreasOther > LocationPriority::Archive);
    }

    #[test]
    fn test_select_canonical_prefers_em() {
        let cluster = DuplicateCluster::new(
            "Test".to_string(),
            vec![
                make_doc(1, "Archive/test.md"),
                make_doc(2, "Areas/Eckenrode Muziekopname/test.md"),
                make_doc(3, "Areas/Master Chronicle/test.md"),
            ],
        );

        assert_eq!(select_canonical(&cluster), 2);
    }

    #[test]
    fn test_select_canonical_prefers_mc_over_archive() {
        let cluster = DuplicateCluster::new(
            "Test".to_string(),
            vec![
                make_doc(1, "Archive/test.md"),
                make_doc(2, "Areas/Master Chronicle/test.md"),
            ],
        );

        assert_eq!(select_canonical(&cluster), 2);
    }
}
