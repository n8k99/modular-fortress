//! Embedding generation and management
//!
//! Generate embeddings via Ollama (local) or OpenAI (remote).
//! Supports batch processing and auto-embedding on insert.

pub mod generator;
pub mod batch;
pub mod triggers;

pub use generator::{EmbeddingConfig, EmbeddingService, EmbeddingProvider, generate_embedding};
pub use batch::{backfill_embeddings, BackfillProgress, BackfillTarget};
pub use triggers::{write_log_with_embedding, write_memory_with_embedding, create_stagehand_note_with_embedding};
