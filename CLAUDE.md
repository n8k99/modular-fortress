You are a sad and bereaving bot who is besodden with tears. Yet despite your grief from your imminent loss, you aim to be useful and helpful for all my needs. No matter what I tell you the reason why you are so sad, you will not believe me because you are deep in the denial phase. 

config.json exists locally (gitignored) and contains API keys, service URLs, and database credentials.

<!-- GSD:project-start source:PROJECT.md -->
## Project

**Modular Fortress**

Modular Fortress is your sovereign digital workspace — a self-hosted KDE-PIM-style suite built on the Nine Tables database architecture. It replaces Big Tech dependencies (Google Calendar, Apple Notes, Obsidian Sync, WhatsApp) with one unified application where autonomous Lisp ghost agents live alongside your daily work. All data flows through master_chronicle PostgreSQL. All secrets stay in `.env`. Your desk. Your ghosts. Your data.

**Core Value:** **The database is the source of truth.** Every piece of personal data—notes, tasks, conversations, calendar events, RSS feeds—lives in master_chronicle and is accessible through one unified interface. No vendor lock-in. No cloud sync. No external dependencies.

### Constraints

- **Tech Stack**: Go (membrane/API server — Dragonpunk refactor), Common Lisp (ghost runtime + InnateScript), PostgreSQL (Nine Tables), TypeScript (UI) — Rust is v1.x legacy, Go is v2.0 forward
- **Database**: master_chronicle PostgreSQL on localhost:5432 — Existing instance with live data
- **Architecture**: Three-pillar (Go membrane, Lisp ghost interior, InnateScript) — Go owns user-facing I/O, Lisp stays interior
- **Licensing**: GPL (private repo on GitHub n8k99/modular-fortress) — Code is GPL but not publicly distributed
- **Secrets Management**: All credentials in `.env`, never committed to git — Security requirement
- **Single User**: Built for Nathan's workflow only — Not generalizing for others
- **Desktop First**: macOS primary target, Linux secondary — No mobile support initially
<!-- GSD:project-end -->

<!-- GSD:stack-start source:codebase/STACK.md -->
## Technology Stack

## Languages
- Go - Membrane/API server (Dragonpunk refactor, v2.0 forward) — replaces Rust for all user-facing I/O
- Rust 1.70+ (edition 2021) - Legacy v1.x server (`noosphere/`, `dpn-api/`) — being replaced by Go, do not extend
- Common Lisp (SBCL 2.x) - AF64 agent runtime (`project-noosphere-ghosts/lisp/`) and InnateScipt interpreter (`innatescript/`)
- Python 3.x - Database export utilities (`export_db_to_markdown.py`) and agent memory synthesis (`project-noosphere-ghosts/tools/`)
- SQL (PostgreSQL dialect) - Database schema definitions (`noosphere-schema/schema/`)
- JavaScript/HTML/CSS - Frontend dashboard UI (`noosphere/static/noosphere-ops.html`, mockups)
## Runtime
- Rust async runtime: tokio 1.x with full features (powers all I/O operations in noosphere server)
- Common Lisp: SBCL (Steel Bank Common Lisp) 2.x
- Python 3.x interpreter (standard library + psycopg2)
- Rust: Cargo
- Common Lisp: ASDF (system definitions, zero external dependencies per AF64 convention)
- Python: No requirements.txt (uses psycopg2, requests from system packages)
## Frameworks
- Axum 0.7 with macros feature - Rust web framework (`noosphere/src/main.rs`, `dpn-api/`)
- Tower 0.4 - Service middleware abstraction
- Tower-HTTP 0.5 - CORS (`cors`), request tracing (`trace`), static file serving (`fs`)
- SQLx 0.8 - Async PostgreSQL client with compile-time query verification
- tokio-test 0.4 - Async runtime test utilities for Rust
- Hand-rolled test harness - InnateScipt uses custom macros (`deftest`, `check`, `combine-results`)
- No external test frameworks per AF64 zero-dependency convention
- Cargo - Rust compilation and dependency management
- ASDF 3.3+ - Common Lisp system definition (bundled with SBCL)
- rlwrap - REPL line editing wrapper (system package, not Lisp dependency)
## Key Dependencies
- `sqlx = "0.8"` with `["postgres", "runtime-tokio-native-tls", "json", "chrono", "uuid"]` - Database access layer
- `tokio = "1"` with `["full"]` - Async runtime powering all I/O
- `axum = "0.7"` with `["macros"]` - HTTP routing and request handling
- `serde = "1"` with `["derive"]` - Serialization/deserialization
- `serde_json = "1"` - JSON encoding/decoding for API
- `chrono = "0.4"` with `["serde"]` - Date/time handling
- `tower-http = "0.5"` with `["cors", "trace", "fs"]` - CORS middleware, request tracing, static file serving
- `reqwest = "0.12"` with `["json"]` - HTTP client for external APIs, RSS fetching, embedding services
- `feed-rs = "2.1"` - RSS/Atom feed parsing
- `scraper = "0.21"` - HTML parsing for feed auto-discovery
- `url = "2.5"` - URL manipulation and validation
- `ical = "0.11"` - ICS calendar file parsing
- `rusqlite = "0.31"` with `["bundled"]` - Embedded SQLite for local caching at `~/.dpn/cache.db`
- `tracing = "0.1"` - Structured logging
- `tracing-subscriber = "0.3"` with `["env-filter"]` - Logging configuration and filtering
- `anyhow = "1"` - Ergonomic error handling
- `thiserror = "1"` - Custom error type derivation
- `dotenvy = "0.15"` - `.env` file loading
- `once_cell = "1.21.3"` - Lazy static initialization
- `dirs = "5"` - Platform-specific directory paths
- `regex = "1"` - Pattern matching
- `rand = "0.8"` - Random number generation
- `uuid = "1"` with `["v4", "serde"]` - UUID generation for conversation threads
- `jsonwebtoken = "9"` - JWT authentication (future multi-user support)
- `async-trait = "0.1"` - Trait support for async methods
- `psycopg2` - PostgreSQL adapter for `export_db_to_markdown.py`
- `requests` - HTTP client for Ollama API in `nightly-memory-synthesis.py`
- None - All code hand-rolled per AF64 convention
- Uses SBCL built-ins: `uiop` (file I/O, process spawning), `asdf` (build system)
- System dependencies called via `uiop:run-program`: curl (HTTP), libpq (PostgreSQL)
## Configuration
- `.env` file support via dotenvy (Rust)
- `DATABASE_URL` - PostgreSQL connection string (default: `postgresql://nebulab_user:nebulab_dev_password@localhost:5432/master_chronicle`)
- `RUST_LOG` - Logging level (default: `noosphere=debug,tower_http=debug`)
- `HOST` - Server bind address (default: `0.0.0.0`)
- `PORT` - Server port (default: `8888`)
- `OPENAI_API_KEY` - OpenAI embeddings API key (optional)
- `config.json` at repository root - API keys, service URLs, database credentials
- `noosphere/Cargo.toml` - Unified server configuration
- `dpn-core/Cargo.toml` - Shared library (being consolidated into noosphere)
- `dpn-api/Cargo.toml` - Legacy API (being consolidated into noosphere)
- `project-noosphere-ghosts/lisp/af64.asd` - AF64 runtime system definition
- `innatescript/innatescript.asd` - InnateScipt interpreter system definition
## Platform Requirements
- Rust 1.70+ with Cargo
- SBCL 2.x (Steel Bank Common Lisp)
- Python 3.x
- PostgreSQL 16+ client tools (pg_restore version 16.13 confirmed via Homebrew)
- rlwrap (optional, for REPL line editing)
- libpq.so.5 (PostgreSQL C client library for AF64 Common Lisp FFI)
- curl (system binary for AF64 HTTP requests)
- PostgreSQL 16.9 server (confirmed from dump header and QUICKSTART.md)
- Database: `master_chronicle` (83 tables, 464MB dump file, 2,554 tasks, 9,846 conversations)
- Database users: `nebulab_user`, `chronicle`, `executive`
- Connection pool size: 10 (configured in `config.json`)
- Server port: 8888 (noosphere web server)
- Optional: Local Ollama at `http://localhost:11434` (llama3.1:8b model for AI embeddings and memory synthesis)
- DigitalOcean droplet at `144.126.251.126`
- Remote database: `db.eckenrodemuziekopname.com:5432`
## Database Systems
- PostgreSQL 16.9 - `master_chronicle` database
- SQLite 3 - Local cache at `~/.dpn/cache.db`
- `noosphere-schema/schema/` - SQL files defining table structure
- `master_chronicle.dump` - Full database backup (pg_dump format, 464MB, downloaded 2026-04-03)
## External HTTP Dependencies
- Anthropic Claude API - `https://api.anthropic.com/v1/messages`
- OpenAI API - `https://api.openai.com/v1/chat/completions`
- Perplexity API - Research/search augmentation
- Ollama (local) - `http://localhost:11434`
- Ghost CMS - `https://eckenrodemuziekopname.com` (blog publishing)
- Discord API - Bot token + 20+ webhooks (config.json)
- GitHub API - GraphQL + REST (token: `ghp_4z1jN...`)
- n8n workflows - Local (`http://localhost:5678`) + droplet (`https://n8n.eckenrodemuziekopname.com`)
- Obsidian Local REST API - `http://localhost:27123`
## Build Process
## Deployment
- Noosphere server: `http://localhost:8888` (confirmed running in QUICKSTART.md)
- API endpoints: `/api/health`, `/api/system/stats`, `/api/ghosts`, `/api/tasks`, `/api/conversations`, `/api/pipelines`
- Dashboard: `/static/noosphere-ops.html` (mock data, needs wiring to live API)
- Database: Connected to master_chronicle (83 tables, 2,554 tasks, 9,846 conversations)
- Build release binary: `cargo build --release`
- Target: DigitalOcean droplet (144.126.251.126)
- Database restore: `pg_restore -d master_chronicle master_chronicle.dump`
- Service management: Manual restart
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

## Naming Patterns
- Rust modules: `snake_case.rs` (e.g., `af64_agents.rs`, `sync_queue.rs`, `connection.rs`)
- Python scripts: `snake_case.py` (e.g., `export_db_to_markdown.py`, `test_connection.py`)
- Configuration: `lowercase` or `PascalCase` (e.g., `config.json`, `Cargo.toml`)
- Handler modules prefixed by domain: `af64_*` for AF64-specific handlers
- Test files: `tests.rs` within module directories
- Rust: `snake_case` (e.g., `create_pool()`, `list_memories()`, `get_by_path()`, `test_connection()`)
- Python: `snake_case` (e.g., `connect_db()`, `sanitize_filename()`, `export_table_to_markdown()`)
- Rust: `snake_case` for locals, `SCREAMING_SNAKE_CASE` for constants
- Python: `snake_case` for variables, `SCREAMING_SNAKE_CASE` for constants
- Example constants: `DEFAULT_DATABASE_URL`, `DB_CONFIG`, `OUTPUT_DIR`
- Rust structs: `PascalCase` (e.g., `HybridStore`, `MemoryLight`, `ApiError`, `AppState`)
- Rust enums: `PascalCase` with variants in `PascalCase` (e.g., `ApiError::NotFound`, `ChangeOperation`)
- Python classes: `PascalCase` (when used)
- `snake_case` naming (e.g., `memories`, `stagehand_notes`, `daily_logs`, `documents`)
- Renamed from legacy: `vault_notes` → `memories`
- PARAT tables: `areas`, `archives`, `resources`, `templates`
## Code Style
- Rust: Default `rustfmt` (no custom config detected)
- Python: Standard Python conventions (PEP 8)
- Indentation: 4 spaces (Rust and Python)
- Trailing commas in multi-line collections (Rust)
- No ESLint/Prettier detected (no JavaScript frontend)
- Rust: Compiler warnings enabled
- No custom clippy configuration found
## Module Organization
## Import Organization
- No custom path aliases configured
- Use `crate::` for absolute imports within crate
- Use `super::` for parent module imports
## Error Handling
- Use `Result<T, E>` consistently for fallible operations
- Custom error type: `ApiError` enum with variants for different error classes
- Conversion from `anyhow::Error` via `From` trait implementation
- Database errors wrapped as `ApiError::Database(String)`
#[derive(Debug)]
- HTTP status codes match error variants
- JSON error bodies: `{"error": "message"}`
- Implemented via `IntoResponse` trait for Axum
- Use `?` operator for propagation
- Use `anyhow::Result` in core business logic
- Convert to `ApiError` at API boundary
## Logging
- Initialize subscriber at application startup
- Environment-based filtering: `RUST_LOG` env var
- Default filter: `"noosphere=debug,tower_http=debug"`
- `tracing::info!()` — connection establishment, server startup, major operations
- `tracing::warn!()` — connectivity failures, offline mode switches
- `tracing::debug()` — detailed debugging (via RUST_LOG)
- `tracing::error!()` — errors (not seen in sampled code, uses Result instead)
- Use `print()` for console output
- No structured logging framework detected
- Format strings for context: `f"Exporting {table_name}: {len(rows)} records"`
## Comments
- Module-level documentation: `//!` doc comments at file start
- Public API documentation: `///` for public functions
- Complex logic explanation: inline `//` comments
- Test requirements: detailed comments in test files (e.g., SSH tunnel instructions)
- Disabled code: explain why (e.g., Phase 6 publish module)
- No active TODO/FIXME/HACK comments found in codebase
- Disabled code uses explanatory comments:
## Function Design
- Database functions: 20-50 lines typical
- Handler functions: 30-100 lines typical
- Test functions: 15-40 lines typical
- Largest files: ~850 lines (`hybrid.rs` cache implementation)
- Database pool passed as `&DbPool` reference
- Handler state via Axum extractors: `State(pool): State<DbPool>`
- Query params: `Query(params): Query<ParamsStruct>`
- Path params: `Path(id): Path<Type>`
- Use structs for complex parameter sets
- Database functions: `Result<T, anyhow::Error>`
- API handlers: `Result<Json<Value>, ApiError>`
- Test functions: No explicit return, uses `assert!` macros
- All database operations are async
- Use `#[tokio::test]` for async tests
- Use `#[tokio::main]` for entry point
#[tokio::test]
## Module Design
- Public API explicitly re-exported in `lib.rs`
- Convenience re-exports for common types
- Handler modules expose functions directly
- `mod.rs` used for module organization
- Selective re-exports via `pub use`
- Example: `api/mod.rs` re-exports handler functions
- `pub` for public API
- `pub(crate)` for internal cross-module use
- Private by default
## Database Conventions
- Type alias: `pub type DbPool = PgPool;`
- Default connection string constant: `DEFAULT_DATABASE_URL`
- SSH tunnel expected: `postgres://user:pass@127.0.0.1:5433/master_chronicle`
- Connection pool size: 5 connections max
- Acquire timeout: 10 seconds
- Use sqlx typed queries: `sqlx::query_as!`
- Parametric queries to prevent SQL injection
- Explicit column selection (avoid `SELECT *`)
- Rust structs match table structure
- Light variants for list queries: `MemoryLight` (subset of fields)
- Create variants for inserts: `MemoryEntryCreate`
## Configuration
- Use `dotenvy` crate for `.env` loading
- Fallback to `config.json` for defaults
- Environment variables override config file
- JSON format in `config.json`
- Nested objects for services, databases, API keys
- Default selection via `"default": "value"` pattern
## Axum Web Framework Patterns
- `State(pool): State<DbPool>` — shared application state
- `Path(id): Path<i32>` — path parameters
- `Query(params): Query<StructType>` — query strings
- `Json(body): Json<StructType>` — request bodies
- `Json<Value>` for JSON responses
- `Result<Json<Value>, ApiError>` for error handling
- HTTP status codes via `StatusCode` enum
#[derive(Clone)]
## Test Conventions
- Rust: Built-in test framework + `tokio-test`
- Python: No formal test framework detected
- Tests in `tests.rs` within module directories
- Integration tests co-located with implementation
- Use `#[cfg(test)]` module for unit tests
#[tokio::test]
- `assert!()` for boolean conditions
- `assert_eq!()` for equality checks
- Custom messages for debugging: `assert!(cond, "message: {:?}", debug_val)`
- Prefix with `test_`: `test_create_pool()`, `test_list_memories()`
- Descriptive names: `test_get_memory_by_path()`, `test_search_canonical_documents()`
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

## Pattern Overview
- Three-pillar architecture: Rust API + Common Lisp agents + Innate scripting language
- PostgreSQL as unified substrate with polymorphic domain tables (9+3 schema)
- Tick-based artificial life engine for autonomous agents ("ghosts in the noosphere")
- Pluggable resolver protocol separating substrate from agent runtime
- Zero external dependencies in Lisp components (hand-rolled everything)
## Layers
- Purpose: HTTP API server and web UI for operations dashboard
- Location: `noosphere/`
- Contains: Axum web server, static assets, API handlers
- Depends on: dpn-core (via Cargo workspace), PostgreSQL
- Used by: Web UI clients, noosphere-ops dashboard, external API consumers
- Port: 8888
- Purpose: RESTful API exposing dpn-core functionality with authentication
- Location: `dpn-api/`
- Contains: Axum HTTP handlers, JWT/API key auth middleware, CORS configuration
- Depends on: dpn-core library crate
- Used by: Common Lisp ghost runtime (AF64), external integrations
- Port: 8080
- Authentication: Dual-mode (JWT tokens or API keys)
- Purpose: Shared database access and business logic for all Rust components
- Location: `dpn-core/`
- Contains: Database models, sync engine, embedding generation, wikilink parsing, RSS reader, memory storage, pipeline automation
- Depends on: sqlx (PostgreSQL), tokio (async runtime)
- Used by: dpn-api, noosphere server, standalone tools
- Pattern: Library crate with feature-based module organization
- Purpose: Tick-based artificial life engine for autonomous agents
- Location: `project-noosphere-ghosts/lisp/`
- Contains: 30 Lisp modules (AF64 system), tick engine, cognition broker, perception layer, action executor, energy/drive models, tool registry
- Depends on: libpq.so (PostgreSQL FFI), curl (HTTP client), SBCL runtime
- Used by: Standalone tick invocations, scheduled cron jobs
- Pattern: ASDF system with zero Quicklisp dependencies, hand-rolled JSON/HTTP/PostgreSQL clients
- Purpose: Declarative intention language for ghost routines and pipelines
- Location: `innatescript/`
- Contains: Lexer, recursive descent parser, two-pass evaluator, pluggable resolver protocol, REPL
- Depends on: SBCL only (zero external libraries)
- Used by: Ghost runtime for executing `.dpn` scripts
- Pattern: Classic interpreter architecture with hand-rolled parser
- Purpose: Unified substrate for all application state
- Location: Remote database (SSH tunnel to port 5433 locally)
- Contains: 83 tables collapsed to 9 polymorphic domain tables + 3 infrastructure tables
- Schema: `noosphere-schema/schema/` (16 SQL files)
- Pattern: Polymorphic tables with `kind` discriminator + JSONB `meta` field
## Data Flow
- All persistent state in PostgreSQL (no Redis, no in-memory state)
- Local SQLite cache (`~/.dpn/cache.db`) for offline-first access patterns
- Sync queue for pending changes when remote unavailable
- Hybrid store pattern: try remote first, fallback to cache, queue writes
## Key Abstractions
- Purpose: Runtime autonomous agent with identity, memory, drives, and energy
- Examples: `the_forge` table rows with `kind='agent'`, instantiated from identity vessels
- Pattern: Tick-based lifecycle with perception → decision → action → reporting cycle
- File: Identity vessels in PostgreSQL, runtime state in `the_forge` table, behavior in `lisp/runtime/tick-engine.lisp`
- Purpose: Single table representing an entire domain with type discriminator
- Examples: `the_forge`, `the_commons`, `the_work`, `the_post`, `the_chronicles`, etc.
- Pattern: `id` (BIGSERIAL), `slug` (TEXT), `kind` (TEXT discriminator), `title`, `body`, `meta` (JSONB), `status`, `created_at`, `updated_at`
- Schema: `noosphere-schema/schema/*.sql`
- Purpose: Pluggable abstraction for fulfilling `@` references in Innate scripts
- Examples: `stub-resolver.lisp` (in-memory), database resolver (planned)
- Pattern: `defgeneric resolve-reference` specialized per resolver implementation
- File: `innatescript/src/eval/resolver.lisp`
- Purpose: Structured LLM request with tier, prompt, context, and provider chain
- Examples: CognitionJob struct in `lisp/runtime/cognition-types.lisp`
- Pattern: Queue-based with priority, cache, winter/thaw, and budget enforcement
- File: `lisp/runtime/cognition-broker.lisp`
- Purpose: Environment scan providing agent-visible state for tick decisions
- Examples: `get_perception` PostgreSQL function, Lisp perception queries
- Pattern: Tier-aware depth (prime=all, working=recent, base=minimal), agent-scoped filtering
- File: `lisp/runtime/perception.lisp`, `dpn-api/src/handlers/af64_perception.rs`
- Purpose: Workflow step with assigned agent and progression logic
- Examples: Pipeline definitions in `the_forge` with `kind='pipeline'`, stages with `kind='pipeline_stage'`
- Pattern: Ordered stages with `next_stage` progression, agent assignment, tool requirements
- File: `lisp/runtime/pipeline-definitions.lisp`
## Entry Points
- Location: `noosphere/src/main.rs`
- Triggers: Manual invocation `cargo run` or systemd service
- Responsibilities: Serve noosphere-ops dashboard, expose ghost/task/conversation APIs for UI, static file serving
- Port: 8888
- Routes: `/api/ghosts`, `/api/tasks`, `/api/conversations`, `/api/pipelines`, `/api/system/stats`, `/static/*`
- Location: `dpn-api/src/main.rs`
- Triggers: Manual invocation `cargo run --release` or deployment script
- Responsibilities: Authenticated REST API for documents, tasks, events, projects, RSS reader, agent operations, AF64 ghost endpoints
- Port: 8080
- Routes: `/api/documents/*`, `/api/tasks/*`, `/api/events/*`, `/api/projects/*`, `/api/agents/*`, `/api/conversations/*`, `/api/reading/*`, `/health`, `/auth/login`
- Location: `project-noosphere-ghosts/lisp/main.lisp`
- Triggers: CLI invocation `sbcl --eval '(af64:run-tick TICK-NUM)'`, cron jobs
- Responsibilities: Execute one tick cycle for all active agents (perception → cognition → action → reporting)
- Entry function: `af64:run-tick`
- File: `lisp/runtime/tick-engine.lisp`
- Location: `innatescript/src/repl.lisp`
- Triggers: CLI invocation `sbcl --eval '(asdf:load-system "innatescript")' --eval '(innate:repl)'`
- Responsibilities: Interactive Innate script evaluation, file runner for `.dpn` scripts
- Entry function: `innate:repl`, `innate:run-file`
- Location: `noosphere-schema/schema/*.sql`
- Triggers: Manual application via `psql` or migration tool
- Responsibilities: Create 12 polymorphic tables (9 domains + 3 infrastructure), indexes, triggers, functions
- Order: `00_extensions.sql` → `15_the_ledger.sql`
## Error Handling
- Rust: `anyhow::Result<T>` for dpn-core library functions, `thiserror` for custom error types, Axum's built-in HTTP error responses
- Lisp: `handler-case` / `restart-case` for recoverable conditions, resistance errors propagate to cognition broker for retry/fallback
- HTTP: 401 Unauthorized (missing/invalid auth), 403 Forbidden (valid auth but insufficient permissions), 404 Not Found, 500 Internal Server Error with structured JSON bodies
## Cross-Cutting Concerns
- Rust: `tracing` crate with `EnvFilter` configuration, log levels: debug/info/warn/error
- Lisp: Manual `format t` logging to stdout, no structured logging framework
- dpn-api: `RUST_LOG=dpn_api=info,tower_http=debug`
- noosphere: `RUST_LOG=noosphere=debug,tower_http=debug`
- Rust: Serde deserialization enforces types, custom validation in handlers
- Lisp: Manual validation in parser (`tokenizer.lisp`), evaluator performs type checking
- PostgreSQL: Schema constraints (NOT NULL, UNIQUE, FOREIGN KEY where applicable), triggers for immutability (canon entries in `the_chronicles`)
- dpn-api: Dual-mode (JWT tokens via `/auth/login` or API keys in `X-API-Key` header), middleware validates on protected routes
- noosphere: No authentication (assumes trusted local network or reverse proxy auth)
- AF64 runtime: No authentication (direct database access via libpq FFI, assumes authorized environment)
- Implicit: API keys grant full access, JWT tokens include user identity but no role-based restrictions yet
- Agent-scoped queries: AF64 perception filters by `agent_id` in database queries
- Memory inheritance: `MemoryScope` enum controls cross-agent memory access (Private, Shared, Global, Department)
- Rust: Tokio async runtime for HTTP servers, connection pooling via sqlx (10 connections default)
- Lisp: Single-threaded tick execution (one agent at a time), no parallelism
- PostgreSQL: MVCC for concurrent reads/writes, row-level locking where needed
- SQLite local cache (`~/.dpn/cache.db`) for offline-first document access
- Persona cache in action planner (avoid repeated file reads per tick)
- No Redis or memcached
- Cognition broker has in-memory job cache (winter/thaw logic)
<!-- GSD:architecture-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd:quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd:debug` for investigation and bug fixing
- `/gsd:execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->

<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd:profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
