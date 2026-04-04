//! Pattern Learning module
//!
//! Extracts recurring patterns from data and provides contextual suggestions.

pub mod extractor;
pub mod instincts;
pub mod suggestions;

pub use extractor::{Pattern, PatternType, extract_patterns};
pub use instincts::{Instinct, save_instinct, get_instincts};
pub use suggestions::{Suggestion, SuggestionContext, get_suggestions};
