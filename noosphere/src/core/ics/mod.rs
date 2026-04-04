//! ICS calendar parsing module
//!
//! Parse .ics feeds and extract event information for stagehand integration.

pub mod parser;

pub use parser::{IcsEvent, parse_ics, parse_ics_file};
