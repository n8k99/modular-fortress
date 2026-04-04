//! Stagehand module
//!
//! High-level operations for show/venue note management.

pub mod notes;
pub mod recall;

pub use notes::{StagehandManager, import_from_ics};
