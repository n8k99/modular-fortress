//! Conversations module for agent-to-agent communication
//!
//! Provides CRUD operations for the conversations table, enabling
//! persistent messaging between agents across distributed hosts.
//!
//! Schema (from conversations table):
//! - thread_id: UUID grouping related messages
//! - from_agent: sender agent_id
//! - to_agent: recipient(s) array
//! - channel: 'direct', 'department', 'broadcast'
//! - message: text content
//! - message_type: 'chat', 'system', 'handoff', 'status'
//! - metadata: flexible JSONB payload
//! - read_by: array of agent_ids who have read it

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

use crate::db::DbPool;

/// A conversation message record
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Conversation {
    pub id: i64,
    pub thread_id: Uuid,
    pub from_agent: String,
    pub to_agent: Vec<String>,
    pub channel: Option<String>,
    pub message: String,
    pub message_type: Option<String>,
    pub metadata: Option<JsonValue>,
    pub read_by: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Input for creating a new conversation message
#[derive(Debug, Clone)]
pub struct ConversationCreate {
    pub thread_id: Option<Uuid>,
    pub from_agent: String,
    pub to_agent: Vec<String>,
    pub channel: Option<String>,
    pub message: String,
    pub message_type: Option<String>,
    pub metadata: Option<JsonValue>,
}

/// Lightweight conversation for listing
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ConversationLight {
    pub id: i64,
    pub thread_id: Uuid,
    pub from_agent: String,
    pub to_agent: Vec<String>,
    pub channel: Option<String>,
    pub message_type: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ============ Core Functions ============

/// Send a message between agents
///
/// Creates a new conversation record. If `thread_id` is None, a new UUID is generated.
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `from` - Sender agent ID
/// * `to` - Recipient agent ID(s)
/// * `message` - Message content
/// * `channel` - Channel type ('direct', 'department', 'broadcast')
///
/// # Returns
/// The created conversation record
pub async fn send_message(
    pool: &DbPool,
    from: &str,
    to: &[&str],
    message: &str,
    channel: Option<&str>,
) -> Result<Conversation> {
    let to_vec: Vec<String> = to.iter().map(|s| s.to_string()).collect();
    
    let conversation = sqlx::query_as::<_, Conversation>(
        r#"
        INSERT INTO conversations (from_agent, to_agent, message, channel, message_type, read_by)
        VALUES ($1, $2, $3, $4, 'chat', ARRAY[]::varchar[])
        RETURNING id, thread_id, from_agent, to_agent, channel, message, message_type,
                  metadata, read_by, created_at
        "#,
    )
    .bind(from)
    .bind(&to_vec)
    .bind(message)
    .bind(channel.unwrap_or("direct"))
    .fetch_one(pool)
    .await?;
    
    Ok(conversation)
}

/// Send a message with full control over all fields
pub async fn send_message_full(pool: &DbPool, create: &ConversationCreate) -> Result<Conversation> {
    let thread_id = create.thread_id.unwrap_or_else(Uuid::new_v4);
    
    let conversation = sqlx::query_as::<_, Conversation>(
        r#"
        INSERT INTO conversations (thread_id, from_agent, to_agent, message, channel, 
                                   message_type, metadata, read_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, ARRAY[]::varchar[])
        RETURNING id, thread_id, from_agent, to_agent, channel, message, message_type,
                  metadata, read_by, created_at
        "#,
    )
    .bind(thread_id)
    .bind(&create.from_agent)
    .bind(&create.to_agent)
    .bind(&create.message)
    .bind(create.channel.as_deref().unwrap_or("direct"))
    .bind(create.message_type.as_deref().unwrap_or("chat"))
    .bind(&create.metadata)
    .fetch_one(pool)
    .await?;
    
    Ok(conversation)
}

/// Get all messages in a thread
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `thread_id` - UUID of the conversation thread
///
/// # Returns
/// All messages in the thread, ordered by creation time
pub async fn get_thread(pool: &DbPool, thread_id: Uuid) -> Result<Vec<Conversation>> {
    let conversations = sqlx::query_as::<_, Conversation>(
        r#"
        SELECT id, thread_id, from_agent, to_agent, channel, message, message_type,
               metadata, read_by, created_at
        FROM conversations
        WHERE thread_id = $1
        ORDER BY created_at ASC
        "#,
    )
    .bind(thread_id)
    .fetch_all(pool)
    .await?;
    
    Ok(conversations)
}

/// Get unread messages for an agent
///
/// Returns messages where the agent is in `to_agent` but not in `read_by`.
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `agent_id` - The agent checking their unread messages
///
/// # Returns
/// Unread messages for the agent, ordered by creation time (newest first)
pub async fn get_unread(pool: &DbPool, agent_id: &str) -> Result<Vec<Conversation>> {
    let conversations = sqlx::query_as::<_, Conversation>(
        r#"
        SELECT id, thread_id, from_agent, to_agent, channel, message, message_type,
               metadata, read_by, created_at
        FROM conversations
        WHERE $1 = ANY(to_agent)
          AND NOT ($1 = ANY(read_by))
        ORDER BY created_at DESC
        "#,
    )
    .bind(agent_id)
    .fetch_all(pool)
    .await?;
    
    Ok(conversations)
}

/// Count unread messages for an agent
pub async fn count_unread(pool: &DbPool, agent_id: &str) -> Result<i64> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM conversations
        WHERE $1 = ANY(to_agent)
          AND NOT ($1 = ANY(read_by))
        "#,
    )
    .bind(agent_id)
    .fetch_one(pool)
    .await?;
    
    Ok(count.0)
}

/// Mark a conversation as read by an agent
///
/// Adds the agent_id to the `read_by` array if not already present.
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `conversation_id` - ID of the conversation to mark read
/// * `agent_id` - The agent marking the message as read
///
/// # Returns
/// The updated conversation record
pub async fn mark_read(pool: &DbPool, conversation_id: i64, agent_id: &str) -> Result<Conversation> {
    let conversation = sqlx::query_as::<_, Conversation>(
        r#"
        UPDATE conversations
        SET read_by = array_append(read_by, $2)
        WHERE id = $1
          AND NOT ($2 = ANY(read_by))
        RETURNING id, thread_id, from_agent, to_agent, channel, message, message_type,
                  metadata, read_by, created_at
        "#,
    )
    .bind(conversation_id)
    .bind(agent_id)
    .fetch_one(pool)
    .await?;
    
    Ok(conversation)
}

/// Mark all messages in a thread as read by an agent
pub async fn mark_thread_read(pool: &DbPool, thread_id: Uuid, agent_id: &str) -> Result<u64> {
    let result: sqlx::postgres::PgQueryResult = sqlx::query(
        r#"
        UPDATE conversations
        SET read_by = array_append(read_by, $2)
        WHERE thread_id = $1
          AND $2 = ANY(to_agent)
          AND NOT ($2 = ANY(read_by))
        "#,
    )
    .bind(thread_id)
    .bind(agent_id)
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected())
}

// ============ Query Functions ============

/// Get a single conversation by ID
pub async fn get_by_id(pool: &DbPool, id: i64) -> Result<Conversation> {
    let conversation = sqlx::query_as::<_, Conversation>(
        r#"
        SELECT id, thread_id, from_agent, to_agent, channel, message, message_type,
               metadata, read_by, created_at
        FROM conversations
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    
    Ok(conversation)
}

/// Get recent conversations for an agent (sent or received)
pub async fn get_recent(pool: &DbPool, agent_id: &str, limit: i64) -> Result<Vec<Conversation>> {
    let conversations = sqlx::query_as::<_, Conversation>(
        r#"
        SELECT id, thread_id, from_agent, to_agent, channel, message, message_type,
               metadata, read_by, created_at
        FROM conversations
        WHERE from_agent = $1 OR $1 = ANY(to_agent)
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(agent_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(conversations)
}

/// Get conversations by channel type
pub async fn get_by_channel(pool: &DbPool, channel: &str, limit: i64) -> Result<Vec<Conversation>> {
    let conversations = sqlx::query_as::<_, Conversation>(
        r#"
        SELECT id, thread_id, from_agent, to_agent, channel, message, message_type,
               metadata, read_by, created_at
        FROM conversations
        WHERE channel = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(channel)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(conversations)
}

/// Get direct messages between two agents
pub async fn get_direct(
    pool: &DbPool,
    agent_a: &str,
    agent_b: &str,
    limit: i64,
) -> Result<Vec<Conversation>> {
    let conversations = sqlx::query_as::<_, Conversation>(
        r#"
        SELECT id, thread_id, from_agent, to_agent, channel, message, message_type,
               metadata, read_by, created_at
        FROM conversations
        WHERE channel = 'direct'
          AND ((from_agent = $1 AND $2 = ANY(to_agent))
               OR (from_agent = $2 AND $1 = ANY(to_agent)))
        ORDER BY created_at DESC
        LIMIT $3
        "#,
    )
    .bind(agent_a)
    .bind(agent_b)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(conversations)
}

/// List unique thread IDs for an agent (for thread listing)
pub async fn list_threads(pool: &DbPool, agent_id: &str, limit: i64) -> Result<Vec<Uuid>> {
    let rows: Vec<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT DISTINCT thread_id
        FROM conversations
        WHERE from_agent = $1 OR $1 = ANY(to_agent)
        ORDER BY thread_id
        LIMIT $2
        "#,
    )
    .bind(agent_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

/// Get the latest message in each thread for an agent
pub async fn get_thread_summaries(pool: &DbPool, agent_id: &str, limit: i64) -> Result<Vec<Conversation>> {
    let conversations = sqlx::query_as::<_, Conversation>(
        r#"
        SELECT DISTINCT ON (thread_id) 
               id, thread_id, from_agent, to_agent, channel, message, message_type,
               metadata, read_by, created_at
        FROM conversations
        WHERE from_agent = $1 OR $1 = ANY(to_agent)
        ORDER BY thread_id, created_at DESC
        LIMIT $2
        "#,
    )
    .bind(agent_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(conversations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_create() {
        let create = ConversationCreate {
            thread_id: None,
            from_agent: "eliana".to_string(),
            to_agent: vec!["devin".to_string()],
            channel: Some("direct".to_string()),
            message: "Hey Devin, ready for the code review?".to_string(),
            message_type: Some("chat".to_string()),
            metadata: None,
        };
        
        assert_eq!(create.from_agent, "eliana");
        assert_eq!(create.to_agent, vec!["devin"]);
        assert_eq!(create.message, "Hey Devin, ready for the code review?");
    }

    #[test]
    fn test_conversation_light() {
        let thread_id = Uuid::new_v4();
        let light = ConversationLight {
            id: 1,
            thread_id,
            from_agent: "nova".to_string(),
            to_agent: vec!["eliana".to_string()],
            channel: Some("direct".to_string()),
            message_type: Some("chat".to_string()),
            created_at: Utc::now(),
        };
        
        assert_eq!(light.from_agent, "nova");
        assert_eq!(light.thread_id, thread_id);
    }

    #[test]
    fn test_multiple_recipients() {
        let create = ConversationCreate {
            thread_id: Some(Uuid::new_v4()),
            from_agent: "eliana".to_string(),
            to_agent: vec![
                "devin".to_string(),
                "casey".to_string(),
                "morgan".to_string(),
            ],
            channel: Some("department".to_string()),
            message: "Team standup in 5 minutes!".to_string(),
            message_type: Some("system".to_string()),
            metadata: None,
        };
        
        assert_eq!(create.to_agent.len(), 3);
        assert_eq!(create.channel, Some("department".to_string()));
    }

    #[test]
    fn test_metadata_json() {
        let metadata = serde_json::json!({
            "priority": "high",
            "related_task": "ng-a6-dpncore-conversations",
            "attachments": []
        });
        
        let create = ConversationCreate {
            thread_id: None,
            from_agent: "eliana".to_string(),
            to_agent: vec!["devin".to_string()],
            channel: Some("direct".to_string()),
            message: "Check the task metadata".to_string(),
            message_type: Some("handoff".to_string()),
            metadata: Some(metadata.clone()),
        };
        
        assert!(create.metadata.is_some());
        let meta = create.metadata.unwrap();
        assert_eq!(meta["priority"], "high");
    }
}
