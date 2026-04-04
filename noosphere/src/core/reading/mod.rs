//! RSS Reader module
//!
//! Provides RSS/Atom feed parsing, feed discovery, and reading comment functionality.
//! Comments are stored as virtual documents in the Thought Police directory.

pub mod db;
pub mod discovery;
pub mod types;

pub use db::*;
pub use discovery::*;
pub use types::*;
