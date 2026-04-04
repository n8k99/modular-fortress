# Codebase Structure

**Analysis Date:** 2026-04-04

## Directory Layout

```
modular-fortress/
├── dpn-api/                   # REST API server (Rust, Axum)
├── dpn-core/                  # Shared library crate (Rust)
├── noosphere/                 # Unified server + UI (Rust, Axum)
├── project-noosphere-ghosts/  # AF64 tick engine (Common Lisp)
├── innatescript/              # Scripting language (Common Lisp)
├── noosphere-schema/          # PostgreSQL schema definitions
├── gotcha-secrets/            # External API credentials (gitignored)
├── specs/                     # Technical specifications
├── markdown/                  # Exported database markdown dumps
├── .planning/                 # GSD planning artifacts
├── config.json                # Service configuration and API keys
├── master_chronicle.dump      # PostgreSQL database dump (443MB)
└── CLAUDE.md                  # Project instructions
```

## Directory Purposes

**dpn-api/**
- Purpose: RESTful API server with authentication
- Contains: Axum HTTP server, auth middleware, API handlers for all domains
- Key files: `src/main.rs`, `src/auth.rs`, `src/handlers/*.rs`
- Entry point: `cargo run --release`
- Port: 8080
- Pattern: Handler-per-domain with shared dpn-core dependency

**dpn-core/**
- Purpose: Shared business logic and database access layer
- Contains: Database models, sync engine, embeddings, wikilinks, RSS reader, memory, context injection, deduplication, notifications, pipelines
- Key files: `src/lib.rs`, `src/db/*.rs`, `src/memory/*.rs`, `src/embeddings/*.rs`, `src/sync/*.rs`, `src/reading/*.rs`
- Entry point: Library crate (no binary)
- Pattern: Feature-based module organization with re-exports in `lib.rs`

**noosphere/**
- Purpose: Unified web server and operations dashboard
- Contains: Axum server, static assets, UI handlers, MCP protocol (Model Context Protocol)
- Key files: `src/main.rs`, `src/lib.rs`, `src/api/*.rs`, `static/noosphere-ops.html`
- Entry point: `cargo run` (port 8888)
- Pattern: Minimal consolidation of dpn-api + dpn-mcp with dashboard focus

**project-noosphere-ghosts/**
- Purpose: AF64 tick-based artificial life engine
- Contains: 30 Common Lisp modules, ASDF system definition, persona config, migrations, SQL utilities, tool wrappers
- Key files: `lisp/af64.asd`, `lisp/main.lisp`, `lisp/runtime/*.lisp`, `config/provider-config.json`, `config/em-field-mapping.lisp`
- Entry point: `sbcl --eval '(asdf:load-system :af64)' --eval '(af64:run-tick N)'`
- Pattern: Zero Quicklisp dependencies, hand-rolled HTTP/JSON/PostgreSQL clients

**innatescript/**
- Purpose: Declarative scripting language for ghost routines
- Contains: Lexer, parser, evaluator, resolver protocol, REPL, test framework
- Key files: `innatescript.asd`, `src/packages.lisp`, `src/types.lisp`, `src/parser/*.lisp`, `src/eval/*.lisp`, `tests/*.lisp`
- Entry point: `sbcl --eval '(asdf:load-system "innatescript")' --eval '(innate:repl)'`
- Pattern: Hand-rolled recursive descent parser, two-pass evaluation, zero external dependencies

**noosphere-schema/**
- Purpose: PostgreSQL schema definitions for all 12 domain tables
- Contains: 16 SQL files defining polymorphic tables, indexes, triggers, functions
- Key files: `schema/00_extensions.sql` through `schema/15_the_ledger.sql`
- Entry point: Apply via `psql -U user -d master_chronicle -f schema/*.sql`
- Pattern: Polymorphic tables with `kind` discriminator + JSONB `meta` field

**gotcha-secrets/**
- Purpose: External service API keys (gitignored)
- Contains: Subdirectories for calendar, kalshi, oanda, venice, xmpp credentials
- Key files: `.env` files, JSON credential files (all gitignored)
- Pattern: Isolated credential storage separate from config.json

**specs/**
- Purpose: Technical specifications and design documents
- Contains: High-level architecture docs, domain models, feature specs
- Pattern: Markdown documentation for planning and reference

**markdown/**
- Purpose: Exported database content as markdown files
- Contains: Subdirectories for agent areas, daily memory, documents, drives, fitness
- Key files: `agent_areas/*.md`, `agent_daily_memory/*.md`, `agent_drives/*.md`
- Pattern: One markdown file per database row, used for LLM context injection

**.planning/**
- Purpose: GSD planning artifacts (milestones, phases, research, codebase docs)
- Contains: `codebase/` (ARCHITECTURE.md, STRUCTURE.md, etc.), archived planning docs
- Key files: `.planning/codebase/ARCHITECTURE.md`, `.planning/codebase/STRUCTURE.md`
- Pattern: GSD workflow coordination and codebase mapping

## Key File Locations

**Entry Points:**
- `noosphere/src/main.rs`: Noosphere web server (port 8888)
- `dpn-api/src/main.rs`: dpn-api REST server (port 8080)
- `project-noosphere-ghosts/lisp/main.lisp`: AF64 tick engine entry
- `innatescript/src/innate.lisp`: Innate interpreter top-level API

**Configuration:**
- `config.json`: Service URLs, API keys, database credentials, LLM provider settings, team rosters
- `project-noosphere-ghosts/config/provider-config.json`: LLM provider chain configuration
- `project-noosphere-ghosts/config/af64.env`: Runtime environment variables (generated by onboarding wizard)
- `dpn-api/.env`: Database URL, JWT secret, API keys
- `noosphere/.env`: Database URL

**Core Logic:**
- `dpn-core/src/lib.rs`: Re-exports for all dpn-core modules
- `dpn-core/src/db/*.rs`: Database access (documents, tasks, events, projects, memories, areas, archives, resources, templates)
- `project-noosphere-ghosts/lisp/runtime/tick-engine.lisp`: AF64 tick orchestration
- `project-noosphere-ghosts/lisp/runtime/cognition-broker.lisp`: LLM request queue and provider chain
- `project-noosphere-ghosts/lisp/runtime/perception.lisp`: Agent environment scanning
- `project-noosphere-ghosts/lisp/runtime/action-executor.lisp`: Side-effect execution (DB writes, tool calls)
- `innatescript/src/eval/evaluator.lisp`: Two-pass Innate script evaluation

**Testing:**
- `dpn-core/tests/`: Rust integration tests (none present yet)
- `innatescript/tests/`: Hand-rolled Lisp test suite (`test-framework.lisp`, `test-*.lisp`)
- `innatescript/run-tests.sh`: Test runner script

## Naming Conventions

**Files:**
- Rust: snake_case (e.g., `documents.rs`, `vault_notes.rs`, `af64_agents.rs`)
- Lisp: kebab-case (e.g., `tick-engine.lisp`, `cognition-broker.lisp`, `action-planner.lisp`)
- SQL: numbered with prefix (e.g., `00_extensions.sql`, `07_the_forge.sql`)
- Markdown: descriptive with underscores (e.g., `agent_areas_1.md`, `ARCHITECTURE.md`)

**Directories:**
- Rust: kebab-case (e.g., `dpn-api`, `dpn-core`)
- Lisp: kebab-case (e.g., `project-noosphere-ghosts`, `innatescript`)
- Modules: lowercase (e.g., `src/db/`, `src/memory/`, `lisp/runtime/`)

## Where to Add New Code

**New REST API Endpoint:**
- Implementation: `dpn-api/src/handlers/[domain].rs` (or create new file)
- Route registration: `dpn-api/src/main.rs` (add route in protected or public router)
- Database logic: `dpn-core/src/db/[table].rs` (or create new module)
- Tests: `dpn-api/tests/` (if integration tests exist) or manual via curl

**New AF64 Runtime Module:**
- Implementation: `project-noosphere-ghosts/lisp/runtime/[module-name].lisp`
- Package definition: Add to `project-noosphere-ghosts/lisp/packages.lisp`
- System definition: Add to `project-noosphere-ghosts/lisp/af64.asd` `:components`
- Tests: Not currently implemented (manual verification via REPL)

**New Innate Language Feature:**
- Parser change: `innatescript/src/parser/parser.lisp` (add grammar rules)
- Tokenizer: `innatescript/src/parser/tokenizer.lisp` (add token types if needed)
- AST node: `innatescript/src/types.lisp` (add struct definition)
- Evaluator: `innatescript/src/eval/evaluator.lisp` (add evaluation logic)
- Tests: `innatescript/tests/test-[feature].lisp`

**New Database Domain:**
- Schema: `noosphere-schema/schema/[NN]_the_[domain].sql`
- dpn-core model: `dpn-core/src/db/[domain].rs`
- API handlers: `dpn-api/src/handlers/[domain].rs`
- Route registration: `dpn-api/src/main.rs`

**New dpn-core Feature:**
- Implementation: `dpn-core/src/[module]/` (create directory if new module)
- Re-export: `dpn-core/src/lib.rs` (add `pub use` statements)
- Documentation: Add doc comments to public functions
- Tests: `dpn-core/tests/` (integration tests) or `#[cfg(test)] mod tests` in module file

**Utilities:**
- Rust shared helpers: `dpn-core/src/[appropriate_module]/` (no separate utils directory)
- Lisp utilities: `project-noosphere-ghosts/lisp/util/*.lisp` (json.lisp, pg.lisp, http.lisp)
- SQL utilities: `project-noosphere-ghosts/sql/` (helper queries)
- Python tools: `project-noosphere-ghosts/tools/` (though not currently present)

## Special Directories

**dpn-api/target/ and dpn-core/target/ and noosphere/target/**
- Purpose: Rust build artifacts
- Generated: Yes (by `cargo build`)
- Committed: No (gitignored)

**project-noosphere-ghosts/config/**
- Purpose: Runtime configuration files (provider config, persona mappings, field mappings)
- Generated: Partially (onboarding wizard creates `af64.env`)
- Committed: Partially (`provider-config.json` template yes, `af64.env` no)

**noosphere-schema/migrate/**
- Purpose: Migration tracking or old migration files
- Generated: No
- Committed: Yes (but no files currently present)

**noosphere-schema/mockups/**
- Purpose: UI/UX mockups or schema mockups
- Generated: No
- Committed: Yes (markdown files with operational mockups)

**markdown/**
- Purpose: Database exports for LLM context (agent memory, areas, documents, drives, fitness)
- Generated: Yes (by `export_db_to_markdown.py`)
- Committed: No (gitignored, regenerated on demand)

**.planning/**
- Purpose: GSD workflow artifacts (milestones, phases, research, codebase documentation)
- Generated: Partially (by GSD commands)
- Committed: Yes (archived files show `D` status, indicating deletion from previous cleanup)

**doltgres-data/**
- Purpose: Doltgres database storage (experimental versioned database)
- Generated: Yes (by Doltgres runtime)
- Committed: No (gitignored)

## Architecture Boundaries

**Rust ↔ Lisp:**
- Lisp runtime calls Rust API via HTTP (dpn-api endpoints)
- No direct FFI between Rust and Lisp
- Lisp has direct PostgreSQL access via libpq FFI (bypassing Rust layer)

**dpn-core ↔ dpn-api:**
- dpn-api depends on dpn-core as Cargo library dependency
- dpn-core exposes all database logic as public functions
- dpn-api provides HTTP/JSON interface on top

**noosphere ↔ dpn-core:**
- noosphere depends on dpn-core (same pattern as dpn-api)
- Consolidates some dpn-api functionality with dashboard focus
- Separate port (8888 vs 8080) for different use cases

**AF64 Runtime ↔ Innate:**
- AF64 runtime loads innatescript as ASDF dependency (planned, not yet implemented)
- Innate provides interpreter for `.dpn` scripts
- AF64 provides resolver implementation for Innate's pluggable protocol

**All Components ↔ PostgreSQL:**
- Rust: sqlx connection pool (10 connections)
- Lisp: Direct libpq FFI calls (no connection pooling)
- Schema: Managed via SQL files in noosphere-schema/
- Migrations: Ad-hoc SQL application (no structured migration tool)

## Import Organization

**Rust (dpn-api/dpn-core):**
1. Standard library (`use std::*`)
2. External crates (`use axum::*`, `use sqlx::*`, `use serde::*`)
3. Internal crate modules (`use crate::*`)
4. dpn-core re-exports (`use dpn_core::*`)

**Lisp (AF64/Innate):**
1. System packages (`(:use :common-lisp)`)
2. Internal project packages (`(:import-from :af64.perception ...)`)
3. No external dependencies (zero Quicklisp)
4. Manual `use-package` in REPL if needed

## Migration Patterns

**Database Schema Changes:**
1. Create new SQL file in `project-noosphere-ghosts/migrations/`
2. Apply manually via `psql`
3. Update relevant Rust models in `dpn-core/src/db/`
4. Update API handlers if needed
5. No automated migration tracking (yet)

**API Changes:**
1. Update dpn-core function signature
2. Update dpn-api handlers using that function
3. Update Lisp runtime if calling that endpoint
4. No versioning strategy (all changes breaking)

**Lisp Module Changes:**
1. Modify `.lisp` file in `project-noosphere-ghosts/lisp/runtime/`
2. Reload system via `(asdf:load-system :af64)`
3. Test via REPL or full tick invocation
4. No test automation (manual verification)

---

*Structure analysis: 2026-04-04*
