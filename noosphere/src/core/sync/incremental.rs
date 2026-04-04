//! Incremental sync using modified_at timestamps
//!
//! Tracks last sync time and pulls only changed records.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::db::DbPool;

/// Sync state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    /// Last successful sync timestamp per table
    pub last_sync: std::collections::HashMap<String, DateTime<Utc>>,
    /// Number of records synced in last operation
    pub last_sync_count: std::collections::HashMap<String, usize>,
    /// Sync errors if any
    pub errors: Vec<String>,
}

impl Default for SyncState {
    fn default() -> Self {
        Self {
            last_sync: std::collections::HashMap::new(),
            last_sync_count: std::collections::HashMap::new(),
            errors: Vec::new(),
        }
    }
}

impl SyncState {
    /// Load sync state from JSON file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let state: SyncState = serde_json::from_str(&content)?;
        Ok(state)
    }

    /// Save sync state to JSON file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get last sync time for a table
    pub fn get_last_sync(&self, table: &str) -> Option<DateTime<Utc>> {
        self.last_sync.get(table).copied()
    }

    /// Update last sync time for a table
    pub fn set_last_sync(&mut self, table: &str, time: DateTime<Utc>, count: usize) {
        self.last_sync.insert(table.to_string(), time);
        self.last_sync_count.insert(table.to_string(), count);
    }
}

/// Incremental sync manager
pub struct IncrementalSync {
    pub state: SyncState,
    state_path: Option<String>,
}

impl IncrementalSync {
    /// Create new sync manager
    pub fn new() -> Self {
        Self {
            state: SyncState::default(),
            state_path: None,
        }
    }

    /// Create sync manager with persistent state
    pub fn with_state_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let state = if path.as_ref().exists() {
            SyncState::load(&path)?
        } else {
            SyncState::default()
        };
        
        Ok(Self {
            state,
            state_path: Some(path.as_ref().to_string_lossy().to_string()),
        })
    }

    /// Save state to file if path is set
    pub fn save_state(&self) -> Result<()> {
        if let Some(path) = &self.state_path {
            self.state.save(path)?;
        }
        Ok(())
    }

    /// Sync memories table incrementally
    pub async fn sync_memories(&mut self, pool: &DbPool) -> Result<SyncResult> {
        let table = "memories";
        let since = self.state.get_last_sync(table);
        
        let sync_start = Utc::now();
        
        let records = match since {
            Some(since_time) => {
                let naive = since_time.naive_utc();
                crate::db::memories::list_modified_since(pool, naive).await?
            }
            None => {
                // First sync - get all records (metadata only for speed)
                let light = crate::db::memories::list_all_light(pool).await?;
                tracing::info!("Initial memories sync: {} records", light.len());
                return Ok(SyncResult {
                    table: table.to_string(),
                    records_synced: light.len(),
                    is_full_sync: true,
                    duration_ms: (Utc::now() - sync_start).num_milliseconds() as u64,
                });
            }
        };

        let count = records.len();
        self.state.set_last_sync(table, sync_start, count);
        self.save_state()?;

        Ok(SyncResult {
            table: table.to_string(),
            records_synced: count,
            is_full_sync: false,
            duration_ms: (Utc::now() - sync_start).num_milliseconds() as u64,
        })
    }

    /// Sync daily_logs table incrementally
    pub async fn sync_daily_logs(&mut self, pool: &DbPool) -> Result<SyncResult> {
        let table = "daily_logs";
        let since = self.state.get_last_sync(table);
        
        let sync_start = Utc::now();
        
        let count = match since {
            Some(since_time) => {
                let naive = since_time.naive_utc();
                let records = crate::memory::store::list_logs_since(pool, naive).await?;
                records.len()
            }
            None => {
                // First sync - count all
                let count = crate::memory::store::get_log_count(pool).await? as usize;
                tracing::info!("Initial daily_logs sync: {} records", count);
                count
            }
        };

        self.state.set_last_sync(table, sync_start, count);
        self.save_state()?;

        Ok(SyncResult {
            table: table.to_string(),
            records_synced: count,
            is_full_sync: since.is_none(),
            duration_ms: (Utc::now() - sync_start).num_milliseconds() as u64,
        })
    }

    /// Sync memory_entries table incrementally
    pub async fn sync_memory_entries(&mut self, pool: &DbPool) -> Result<SyncResult> {
        let table = "memory_entries";
        let since = self.state.get_last_sync(table);
        
        let sync_start = Utc::now();
        
        let count = match since {
            Some(since_time) => {
                let naive = since_time.naive_utc();
                let records = crate::memory::store::list_memories_since(pool, naive).await?;
                records.len()
            }
            None => {
                let count = crate::memory::store::get_memory_count(pool).await? as usize;
                tracing::info!("Initial memory_entries sync: {} records", count);
                count
            }
        };

        self.state.set_last_sync(table, sync_start, count);
        self.save_state()?;

        Ok(SyncResult {
            table: table.to_string(),
            records_synced: count,
            is_full_sync: since.is_none(),
            duration_ms: (Utc::now() - sync_start).num_milliseconds() as u64,
        })
    }

    /// Sync all tables
    pub async fn sync_all(&mut self, pool: &DbPool) -> Result<Vec<SyncResult>> {
        let mut results = Vec::new();
        
        results.push(self.sync_memories(pool).await?);
        results.push(self.sync_daily_logs(pool).await?);
        results.push(self.sync_memory_entries(pool).await?);
        
        Ok(results)
    }
}

/// Result of a sync operation
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub table: String,
    pub records_synced: usize,
    pub is_full_sync: bool,
    pub duration_ms: u64,
}

impl std::fmt::Display for SyncResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sync_type = if self.is_full_sync { "full" } else { "incremental" };
        write!(
            f,
            "{}: {} records ({} sync, {}ms)",
            self.table, self.records_synced, sync_type, self.duration_ms
        )
    }
}
