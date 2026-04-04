//! Sync queue for tracking pending changes
//!
//! Queues local changes (inserts, updates, deletes) for later sync to PostgreSQL.
//! Provides retry tracking and error handling.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::sqlite::CachePool;

/// Type of change operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeOperation {
    Insert,
    Update,
    Delete,
}

impl ChangeOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChangeOperation::Insert => "insert",
            ChangeOperation::Update => "update",
            ChangeOperation::Delete => "delete",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "insert" => Some(ChangeOperation::Insert),
            "update" => Some(ChangeOperation::Update),
            "delete" => Some(ChangeOperation::Delete),
            _ => None,
        }
    }
}

/// A pending change waiting to be synced
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingChange {
    pub id: i64,
    pub table_name: String,
    pub operation: ChangeOperation,
    pub record_id: i64,
    pub data: Option<String>,  // JSON serialized record data
    pub created_at: DateTime<Utc>,
    pub attempts: i32,
    pub last_error: Option<String>,
}

/// Sync queue manager
pub struct SyncQueue {
    pool: CachePool,
}

impl SyncQueue {
    /// Create a new sync queue manager
    pub fn new(pool: CachePool) -> Self {
        Self { pool }
    }
    
    /// Queue a change for later sync
    pub fn queue_change(
        &self,
        table: &str,
        operation: ChangeOperation,
        record_id: i64,
        data: Option<&str>,
    ) -> Result<i64> {
        let conn = self.pool.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            r#"
            INSERT INTO sync_queue (table_name, operation, record_id, data, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            rusqlite::params![
                table,
                operation.as_str(),
                record_id,
                data,
                now,
            ],
        )?;
        
        let id = conn.last_insert_rowid();
        tracing::debug!("Queued {} on {} id={}", operation.as_str(), table, record_id);
        Ok(id)
    }
    
    /// Get all pending changes (oldest first)
    pub fn get_pending_changes(&self) -> Result<Vec<PendingChange>> {
        let conn = self.pool.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        
        let mut stmt = conn.prepare(
            r#"
            SELECT id, table_name, operation, record_id, data, created_at, attempts, last_error
            FROM sync_queue
            ORDER BY created_at ASC
            "#
        )?;
        
        let changes = stmt.query_map([], |row| {
            let op_str: String = row.get(2)?;
            let created_str: String = row.get(5)?;
            
            Ok(PendingChange {
                id: row.get(0)?,
                table_name: row.get(1)?,
                operation: ChangeOperation::from_str(&op_str).unwrap_or(ChangeOperation::Update),
                record_id: row.get(3)?,
                data: row.get(4)?,
                created_at: DateTime::parse_from_rfc3339(&created_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                attempts: row.get(6)?,
                last_error: row.get(7)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(changes)
    }
    
    /// Get pending changes for a specific table
    pub fn get_pending_for_table(&self, table: &str) -> Result<Vec<PendingChange>> {
        let conn = self.pool.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        
        let mut stmt = conn.prepare(
            r#"
            SELECT id, table_name, operation, record_id, data, created_at, attempts, last_error
            FROM sync_queue
            WHERE table_name = ?1
            ORDER BY created_at ASC
            "#
        )?;
        
        let changes = stmt.query_map([table], |row| {
            let op_str: String = row.get(2)?;
            let created_str: String = row.get(5)?;
            
            Ok(PendingChange {
                id: row.get(0)?,
                table_name: row.get(1)?,
                operation: ChangeOperation::from_str(&op_str).unwrap_or(ChangeOperation::Update),
                record_id: row.get(3)?,
                data: row.get(4)?,
                created_at: DateTime::parse_from_rfc3339(&created_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                attempts: row.get(6)?,
                last_error: row.get(7)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(changes)
    }
    
    /// Mark a change as synced (remove from queue)
    pub fn mark_synced(&self, change_id: i64) -> Result<()> {
        let conn = self.pool.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        
        conn.execute("DELETE FROM sync_queue WHERE id = ?1", [change_id])?;
        tracing::debug!("Marked change {} as synced", change_id);
        Ok(())
    }
    
    /// Mark multiple changes as synced
    pub fn mark_batch_synced(&self, change_ids: &[i64]) -> Result<()> {
        let conn = self.pool.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        
        let placeholders: String = change_ids.iter()
            .enumerate()
            .map(|(i, _)| format!("?{}", i + 1))
            .collect::<Vec<_>>()
            .join(", ");
        
        let sql = format!("DELETE FROM sync_queue WHERE id IN ({})", placeholders);
        let mut stmt = conn.prepare(&sql)?;
        
        let params: Vec<&dyn rusqlite::ToSql> = change_ids.iter()
            .map(|id| id as &dyn rusqlite::ToSql)
            .collect();
        
        stmt.execute(params.as_slice())?;
        tracing::debug!("Marked {} changes as synced", change_ids.len());
        Ok(())
    }
    
    /// Record a sync attempt failure
    pub fn record_failure(&self, change_id: i64, error: &str) -> Result<()> {
        let conn = self.pool.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        
        conn.execute(
            "UPDATE sync_queue SET attempts = attempts + 1, last_error = ?1 WHERE id = ?2",
            rusqlite::params![error, change_id],
        )?;
        
        Ok(())
    }
    
    /// Get count of pending changes
    pub fn pending_count(&self) -> Result<i64> {
        let conn = self.pool.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sync_queue",
            [],
            |row| row.get(0)
        )?;
        Ok(count)
    }
    
    /// Clear all pending changes (use with caution!)
    pub fn clear_all(&self) -> Result<i64> {
        let conn = self.pool.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM sync_queue", [], |row| row.get(0))?;
        conn.execute("DELETE FROM sync_queue", [])?;
        tracing::warn!("Cleared {} pending changes from sync queue", count);
        Ok(count)
    }
    
    /// Remove changes that have failed too many times
    pub fn prune_failed(&self, max_attempts: i32) -> Result<i64> {
        let conn = self.pool.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sync_queue WHERE attempts >= ?1",
            [max_attempts],
            |row| row.get(0)
        )?;
        
        conn.execute(
            "DELETE FROM sync_queue WHERE attempts >= ?1",
            [max_attempts],
        )?;
        
        if count > 0 {
            tracing::warn!("Pruned {} failed changes (>= {} attempts)", count, max_attempts);
        }
        
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    
    fn create_test_queue() -> SyncQueue {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE sync_queue (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                table_name TEXT NOT NULL,
                operation TEXT NOT NULL,
                record_id INTEGER NOT NULL,
                data TEXT,
                created_at TEXT NOT NULL,
                attempts INTEGER DEFAULT 0,
                last_error TEXT
            );
            "#
        ).unwrap();
        
        SyncQueue::new(Arc::new(Mutex::new(conn)))
    }
    
    #[test]
    fn test_queue_and_retrieve() {
        let queue = create_test_queue();
        
        // Queue a change
        let id = queue.queue_change(
            "memories",
            ChangeOperation::Insert,
            42,
            Some(r#"{"title": "Test"}"#),
        ).unwrap();
        
        assert!(id > 0);
        
        // Retrieve pending changes
        let changes = queue.get_pending_changes().unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].table_name, "memories");
        assert_eq!(changes[0].record_id, 42);
        assert_eq!(changes[0].operation, ChangeOperation::Insert);
    }
    
    #[test]
    fn test_mark_synced() {
        let queue = create_test_queue();
        
        let id = queue.queue_change("daily_logs", ChangeOperation::Update, 1, None).unwrap();
        assert_eq!(queue.pending_count().unwrap(), 1);
        
        queue.mark_synced(id).unwrap();
        assert_eq!(queue.pending_count().unwrap(), 0);
    }
}
