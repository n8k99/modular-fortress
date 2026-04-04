# Architecture

**Analysis Date:** 2026-04-04

## Pattern Overview

**Overall:** Multi-layer sovereign agent platform with polyglot implementation

**Key Characteristics:**
- Three-pillar architecture: Rust API + Common Lisp agents + Innate scripting language
- PostgreSQL as unified substrate with polymorphic domain tables (9+3 schema)
- Tick-based artificial life engine for autonomous agents ("ghosts in the noosphere")
- Pluggable resolver protocol separating substrate from agent runtime
- Zero external dependencies in Lisp components (hand-rolled everything)

## Layers

**Presentation Layer (noosphere Rust):**
- Purpose: HTTP API server and web UI for operations dashboard
- Location: `noosphere/`
- Contains: Axum web server, static assets, API handlers
- Depends on: dpn-core (via Cargo workspace), PostgreSQL
- Used by: Web UI clients, noosphere-ops dashboard, external API consumers
- Port: 8888

**API Layer (dpn-api Rust):**
- Purpose: RESTful API exposing dpn-core functionality with authentication
- Location: `dpn-api/`
- Contains: Axum HTTP handlers, JWT/API key auth middleware, CORS configuration
- Depends on: dpn-core library crate
- Used by: Common Lisp ghost runtime (AF64), external integrations
- Port: 8080
- Authentication: Dual-mode (JWT tokens or API keys)

**Core Library (dpn-core Rust):**
- Purpose: Shared database access and business logic for all Rust components
- Location: `dpn-core/`
- Contains: Database models, sync engine, embedding generation, wikilink parsing, RSS reader, memory storage, pipeline automation
- Depends on: sqlx (PostgreSQL), tokio (async runtime)
- Used by: dpn-api, noosphere server, standalone tools
- Pattern: Library crate with feature-based module organization

**Agent Runtime (project-noosphere-ghosts Lisp):**
- Purpose: Tick-based artificial life engine for autonomous agents
- Location: `project-noosphere-ghosts/lisp/`
- Contains: 30 Lisp modules (AF64 system), tick engine, cognition broker, perception layer, action executor, energy/drive models, tool registry
- Depends on: libpq.so (PostgreSQL FFI), curl (HTTP client), SBCL runtime
- Used by: Standalone tick invocations, scheduled cron jobs
- Pattern: ASDF system with zero Quicklisp dependencies, hand-rolled JSON/HTTP/PostgreSQL clients

**Scripting Language (innatescript Lisp):**
- Purpose: Declarative intention language for ghost routines and pipelines
- Location: `innatescript/`
- Contains: Lexer, recursive descent parser, two-pass evaluator, pluggable resolver protocol, REPL
- Depends on: SBCL only (zero external libraries)
- Used by: Ghost runtime for executing `.dpn` scripts
- Pattern: Classic interpreter architecture with hand-rolled parser

**Data Layer (PostgreSQL):**
- Purpose: Unified substrate for all application state
- Location: Remote database (SSH tunnel to port 5433 locally)
- Contains: 83 tables collapsed to 9 polymorphic domain tables + 3 infrastructure tables
- Schema: `noosphere-schema/schema/` (16 SQL files)
- Pattern: Polymorphic tables with `kind` discriminator + JSONB `meta` field

## Data Flow

**Ghost Tick Cycle:**

1. **Perception** (`perception.lisp` → PostgreSQL via libpq FFI)
   - Query substrate for agent-relevant state (tasks, conversations, memories, decisions)
   - Tier-aware scans (prime/working/base determine scope depth)
   - Return structured perception data to tick engine

2. **Drive Evaluation** (`drive.lisp`, `energy.lisp`)
   - Tick drive counters (curiosity, purpose, communication, etc.)
   - Calculate pressure levels and energy constraints
   - Determine agent readiness for cognition

3. **Action Planning** (`action-planner.lisp`)
   - Build cognition jobs based on perception + drives + available energy
   - Cache persona context to avoid repeated reads
   - Submit jobs to cognition broker

4. **Cognition Brokering** (`cognition-broker.lisp`)
   - Queue/priority management for LLM requests
   - Provider chain (claude-code → anthropic → stub)
   - Winter/thaw logic for cognitive economy
   - Budget enforcement ($0.50 per request limit)

5. **Action Execution** (`action-executor.lisp`)
   - Apply cognition results to side effects (database writes, tool calls, conversations)
   - Tool registry dispatch (`tool-socket.lisp`)
   - Transaction management

6. **Reporting** (`tick-reporting.lisp`, `empirical-rollups.lisp`)
   - Write tick logs to `the_ledger` (hot/warm/cold tiers)
   - Generate empirical rollups (daily/weekly/monthly/quarterly/yearly)
   - Persist to `the_forge` memory tables

**HTTP Request Flow:**

1. Client → Axum router (`noosphere/src/main.rs` or `dpn-api/src/main.rs`)
2. CORS + tracing middleware
3. Auth middleware (JWT or API key validation) — dpn-api only
4. Handler function (`src/api/handlers/*.rs`)
5. dpn-core function call (`use dpn_core::{...}`)
6. sqlx query to PostgreSQL
7. Response serialization (JSON via serde)
8. Client

**State Management:**
- All persistent state in PostgreSQL (no Redis, no in-memory state)
- Local SQLite cache (`~/.dpn/cache.db`) for offline-first access patterns
- Sync queue for pending changes when remote unavailable
- Hybrid store pattern: try remote first, fallback to cache, queue writes

## Key Abstractions

**Ghost (AF64 Agent):**
- Purpose: Runtime autonomous agent with identity, memory, drives, and energy
- Examples: `the_forge` table rows with `kind='agent'`, instantiated from identity vessels
- Pattern: Tick-based lifecycle with perception → decision → action → reporting cycle
- File: Identity vessels in PostgreSQL, runtime state in `the_forge` table, behavior in `lisp/runtime/tick-engine.lisp`

**Polymorphic Domain Table:**
- Purpose: Single table representing an entire domain with type discriminator
- Examples: `the_forge`, `the_commons`, `the_work`, `the_post`, `the_chronicles`, etc.
- Pattern: `id` (BIGSERIAL), `slug` (TEXT), `kind` (TEXT discriminator), `title`, `body`, `meta` (JSONB), `status`, `created_at`, `updated_at`
- Schema: `noosphere-schema/schema/*.sql`

**Resolver Protocol:**
- Purpose: Pluggable abstraction for fulfilling `@` references in Innate scripts
- Examples: `stub-resolver.lisp` (in-memory), database resolver (planned)
- Pattern: `defgeneric resolve-reference` specialized per resolver implementation
- File: `innatescript/src/eval/resolver.lisp`

**Cognition Job:**
- Purpose: Structured LLM request with tier, prompt, context, and provider chain
- Examples: CognitionJob struct in `lisp/runtime/cognition-types.lisp`
- Pattern: Queue-based with priority, cache, winter/thaw, and budget enforcement
- File: `lisp/runtime/cognition-broker.lisp`

**Perception Substrate:**
- Purpose: Environment scan providing agent-visible state for tick decisions
- Examples: `get_perception` PostgreSQL function, Lisp perception queries
- Pattern: Tier-aware depth (prime=all, working=recent, base=minimal), agent-scoped filtering
- File: `lisp/runtime/perception.lisp`, `dpn-api/src/handlers/af64_perception.rs`

**Pipeline Stage:**
- Purpose: Workflow step with assigned agent and progression logic
- Examples: Pipeline definitions in `the_forge` with `kind='pipeline'`, stages with `kind='pipeline_stage'`
- Pattern: Ordered stages with `next_stage` progression, agent assignment, tool requirements
- File: `lisp/runtime/pipeline-definitions.lisp`

## Entry Points

**noosphere HTTP Server:**
- Location: `noosphere/src/main.rs`
- Triggers: Manual invocation `cargo run` or systemd service
- Responsibilities: Serve noosphere-ops dashboard, expose ghost/task/conversation APIs for UI, static file serving
- Port: 8888
- Routes: `/api/ghosts`, `/api/tasks`, `/api/conversations`, `/api/pipelines`, `/api/system/stats`, `/static/*`

**dpn-api HTTP Server:**
- Location: `dpn-api/src/main.rs`
- Triggers: Manual invocation `cargo run --release` or deployment script
- Responsibilities: Authenticated REST API for documents, tasks, events, projects, RSS reader, agent operations, AF64 ghost endpoints
- Port: 8080
- Routes: `/api/documents/*`, `/api/tasks/*`, `/api/events/*`, `/api/projects/*`, `/api/agents/*`, `/api/conversations/*`, `/api/reading/*`, `/health`, `/auth/login`

**AF64 Tick Engine:**
- Location: `project-noosphere-ghosts/lisp/main.lisp`
- Triggers: CLI invocation `sbcl --eval '(af64:run-tick TICK-NUM)'`, cron jobs
- Responsibilities: Execute one tick cycle for all active agents (perception → cognition → action → reporting)
- Entry function: `af64:run-tick`
- File: `lisp/runtime/tick-engine.lisp`

**Innate REPL:**
- Location: `innatescript/src/repl.lisp`
- Triggers: CLI invocation `sbcl --eval '(asdf:load-system "innatescript")' --eval '(innate:repl)'`
- Responsibilities: Interactive Innate script evaluation, file runner for `.dpn` scripts
- Entry function: `innate:repl`, `innate:run-file`

**Database Schema Initialization:**
- Location: `noosphere-schema/schema/*.sql`
- Triggers: Manual application via `psql` or migration tool
- Responsibilities: Create 12 polymorphic tables (9 domains + 3 infrastructure), indexes, triggers, functions
- Order: `00_extensions.sql` → `15_the_ledger.sql`

## Error Handling

**Strategy:** Layered error propagation with domain-specific error types

**Patterns:**
- Rust: `anyhow::Result<T>` for dpn-core library functions, `thiserror` for custom error types, Axum's built-in HTTP error responses
- Lisp: `handler-case` / `restart-case` for recoverable conditions, resistance errors propagate to cognition broker for retry/fallback
- HTTP: 401 Unauthorized (missing/invalid auth), 403 Forbidden (valid auth but insufficient permissions), 404 Not Found, 500 Internal Server Error with structured JSON bodies

**Rust Error Flow:**
1. Database query fails (sqlx::Error)
2. Wrapped in anyhow context (`context("Failed to query vault_notes")`)
3. Handler returns `Result<Json<T>, StatusCode>`
4. Axum converts error to HTTP response with tracing

**Lisp Error Flow:**
1. Perception query fails (PostgreSQL connection lost)
2. Caught by `handler-case` in tick engine
3. Log error, skip agent's tick, continue with next agent
4. Tick report includes resistance note

**Cognition Broker Fallback Chain:**
1. Primary provider (claude-code) fails → log error, try next
2. Secondary provider (anthropic HTTP) fails → log error, try next
3. Stub provider returns deterministic fallback → no LLM call, proceed with cached/template response

## Cross-Cutting Concerns

**Logging:**
- Rust: `tracing` crate with `EnvFilter` configuration, log levels: debug/info/warn/error
- Lisp: Manual `format t` logging to stdout, no structured logging framework
- dpn-api: `RUST_LOG=dpn_api=info,tower_http=debug`
- noosphere: `RUST_LOG=noosphere=debug,tower_http=debug`

**Validation:**
- Rust: Serde deserialization enforces types, custom validation in handlers
- Lisp: Manual validation in parser (`tokenizer.lisp`), evaluator performs type checking
- PostgreSQL: Schema constraints (NOT NULL, UNIQUE, FOREIGN KEY where applicable), triggers for immutability (canon entries in `the_chronicles`)

**Authentication:**
- dpn-api: Dual-mode (JWT tokens via `/auth/login` or API keys in `X-API-Key` header), middleware validates on protected routes
- noosphere: No authentication (assumes trusted local network or reverse proxy auth)
- AF64 runtime: No authentication (direct database access via libpq FFI, assumes authorized environment)

**Authorization:**
- Implicit: API keys grant full access, JWT tokens include user identity but no role-based restrictions yet
- Agent-scoped queries: AF64 perception filters by `agent_id` in database queries
- Memory inheritance: `MemoryScope` enum controls cross-agent memory access (Private, Shared, Global, Department)

**Concurrency:**
- Rust: Tokio async runtime for HTTP servers, connection pooling via sqlx (10 connections default)
- Lisp: Single-threaded tick execution (one agent at a time), no parallelism
- PostgreSQL: MVCC for concurrent reads/writes, row-level locking where needed

**Caching:**
- SQLite local cache (`~/.dpn/cache.db`) for offline-first document access
- Persona cache in action planner (avoid repeated file reads per tick)
- No Redis or memcached
- Cognition broker has in-memory job cache (winter/thaw logic)

---

*Architecture analysis: 2026-04-04*
