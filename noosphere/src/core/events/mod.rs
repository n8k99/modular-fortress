//! Event management and parsing module
//!
//! Handles event parsing from Weekly Notes and daily notes.

pub mod parser;
pub mod types;

pub use parser::{parse_events, parse_event_line, is_weekly_note};
pub use types::{ParsedEvent, EventType};
