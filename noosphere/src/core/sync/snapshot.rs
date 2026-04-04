//! Full table snapshot sync
//!
//! For when incremental sync isn't enough or for initial setup.

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;

/// Snapshot sync manager
pub struct SnapshotSync;

impl SnapshotSync {
    /// Create a full snapshot of memories (metadata only for speed)
    pub async fn snapshot_memories(pool: &DbPool) -> Result<TableSnapshot> {
        let start = Utc::now();
        let notes = crate::db::memories::list_all_light(pool).await?;
        let duration = (Utc::now() - start).num_milliseconds() as u64;

        Ok(TableSnapshot {
            table: "memories".to_string(),
            record_count: notes.len(),
            snapshot_time: start,
            duration_ms: duration,
        })
    }

    /// Create a full snapshot of daily_logs
    pub async fn snapshot_daily_logs(pool: &DbPool) -> Result<TableSnapshot> {
        let start = Utc::now();
        let count = crate::memory::store::get_log_count(pool).await? as usize;
        let duration = (Utc::now() - start).num_milliseconds() as u64;

        Ok(TableSnapshot {
            table: "daily_logs".to_string(),
            record_count: count,
            snapshot_time: start,
            duration_ms: duration,
        })
    }

    /// Create a full snapshot of memory_entries
    pub async fn snapshot_memory_entries(pool: &DbPool) -> Result<TableSnapshot> {
        let start = Utc::now();
        let count = crate::memory::store::get_memory_count(pool).await? as usize;
        let duration = (Utc::now() - start).num_milliseconds() as u64;

        Ok(TableSnapshot {
            table: "memory_entries".to_string(),
            record_count: count,
            snapshot_time: start,
            duration_ms: duration,
        })
    }

    /// Snapshot all tables and return stats
    pub async fn snapshot_all(pool: &DbPool) -> Result<Vec<TableSnapshot>> {
        let mut snapshots = Vec::new();
        
        snapshots.push(Self::snapshot_memories(pool).await?);
        snapshots.push(Self::snapshot_daily_logs(pool).await?);
        snapshots.push(Self::snapshot_memory_entries(pool).await?);
        
        Ok(snapshots)
    }
}

/// Snapshot metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSnapshot {
    pub table: String,
    pub record_count: usize,
    pub snapshot_time: chrono::DateTime<Utc>,
    pub duration_ms: u64,
}

impl std::fmt::Display for TableSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} records ({}ms)",
            self.table, self.record_count, self.duration_ms
        )
    }
}
