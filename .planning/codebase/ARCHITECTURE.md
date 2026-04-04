# Architecture

**Analysis Date:** 2026-04-04

## Pattern Overview

**Overall:** Multi-layer cognitive AI system with tick-based execution engine

**Key Characteristics:**
- Tick-based ghost agent runtime in Common Lisp (SBCL)
- Rust infrastructure layer for database and REST API
- Custom scripting language (innatescript) for ghost programming
- Nine-table polymorphic PostgreSQL schema
- Two-phase cognition (probe/commit) with LLM broker
- CLOS resolver protocol for entity resolution
- Perception-Cognition-Execution cycle

## Layers

**Ghost Runtime (project-noosphere-ghosts/):**
- Purpose: Cognitive agent execution engine
- Location: `project-noosphere-ghosts/lisp/`
- Contains: Tick engine, cognition broker, action executor, perception pipeline
- Depends on: dpn-core API, innatescript evaluator, PostgreSQL (via `db-client.lisp`)
- Used by: Scheduled cron jobs, manual REPL invocation
- Language: Common Lisp (ASDF system `af64.asd`)

**Infrastructure Layer (dpn-core/):**
- Purpose: Database models, memory storage, embedding generation
- Location: `dpn-core/src/`
- Contains: PostgreSQL connection pool, memory scopes, context injection, deduplication
- Depends on: PostgreSQL, SQLite (cache), external LLM APIs
- Used by: dpn-api, ghost runtime (via HTTP), CLI tools
- Language: Rust

**API Layer (dpn-api/):**
- Purpose: REST endpoints for ghost runtime and external clients
- Location: `dpn-api/src/`
- Contains: Axum HTTP handlers, auth middleware, CORS config
- Depends on: dpn-core
- Used by: Ghost runtime (via HTTP), external clients, UI (future)
- Language: Rust

**Scripting Layer (innatescript/):**
- Purpose: Domain-specific language for ghost routines and entity references
- Location: `innatescript/src/`
- Contains: Tokenizer, parser, evaluator, resolver protocol
- Depends on: Nothing (standalone)
- Used by: Ghost runtime (loaded as Lisp package `innate`)
- Language: Common Lisp (consumed by af64 runtime)

**Schema Layer (noosphere-schema/):**
- Purpose: Nine-table polymorphic database design
- Location: `noosphere-schema/schema/`
- Contains: SQL DDL files, migration scripts
- Depends on: PostgreSQL with pg_trgm, vector extensions
- Used by: dpn-core, ghost runtime
- Language: SQL

## Data Flow

**Ghost Tick Cycle:**

1. **Tick Trigger** (cron/manual) → `run-tick` in `tick-engine.lisp`
2. **Agent Activation** → Fetch active agents + dormant with Nathan messages
3. **Perception** → `db-perceive` aggregates unread conversations, tasks, drives
4. **Cognition Broker** → Builds cognition jobs per agent, submits to provider chain
5. **LLM Resolution** → Providers (Anthropic/OpenClaw gateway) return structured JSON
6. **Action Execution** → `execute-cognition-result` parses tool calls, updates database
7. **Memory Consolidation** → `db-upsert-daily-memory` writes agent daily log
8. **Tick Report** → `write-tick-report` generates rollup, rebuilds empirical summaries

**API Request Flow:**

1. **Client Request** → HTTP to dpn-api (port 8080)
2. **Auth Middleware** → `auth_middleware` validates bearer token
3. **Handler Dispatch** → Axum routes to specific handler (e.g., `af64_tasks::list_tasks`)
4. **Database Query** → dpn-core functions query PostgreSQL via connection pool
5. **Response** → JSON serialization, CORS headers, HTTP 200/40x/50x

**Innate Evaluation Flow:**

1. **Ghost Action Planner** → Builds Innate expression (e.g., `@agents.eliana`, `![tasks where department=engineering]`)
2. **Tokenizer** → Parses expression into tokens
3. **Parser** → AST node construction
4. **Resolver** → `noosphere-resolver` (CLOS subclass) queries PostgreSQL
5. **Evaluator** → Produces `innate-result` with value/context
6. **Error Handling** → `innate-resistance` conditions surface to ghost as resistance messages

**State Management:**
- Ghost state: `agents` table (id, status, tier, energy, drives)
- Agent energy: Consumed by actions, restored by tick rewards, decayed over time
- Cognition jobs: `cognition_job` kind in `the_forge` table
- Memory: `memory_daily` and `memory_entry` kinds in `the_forge` table
- Tick logs: `tick_log` and `tick_report` in `the_ledger` (schema v2)

## Key Abstractions

**Cognition Job:**
- Purpose: Represents a pending LLM request
- Examples: `project-noosphere-ghosts/lisp/runtime/cognition-types.lisp`
- Pattern: CLOS struct with `cognition-job` type
- Fields: `agent-id`, `kind` (perceive/act/reflect), `input-context`, `requested-model-tier`, `priority`, `status`, `cache-key`

**Cognition Broker:**
- Purpose: LLM request queue with cognitive winter throttling
- Examples: `project-noosphere-ghosts/lisp/runtime/cognition-broker.lisp`
- Pattern: Singleton struct with provider chain
- Strategy: Probe (haiku/base) before commit (sonnet/opus), cache results with TTL

**Noosphere Resolver:**
- Purpose: CLOS resolver protocol for Innate entity references
- Examples: `project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp`
- Pattern: Subclass of `innate.eval.resolver:resolver`
- Methods: `resolve-reference` (@agents.eliana), `resolve-search` (![tasks where status=open])

**Ghost Capabilities:**
- Purpose: Per-ghost YAML declaration of responsibilities and tools
- Examples: `project-noosphere-ghosts/config/agents/eliana.yaml`
- Pattern: YAML file with responsibilities (Innate expressions), tools, metadata
- Loading: `load-ghost-capabilities` in `ghost-capabilities.lisp`

**Pipeline Stages:**
- Purpose: Multi-stage workflows (e.g., engineering pipeline)
- Examples: `pipeline` and `pipeline_stage` kinds in `the_forge` table
- Pattern: Ordered stages with assigned ghosts, next_stage pointers
- Advancement: `get-pipeline-advancement` determines next stage based on completion

**Memory Scopes:**
- Purpose: Hierarchical memory inheritance (vault → project → area)
- Examples: `dpn-core/src/memory/inheritance.rs`
- Pattern: Union of memory entries from broader to narrower scope
- Loading: `MemoryInheritance::get_inherited_memories`

**Tool Definitions:**
- Purpose: Registry of available tools for ghost action execution
- Examples: `tool_definition` kind in `the_forge` table
- Pattern: Name, description, parameter schema (JSON), handler function
- Loading: `reload-tool-definitions` caches in `*tool-definition-cache*`

## Entry Points

**Ghost Tick Engine:**
- Location: `project-noosphere-ghosts/lisp/main.lisp`
- Triggers: Cron job (every 10 minutes default), manual REPL `(af64:run-tick)`
- Responsibilities: Fetch agents, perceive, cognition, execute, report

**API Server:**
- Location: `dpn-api/src/main.rs`
- Triggers: HTTP requests on port 8080
- Responsibilities: Serve REST API, authenticate requests, proxy to dpn-core

**Database Initialization:**
- Location: `noosphere-schema/schema/*.sql`
- Triggers: Manual psql execution, migration scripts
- Responsibilities: Create 9 domain tables, 3 infrastructure tables, indexes, triggers

**Innate REPL:**
- Location: `innatescript/src/repl.lisp` (if exists, not confirmed)
- Triggers: Manual SBCL session
- Responsibilities: Interactive Innate expression evaluation

## Error Handling

**Strategy:** Resistance-based error propagation with fallback to dormancy

**Patterns:**
- **Innate Resistance:** `innate-resistance` condition signals to ghost, logged but not fatal
- **Database Errors:** Handler-case with fallback nil, logged to stderr
- **LLM Provider Failures:** Provider chain fallback (Anthropic → OpenClaw → cached)
- **Energy Depletion:** Ghost status → "dormant", skipped in next tick
- **Cache Expiry:** Expired cognition results pruned, cache-miss fallback to LLM
- **Parse Errors:** Innate parse errors surface as resistance messages in action result

**Logging:**
- Format: Plain text to stdout/stderr, JSON telemetry to `~/.af64/telemetry.jsonl`
- Critical: Database connection failures, provider API errors, SBCL crashes
- Info: Tick start/end, cognition job submissions, action executions
- Debug: Perception queries, cache hits, energy deltas

## Cross-Cutting Concerns

**Logging:** Lisp `format t` to stdout, Rust `tracing` crate to stderr

**Validation:** Innate expressions validated on template creation, not at evaluation time

**Authentication:** Bearer token auth in dpn-api (`auth_middleware`), no auth in ghost runtime (trusted localhost)

**Caching:**
- Cognition broker: In-memory cache with 6-hour TTL, disk-backed to `~/.af64/broker-state.json`
- dpn-core: SQLite cache at `~/.dpn/cache.db` for offline-first access
- Embeddings: Generated on write, cached in PostgreSQL `vector(1536)` columns

**Two-Phase Cognition:**
- Phase 1: Probe tier (haiku/base) for low-stakes decisions
- Phase 2: Commit tier (sonnet/opus) only when probe returns `needs_deeper_thought: true`
- Rationale: 90% of ticks resolve at probe tier, massive cost savings

**Cognitive Winter:**
- Trigger: Pending job count exceeds threshold (default 18)
- Effect: Max jobs per tick reduced (6 → 3), throttles LLM usage
- Thaw: When pending count drops below thaw threshold (9) for 2+ ticks

**Standing Orders:**
- Definition: Cron-based recurring tasks (daily note creation, rollup generation)
- Matching: `cron-matcher.lisp` parses cron strings, fires labels once per tick
- Tracking: `*schedule-fired-labels*` hash table prevents duplicate fires

---

*Architecture analysis: 2026-04-04*
