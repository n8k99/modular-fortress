//! Shared types for document deduplication
//!
//! These types represent duplicate document clusters and their metadata,
//! used throughout the dedup module for analysis and migration.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Information about a single document in a duplicate cluster
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DocInfo {
    /// Document ID in the database
    pub id: i32,
    /// Full path to the document (e.g., "Areas/Eckenrode Muziekopname/...")
    pub path: String,
    /// Document title (may be None for untitled docs)
    pub title: Option<String>,
    /// Size in bytes
    pub size_bytes: Option<i32>,
    /// Last modification timestamp
    pub modified_at: Option<NaiveDateTime>,
}

impl DocInfo {
    /// Get display title, falling back to path if title is None
    pub fn display_title(&self) -> String {
        self.title.clone().unwrap_or_else(|| self.path.clone())
    }

    /// Extract the area/folder prefix from the path (e.g., "Areas/Eckenrode Muziekopname")
    pub fn area(&self) -> Option<&str> {
        // Path format: "Areas/SomeName/..." or "Archive/..."
        let parts: Vec<&str> = self.path.split('/').collect();
        if parts.len() >= 2 {
            // For "Areas/X/...", return "Areas/X"
            // For "Archive/...", return "Archive"
            if parts[0] == "Areas" && parts.len() >= 2 {
                // Find the position after "Areas/X"
                let prefix_len = parts[0].len() + 1 + parts[1].len();
                if prefix_len <= self.path.len() {
                    return Some(&self.path[..prefix_len]);
                }
            } else {
                return Some(parts[0]);
            }
        }
        None
    }
}

/// A cluster of documents that share the same title (duplicates)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateCluster {
    /// The shared title of all documents in this cluster
    pub title: String,
    /// All documents with this title
    pub documents: Vec<DocInfo>,
}

impl DuplicateCluster {
    /// Create a new cluster with a title and initial documents
    pub fn new(title: String, documents: Vec<DocInfo>) -> Self {
        Self { title, documents }
    }

    /// Number of documents in this cluster
    pub fn count(&self) -> usize {
        self.documents.len()
    }

    /// Check if this is actually a duplicate (more than one document)
    pub fn is_duplicate(&self) -> bool {
        self.documents.len() > 1
    }

    /// Get all document IDs in this cluster
    pub fn ids(&self) -> Vec<i32> {
        self.documents.iter().map(|d| d.id).collect()
    }

    /// Get non-canonical documents (all except the one with the given ID)
    pub fn non_canonical(&self, canonical_id: i32) -> Vec<&DocInfo> {
        self.documents.iter().filter(|d| d.id != canonical_id).collect()
    }
}

/// Result of a migration operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationResult {
    /// The cluster that was migrated
    pub cluster_title: String,
    /// ID of the canonical document
    pub canonical_id: i32,
    /// IDs of documents moved to versions table
    pub migrated_ids: Vec<i32>,
    /// Whether the migration succeeded
    pub success: bool,
    /// Error message if migration failed
    pub error: Option<String>,
}

impl MigrationResult {
    pub fn success(cluster_title: String, canonical_id: i32, migrated_ids: Vec<i32>) -> Self {
        Self {
            cluster_title,
            canonical_id,
            migrated_ids,
            success: true,
            error: None,
        }
    }

    pub fn failure(cluster_title: String, canonical_id: i32, error: String) -> Self {
        Self {
            cluster_title,
            canonical_id,
            migrated_ids: vec![],
            success: false,
            error: Some(error),
        }
    }
}

/// Summary of all migration operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MigrationSummary {
    /// Total clusters processed
    pub clusters_processed: usize,
    /// Clusters successfully migrated
    pub clusters_succeeded: usize,
    /// Clusters that failed migration
    pub clusters_failed: usize,
    /// Total documents moved to versions
    pub documents_migrated: usize,
    /// Detailed results per cluster
    pub results: Vec<MigrationResult>,
}

impl MigrationSummary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_result(&mut self, result: MigrationResult) {
        self.clusters_processed += 1;
        if result.success {
            self.clusters_succeeded += 1;
            self.documents_migrated += result.migrated_ids.len();
        } else {
            self.clusters_failed += 1;
        }
        self.results.push(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_info_area_extraction() {
        let doc = DocInfo {
            id: 1,
            path: "Areas/Eckenrode Muziekopname/Notes/test.md".to_string(),
            title: Some("Test".to_string()),
            size_bytes: Some(100),
            modified_at: None,
        };
        assert_eq!(doc.area(), Some("Areas/Eckenrode Muziekopname"));

        let archive_doc = DocInfo {
            id: 2,
            path: "Archive/old/test.md".to_string(),
            title: None,
            size_bytes: None,
            modified_at: None,
        };
        assert_eq!(archive_doc.area(), Some("Archive"));
    }

    #[test]
    fn test_cluster_operations() {
        let docs = vec![
            DocInfo {
                id: 1,
                path: "Areas/A/test.md".to_string(),
                title: Some("Test".to_string()),
                size_bytes: Some(100),
                modified_at: None,
            },
            DocInfo {
                id: 2,
                path: "Archive/test.md".to_string(),
                title: Some("Test".to_string()),
                size_bytes: Some(50),
                modified_at: None,
            },
        ];

        let cluster = DuplicateCluster::new("Test".to_string(), docs);
        assert!(cluster.is_duplicate());
        assert_eq!(cluster.count(), 2);
        assert_eq!(cluster.ids(), vec![1, 2]);
        assert_eq!(cluster.non_canonical(1).len(), 1);
        assert_eq!(cluster.non_canonical(1)[0].id, 2);
    }
}
