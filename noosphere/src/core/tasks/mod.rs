//! Task management and parsing module
//!
//! Handles Obsidian Tasks format parsing, extraction, and serialization.

pub mod parser;
pub mod types;

pub use parser::{parse_document, parse_task_line, serialize_task};
pub use types::{Task, TaskStatus, TaskPriority};
