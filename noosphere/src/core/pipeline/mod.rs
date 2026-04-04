//! Pipeline module for dpn-core
//!
//! Automated workflows for vault maintenance:
//! - tasks_due: Insert tasks due today into Daily Notes

pub mod tasks_due;

pub use tasks_due::{
    insert_tasks_due_into_daily_note,
    InsertionResult,
    InsertionMode,
};
