//! SQLite local cache
//!
//! Creates and manages local SQLite database at ~/.dpn/cache.db
//! Mirrors schema from PostgreSQL for offline access.

use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// SQLite connection pool (thread-safe wrapper)
pub type CachePool = Arc<Mutex<rusqlite::Connection>>;

/// Get the cache database path (~/.dpn/cache.db)
pub fn get_cache_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    let dpn_dir = home.join(".dpn");
    std::fs::create_dir_all(&dpn_dir)?;
    Ok(dpn_dir.join("cache.db"))
}

/// Get the sync state file path (~/.dpn/sync_state.json)
pub fn get_sync_state_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    let dpn_dir = home.join(".dpn");
    std::fs::create_dir_all(&dpn_dir)?;
    Ok(dpn_dir.join("sync_state.json"))
}

/// Initialize the local cache database
///
/// Creates tables if they don't exist, mirroring the PostgreSQL schema.
pub fn init_cache() -> Result<CachePool> {
    let db_path = get_cache_path()?;
    tracing::info!("Initializing local cache at {:?}", db_path);
    
    let conn = rusqlite::Connection::open(&db_path)?;
    
    // Enable WAL mode for better concurrent access
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
    
    // Create tables mirroring PostgreSQL schema
    conn.execute_batch(SCHEMA_SQL)?;
    
    tracing::info!("Local cache initialized");
    Ok(Arc::new(Mutex::new(conn)))
}

/// Open existing cache (does not create tables)
pub fn open_cache() -> Result<CachePool> {
    let db_path = get_cache_path()?;
    let conn = rusqlite::Connection::open(&db_path)?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
    Ok(Arc::new(Mutex::new(conn)))
}

/// Check if the cache database exists
pub fn cache_exists() -> bool {
    get_cache_path().map(|p| p.exists()).unwrap_or(false)
}

/// SQLite schema mirroring PostgreSQL tables
const SCHEMA_SQL: &str = r#"
-- Memories (mirrors memories table, renamed from vault_notes)
CREATE TABLE IF NOT EXISTS memories (
    id INTEGER PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    title TEXT,
    content TEXT,
    frontmatter TEXT,
    size_bytes INTEGER,
    note_type TEXT,
    note_date TEXT,  -- ISO date string
    modified_at TEXT,  -- ISO datetime string
    created_at TEXT,
    -- Local sync metadata
    local_modified_at TEXT,
    sync_status TEXT DEFAULT 'synced'  -- 'synced', 'pending', 'conflict'
);

CREATE INDEX IF NOT EXISTS idx_memories_path ON memories(path);
CREATE INDEX IF NOT EXISTS idx_memories_type ON memories(note_type);
CREATE INDEX IF NOT EXISTS idx_memories_date ON memories(note_date);
CREATE INDEX IF NOT EXISTS idx_memories_sync ON memories(sync_status);

-- Stagehand notes (mirrors stagehand_notes table)
CREATE TABLE IF NOT EXISTS stagehand_notes (
    id INTEGER PRIMARY KEY,
    show_name TEXT NOT NULL,
    venue TEXT,
    event_date TEXT NOT NULL,  -- ISO date string
    call_time TEXT,  -- ISO time string
    notes TEXT,
    tags TEXT,  -- JSON array as text
    created_at TEXT,
    modified_at TEXT,
    -- Local sync metadata
    local_modified_at TEXT,
    sync_status TEXT DEFAULT 'synced'
);

CREATE INDEX IF NOT EXISTS idx_stagehand_date ON stagehand_notes(event_date);
CREATE INDEX IF NOT EXISTS idx_stagehand_show ON stagehand_notes(show_name);
CREATE INDEX IF NOT EXISTS idx_stagehand_sync ON stagehand_notes(sync_status);

-- Daily logs (mirrors daily_logs table)
CREATE TABLE IF NOT EXISTS daily_logs (
    id INTEGER PRIMARY KEY,
    log_date TEXT NOT NULL,  -- ISO date string
    entry_time TEXT NOT NULL,  -- ISO datetime string
    category TEXT,
    content TEXT NOT NULL,
    source TEXT,
    session_id TEXT,
    importance INTEGER DEFAULT 3,
    agent_id TEXT,
    created_at TEXT,
    -- Local sync metadata
    local_modified_at TEXT,
    sync_status TEXT DEFAULT 'synced'
);

CREATE INDEX IF NOT EXISTS idx_daily_logs_date ON daily_logs(log_date);
CREATE INDEX IF NOT EXISTS idx_daily_logs_agent ON daily_logs(agent_id);
CREATE INDEX IF NOT EXISTS idx_daily_logs_sync ON daily_logs(sync_status);

-- Memory entries (mirrors memory_entries table)
CREATE TABLE IF NOT EXISTS memory_entries (
    id INTEGER PRIMARY KEY,
    content TEXT NOT NULL,
    entry_type TEXT DEFAULT 'fact',
    importance INTEGER DEFAULT 5,
    source TEXT,
    session_id TEXT,
    tags TEXT,  -- JSON array as text
    agent_id TEXT,
    created_at TEXT,
    modified_at TEXT,
    -- Local sync metadata
    local_modified_at TEXT,
    sync_status TEXT DEFAULT 'synced'
);

CREATE INDEX IF NOT EXISTS idx_memory_type ON memory_entries(entry_type);
CREATE INDEX IF NOT EXISTS idx_memory_agent ON memory_entries(agent_id);
CREATE INDEX IF NOT EXISTS idx_memory_sync ON memory_entries(sync_status);

-- Sync queue for pending changes
CREATE TABLE IF NOT EXISTS sync_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_name TEXT NOT NULL,
    operation TEXT NOT NULL,  -- 'insert', 'update', 'delete'
    record_id INTEGER NOT NULL,
    data TEXT,  -- JSON serialized data
    created_at TEXT NOT NULL,
    attempts INTEGER DEFAULT 0,
    last_error TEXT
);

CREATE INDEX IF NOT EXISTS idx_sync_queue_table ON sync_queue(table_name);
CREATE INDEX IF NOT EXISTS idx_sync_queue_created ON sync_queue(created_at);

-- Sync state tracking
CREATE TABLE IF NOT EXISTS sync_state (
    table_name TEXT PRIMARY KEY,
    last_sync_at TEXT,
    last_remote_modified TEXT,
    record_count INTEGER DEFAULT 0
);
"#;

/// Get statistics about the local cache
pub fn get_cache_stats(pool: &CachePool) -> Result<CacheStats> {
    let conn = pool.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
    
    let vault_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM memories",
        [],
        |row| row.get(0)
    )?;
    
    let stagehand_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM stagehand_notes",
        [],
        |row| row.get(0)
    )?;
    
    let daily_log_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM daily_logs",
        [],
        |row| row.get(0)
    )?;
    
    let memory_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM memory_entries",
        [],
        |row| row.get(0)
    )?;
    
    let pending_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sync_queue",
        [],
        |row| row.get(0)
    )?;
    
    Ok(CacheStats {
        memories: vault_count,
        stagehand_notes: stagehand_count,
        daily_logs: daily_log_count,
        memory_entries: memory_count,
        pending_changes: pending_count,
    })
}

/// Statistics about the local cache
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub memories: i64,
    pub stagehand_notes: i64,
    pub daily_logs: i64,
    pub memory_entries: i64,
    pub pending_changes: i64,
}

impl CacheStats {
    pub fn total_records(&self) -> i64 {
        self.memories + self.stagehand_notes + self.daily_logs + self.memory_entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_init_cache() {
        // Use a temp directory for testing
        let temp_dir = std::env::temp_dir().join("dpn-test-cache");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let db_path = temp_dir.join("test-cache.db");
        
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch(SCHEMA_SQL).unwrap();
        
        // Verify tables exist
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
            [],
            |row| row.get(0)
        ).unwrap();
        
        assert!(count >= 5, "Expected at least 5 tables");
        
        // Cleanup
        std::fs::remove_file(&db_path).ok();
    }
}
