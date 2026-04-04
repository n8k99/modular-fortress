# Architecture

**Analysis Date:** 2026-04-03

## Pattern Overview

**Overall:** Layered agent runtime with pluggable substrate access

**Key Characteristics:**
- Tick-based discrete simulation engine for artificial life
- Generic resolver protocol enabling substrate independence
- PostgreSQL-backed persistence with direct libpq FFI bindings
- Zero-dependency Common Lisp design (no Quicklisp)
- Two-pass evaluation model (collect-then-resolve)

## Layers

**AF64 Tick Engine (Noosphere Ghosts):**
- Purpose: Orchestrates discrete-time agent simulation lifecycle
- Location: `project-noosphere-ghosts/lisp/runtime/`
- Contains: 22 Common Lisp modules forming the complete agent runtime
- Depends on: PostgreSQL via libpq FFI, HTTP via curl subprocess
- Used by: Ghost agent instances, tick orchestrator, standing order scheduler

**Innate Language Interpreter:**
- Purpose: Executable documentation language with pluggable resolution
- Location: `innatescript/src/`
- Contains: Tokenizer, parser, evaluator, resolver protocol, REPL
- Depends on: SBCL, ASDF (zero external Lisp libraries)
- Used by: Noosphere resolver (connects to master_chronicle), agent instruction execution

**Database Layer (dpn-core):**
- Purpose: Shared Rust library providing PostgreSQL access patterns
- Location: `dpn-core/src/`
- Contains: vault_notes, memory scopes, agent state, incremental sync, embeddings
- Depends on: sqlx (async PostgreSQL), tokio runtime
- Used by: dpn-api REST endpoints, agent memory systems, sync tools

**REST API (dpn-api):**
- Purpose: HTTP interface to database layer with authentication
- Location: `dpn-api/src/`
- Contains: Auth handlers, document CRUD, timeline, graph, agent requests
- Depends on: dpn-core library, axum web framework
- Used by: External clients, CI/CD integrations, web interfaces

**Schema Definition (noosphere-schema):**
- Purpose: Authoritative PostgreSQL schema for master_chronicle database
- Location: `noosphere-schema/schema/`
- Contains: 15 SQL migration files organized by domain (chronicles, realms, press, markets, work, etc.)
- Depends on: PostgreSQL 14+ with pgvector extension
- Used by: All layers requiring database access

## Data Flow

**Tick Execution Flow:**

1. `tick-engine.lisp` fetches active agents from `agents` table via `db-client.lisp`
2. `perception.lisp` scans substrate (conversations, tasks, vault_notes) to build agent context
3. `action-planner.lisp` creates CognitionJob structs for each agent action
4. `cognition-broker.lisp` routes jobs to LLM providers (claude-code, anthropic, stub)
5. Provider returns CognitionResult with action recommendation
6. `action-executor.lisp` applies side effects (writes to DB, calls tools via `tool-socket.lisp`)
7. `tick-reporting.lisp` persists tick logs to `tick_reports` table
8. `empirical-rollups.lisp` aggregates tick data into daily/weekly/monthly memory layers

**Innate Evaluation Flow:**

1. `tokenizer.lisp` converts `.dpn` source text to token stream
2. `parser.lisp` builds AST with node types: `:bracket`, `:reference`, `:commission`, `:prose`
3. `evaluator.lisp` pass 1: collect `decree` definitions into symbol table
4. `evaluator.lisp` pass 2: resolve `@references` via `resolve-reference` generic function
5. `noosphere-resolver.lisp` (concrete resolver) queries PostgreSQL to fulfill references
6. Results flow back through evaluator as `innate-result` or `resistance` structs
7. REPL or file runner outputs final values

**Agent Memory Flow:**

1. Agent performs action during tick (via `action-executor.lisp`)
2. Tick report written to `tick_reports` table with JSON action log
3. `empirical-rollups.lisp` runs periodic aggregation jobs
4. Memory layers written to `vault_notes` table with layer tag (daily/weekly/monthly/quarterly/yearly)
5. Future ticks query memory via `db-fetch-recent-memory` in `perception.lisp`
6. Cross-agent memory access controlled by `MemoryScope` enum in `dpn-core/src/memory/`

**State Management:**
- Agent runtime state: PostgreSQL `agent_state` table (energy, tier, last_tick)
- Conversation threads: `conversations` and `conversation_messages` tables
- Task queue: `tasks` table with status, priority, blocked_by relationships
- Tick history: `tick_reports` table with JSONB action logs
- Memory compression: `vault_notes` table with layer tags

## Key Abstractions

**CognitionJob / CognitionResult:**
- Purpose: Stateless request/response for LLM invocations
- Examples: `cognition-types.lisp`, `cognition-broker.lisp`
- Pattern: Struct-based with priority queue, cache, winter/thaw states

**Resolver Protocol (CLOS):**
- Purpose: Generic interface for fulfilling Innate `@references` and `[context[verb]]` expressions
- Examples: `innatescript/src/eval/resolver.lisp` (base protocol), `project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` (concrete implementation)
- Pattern: `defgeneric` with specialized `defmethod` per resolver type

**Memory Layers:**
- Purpose: Temporal abstraction hierarchy (daily → yearly)
- Examples: `empirical-rollups.lisp` generates from tick_reports
- Pattern: Increasing compression and abstraction per layer

**Tool Socket:**
- Purpose: Extensible registry for ghost-invokable capabilities
- Examples: `tool-socket.lisp` with handler registration
- Pattern: Named tool lookup with argument validation and telemetry

**Agent Identity Vessel:**
- Purpose: Immutable baseline identity definition (name, role, skills, bio)
- Examples: `agents` table records, persona JSON files
- Pattern: Separation of identity (static) from runtime state (mutable)

## Entry Points

**AF64 Tick Engine:**
- Location: `project-noosphere-ghosts/lisp/main.lisp`
- Triggers: CLI invocation via `sbcl --eval '(af64:run-tick N)'`
- Responsibilities: Load all 22 modules, initialize `*db-pool*`, run tick loop

**Innate REPL:**
- Location: `innatescript/src/repl.lisp`
- Triggers: Shell script `run-repl.sh` or direct SBCL load
- Responsibilities: Interactive read-eval-print loop with resolver injection

**dpn-api Server:**
- Location: `dpn-api/src/main.rs`
- Triggers: `cargo run` or systemd service
- Responsibilities: Start axum HTTP server, load auth middleware, route requests

**Innate File Runner:**
- Location: `innatescript/src/repl.lisp` function `run-file`
- Triggers: Programmatic call from Lisp or shell wrapper
- Responsibilities: Parse and evaluate `.dpn` file, return results

## Error Handling

**Strategy:** Resistance-based error propagation with restart capability

**Patterns:**
- Innate evaluation: `resistance` struct returned instead of throwing, evaluator can signal `innate-resistance` condition
- AF64 tick engine: `handler-case` wraps all DB/HTTP calls, logs errors, continues tick
- dpn-core Rust: `anyhow::Result` for fallible operations, `thiserror` for domain errors
- dpn-api: Structured error responses with HTTP status codes (401, 403, 500)

## Cross-Cutting Concerns

**Logging:**
- AF64: `format t` to stdout, captured by systemd or redirect
- dpn-core/dpn-api: `tracing` crate with env-filter levels (INFO, DEBUG, TRACE)
- Innate: Error messages via `format t`, parser errors include line/column

**Validation:**
- AF64: Schema validation via PostgreSQL constraints, tool-socket argument checks
- Innate: Parse-time validation in `parser.lisp`, resolver returns resistance on invalid references
- dpn-api: JWT validation via jsonwebtoken crate, API key comparison in middleware

**Authentication:**
- dpn-api: Dual-mode (JWT bearer tokens or X-API-Key header)
- AF64: Direct PostgreSQL access (trusted internal component)
- Innate: No built-in auth (resolver implementation determines access control)

---

*Architecture analysis: 2026-04-03*
