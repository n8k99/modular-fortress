# Codebase Structure

**Analysis Date:** 2026-04-04

## Directory Layout

```
modular-fortress/
├── project-noosphere-ghosts/   # Common Lisp ghost runtime
│   ├── lisp/                   # Core runtime modules
│   │   ├── runtime/            # 31 runtime modules (tick-engine, broker, etc.)
│   │   ├── util/               # Utilities (json, pg, http, github)
│   │   ├── tools/              # Tool implementations
│   │   └── tests/              # Test suite
│   ├── config/                 # Agent YAML declarations
│   │   ├── agents/             # Per-agent capabilities (eliana.yaml, etc.)
│   │   ├── af64.env            # Runtime environment variables
│   │   └── provider-config.json # LLM provider configuration
│   ├── migrations/             # SQL migration scripts
│   ├── sql/                    # SQL query templates
│   └── tools/                  # Utility scripts (onboard.lisp)
├── dpn-core/                   # Rust infrastructure library
│   ├── src/
│   │   ├── db/                 # Database models (11 modules)
│   │   ├── memory/             # Agent memory storage
│   │   ├── embeddings/         # Embedding generation
│   │   ├── context/            # Context injection
│   │   ├── cache/              # SQLite local cache
│   │   ├── dedup/              # Document deduplication
│   │   └── [other modules]     # ics, events, tasks, reading, etc.
│   ├── analysis/               # Data analysis tools
│   └── target/                 # Rust build artifacts
├── dpn-api/                    # Rust REST API server
│   ├── src/
│   │   ├── handlers/           # HTTP route handlers (27 modules)
│   │   ├── auth.rs             # Bearer token auth middleware
│   │   ├── error.rs            # Error types
│   │   └── main.rs             # Axum server entry point
│   └── target/                 # Rust build artifacts
├── innatescript/               # Innate scripting language
│   ├── src/                    # Tokenizer, parser, evaluator (not Rust, likely Lisp)
│   ├── docs/                   # Language documentation
│   └── tests/                  # Innate test suite
├── noosphere-schema/           # Database schema definitions
│   ├── schema/                 # 15 SQL files (9 domain + 3 infra + extensions)
│   ├── migrate/                # Migration tooling
│   ├── mockups/                # Schema mockups
│   └── prompts/                # Design prompts
├── gotcha-secrets/             # API credentials (NOT committed to repo)
│   ├── calendar/               # Google Calendar credentials
│   ├── kalshi/                 # Kalshi API keys
│   ├── oanda/                  # OANDA trading credentials
│   ├── venice/                 # Venice API keys
│   └── xmpp/                   # XMPP credentials
├── specs/                      # Project specifications
├── config.json                 # Master configuration (API keys, services)
├── master_chronicle.dump       # PostgreSQL database dump (443 MB)
├── CLAUDE.md                   # Project instructions for Claude
└── Modular Fortress.md         # Project overview document
```

## Directory Purposes

**project-noosphere-ghosts/:**
- Purpose: Ghost agent runtime engine
- Contains: 40+ Common Lisp modules, ASDF system definition, agent YAML configs
- Key files:
  - `lisp/af64.asd` - ASDF system definition with module load order
  - `lisp/main.lisp` - Entry point (exports `af64:run-tick`)
  - `lisp/packages.lisp` - Package definitions (16 packages)
  - `lisp/runtime/tick-engine.lisp` - Main tick loop (400+ lines)
  - `lisp/runtime/cognition-broker.lisp` - LLM request broker with cognitive winter
  - `lisp/runtime/noosphere-resolver.lisp` - CLOS resolver for Innate
  - `lisp/runtime/action-executor.lisp` - Tool call execution and database updates
  - `config/agents/*.yaml` - Ghost capability declarations (9 agents)

**dpn-core/:**
- Purpose: Rust infrastructure library for database and memory
- Contains: 15+ module directories, Cargo.toml
- Key files:
  - `src/lib.rs` - Public API surface (140+ exports)
  - `src/db/mod.rs` - Database connection pooling
  - `src/db/memories.rs` - Memory storage (renamed from vault_notes)
  - `src/memory/inheritance.rs` - Hierarchical memory scopes
  - `src/embeddings/generator.rs` - OpenAI embedding generation
  - `src/context/injection.rs` - Smart context building
  - `src/cache/sqlite.rs` - Local SQLite cache for offline-first

**dpn-api/:**
- Purpose: REST API server (Axum framework)
- Contains: 27 handler modules, auth middleware
- Key files:
  - `src/main.rs` - Axum server setup (195 lines)
  - `src/handlers/af64_*.rs` - Ghost runtime endpoints (10 modules)
  - `src/handlers/documents.rs` - Document CRUD
  - `src/handlers/tasks.rs` - Task management
  - `src/auth.rs` - Bearer token authentication

**innatescript/:**
- Purpose: Domain-specific scripting language for ghosts
- Contains: Parser, evaluator, resolver protocol
- Key files: (structure inferred, not Rust based on lack of .rs files)
  - Likely `src/parser.lisp`, `src/evaluator.lisp`, `src/resolver.lisp`
  - `docs/` - Language specification and examples

**noosphere-schema/:**
- Purpose: PostgreSQL schema for 9-table polymorphic design
- Contains: SQL DDL files, migration scripts
- Key files:
  - `schema/07_the_forge.sql` - Agents, memories, tick engine (107 lines)
  - `schema/08_the_commons.sql` - Shared resources (59 lines)
  - `schema/09_the_work.sql` - Tasks and goals (70 lines)
  - `schema/10_the_post.sql` - Conversations
  - `schema/12_the_index.sql` - Materialized view for wikilink resolution
  - `schema/14_triggers.sql` - Database triggers
  - `noosphere-bundle.md` - Full schema documentation

**gotcha-secrets/:**
- Purpose: Sensitive API credentials (not in git)
- Contains: Calendar, trading, XMPP credentials
- Security: `.gitignore` excludes this directory

## Key File Locations

**Entry Points:**
- Ghost Runtime: `project-noosphere-ghosts/lisp/main.lisp` → `(af64:run-tick)`
- API Server: `dpn-api/src/main.rs` → `#[tokio::main] async fn main()`
- Database Schema: `noosphere-schema/schema/*.sql` (15 files, run in order)

**Configuration:**
- Master Config: `config.json` (770 lines, API keys, service URLs)
- Ghost Config: `project-noosphere-ghosts/config/af64.env` (runtime vars)
- Provider Config: `project-noosphere-ghosts/config/provider-config.json` (LLM routing)
- Agent Capabilities: `project-noosphere-ghosts/config/agents/*.yaml` (9 files)

**Core Logic:**
- Tick Engine: `project-noosphere-ghosts/lisp/runtime/tick-engine.lisp`
- Cognition Broker: `project-noosphere-ghosts/lisp/runtime/cognition-broker.lisp`
- Action Executor: `project-noosphere-ghosts/lisp/runtime/action-executor.lisp`
- Perception: `project-noosphere-ghosts/lisp/runtime/perception.lisp`
- Noosphere Resolver: `project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp`

**Testing:**
- Lisp Tests: `project-noosphere-ghosts/lisp/tests/test-pg.lisp`
- Rust Tests: `dpn-core/src/db/tests.rs` (unit tests)

## Naming Conventions

**Files:**
- Lisp: `kebab-case.lisp` (e.g., `tick-engine.lisp`, `cognition-broker.lisp`)
- Rust: `snake_case.rs` (e.g., `af64_agents.rs`, `stagehand.rs`)
- SQL: `##_the_domain.sql` (e.g., `07_the_forge.sql`, `08_the_commons.sql`)
- YAML: `lowercase.yaml` (e.g., `eliana.yaml`, `nova.yaml`)

**Directories:**
- Rust crates: `kebab-case` (e.g., `dpn-core`, `dpn-api`)
- Lisp modules: `lowercase` (e.g., `runtime`, `util`, `tools`)
- Schema files: `lowercase` (e.g., `schema`, `migrate`, `mockups`)

**Packages (Lisp):**
- Pattern: `:af64.subsystem.module` (e.g., `:af64.runtime.tick-engine`, `:af64.utils.json`)
- Exports: `defpackage` with `:export` list at top of file

**Functions (Lisp):**
- Pattern: `kebab-case` (e.g., `run-tick`, `broker-submit-job`, `db-perceive`)
- Predicates: `-p` suffix (e.g., `task-ready-p`, `job-expired-p`)

**Structs (Rust):**
- Pattern: `PascalCase` (e.g., `DbPool`, `CognitionJob`, `MemoryEntry`)
- Methods: `snake_case` (e.g., `get_agent`, `list_tasks`, `update_state`)

**Database Tables:**
- Pattern: `the_domain` (e.g., `the_forge`, `the_commons`, `the_work`)
- Columns: `snake_case` (e.g., `agent_id`, `created_at`, `meta`)

**Database Kinds:**
- Pattern: `snake_case` (e.g., `agent`, `memory_daily`, `cognition_job`)
- Stored in `kind` column of polymorphic tables

## Where to Add New Code

**New Ghost Behavior:**
- Primary code: `project-noosphere-ghosts/lisp/runtime/action-executor.lisp` (add tool handler)
- Tests: `project-noosphere-ghosts/lisp/tests/test-[feature].lisp`
- Config: `project-noosphere-ghosts/config/agents/[ghost].yaml` (add responsibility)

**New API Endpoint:**
- Implementation: `dpn-api/src/handlers/[domain].rs` (add handler function)
- Route: `dpn-api/src/main.rs` (register in Axum router)
- Model: `dpn-core/src/db/[domain].rs` (add database query function)

**New Database Kind:**
- Schema: `noosphere-schema/schema/##_the_[domain].sql` (add to kind taxonomy comment)
- Index: Add domain-specific index if needed
- Model: `dpn-core/src/db/[domain].rs` (add Rust struct if needed)

**New Innate Feature:**
- Parser: `innatescript/src/parser.lisp` (extend AST nodes)
- Evaluator: `innatescript/src/evaluator.lisp` (add evaluation rule)
- Resolver: `project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` (add table resolver)

**New Memory Scope:**
- Implementation: `dpn-core/src/memory/inheritance.rs` (extend `MemoryInheritance`)
- Database: Add scope column to relevant table (e.g., `project_id`, `area_id`)

**New LLM Provider:**
- Adapter: `project-noosphere-ghosts/lisp/runtime/provider-adapters.lisp` (add provider class)
- Config: `project-noosphere-ghosts/config/provider-config.json` (add provider entry)

## Special Directories

**target/ (Rust build artifacts):**
- Purpose: Compiled Rust binaries and intermediate files
- Generated: Yes (by `cargo build`)
- Committed: No (excluded in `.gitignore`)

**~/.af64/ (Ghost runtime state):**
- Purpose: Broker state, telemetry logs
- Generated: Yes (by ghost runtime)
- Committed: No (local machine state)
- Files: `broker-state.json`, `telemetry.jsonl`, `tick-reports/`

**~/.dpn/ (Local cache):**
- Purpose: SQLite offline-first cache
- Generated: Yes (by dpn-core)
- Committed: No (local machine cache)
- Files: `cache.db`

**master_chronicle.dump:**
- Purpose: PostgreSQL database snapshot
- Generated: Yes (by `pg_dump`)
- Committed: Maybe (443 MB, synced from droplet at 00:07)
- Note: Contains actual noosphere substrate

**gotcha-secrets/:**
- Purpose: API credentials for external services
- Generated: No (manual setup)
- Committed: No (excluded in `.gitignore`)
- Security: Never commit this directory

**migrations/ (SQL migration scripts):**
- Purpose: Database schema evolution
- Generated: No (manually written)
- Committed: Yes
- Pattern: `###_migration_name.sql` (e.g., `001_blocked_by_array_migration.sql`)

## Dependency Graph

**project-noosphere-ghosts depends on:**
- dpn-api (HTTP client via `http-request` in `util/http.lisp`)
- PostgreSQL (direct connection via `util/pg.lisp`)
- innatescript (loaded as ASDF dependency, not explicitly listed in `af64.asd`)

**dpn-api depends on:**
- dpn-core (Rust crate dependency)
- PostgreSQL (via dpn-core)

**dpn-core depends on:**
- PostgreSQL (via `sqlx` crate)
- SQLite (via `rusqlite` crate for cache)
- OpenAI API (for embeddings)

**innatescript depends on:**
- Nothing (standalone Common Lisp library)

**Inter-project communication:**
- Ghost → API: HTTP POST/GET/PATCH to `http://localhost:8080/api/*`
- API → Database: SQL via `sqlx` connection pool
- Ghost → Database: SQL via Lisp `cl-postgres` (direct connection)
- Ghost → Innate: Function calls to `innate.eval:evaluate`

## Build System

**Common Lisp (ASDF):**
- System Definition: `project-noosphere-ghosts/lisp/af64.asd`
- Load Command: `(asdf:load-system :af64)`
- Build: No compilation step, SBCL loads `.lisp` files

**Rust (Cargo):**
- Workspace: No workspace, two independent crates (dpn-core, dpn-api)
- Build: `cargo build --release` in each directory
- Binaries: `dpn-api/target/release/dpn-api`

**Database:**
- No build system, manual `psql` execution
- Migration: Run numbered schema files in order

## Project Meta

**License:** AGPL (stated in `Modular Fortress.md`)

**Target Platform:** macOS (Darwin 25.3.0), Linux (droplet deployment)

**Runtime Requirements:**
- SBCL (Common Lisp compiler)
- Rust 1.70+ (for dpn-core, dpn-api)
- PostgreSQL 14+
- SQLite 3

**Development Setup:**
1. Clone repo
2. Install SBCL, Rust, PostgreSQL
3. Copy `config.json` and set API keys
4. Load database schema from `noosphere-schema/schema/*.sql`
5. Load ghost runtime: `(asdf:load-system :af64)`
6. Start API server: `cargo run --release` in `dpn-api/`
7. Run tick: `(af64:run-tick)` in SBCL REPL

---

*Structure analysis: 2026-04-04*
