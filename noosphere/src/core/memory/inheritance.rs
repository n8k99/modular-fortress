//! Cross-agent memory inheritance
//!
//! Allows agents to access memories from other agents
//! based on inheritance rules.

use anyhow::Result;
use std::collections::HashMap;

use crate::db::DbPool;
use super::store::{DailyLog, MemoryEntry};
use super::recall::{RecallResult, SemanticRecall};

/// Memory inheritance manager
pub struct MemoryInheritance {
    /// Inheritance rules: agent_id -> list of agent_ids they can read from
    rules: HashMap<String, Vec<String>>,
}

impl Default for MemoryInheritance {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryInheritance {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Allow an agent to inherit memories from another agent
    pub fn allow_inheritance(&mut self, agent_id: &str, from_agent_id: &str) {
        self.rules
            .entry(agent_id.to_string())
            .or_insert_with(Vec::new)
            .push(from_agent_id.to_string());
    }

    /// Remove inheritance permission
    pub fn revoke_inheritance(&mut self, agent_id: &str, from_agent_id: &str) {
        if let Some(sources) = self.rules.get_mut(agent_id) {
            sources.retain(|id| id != from_agent_id);
        }
    }

    /// Get list of agents that an agent can read memories from
    pub fn get_sources(&self, agent_id: &str) -> Vec<String> {
        self.rules.get(agent_id).cloned().unwrap_or_default()
    }

    /// Check if an agent can read from another agent
    pub fn can_read(&self, agent_id: &str, from_agent_id: &str) -> bool {
        self.rules
            .get(agent_id)
            .map(|sources| sources.iter().any(|s| s == from_agent_id))
            .unwrap_or(false)
    }

    /// Recall memories from allowed sources for an agent
    pub async fn recall_with_inheritance(
        &self,
        pool: &DbPool,
        query_embedding: &[f32],
        agent_id: &str,
        limit: i64,
    ) -> Result<Vec<RecallResult>> {
        let mut results = Vec::new();
        
        // Get own memories
        let own_results = SemanticRecall::recall(pool, query_embedding, agent_id, limit).await?;
        results.extend(own_results);
        
        // Get inherited memories
        let sources = self.get_sources(agent_id);
        for source_id in sources {
            let inherited = SemanticRecall::recall(pool, query_embedding, &source_id, limit).await?;
            results.extend(inherited);
        }
        
        // Sort by similarity and truncate
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit as usize);
        
        Ok(results)
    }

    /// Load inheritance rules from the database
    /// 
    /// Sets up default "all agents can read from shared memories" pattern
    pub async fn load_from_db(pool: &DbPool) -> Result<Self> {
        let mut inheritance = Self::new();
        
        let agents = super::store::list_agents(pool).await?;
        
        // Find a "shared" or "system" agent if one exists
        let shared_agent = agents.iter().find(|a| 
            a.id.to_lowercase() == "shared" || 
            a.id.to_lowercase() == "system" ||
            a.full_name.to_lowercase().contains("shared")
        );
        
        if let Some(shared) = shared_agent {
            // All agents can read from shared agent
            for agent in &agents {
                if agent.id != shared.id {
                    inheritance.allow_inheritance(&agent.id, &shared.id);
                }
            }
        }
        
        Ok(inheritance)
    }
}

/// Get all accessible daily logs for an agent (including inherited)
pub async fn get_accessible_logs(
    pool: &DbPool,
    inheritance: &MemoryInheritance,
    agent_id: &str,
    limit: i64,
) -> Result<Vec<DailyLog>> {
    let mut all_logs = Vec::new();
    
    // Own logs
    let own = super::store::get_logs_by_agent(pool, agent_id, limit).await?;
    all_logs.extend(own);
    
    // Inherited logs
    for source_id in inheritance.get_sources(agent_id) {
        let inherited = super::store::get_logs_by_agent(pool, &source_id, limit).await?;
        all_logs.extend(inherited);
    }
    
    // Sort by date descending and truncate
    all_logs.sort_by(|a, b| b.log_date.cmp(&a.log_date));
    all_logs.truncate(limit as usize);
    
    Ok(all_logs)
}

/// Get all accessible memories for an agent (including inherited)
pub async fn get_accessible_memories(
    pool: &DbPool,
    inheritance: &MemoryInheritance,
    agent_id: &str,
    limit: i64,
) -> Result<Vec<MemoryEntry>> {
    let mut all_memories = Vec::new();
    
    // Own memories
    let own = super::store::get_memories_by_agent(pool, agent_id, limit).await?;
    all_memories.extend(own);
    
    // Inherited memories
    for source_id in inheritance.get_sources(agent_id) {
        let inherited = super::store::get_memories_by_agent(pool, &source_id, limit).await?;
        all_memories.extend(inherited);
    }
    
    // Sort by importance and date
    all_memories.sort_by(|a, b| {
        match (b.importance, a.importance) {
            (Some(bi), Some(ai)) => bi.cmp(&ai),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => b.created_at.cmp(&a.created_at),
        }
    });
    all_memories.truncate(limit as usize);
    
    Ok(all_memories)
}

/// Statistics about an agent's memory access
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub agent_id: String,
    pub own_logs: i64,
    pub own_memories: i64,
    pub inherited_sources: usize,
    pub total_accessible_logs: i64,
    pub total_accessible_memories: i64,
}

/// Get memory statistics for an agent
pub async fn get_memory_stats(
    pool: &DbPool,
    inheritance: &MemoryInheritance,
    agent_id: &str,
) -> Result<MemoryStats> {
    // Count own records
    let own_logs: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM daily_logs WHERE agent_id = $1"
    )
    .bind(agent_id)
    .fetch_one(pool)
    .await?;
    
    let own_memories: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM memory_entries WHERE agent_id = $1"
    )
    .bind(agent_id)
    .fetch_one(pool)
    .await?;
    
    let sources = inheritance.get_sources(agent_id);
    
    // Count accessible records (own + inherited)
    let mut total_logs = own_logs.0;
    let mut total_memories = own_memories.0;
    
    for source_id in &sources {
        let source_logs: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM daily_logs WHERE agent_id = $1"
        )
        .bind(source_id)
        .fetch_one(pool)
        .await?;
        
        let source_memories: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM memory_entries WHERE agent_id = $1"
        )
        .bind(source_id)
        .fetch_one(pool)
        .await?;
        
        total_logs += source_logs.0;
        total_memories += source_memories.0;
    }
    
    Ok(MemoryStats {
        agent_id: agent_id.to_string(),
        own_logs: own_logs.0,
        own_memories: own_memories.0,
        inherited_sources: sources.len(),
        total_accessible_logs: total_logs,
        total_accessible_memories: total_memories,
    })
}
