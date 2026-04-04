//! Memory storage operations
//!
//! Write and read from daily_logs and memory_entries tables.

use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;

use crate::db::DbPool;

/// Daily log entry
#[derive(Debug, Clone, FromRow)]
pub struct DailyLog {
    pub id: i64,
    pub log_date: NaiveDate,
    pub entry_time: NaiveDateTime,
    pub category: Option<String>,
    pub content: String,
    pub source: Option<String>,
    pub session_id: Option<String>,
    pub importance: Option<i16>,
    pub agent_id: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

/// Structured memory entry
#[derive(Debug, Clone, FromRow)]
pub struct MemoryEntry {
    pub id: i64,
    pub content: String,
    pub entry_type: Option<String>,
    pub importance: Option<i16>,
    pub source: Option<String>,
    pub session_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub agent_id: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
}

/// Agent record
#[derive(Debug, Clone, FromRow)]
pub struct Agent {
    pub id: String,
    pub full_name: String,
    pub role: Option<String>,
    pub department: Option<String>,
    pub status: Option<String>,
    pub tool_scope: Option<Vec<String>>,
    pub machine: Option<String>,
}

/// Input for creating a daily log
#[derive(Debug, Clone)]
pub struct DailyLogCreate {
    pub content: String,
    pub category: Option<String>,
    pub source: Option<String>,
    pub session_id: Option<String>,
    pub importance: Option<i16>,
    pub agent_id: Option<String>,
}

/// Input for creating a memory entry
#[derive(Debug, Clone)]
pub struct MemoryEntryCreate {
    pub content: String,
    pub entry_type: Option<String>,
    pub importance: Option<i16>,
    pub source: Option<String>,
    pub session_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub agent_id: Option<String>,
}

// ============ Daily Logs ============

/// Write a daily log entry
pub async fn write_log(pool: &DbPool, log: &DailyLogCreate) -> Result<i64> {
    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO daily_logs (content, category, source, session_id, importance, agent_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#
    )
    .bind(&log.content)
    .bind(&log.category)
    .bind(&log.source)
    .bind(&log.session_id)
    .bind(log.importance.unwrap_or(3))
    .bind(&log.agent_id)
    .fetch_one(pool)
    .await?;
    
    Ok(row.0)
}

/// Get daily logs for a specific date
pub async fn get_logs_by_date(pool: &DbPool, date: NaiveDate) -> Result<Vec<DailyLog>> {
    let logs = sqlx::query_as::<_, DailyLog>(
        r#"
        SELECT id, log_date, entry_time::timestamp as entry_time, category, content, 
               source, session_id, importance, agent_id, created_at::timestamp as created_at
        FROM daily_logs
        WHERE log_date = $1
        ORDER BY entry_time
        "#
    )
    .bind(date)
    .fetch_all(pool)
    .await?;
    
    Ok(logs)
}

/// Get daily logs for an agent
pub async fn get_logs_by_agent(pool: &DbPool, agent_id: &str, limit: i64) -> Result<Vec<DailyLog>> {
    let logs = sqlx::query_as::<_, DailyLog>(
        r#"
        SELECT id, log_date, entry_time::timestamp as entry_time, category, content, 
               source, session_id, importance, agent_id, created_at::timestamp as created_at
        FROM daily_logs
        WHERE agent_id = $1
        ORDER BY entry_time DESC
        LIMIT $2
        "#
    )
    .bind(agent_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(logs)
}

/// Get logs modified since a timestamp
pub async fn list_logs_since(pool: &DbPool, since: NaiveDateTime) -> Result<Vec<DailyLog>> {
    let logs = sqlx::query_as::<_, DailyLog>(
        r#"
        SELECT id, log_date, entry_time::timestamp as entry_time, category, content, 
               source, session_id, importance, agent_id, created_at::timestamp as created_at
        FROM daily_logs
        WHERE created_at > $1
        ORDER BY created_at DESC
        "#
    )
    .bind(since)
    .fetch_all(pool)
    .await?;
    
    Ok(logs)
}

/// Get total log count
pub async fn get_log_count(pool: &DbPool) -> Result<i64> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM daily_logs")
        .fetch_one(pool)
        .await?;
    Ok(count.0)
}

// ============ Memory Entries ============

/// Write a memory entry
pub async fn write_memory(pool: &DbPool, entry: &MemoryEntryCreate) -> Result<i64> {
    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO memory_entries (content, entry_type, importance, source, session_id, tags, agent_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#
    )
    .bind(&entry.content)
    .bind(entry.entry_type.as_deref().unwrap_or("fact"))
    .bind(entry.importance.unwrap_or(5))
    .bind(&entry.source)
    .bind(&entry.session_id)
    .bind(&entry.tags)
    .bind(&entry.agent_id)
    .fetch_one(pool)
    .await?;
    
    Ok(row.0)
}

/// Get memory entries by type
pub async fn get_memories_by_type(pool: &DbPool, entry_type: &str, limit: i64) -> Result<Vec<MemoryEntry>> {
    let entries = sqlx::query_as::<_, MemoryEntry>(
        r#"
        SELECT id, content, entry_type, importance, source, session_id, tags, agent_id,
               created_at::timestamp as created_at, modified_at::timestamp as modified_at
        FROM memory_entries
        WHERE entry_type = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#
    )
    .bind(entry_type)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(entries)
}

/// Get memory entries by agent
pub async fn get_memories_by_agent(pool: &DbPool, agent_id: &str, limit: i64) -> Result<Vec<MemoryEntry>> {
    let entries = sqlx::query_as::<_, MemoryEntry>(
        r#"
        SELECT id, content, entry_type, importance, source, session_id, tags, agent_id,
               created_at::timestamp as created_at, modified_at::timestamp as modified_at
        FROM memory_entries
        WHERE agent_id = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#
    )
    .bind(agent_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(entries)
}

/// Get memory entries for a specific date
pub async fn get_memories_by_date(pool: &DbPool, date: NaiveDate) -> Result<Vec<MemoryEntry>> {
    let entries = sqlx::query_as::<_, MemoryEntry>(
        r#"
        SELECT id, content, entry_type, importance, source, session_id, tags, agent_id,
               created_at::timestamp as created_at, modified_at::timestamp as modified_at
        FROM memory_entries
        WHERE DATE(created_at) = $1
        ORDER BY created_at
        "#
    )
    .bind(date)
    .fetch_all(pool)
    .await?;
    
    Ok(entries)
}

/// Get memory entries by tags
pub async fn get_memories_by_tag(pool: &DbPool, tag: &str, limit: i64) -> Result<Vec<MemoryEntry>> {
    let entries = sqlx::query_as::<_, MemoryEntry>(
        r#"
        SELECT id, content, entry_type, importance, source, session_id, tags, agent_id,
               created_at::timestamp as created_at, modified_at::timestamp as modified_at
        FROM memory_entries
        WHERE $1 = ANY(tags)
        ORDER BY created_at DESC
        LIMIT $2
        "#
    )
    .bind(tag)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(entries)
}

/// Get memories modified since a timestamp
pub async fn list_memories_since(pool: &DbPool, since: NaiveDateTime) -> Result<Vec<MemoryEntry>> {
    let entries = sqlx::query_as::<_, MemoryEntry>(
        r#"
        SELECT id, content, entry_type, importance, source, session_id, tags, agent_id,
               created_at::timestamp as created_at, modified_at::timestamp as modified_at
        FROM memory_entries
        WHERE modified_at > $1
        ORDER BY modified_at DESC
        "#
    )
    .bind(since)
    .fetch_all(pool)
    .await?;
    
    Ok(entries)
}

/// Get total memory entry count
pub async fn get_memory_count(pool: &DbPool) -> Result<i64> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM memory_entries")
        .fetch_one(pool)
        .await?;
    Ok(count.0)
}

/// Search memories by text (simple ILIKE)
pub async fn search_memories(pool: &DbPool, query: &str, limit: i64) -> Result<Vec<MemoryEntry>> {
    let pattern = format!("%{}%", query);
    let entries = sqlx::query_as::<_, MemoryEntry>(
        r#"
        SELECT id, content, entry_type, importance, source, session_id, tags, agent_id,
               created_at::timestamp as created_at, modified_at::timestamp as modified_at
        FROM memory_entries
        WHERE content ILIKE $1
        ORDER BY importance DESC, created_at DESC
        LIMIT $2
        "#
    )
    .bind(&pattern)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(entries)
}

// ============ Agents ============

/// Get all agents
pub async fn list_agents(pool: &DbPool) -> Result<Vec<Agent>> {
    let agents = sqlx::query_as::<_, Agent>(
        r#"
        SELECT id, full_name, role, department, status, tool_scope, machine
        FROM agents
        WHERE status = 'active'
        ORDER BY full_name
        "#
    )
    .fetch_all(pool)
    .await?;
    
    Ok(agents)
}

/// Get agent by ID
pub async fn get_agent(pool: &DbPool, agent_id: &str) -> Result<Agent> {
    let agent = sqlx::query_as::<_, Agent>(
        r#"
        SELECT id, full_name, role, department, status, tool_scope, machine
        FROM agents
        WHERE id = $1
        "#
    )
    .bind(agent_id)
    .fetch_one(pool)
    .await?;
    
    Ok(agent)
}
