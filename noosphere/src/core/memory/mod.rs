//! Agent memory module
//!
//! Provides storage and semantic recall for agent memories.
//! Uses existing tables on droplet:
//! - daily_logs (agent daily entries with embeddings)
//! - memory_entries (structured memories)
//! - agents (agent registry)

pub mod store;
pub mod recall;
pub mod inheritance;

pub use store::{DailyLog, DailyLogCreate, MemoryEntry, MemoryEntryCreate, Agent};
pub use store::{write_log, write_memory, list_agents, get_agent, get_memories_by_date};
pub use recall::{RecallResult, SemanticRecall};
pub use inheritance::MemoryInheritance;
