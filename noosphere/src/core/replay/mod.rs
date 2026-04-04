//! Day Replay module
//!
//! Generates narrative summaries from daily activities across all data sources.

pub mod collector;
pub mod narrative;
pub mod templates;

pub use collector::{DayData, DayEntry, EntrySource, collect_day};
pub use narrative::{generate_replay, ReplayOptions};
pub use templates::{NarrativeTemplate, TimeOfDay};
