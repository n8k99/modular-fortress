//! Smart context retrieval and injection
//!
//! Pull semantically related memories for context injection.

pub mod injection;
pub mod relevance;

pub use injection::{get_related_context, ContextResult, ContextSource};
pub use relevance::{RelevanceScorer, ScoredResult};
