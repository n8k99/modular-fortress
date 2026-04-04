//! Local cache module for offline-first access
//!
//! Provides SQLite-based local caching with sync queue for eventual consistency
//! with the remote PostgreSQL database.
//!
//! Architecture:
//! - `sqlite` - Local SQLite connection and schema
//! - `sync_queue` - Queue changes for later sync to remote
//! - `hybrid` - HybridStore that reads local-first, writes through to remote

pub mod sqlite;
pub mod sync_queue;
pub mod hybrid;

pub use sqlite::{CachePool, init_cache, get_cache_path};
pub use sync_queue::{PendingChange, ChangeOperation, SyncQueue};
pub use hybrid::HybridStore;
