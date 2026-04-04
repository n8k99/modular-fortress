//! Hybrid storage layer
//!
//! Provides offline-first access by reading from local cache first,
//! falling back to remote PostgreSQL, and queuing writes for sync.

use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::sqlite::{CachePool, CacheStats, get_cache_stats, get_sync_state_path};
use super::sync_queue::{ChangeOperation, SyncQueue};
use crate::db::DbPool;
use crate::db::memories::{Memory, MemoryLight};
use crate::db::stagehand::{StagehandNote, StagehandNoteCreate};
use crate::memory::{DailyLog, DailyLogCreate, MemoryEntryCreate};

/// Sync state persisted to disk
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncStateFile {
    pub last_sync: Option<String>,  // ISO datetime
    pub memories_last_modified: Option<String>,
    pub stagehand_notes_last_modified: Option<String>,
    pub daily_logs_last_modified: Option<String>,
    pub memory_entries_last_modified: Option<String>,
    pub is_online: bool,
}

impl SyncStateFile {
    pub fn load() -> Result<Self> {
        let path = get_sync_state_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let path = get_sync_state_path()?;
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Hybrid store combining local SQLite cache with remote PostgreSQL
pub struct HybridStore {
    local: CachePool,
    remote: Option<DbPool>,
    sync_queue: SyncQueue,
    is_online: bool,
}

impl HybridStore {
    /// Create a new hybrid store with both local and remote connections
    pub fn new(local: CachePool, remote: Option<DbPool>) -> Self {
        let sync_queue = SyncQueue::new(local.clone());
        let is_online = remote.is_some();
        
        Self {
            local,
            remote,
            sync_queue,
            is_online,
        }
    }
    
    /// Create a hybrid store in offline mode (local only)
    pub fn offline(local: CachePool) -> Self {
        Self::new(local, None)
    }
    
    /// Check if we're currently online
    pub fn is_online(&self) -> bool {
        self.is_online
    }
    
    /// Test remote connectivity and update online status
    pub async fn check_connectivity(&mut self) -> bool {
        if let Some(ref pool) = self.remote {
            match tokio::time::timeout(
                Duration::from_secs(5),
                crate::db::test_connection(pool)
            ).await {
                Ok(Ok(())) => {
                    self.is_online = true;
                    true
                }
                _ => {
                    tracing::warn!("PostgreSQL connectivity check failed, switching to offline mode");
                    self.is_online = false;
                    false
                }
            }
        } else {
            self.is_online = false;
            false
        }
    }
    
    /// Get local cache statistics
    pub fn cache_stats(&self) -> Result<CacheStats> {
        get_cache_stats(&self.local)
    }
    
    /// Get pending change count
    pub fn pending_changes(&self) -> Result<i64> {
        self.sync_queue.pending_count()
    }
    
    // ============ Memories ============

    /// Get a memory by ID (local first, then remote)
    pub async fn get_memory(&self, id: i32) -> Result<Memory> {
        // Try local first
        if let Ok(note) = self.get_memory_local(id) {
            return Ok(note);
        }
        
        // Fall back to remote if online
        if self.is_online {
            if let Some(ref pool) = self.remote {
                let note = crate::db::memories::get_by_id(pool, id).await?;
                // Cache locally for future access
                self.cache_memory(&note)?;
                return Ok(note);
            }
        }
        
        anyhow::bail!("Memory {} not found in local cache and remote unavailable", id)
    }
    
    /// Get memory from local cache only
    fn get_memory_local(&self, id: i32) -> Result<Memory> {
        let conn = self.local.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        
        let note = conn.query_row(
            r#"
            SELECT id, path, title, content, frontmatter, size_bytes, note_type,
                   note_date, modified_at, created_at
            FROM memories WHERE id = ?1
            "#,
            [id],
            |row| {
                Ok(Memory {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    frontmatter: row.get(4)?,
                    size_bytes: row.get(5)?,
                    note_type: row.get(6)?,
                    note_date: parse_date_opt(row.get::<_, Option<String>>(7)?),
                    modified_at: parse_datetime_opt(row.get::<_, Option<String>>(8)?),
                    created_at: parse_datetime_opt(row.get::<_, Option<String>>(9)?),
                    compression_tier: "daily".to_string(),
                    compressed_from: None,
                })
            },
        )?;

        Ok(note)
    }
    
    /// Cache a memory locally
    fn cache_memory(&self, note: &Memory) -> Result<()> {
        let conn = self.local.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        
        conn.execute(
            r#"
            INSERT OR REPLACE INTO memories
            (id, path, title, content, frontmatter, size_bytes, note_type, note_date, modified_at, created_at, sync_status)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'synced')
            "#,
            rusqlite::params![
                note.id,
                note.path,
                note.title,
                note.content,
                note.frontmatter,
                note.size_bytes,
                note.note_type,
                note.note_date.map(|d| d.to_string()),
                note.modified_at.map(|dt| dt.to_string()),
                note.created_at.map(|dt| dt.to_string()),
            ],
        )?;
        
        Ok(())
    }
    
    /// List memories (light, from local cache)
    pub fn list_memories_local(&self, limit: i64, offset: i64) -> Result<Vec<MemoryLight>> {
        let conn = self.local.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        
        let mut stmt = conn.prepare(
            r#"
            SELECT id, path, title, size_bytes, note_type, note_date, modified_at
            FROM memories
            ORDER BY path
            LIMIT ?1 OFFSET ?2
            "#
        )?;

        let notes = stmt.query_map([limit, offset], |row| {
            Ok(MemoryLight {
                id: row.get(0)?,
                path: row.get(1)?,
                title: row.get(2)?,
                size_bytes: row.get(3)?,
                note_type: row.get(4)?,
                note_date: parse_date_opt(row.get::<_, Option<String>>(5)?),
                modified_at: parse_datetime_opt(row.get::<_, Option<String>>(6)?),
                compression_tier: "daily".to_string(),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(notes)
    }
    
    /// Search memories locally
    pub fn search_memories_local(&self, query: &str, limit: i64) -> Result<Vec<Memory>> {
        let conn = self.local.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let pattern = format!("%{}%", query);
        
        let mut stmt = conn.prepare(
            r#"
            SELECT id, path, title, content, frontmatter, size_bytes, note_type,
                   note_date, modified_at, created_at
            FROM memories
            WHERE path LIKE ?1 OR title LIKE ?1 OR content LIKE ?1
            ORDER BY path
            LIMIT ?2
            "#
        )?;

        let notes = stmt.query_map(rusqlite::params![pattern, limit], |row| {
            Ok(Memory {
                id: row.get(0)?,
                path: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                frontmatter: row.get(4)?,
                size_bytes: row.get(5)?,
                note_type: row.get(6)?,
                note_date: parse_date_opt(row.get::<_, Option<String>>(7)?),
                modified_at: parse_datetime_opt(row.get::<_, Option<String>>(8)?),
                created_at: parse_datetime_opt(row.get::<_, Option<String>>(9)?),
                compression_tier: "daily".to_string(),
                compressed_from: None,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(notes)
    }
    
    // ============ Stagehand Notes ============
    
    /// Get stagehand notes for a date (local first)
    pub async fn get_stagehand_by_date(&self, date: NaiveDate) -> Result<Vec<StagehandNote>> {
        // Try local first
        let local_notes = self.get_stagehand_by_date_local(date)?;
        if !local_notes.is_empty() {
            return Ok(local_notes);
        }
        
        // Fall back to remote
        if self.is_online {
            if let Some(ref pool) = self.remote {
                let notes = crate::db::stagehand::get_by_date(pool, date).await?;
                for note in &notes {
                    self.cache_stagehand_note(note)?;
                }
                return Ok(notes);
            }
        }
        
        Ok(vec![])
    }
    
    /// Get stagehand notes from local cache
    fn get_stagehand_by_date_local(&self, date: NaiveDate) -> Result<Vec<StagehandNote>> {
        let conn = self.local.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let date_str = date.to_string();
        
        let mut stmt = conn.prepare(
            r#"
            SELECT id, show_name, venue, event_date, call_time, notes, tags, created_at, modified_at
            FROM stagehand_notes
            WHERE event_date = ?1
            ORDER BY call_time
            "#
        )?;
        
        let notes = stmt.query_map([&date_str], |row| {
            let tags_str: Option<String> = row.get(6)?;
            let tags: Option<Vec<String>> = tags_str.and_then(|s| serde_json::from_str(&s).ok());
            
            Ok(StagehandNote {
                id: row.get(0)?,
                show_name: row.get(1)?,
                venue: row.get(2)?,
                event_date: parse_date(row.get::<_, String>(3)?),
                call_time: parse_time_opt(row.get::<_, Option<String>>(4)?),
                notes: row.get(5)?,
                tags,
                created_at: parse_datetime_opt(row.get::<_, Option<String>>(7)?),
                modified_at: parse_datetime_opt(row.get::<_, Option<String>>(8)?),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(notes)
    }
    
    /// Cache a stagehand note locally
    fn cache_stagehand_note(&self, note: &StagehandNote) -> Result<()> {
        let conn = self.local.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let tags_json = note.tags.as_ref().map(|t| serde_json::to_string(t).unwrap_or_default());
        
        conn.execute(
            r#"
            INSERT OR REPLACE INTO stagehand_notes 
            (id, show_name, venue, event_date, call_time, notes, tags, created_at, modified_at, sync_status)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'synced')
            "#,
            rusqlite::params![
                note.id,
                note.show_name,
                note.venue,
                note.event_date.to_string(),
                note.call_time.map(|t| t.to_string()),
                note.notes,
                tags_json,
                note.created_at.map(|dt| dt.to_string()),
                note.modified_at.map(|dt| dt.to_string()),
            ],
        )?;
        
        Ok(())
    }
    
    /// Create a stagehand note (write-through)
    pub async fn create_stagehand_note(&self, note: &StagehandNoteCreate) -> Result<i32> {
        let data = serde_json::to_string(note)?;
        
        // If online, write to remote first
        if self.is_online {
            if let Some(ref pool) = self.remote {
                let id = crate::db::stagehand::create(pool, note).await?;
                
                // Fetch and cache the created note
                let created = crate::db::stagehand::get_by_id(pool, id).await?;
                self.cache_stagehand_note(&created)?;
                
                return Ok(id);
            }
        }
        
        // Offline: insert locally with negative temp ID, queue for sync
        let conn = self.local.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let now = Utc::now().naive_utc().to_string();
        let tags_json = note.tags.as_ref().map(|t| serde_json::to_string(t).unwrap_or_default());
        
        conn.execute(
            r#"
            INSERT INTO stagehand_notes 
            (show_name, venue, event_date, call_time, notes, tags, created_at, modified_at, sync_status)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7, 'pending')
            "#,
            rusqlite::params![
                note.show_name,
                note.venue,
                note.event_date.to_string(),
                note.call_time.map(|t| t.to_string()),
                note.notes,
                tags_json,
                now,
            ],
        )?;
        
        let local_id = conn.last_insert_rowid() as i32;
        drop(conn);  // Release lock before queueing
        
        // Queue for sync
        self.sync_queue.queue_change("stagehand_notes", ChangeOperation::Insert, local_id as i64, Some(&data))?;
        
        Ok(local_id)
    }
    
    // ============ Daily Logs ============
    
    /// Write a daily log (write-through)
    pub async fn write_daily_log(&self, log: &DailyLogCreate) -> Result<i64> {
        let data = serde_json::to_string(log)?;
        
        // If online, write to remote first
        if self.is_online {
            if let Some(ref pool) = self.remote {
                let id = crate::memory::write_log(pool, log).await?;
                // Cache locally
                self.cache_daily_log_from_create(id, log)?;
                return Ok(id);
            }
        }
        
        // Offline: insert locally, queue for sync
        let conn = self.local.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let now = Utc::now();
        let log_date = now.date_naive().to_string();
        let entry_time = now.naive_utc().to_string();
        
        conn.execute(
            r#"
            INSERT INTO daily_logs 
            (log_date, entry_time, category, content, source, session_id, importance, agent_id, created_at, sync_status)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?2, 'pending')
            "#,
            rusqlite::params![
                log_date,
                entry_time,
                log.category,
                log.content,
                log.source,
                log.session_id,
                log.importance.unwrap_or(3),
                log.agent_id,
            ],
        )?;
        
        let local_id = conn.last_insert_rowid();
        drop(conn);
        
        self.sync_queue.queue_change("daily_logs", ChangeOperation::Insert, local_id, Some(&data))?;
        
        Ok(local_id)
    }
    
    /// Cache a daily log from create params
    fn cache_daily_log_from_create(&self, id: i64, log: &DailyLogCreate) -> Result<()> {
        let conn = self.local.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let now = Utc::now();
        let log_date = now.date_naive().to_string();
        let entry_time = now.naive_utc().to_string();
        
        conn.execute(
            r#"
            INSERT OR REPLACE INTO daily_logs 
            (id, log_date, entry_time, category, content, source, session_id, importance, agent_id, created_at, sync_status)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?3, 'synced')
            "#,
            rusqlite::params![
                id,
                log_date,
                entry_time,
                log.category,
                log.content,
                log.source,
                log.session_id,
                log.importance.unwrap_or(3),
                log.agent_id,
            ],
        )?;
        
        Ok(())
    }
    
    /// Get daily logs for a date
    pub fn get_daily_logs_local(&self, date: NaiveDate) -> Result<Vec<DailyLog>> {
        let conn = self.local.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let date_str = date.to_string();
        
        let mut stmt = conn.prepare(
            r#"
            SELECT id, log_date, entry_time, category, content, source, session_id, importance, agent_id, created_at
            FROM daily_logs
            WHERE log_date = ?1
            ORDER BY entry_time
            "#
        )?;
        
        let logs = stmt.query_map([&date_str], |row| {
            Ok(DailyLog {
                id: row.get(0)?,
                log_date: parse_date(row.get::<_, String>(1)?),
                entry_time: parse_datetime(row.get::<_, String>(2)?),
                category: row.get(3)?,
                content: row.get(4)?,
                source: row.get(5)?,
                session_id: row.get(6)?,
                importance: row.get(7)?,
                agent_id: row.get(8)?,
                created_at: parse_datetime_opt(row.get::<_, Option<String>>(9)?),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(logs)
    }
    
    // ============ Sync Operations ============
    
    /// Push all pending changes to remote PostgreSQL
    pub async fn sync_pending(&mut self) -> Result<SyncResult> {
        if !self.check_connectivity().await {
            return Ok(SyncResult {
                pushed: 0,
                failed: 0,
                remaining: self.sync_queue.pending_count()?,
            });
        }
        
        let pool = match &self.remote {
            Some(p) => p,
            None => return Ok(SyncResult { pushed: 0, failed: 0, remaining: 0 }),
        };
        
        let changes = self.sync_queue.get_pending_changes()?;
        let mut pushed = 0;
        let mut failed = 0;
        
        for change in changes {
            let result = match change.table_name.as_str() {
                "stagehand_notes" => self.sync_stagehand_change(pool, &change).await,
                "daily_logs" => self.sync_daily_log_change(pool, &change).await,
                "memory_entries" => self.sync_memory_change(pool, &change).await,
                "memories" => self.sync_memory_change_vault(pool, &change).await,
                _ => {
                    tracing::warn!("Unknown table in sync queue: {}", change.table_name);
                    continue;
                }
            };
            
            match result {
                Ok(()) => {
                    self.sync_queue.mark_synced(change.id)?;
                    pushed += 1;
                }
                Err(e) => {
                    self.sync_queue.record_failure(change.id, &e.to_string())?;
                    failed += 1;
                }
            }
        }
        
        // Update sync state
        let mut state = SyncStateFile::load().unwrap_or_default();
        state.last_sync = Some(Utc::now().to_rfc3339());
        state.is_online = self.is_online;
        state.save()?;
        
        Ok(SyncResult {
            pushed,
            failed,
            remaining: self.sync_queue.pending_count()?,
        })
    }
    
    async fn sync_stagehand_change(&self, pool: &DbPool, change: &super::sync_queue::PendingChange) -> Result<()> {
        match change.operation {
            ChangeOperation::Insert => {
                if let Some(ref data) = change.data {
                    let note: StagehandNoteCreate = serde_json::from_str(data)?;
                    crate::db::stagehand::create(pool, &note).await?;
                }
            }
            ChangeOperation::Delete => {
                crate::db::stagehand::delete(pool, change.record_id as i32).await?;
            }
            ChangeOperation::Update => {
                // For updates, we'd need the full record data
                tracing::warn!("Stagehand update sync not fully implemented");
            }
        }
        Ok(())
    }
    
    async fn sync_daily_log_change(&self, pool: &DbPool, change: &super::sync_queue::PendingChange) -> Result<()> {
        match change.operation {
            ChangeOperation::Insert => {
                if let Some(ref data) = change.data {
                    let log: DailyLogCreate = serde_json::from_str(data)?;
                    crate::memory::write_log(pool, &log).await?;
                }
            }
            _ => {
                tracing::warn!("Daily log {} sync not implemented", change.operation.as_str());
            }
        }
        Ok(())
    }
    
    async fn sync_memory_change(&self, pool: &DbPool, change: &super::sync_queue::PendingChange) -> Result<()> {
        match change.operation {
            ChangeOperation::Insert => {
                if let Some(ref data) = change.data {
                    let entry: MemoryEntryCreate = serde_json::from_str(data)?;
                    crate::memory::write_memory(pool, &entry).await?;
                }
            }
            _ => {
                tracing::warn!("Memory entry {} sync not implemented", change.operation.as_str());
            }
        }
        Ok(())
    }
    
    async fn sync_memory_change_vault(&self, _pool: &DbPool, change: &super::sync_queue::PendingChange) -> Result<()> {
        // Memories are typically read-only from the perspective of this app
        tracing::warn!("Memory sync for {} not implemented", change.operation.as_str());
        Ok(())
    }
    
    /// Pull updates from remote PostgreSQL to local cache
    pub async fn pull_updates(&mut self) -> Result<PullResult> {
        if !self.check_connectivity().await {
            return Ok(PullResult { pulled: 0, tables_updated: vec![] });
        }
        
        let pool = match &self.remote {
            Some(p) => p,
            None => return Ok(PullResult { pulled: 0, tables_updated: vec![] }),
        };
        
        let state = SyncStateFile::load().unwrap_or_default();
        let since = state.last_sync
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.naive_utc())
            .unwrap_or_else(|| NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap());
        
        let mut pulled = 0;
        let mut tables_updated = vec![];
        
        // Pull memories
        let memories = crate::db::memories::list_modified_since(pool, since).await?;
        if !memories.is_empty() {
            for note in &memories {
                self.cache_memory(note)?;
            }
            pulled += memories.len();
            tables_updated.push("memories".to_string());
        }
        
        // Pull stagehand notes (all for simplicity - could optimize with modified_at)
        let stagehand_notes = crate::db::stagehand::list_all(pool).await?;
        for note in &stagehand_notes {
            self.cache_stagehand_note(note)?;
        }
        if !stagehand_notes.is_empty() {
            pulled += stagehand_notes.len();
            tables_updated.push("stagehand_notes".to_string());
        }
        
        // Update sync state
        let mut new_state = SyncStateFile::load().unwrap_or_default();
        new_state.last_sync = Some(Utc::now().to_rfc3339());
        new_state.is_online = true;
        new_state.save()?;
        
        Ok(PullResult { pulled, tables_updated })
    }
}

/// Result of sync_pending operation
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub pushed: usize,
    pub failed: usize,
    pub remaining: i64,
}

/// Result of pull_updates operation
#[derive(Debug, Clone)]
pub struct PullResult {
    pub pulled: usize,
    pub tables_updated: Vec<String>,
}

// ============ Date/Time Parsing Helpers ============

fn parse_date(s: String) -> NaiveDate {
    NaiveDate::parse_from_str(&s, "%Y-%m-%d")
        .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
}

fn parse_date_opt(s: Option<String>) -> Option<NaiveDate> {
    s.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
}

fn parse_datetime(s: String) -> NaiveDateTime {
    // Try multiple formats
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f")
        .or_else(|_| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S"))
        .or_else(|_| NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.f"))
        .or_else(|_| NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S"))
        .unwrap_or_else(|_| NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap())
}

fn parse_datetime_opt(s: Option<String>) -> Option<NaiveDateTime> {
    s.and_then(|s| {
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f")
            .or_else(|_| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S"))
            .or_else(|_| NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.f"))
            .or_else(|_| NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S"))
            .ok()
    })
}

fn parse_time_opt(s: Option<String>) -> Option<chrono::NaiveTime> {
    s.and_then(|s| chrono::NaiveTime::parse_from_str(&s, "%H:%M:%S").ok()
        .or_else(|| chrono::NaiveTime::parse_from_str(&s, "%H:%M").ok()))
}

// Make StagehandNoteCreate serializable for queueing
impl Serialize for StagehandNoteCreate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("StagehandNoteCreate", 6)?;
        state.serialize_field("show_name", &self.show_name)?;
        state.serialize_field("venue", &self.venue)?;
        state.serialize_field("event_date", &self.event_date.to_string())?;
        state.serialize_field("call_time", &self.call_time.map(|t| t.to_string()))?;
        state.serialize_field("notes", &self.notes)?;
        state.serialize_field("tags", &self.tags)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for StagehandNoteCreate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            show_name: String,
            venue: Option<String>,
            event_date: String,
            call_time: Option<String>,
            notes: Option<String>,
            tags: Option<Vec<String>>,
        }
        
        let helper = Helper::deserialize(deserializer)?;
        Ok(StagehandNoteCreate {
            show_name: helper.show_name,
            venue: helper.venue,
            event_date: NaiveDate::parse_from_str(&helper.event_date, "%Y-%m-%d")
                .map_err(serde::de::Error::custom)?,
            call_time: helper.call_time.and_then(|s| chrono::NaiveTime::parse_from_str(&s, "%H:%M:%S").ok()),
            notes: helper.notes,
            tags: helper.tags,
        })
    }
}

// Make DailyLogCreate serializable
impl Serialize for DailyLogCreate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("DailyLogCreate", 6)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("category", &self.category)?;
        state.serialize_field("source", &self.source)?;
        state.serialize_field("session_id", &self.session_id)?;
        state.serialize_field("importance", &self.importance)?;
        state.serialize_field("agent_id", &self.agent_id)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for DailyLogCreate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            content: String,
            category: Option<String>,
            source: Option<String>,
            session_id: Option<String>,
            importance: Option<i16>,
            agent_id: Option<String>,
        }
        
        let helper = Helper::deserialize(deserializer)?;
        Ok(DailyLogCreate {
            content: helper.content,
            category: helper.category,
            source: helper.source,
            session_id: helper.session_id,
            importance: helper.importance,
            agent_id: helper.agent_id,
        })
    }
}

// Make MemoryEntryCreate serializable
impl Serialize for MemoryEntryCreate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("MemoryEntryCreate", 7)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("entry_type", &self.entry_type)?;
        state.serialize_field("importance", &self.importance)?;
        state.serialize_field("source", &self.source)?;
        state.serialize_field("session_id", &self.session_id)?;
        state.serialize_field("tags", &self.tags)?;
        state.serialize_field("agent_id", &self.agent_id)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for MemoryEntryCreate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            content: String,
            entry_type: Option<String>,
            importance: Option<i16>,
            source: Option<String>,
            session_id: Option<String>,
            tags: Option<Vec<String>>,
            agent_id: Option<String>,
        }
        
        let helper = Helper::deserialize(deserializer)?;
        Ok(MemoryEntryCreate {
            content: helper.content,
            entry_type: helper.entry_type,
            importance: helper.importance,
            source: helper.source,
            session_id: helper.session_id,
            tags: helper.tags,
            agent_id: helper.agent_id,
        })
    }
}
