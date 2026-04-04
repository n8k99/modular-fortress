//! Document deduplication module
//!
//! This module provides tools for identifying and consolidating duplicate documents
//! in the Master Chronicle database. Documents are considered duplicates when they
//! share the same title.
//!
//! # Workflow
//!
//! 1. **Analysis**: Find duplicate clusters using [`find_duplicates_by_title`]
//! 2. **Preview**: Review what would happen with [`dry_run_all`] or [`preview_selection`]
//! 3. **Canonicalization**: Determine which version to keep using [`select_canonical`]
//! 4. **Migration**: Move non-canonical versions to history with [`migrate_all`]
//!
//! # Priority Rules
//!
//! When selecting the canonical document, location takes precedence:
//! 1. `Areas/Eckenrode Muziekopname` — active project, highest priority
//! 2. `Areas/Master Chronicle` — knowledge base
//! 3. `Areas/*` — other active areas
//! 4. `Archive/*` — archived content, lowest priority
//!
//! Within the same priority, larger and more recent files are preferred.
//!
//! # Example
//!
//! ```ignore
//! use dpn_core::db::create_pool;
//! use dpn_core::dedup::{find_duplicates_by_title, select_canonical, dry_run_all};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let pool = create_pool("postgres://...").await?;
//!
//!     // Preview what would happen
//!     let summary = dry_run_all(&pool).await?;
//!     println!("Would migrate {} documents", summary.documents_migrated);
//!
//!     // Or analyze specific clusters
//!     let clusters = find_duplicates_by_title(&pool).await?;
//!     for cluster in &clusters[..5] {
//!         let canonical = select_canonical(cluster);
//!         println!("{}: keep doc #{}", cluster.title, canonical);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Safety
//!
//! Migration is destructive — documents are moved from `documents` to `document_versions`.
//! Always run [`dry_run_all`] first and review the output. The [`restore_from_version`]
//! function can recover individual documents if needed.

pub mod types;
pub mod analyzer;
pub mod canonicalizer;
pub mod migrator;

// Re-export main types
pub use types::{DocInfo, DuplicateCluster, MigrationResult, MigrationSummary};

// Re-export analyzer functions
pub use analyzer::{find_duplicates_by_title, find_duplicates_for_title, get_duplicate_stats, DuplicateStats};

// Re-export canonicalizer functions
pub use canonicalizer::{
    select_canonical, 
    select_canonical_detailed, 
    preview_selection,
    LocationPriority,
    CanonicalSelection,
};

// Re-export migrator functions
pub use migrator::{
    ensure_versions_table,
    migrate_cluster,
    migrate_all,
    migrate_by_titles,
    dry_run_all,
    restore_from_version,
};
