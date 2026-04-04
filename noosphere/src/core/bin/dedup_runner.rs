//! CLI tool to run deduplication

use anyhow::Result;
use dpn_core::db::create_pool;
use dpn_core::dedup::{find_duplicates_by_title, migrate_all, select_canonical, dry_run_all};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_env_filter("dpn_core=info")
        .init();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://chronicle:chronicle2026@127.0.0.1:5433/master_chronicle".to_string());
    
    println!("Connecting to database...");
    let pool = create_pool(&database_url).await?;
    
    let args: Vec<String> = env::args().collect();
    let dry_run = args.get(1).map(|s| s == "--dry-run").unwrap_or(false);
    
    println!("Finding duplicates...");
    let clusters = find_duplicates_by_title(&pool).await?;
    
    println!("\n=== Duplicate Analysis ===");
    println!("Found {} duplicate clusters", clusters.len());
    
    let total_dupes: usize = clusters.iter().map(|c| c.documents.len() - 1).sum();
    println!("Total documents to migrate: {}", total_dupes);
    
    if dry_run {
        println!("\n[DRY RUN MODE - No changes will be made]");
        println!("\nTop 10 duplicate clusters:");
        for cluster in clusters.iter().take(10) {
            let canonical_id = select_canonical(&cluster);
            println!("\n  '{}' ({} copies)", cluster.title, cluster.documents.len());
            for doc in &cluster.documents {
                let marker = if doc.id == canonical_id { " [CANONICAL]" } else { "" };
                println!("    - {}{} (id={})", doc.path, marker, doc.id);
            }
        }
        
        // Run dry run
        println!("\n\nRunning full dry run analysis...");
        let summary = dry_run_all(&pool).await?;
        println!("\nDry run summary:");
        println!("  Would process {} clusters", summary.clusters_processed);
        println!("  Would migrate {} documents to versions", summary.documents_migrated);
    } else {
        println!("\n=== Running Migration ===");
        println!("This will modify the database. Proceeding...\n");
        
        let summary = migrate_all(&pool).await?;
        
        println!("\n=== Migration Complete ===");
        println!("Clusters processed: {}", summary.clusters_processed);
        println!("Clusters succeeded: {}", summary.clusters_succeeded);
        println!("Clusters failed: {}", summary.clusters_failed);
        println!("Documents migrated to versions: {}", summary.documents_migrated);
        
        // Show failed clusters
        let failed: Vec<_> = summary.results.iter().filter(|r| !r.success).collect();
        if !failed.is_empty() {
            println!("\nFailed clusters:");
            for result in failed.iter().take(10) {
                println!("  - {} (canonical_id={}): {:?}", 
                    result.cluster_title, result.canonical_id, result.error);
            }
        }
    }
    
    Ok(())
}
