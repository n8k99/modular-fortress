//! Publish module for RSS-first publishing
//!
//! Provides:
//! - Drop management (publishable content CRUD)
//! - Stream handling (content streams like Myths of Orbis, Living Room Music, Thought Police)
//! - RSS/Atom feed generation
//! - Podcast RSS with enclosures
//! - Markdown → HTML content preparation
//!
//! This replaces external CMS dependencies (Ghost) with DB-as-source-of-truth.

pub mod types;
pub mod db;
pub mod feed;
pub mod content;
pub mod podcast;

pub use types::*;
pub use db::*;
pub use feed::*;
pub use content::*;
pub use podcast::*;
