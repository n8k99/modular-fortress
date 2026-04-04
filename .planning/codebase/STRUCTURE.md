# Codebase Structure

**Analysis Date:** 2026-04-03

## Directory Layout

```
Modular Fortress/
├── project-noosphere-ghosts/   # AF64 tick engine (Common Lisp)
├── innatescript/               # Innate language interpreter (Common Lisp)
├── dpn-core/                   # Database layer (Rust library)
├── dpn-api/                    # REST API (Rust binary)
├── noosphere-schema/           # PostgreSQL schema definitions
├── specs/                      # Specifications and documentation
├── gotcha-secrets/             # Credential management
├── config.json                 # Global service configuration
└── master_chronicle.dump       # Database snapshot
```

## Directory Purposes

**project-noosphere-ghosts/**
- Purpose: AF64 artificial life engine implementation
- Contains: 22 Common Lisp modules, ASDF system definition, config files
- Key files: `lisp/af64.asd` (system definition), `lisp/main.lisp` (entry point), `launch.sh` (startup script)

**project-noosphere-ghosts/lisp/runtime/**
- Purpose: Core tick engine implementation
- Contains: tick-engine, perception, action-planner, cognition-broker, tool-socket, empirical-rollups
- Key files:
  - `tick-engine.lisp`: Main tick orchestrator
  - `noosphere-resolver.lisp`: Connects Innate to master_chronicle
  - `cognition-broker.lisp`: LLM provider dispatch with caching
  - `db-client.lisp`: Direct PostgreSQL queries via libpq FFI
  - `perception.lisp`: Substrate scanning for agent context

**project-noosphere-ghosts/lisp/util/**
- Purpose: Zero-dependency utility libraries
- Contains: `json.lisp` (hand-rolled JSON codec), `pg.lisp` (libpq FFI bindings), `http.lisp` (curl wrapper)

**project-noosphere-ghosts/config/**
- Purpose: Runtime configuration files
- Contains: `provider-config.json` (LLM provider chain), `agents/` (persona JSON files)

**innatescript/**
- Purpose: Innate language interpreter root
- Contains: ASDF system definition, shell scripts, documentation
- Key files: `innatescript.asd` (ASDF system), `run-repl.sh`, `run-tests.sh`

**innatescript/src/**
- Purpose: Innate implementation source
- Contains: `packages.lisp`, `types.lisp`, `conditions.lisp`, `innate.lisp`

**innatescript/src/parser/**
- Purpose: Lexer and parser for `.dpn` files
- Contains: `tokenizer.lisp` (character-level lexing), `parser.lisp` (recursive descent AST builder)

**innatescript/src/eval/**
- Purpose: Evaluation engine and resolver protocol
- Contains: `resolver.lisp` (generic interface), `evaluator.lisp` (two-pass eval), `stub-resolver.lisp` (testing resolver)

**innatescript/tests/**
- Purpose: Automated test suite
- Contains: `test-framework.lisp` (hand-rolled harness), `test-*.lisp` files per module

**dpn-core/src/**
- Purpose: Rust shared library modules
- Contains: 20+ modules organized by concern (db, memory, sync, embeddings, etc.)
- Key files:
  - `lib.rs`: Public API surface
  - `db/connection.rs`: Pool creation and management
  - `db/vault_notes.rs`: Primary note storage
  - `memory/store.rs`: Agent memory persistence
  - `memory/recall.rs`: Memory search and retrieval

**dpn-core/src/db/**
- Purpose: Database access patterns
- Contains: `connection.rs`, `vault_notes.rs`, `conversations.rs`, `tasks.rs`, `events.rs`, `projects.rs`

**dpn-core/src/memory/**
- Purpose: Cross-agent memory system
- Contains: `store.rs`, `recall.rs`, `inheritance.rs` (MemoryScope implementation)

**dpn-api/src/**
- Purpose: REST API implementation
- Contains: `main.rs` (server entry), `auth.rs` (JWT/API key middleware), `error.rs`, `handlers/` (endpoint modules)

**dpn-api/src/handlers/**
- Purpose: HTTP endpoint handlers
- Contains: One module per resource type (documents, tasks, timeline, graph, agent-requests)

**noosphere-schema/schema/**
- Purpose: SQL migration files for master_chronicle database
- Contains: 15 numbered SQL files (00-15) defining tables, functions, triggers, indexes
- Key files:
  - `00_extensions.sql`: pgvector, uuid-ossp
  - `02_the_chronicles.sql`: agents, conversations, tasks, vault_notes
  - `09_the_work.sql`: projects, goals, archives
  - `14_triggers.sql`: modified_at auto-update triggers

**specs/**
- Purpose: System specifications and design documents
- Contains: Placeholder for future specs

**gotcha-secrets/**
- Purpose: Credential storage
- Contains: API keys, tokens, certificates (NOT for commit)

## Key File Locations

**Entry Points:**
- `project-noosphere-ghosts/lisp/main.lisp`: AF64 tick engine entry
- `project-noosphere-ghosts/launch.sh`: AF64 startup wrapper
- `innatescript/src/innate.lisp`: Innate interpreter entry
- `innatescript/run-repl.sh`: Innate REPL launcher
- `dpn-api/src/main.rs`: REST API server entry

**Configuration:**
- `/config.json`: Global service URLs, API keys, database connections
- `project-noosphere-ghosts/config/provider-config.json`: LLM provider chain
- `project-noosphere-ghosts/config/agents/*.json`: Agent persona files
- `dpn-api/.env`: REST API environment variables

**Core Logic:**
- `project-noosphere-ghosts/lisp/runtime/tick-engine.lisp`: Tick orchestration
- `project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp`: Innate-to-database bridge
- `innatescript/src/eval/evaluator.lisp`: Innate evaluation engine
- `dpn-core/src/db/vault_notes.rs`: Primary note storage operations
- `dpn-api/src/handlers/`: All REST endpoint logic

**Testing:**
- `innatescript/tests/test-*.lisp`: Innate interpreter test suite
- `dpn-core/src/` (inline `#[cfg(test)]` modules): Rust unit tests
- `dpn-api/test_integration.sh`: API integration test script

## Naming Conventions

**Files:**
- Common Lisp: `kebab-case.lisp` (e.g., `tick-engine.lisp`, `noosphere-resolver.lisp`)
- Rust: `snake_case.rs` (e.g., `vault_notes.rs`, `agent_requests.rs`)
- ASDF systems: `lowercase.asd` (e.g., `af64.asd`, `innatescript.asd`)
- Shell scripts: `kebab-case.sh` (e.g., `run-repl.sh`, `launch.sh`)

**Directories:**
- Common Lisp: `lowercase` without separators (e.g., `runtime`, `util`, `tests`)
- Rust: `snake_case` (e.g., `db`, `memory`, `handlers`)

## Where to Add New Code

**New AF64 Module:**
- Primary code: `project-noosphere-ghosts/lisp/runtime/module-name.lisp`
- Add to ASDF: Edit `project-noosphere-ghosts/lisp/af64.asd` components list
- Tests: `project-noosphere-ghosts/lisp/tests/test-module-name.lisp`

**New Innate Feature:**
- Parser extension: `innatescript/src/parser/parser.lisp`
- Evaluator extension: `innatescript/src/eval/evaluator.lisp`
- New resolver method: Add `defmethod` in `innatescript/src/eval/resolver.lisp`
- Tests: `innatescript/tests/test-feature.lisp`

**New dpn-core Module:**
- Implementation: `dpn-core/src/module_name/` (new directory) or `dpn-core/src/module_name.rs` (single file)
- Public API: Add to `dpn-core/src/lib.rs` public re-exports
- Tests: Inline `#[cfg(test)]` module or `dpn-core/src/module_name/tests.rs`

**New REST Endpoint:**
- Handler: `dpn-api/src/handlers/resource.rs`
- Register route: Add to `dpn-api/src/main.rs` router setup
- Tests: Add cases to `dpn-api/test_integration.sh`

**New Database Table:**
- Schema: Add to appropriate `noosphere-schema/schema/NN_*.sql` file
- Access: Add module to `dpn-core/src/db/table_name.rs`
- Queries: Use sqlx macros for compile-time verification

**Utilities:**
- AF64 utilities: `project-noosphere-ghosts/lisp/util/utility-name.lisp`
- Innate utilities: Add to existing `innatescript/src/` module or create new if generic
- Rust shared helpers: `dpn-core/src/` top-level or domain module

## Special Directories

**project-noosphere-ghosts/lisp/runtime/**
- Purpose: All AF64 tick engine modules
- Generated: No
- Committed: Yes

**project-noosphere-ghosts/config/agents/**
- Purpose: Persona JSON files for agent identity
- Generated: No (authored by user)
- Committed: Yes (example personas), No (production personas with sensitive info)

**innatescript/.planning/**
- Purpose: Phase execution artifacts from GSD workflow
- Generated: Yes (by GSD commands)
- Committed: Yes (tracks development history)

**dpn-core/target/**
- Purpose: Rust build artifacts
- Generated: Yes (by cargo)
- Committed: No (in .gitignore)

**dpn-api/target/**
- Purpose: Rust build artifacts for API binary
- Generated: Yes (by cargo)
- Committed: No (in .gitignore)

**noosphere-schema/migrate/**
- Purpose: Migration tooling or tracking
- Generated: Varies by migration tool
- Committed: Tool-dependent

**gotcha-secrets/**
- Purpose: API keys, tokens, certificates
- Generated: No (managed manually or via tooling)
- Committed: NEVER (must be in .gitignore)

---

*Structure analysis: 2026-04-03*
