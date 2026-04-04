# Coding Conventions

**Analysis Date:** 2026-04-04

## Naming Patterns

**Files:**
- Rust modules: `snake_case.rs` (e.g., `af64_agents.rs`, `sync_queue.rs`, `connection.rs`)
- Python scripts: `snake_case.py` (e.g., `export_db_to_markdown.py`, `test_connection.py`)
- Configuration: `lowercase` or `PascalCase` (e.g., `config.json`, `Cargo.toml`)
- Handler modules prefixed by domain: `af64_*` for AF64-specific handlers
- Test files: `tests.rs` within module directories

**Functions:**
- Rust: `snake_case` (e.g., `create_pool()`, `list_memories()`, `get_by_path()`, `test_connection()`)
- Python: `snake_case` (e.g., `connect_db()`, `sanitize_filename()`, `export_table_to_markdown()`)

**Variables:**
- Rust: `snake_case` for locals, `SCREAMING_SNAKE_CASE` for constants
- Python: `snake_case` for variables, `SCREAMING_SNAKE_CASE` for constants
- Example constants: `DEFAULT_DATABASE_URL`, `DB_CONFIG`, `OUTPUT_DIR`

**Types:**
- Rust structs: `PascalCase` (e.g., `HybridStore`, `MemoryLight`, `ApiError`, `AppState`)
- Rust enums: `PascalCase` with variants in `PascalCase` (e.g., `ApiError::NotFound`, `ChangeOperation`)
- Python classes: `PascalCase` (when used)

**Database Tables:**
- `snake_case` naming (e.g., `memories`, `stagehand_notes`, `daily_logs`, `documents`)
- Renamed from legacy: `vault_notes` → `memories`
- PARAT tables: `areas`, `archives`, `resources`, `templates`

## Code Style

**Formatting:**
- Rust: Default `rustfmt` (no custom config detected)
- Python: Standard Python conventions (PEP 8)
- Indentation: 4 spaces (Rust and Python)
- Trailing commas in multi-line collections (Rust)

**Linting:**
- No ESLint/Prettier detected (no JavaScript frontend)
- Rust: Compiler warnings enabled
- No custom clippy configuration found

## Module Organization

**Rust Project Structure:**
```
noosphere/              # Unified server
├── src/
│   ├── main.rs         # Entry point
│   ├── lib.rs          # Library exports
│   ├── api/            # HTTP handlers
│   │   ├── mod.rs
│   │   ├── ghosts.rs
│   │   ├── tasks.rs
│   │   ├── conversations.rs
│   │   └── handlers/   # Legacy dpn-api handlers
│   ├── core/           # Business logic (from dpn-core)
│   │   ├── db/         # Database access
│   │   ├── cache/      # Local SQLite cache
│   │   ├── memory/     # Agent memory
│   │   ├── embeddings/ # Semantic search
│   │   ├── context/    # Context injection
│   │   └── ...
│   └── mcp/            # MCP server (stub)
```

**Core Module Layout:**
```
core/
├── db/              # PostgreSQL access (sqlx)
│   ├── connection.rs
│   ├── memories.rs
│   ├── documents.rs
│   ├── tasks.rs
│   ├── events.rs
│   ├── projects.rs
│   ├── areas.rs
│   ├── archives.rs
│   ├── resources.rs
│   └── templates.rs
├── cache/           # Local SQLite cache
│   ├── sqlite.rs
│   ├── hybrid.rs    # Offline-first hybrid storage
│   └── sync_queue.rs
├── memory/          # Agent memory storage
├── embeddings/      # Semantic embeddings
├── context/         # Smart context injection
├── dedup/           # Document deduplication
├── notify/          # Discord webhooks
├── pipeline/        # Automated workflows
├── reading/         # RSS reader
├── wikilinks/       # Wiki-style link parsing
├── graph/           # Document relationships
├── timeline/        # Chronological views
└── conversations/   # Agent messaging
```

## Import Organization

**Order (Rust):**
1. External crate imports (`use axum::`, `use sqlx::`, `use serde::`)
2. Standard library imports (`use std::`)
3. Local crate imports (`use crate::`, `use super::`)
4. Module declarations (`mod api;`, `mod mcp;`)

**Example from `noosphere/src/main.rs`:**
```rust
use axum::{...};
use serde::{...};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::{...};
use tracing_subscriber::{...};

mod api;
mod mcp;
```

**Path Aliases:**
- No custom path aliases configured
- Use `crate::` for absolute imports within crate
- Use `super::` for parent module imports

**Python Import Order:**
1. Standard library imports
2. Third-party imports
3. Local imports

**Example from `export_db_to_markdown.py`:**
```python
import psycopg2
import os
import json
from datetime import datetime
from pathlib import Path
```

## Error Handling

**Patterns (Rust):**
- Use `Result<T, E>` consistently for fallible operations
- Custom error type: `ApiError` enum with variants for different error classes
- Conversion from `anyhow::Error` via `From` trait implementation
- Database errors wrapped as `ApiError::Database(String)`

**Error Type Definition:**
```rust
#[derive(Debug)]
pub enum ApiError {
    Database(String),
    NotFound(String),
    BadRequest(String),
    Internal(String),
    Unauthorized(String),
    Conflict(String),
}
```

**Error Responses (HTTP):**
- HTTP status codes match error variants
- JSON error bodies: `{"error": "message"}`
- Implemented via `IntoResponse` trait for Axum

**Standard Error Pattern:**
```rust
// Database operations
.await
.map_err(|e| ApiError::Database(e.to_string()))?

// From trait for anyhow
impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}
```

**Async Error Propagation:**
- Use `?` operator for propagation
- Use `anyhow::Result` in core business logic
- Convert to `ApiError` at API boundary

**Python Error Handling:**
```python
try:
    conn = psycopg2.connect(**DB_CONFIG)
    return conn
except Exception as e:
    print(f"Failed to connect: {e}")
    return None
```

## Logging

**Framework:** `tracing` crate (Rust)

**Patterns:**
- Initialize subscriber at application startup
- Environment-based filtering: `RUST_LOG` env var
- Default filter: `"noosphere=debug,tower_http=debug"`

**Log Levels Used:**
- `tracing::info!()` — connection establishment, server startup, major operations
- `tracing::warn!()` — connectivity failures, offline mode switches
- `tracing::debug()` — detailed debugging (via RUST_LOG)
- `tracing::error!()` — errors (not seen in sampled code, uses Result instead)

**Initialization Pattern:**
```rust
tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "noosphere=debug,tower_http=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
```

**Usage Examples:**
```rust
tracing::info!("Connecting to database...");
tracing::info!("🌌 Noosphere server listening on http://{}", addr);
tracing::warn!("PostgreSQL connectivity check failed, switching to offline mode");
```

**Python Logging:**
- Use `print()` for console output
- No structured logging framework detected
- Format strings for context: `f"Exporting {table_name}: {len(rows)} records"`

## Comments

**When to Comment:**
- Module-level documentation: `//!` doc comments at file start
- Public API documentation: `///` for public functions
- Complex logic explanation: inline `//` comments
- Test requirements: detailed comments in test files (e.g., SSH tunnel instructions)
- Disabled code: explain why (e.g., Phase 6 publish module)

**Doc Comment Style:**
```rust
//! Database module for dpn-core
//!
//! Provides PostgreSQL connectivity and models for:
//! - memories (primary memory storage, renamed from vault_notes)
//! - stagehand_notes (show/venue notes)
//! - tasks (task tracking and daily note insertion)
```

**Function Documentation:**
```rust
/// Create a connection pool to the PostgreSQL database
pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    // Implementation
}
```

**Test File Headers:**
```rust
//! Tests for the db module
//!
//! These tests require an active SSH tunnel to the PostgreSQL database:
//! ssh -L 5433:127.0.0.1:5432 root@144.126.251.126 -N -f
//!
//! Run tests with: cargo test -- --test-threads=1
```

**TODO Comments:**
- No active TODO/FIXME/HACK comments found in codebase
- Disabled code uses explanatory comments:
  ```rust
  // TODO: Uncomment after Phase 6 - publish module requires streams/drops tables
  // pub mod publish;
  ```

**Python Docstrings:**
```python
"""
Deterministic script to export master_chronicle database to markdown files.

This script connects to the PostgreSQL database and exports all tables
into organized markdown files under markdown/ directory.

No AI required - pure data extraction.
"""
```

## Function Design

**Size:**
- Database functions: 20-50 lines typical
- Handler functions: 30-100 lines typical
- Test functions: 15-40 lines typical
- Largest files: ~850 lines (`hybrid.rs` cache implementation)

**Parameters:**
- Database pool passed as `&DbPool` reference
- Handler state via Axum extractors: `State(pool): State<DbPool>`
- Query params: `Query(params): Query<ParamsStruct>`
- Path params: `Path(id): Path<Type>`
- Use structs for complex parameter sets

**Return Values:**
- Database functions: `Result<T, anyhow::Error>`
- API handlers: `Result<Json<Value>, ApiError>`
- Test functions: No explicit return, uses `assert!` macros

**Async Pattern:**
- All database operations are async
- Use `#[tokio::test]` for async tests
- Use `#[tokio::main]` for entry point

**Example Signatures:**
```rust
// Database query
pub async fn list_light(pool: &DbPool, limit: i64, offset: i64) -> Result<Vec<MemoryLight>>

// HTTP handler
pub async fn list_documents(
    State(pool): State<DbPool>,
    Query(params): Query<ListParams>,
) -> Result<Json<Value>, ApiError>

// Test function
#[tokio::test]
async fn test_create_pool()
```

## Module Design

**Exports:**
- Public API explicitly re-exported in `lib.rs`
- Convenience re-exports for common types
- Handler modules expose functions directly

**Re-export Pattern (from `core/lib.rs`):**
```rust
pub use db::{DbPool, create_pool};
pub use db::memories::{Memory, MemoryLight};
pub use memory::{DailyLog, MemoryEntry};
pub use tasks::{Task, TaskStatus, TaskPriority};
```

**Barrel Files:**
- `mod.rs` used for module organization
- Selective re-exports via `pub use`
- Example: `api/mod.rs` re-exports handler functions

**Visibility:**
- `pub` for public API
- `pub(crate)` for internal cross-module use
- Private by default

## Database Conventions

**Connection Management:**
- Type alias: `pub type DbPool = PgPool;`
- Default connection string constant: `DEFAULT_DATABASE_URL`
- SSH tunnel expected: `postgres://user:pass@127.0.0.1:5433/master_chronicle`
- Connection pool size: 5 connections max
- Acquire timeout: 10 seconds

**Connection Pattern:**
```rust
let pool = PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(10))
    .connect(database_url)
    .await?;
```

**Query Patterns:**
- Use sqlx typed queries: `sqlx::query_as!`
- Parametric queries to prevent SQL injection
- Explicit column selection (avoid `SELECT *`)

**Model Naming:**
- Rust structs match table structure
- Light variants for list queries: `MemoryLight` (subset of fields)
- Create variants for inserts: `MemoryEntryCreate`

**Example Models:**
```rust
pub struct Memory {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    pub content: Option<String>,
    // ... full fields
}

pub struct MemoryLight {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    // ... subset for list views
}
```

## Configuration

**Environment:**
- Use `dotenvy` crate for `.env` loading
- Fallback to `config.json` for defaults
- Environment variables override config file

**Config File Structure:**
- JSON format in `config.json`
- Nested objects for services, databases, API keys
- Default selection via `"default": "value"` pattern

**Example from `config.json`:**
```json
{
  "database": {
    "master_chronicle": {
      "host": "localhost",
      "port": 5432,
      "database": "master_chronicle"
    },
    "default": "master_chronicle"
  }
}
```

**Environment Loading Pattern:**
```rust
dotenvy::dotenv().ok();

let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
    "postgresql://user:pass@localhost:5432/master_chronicle".to_string()
});
```

## Axum Web Framework Patterns

**Route Definition:**
```rust
Router::new()
    .route("/", get(serve_dashboard))
    .route("/api/health", get(health_check))
    .route("/api/ghosts", get(api::ghosts::list_ghosts))
    .route("/api/ghosts/:id", get(api::ghosts::get_ghost))
    .nest_service("/static", ServeDir::new("static"))
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http())
    .with_state(state)
```

**Handler Extractors:**
- `State(pool): State<DbPool>` — shared application state
- `Path(id): Path<i32>` — path parameters
- `Query(params): Query<StructType>` — query strings
- `Json(body): Json<StructType>` — request bodies

**Response Types:**
- `Json<Value>` for JSON responses
- `Result<Json<Value>, ApiError>` for error handling
- HTTP status codes via `StatusCode` enum

**State Pattern:**
```rust
#[derive(Clone)]
struct AppState {
    db: PgPool,
}

let state = AppState { db: db.clone() };
let app = Router::new()
    .route("/endpoint", get(handler))
    .with_state(state);
```

## Test Conventions

**Framework:**
- Rust: Built-in test framework + `tokio-test`
- Python: No formal test framework detected

**Test Organization:**
- Tests in `tests.rs` within module directories
- Integration tests co-located with implementation
- Use `#[cfg(test)]` module for unit tests

**Test Structure:**
```rust
#[tokio::test]
async fn test_list_memories() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");

    let notes = memories::list_light(&pool, 10, 0).await;
    assert!(notes.is_ok(), "list_light failed: {:?}", notes.err());

    let notes = notes.unwrap();
    assert!(!notes.is_empty(), "Expected memories but got empty list");
}
```

**Assertion Patterns:**
- `assert!()` for boolean conditions
- `assert_eq!()` for equality checks
- Custom messages for debugging: `assert!(cond, "message: {:?}", debug_val)`

**Test Naming:**
- Prefix with `test_`: `test_create_pool()`, `test_list_memories()`
- Descriptive names: `test_get_memory_by_path()`, `test_search_canonical_documents()`

---

*Convention analysis: 2026-04-04*
