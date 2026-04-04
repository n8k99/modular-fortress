//! Database module for dpn-core
//!
//! Provides PostgreSQL connectivity and models for:
//! - memories (primary memory storage, renamed from vault_notes)
//! - stagehand_notes (show/venue notes)
//! - tasks (task tracking and daily note insertion)
//! - events (calendar events and scheduling)
//! - projects (project management)
//! - documents (legacy, light queries only)

pub mod connection;
pub mod memories;
pub mod stagehand;
pub mod documents;
pub mod tasks;
pub mod events;
pub mod projects;
pub mod areas;
pub mod archives;
pub mod resources;
pub mod templates;

#[cfg(test)]
mod tests;

pub use connection::{DbPool, create_pool, test_connection};
