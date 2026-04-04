# Coding Conventions

**Analysis Date:** 2026-04-04

## Naming Patterns

**Files:**
- Common Lisp: Hyphenated lowercase with `.lisp` extension (e.g., `test-framework.lisp`, `db-conversations.lisp`, `action-planner.lisp`)
- Rust: Snake_case with `.rs` extension (e.g., `connection.rs`, `agent_requests.rs`, `af64_perception.rs`)
- ASDF system definitions: Project name + `.asd` (e.g., `innatescript.asd`, `af64.asd`)
- Cargo manifests: `Cargo.toml` per project
- Test files: Prefix `test-` for Lisp tests (e.g., `test-tokenizer.lisp`, `test-parser.lisp`)

**Functions:**
- Common Lisp: Hyphenated lowercase (e.g., `make-eval-env`, `resolve-reference`, `db-fetch-agents`, `empty-perception`)
- Rust: Snake_case (e.g., `create_pool`, `list_documents`, `get_recent_conversations`)
- Internal functions in Lisp: Prefix with `%` (e.g., `%report-failure`, `%read-file-to-string`)

**Variables:**
- Lisp global dynamic variables: Earmuffs (e.g., `*test-registry*`, `*db-pool*`, `*noosphere-resolver*`)
- Lisp constants: Plus signs (e.g., `+node-program+`, `+node-bracket+`, `+energy-costs+`)
- Rust: Snake_case for locals, SCREAMING_SNAKE_CASE for constants (e.g., `DEFAULT_DATABASE_URL`)

**Types:**
- Lisp structs: Hyphenated lowercase (e.g., `cognition-job`, `innate-result`, `resistance`)
- Lisp conditions: Hyphenated with descriptive suffix (e.g., `innate-parse-error`, `innate-resistance`)
- Rust structs/enums: PascalCase (e.g., `DbPool`, `ApiError`, `ConversationLight`)
- Lisp packages: Dotted hierarchical lowercase (e.g., `:innate.types`, `:af64.runtime.perception`, `:innate.parser.tokenizer`)
- Rust modules: Snake_case (e.g., `mod handlers`, `mod auth`)

## Code Style

**Formatting:**
- Common Lisp: Hand-formatted with visual alignment on closing parens, 2-space indentation standard
- Rust: `cargo fmt` with default settings (4-space indentation, trailing commas)
- Line width: Not enforced programmatically; natural breaks preferred

**Linting:**
- Common Lisp: No external linter — manual review against style guide
- Rust: `cargo clippy` enabled but not blocking (warnings allowed)

## Import Organization

**Order (Common Lisp):**
1. `(in-package ...)` at top of file
2. No implicit imports — all packages declare explicit `:use` and `:import-from`
3. Never `:use :common-lisp-user` — always `:use :cl` only
4. Cross-package imports via `:import-from` with explicit symbol lists

**Pattern (Lisp packages.lisp):**
```lisp
(defpackage :innate.eval
  (:use :cl)
  (:import-from :innate.eval.resolver
    #:resolver #:eval-env #:resolve-reference)
  (:import-from :innate.types
    #:node-kind #:node-value)
  (:export
   #:evaluate))
```

**Order (Rust):**
1. Standard library imports
2. External crate imports (grouped by crate)
3. Internal crate imports (`dpn_core`, local modules)
4. Re-exports at top of `lib.rs` for public API

**Pattern (Rust):**
```rust
use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

use dpn_core::{create_pool, Memory};
```

## Error Handling

**Patterns (Common Lisp):**
- Use `handler-case` for recoverable conditions (e.g., missing references, parse errors)
- Use `restart-case` when offering recovery options
- Signal resistance conditions with `(signal 'innate-resistance ...)` — NOT `error`
- Error conditions inherit from both `innate-condition` and `error`
- Resistance conditions inherit from `innate-condition` and `condition` (not `error`)
- Always provide `:report` methods for user-facing condition messages

**Pattern (Lisp handler-case):**
```lisp
(handler-case
    (db-perceive *db-pool* agent-id tier since)
  (error (e)
    (format t "  [perception-error] ~a: ~a~%" agent-id e)
    (empty-perception)))
```

**Patterns (Rust):**
- Use `Result<T, E>` for all fallible operations
- Prefer `anyhow::Result` for application-level errors
- Use `thiserror` for domain-specific error types
- Implement `IntoResponse` for API error types
- Convert errors at API boundary with `From` traits

**Pattern (Rust error enum):**
```rust
#[derive(Debug)]
pub enum ApiError {
    Database(String),
    NotFound(String),
    BadRequest(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self { /* ... */ };
        (status, Json(json!({"error": message}))).into_response()
    }
}
```

## Logging

**Framework (Common Lisp):** Console output via `format`

**Patterns:**
- Prefix messages with context tags: `"[perception-error]"`, `"[action-executor]"`
- Use `~a` for string interpolation, `~%` for newlines
- Log to stdout — no file-based logging in core runtime

**Framework (Rust):** `tracing` crate

**Patterns:**
- Use `tracing::info!`, `tracing::debug!`, `tracing::error!` macros
- Configure via `RUST_LOG` environment variable
- Default filter: `"dpn_api=debug,tower_http=debug"`

**Pattern (Rust):**
```rust
tracing::info!("Database pool created");
tracing::debug!("Connecting to database...");
```

## Comments

**When to Comment:**
- File headers: Purpose, phase number, task IDs (e.g., `;;;; test-tokenizer.lisp — tests for the Innate tokenizer (Phase 3)`)
- Section dividers: Major logical sections (e.g., `;;; ─── Task 1: Single-character token tests (TOK-01 through TOK-10) ───`)
- Complex algorithms: Explain WHY, not WHAT
- TODOs: Prefix with `TODO:` or `FIXME:` (e.g., `// TODO: Uncomment after Phase 6`)

**Docstrings:**
- Lisp: `:documentation` slot in `defstruct`, `defclass`, `define-condition`
- Lisp functions: String literal after parameter list (e.g., `"Assert that ACTUAL is EQUAL to EXPECTED."`)
- Rust: `///` for public items, `//!` for module-level docs

**Pattern (Lisp):**
```lisp
(defstruct (node (:constructor make-node (&key kind value children props)))
  "Universal AST node. Dispatch on (node-kind n) using etypecase."
  (kind nil))
```

**Pattern (Rust):**
```rust
//! Database connection management
//!
//! Expects PostgreSQL via SSH tunnel on port 5433

/// Create a connection pool to the PostgreSQL database
pub async fn create_pool(database_url: &str) -> Result<DbPool> {
```

## Function Design

**Size:** No hard limit — prefer readability over line count

**Parameters:**
- Lisp: Keyword arguments for >3 parameters or when order is unclear
- Rust: Use structs for complex parameter sets

**Return Values:**
- Lisp: Return value or `resistance` struct (not error) for missing resources
- Rust: `Result<T, E>` for fallible operations, bare `T` for infallible

## Module Design

**Exports (Common Lisp):**
- Explicit `:export` lists in `defpackage`
- Export symbols with `#:` reader syntax (e.g., `#:evaluate`)
- One package per logical module (e.g., `:innate.parser.tokenizer`, `:af64.runtime.perception`)

**Exports (Rust):**
- Use `pub` keyword for public items
- Re-export commonly used types in `lib.rs` (e.g., `pub use db::{DbPool, create_pool}`)
- Prefer `pub(crate)` for internal APIs

**Barrel Files:**
- Lisp: All packages declared in single `packages.lisp` file loaded first
- Rust: `mod.rs` files export submodules (e.g., `handlers/mod.rs`)

## AF64 Zero-Dependencies Policy

**Applies to:**
- `project-noosphere-ghosts/` (Common Lisp runtime)
- `innatescript/` (Common Lisp interpreter)

**Rules:**
- No Quicklisp dependencies
- ASDF + SBCL built-ins only
- Hand-rolled JSON parser (`util/json.lisp`)
- Hand-rolled HTTP client via `curl` subprocess (`util/http.lisp`)
- Hand-rolled PostgreSQL client via `libpq.so` FFI (`util/pg.lisp`)
- Hand-rolled YAML parser (`util/yaml.lisp`)
- Hand-rolled test framework (91-line `test-framework.lisp`)

**Not subject to zero-deps:**
- `dpn-core/` and `dpn-api/` (Rust projects use standard crates: `sqlx`, `axum`, `tokio`, `serde`, etc.)

## Package/Module Conventions

**Lisp Package Hierarchy:**
- Root package exports main API (e.g., `:innate`, `:af64`)
- Subpackages for implementation details (e.g., `:innate.parser`, `:af64.runtime.perception`)
- Utility packages prefixed `utils` (e.g., `:af64.utils.json`, `:af64.utils.pg`)

**Rust Crate Structure:**
- `dpn-core`: Shared library crate (`[lib]`)
- `dpn-api`: Binary crate (`[[bin]]`) with `main.rs`
- `dpn-core` exports high-level API in `lib.rs`
- Submodules in `src/` directories (e.g., `src/db/`, `src/handlers/`)

## Commit Message Conventions

**Format:** `<type>(<scope>): <subject>`

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `chore`: Maintenance (e.g., archiving files)
- `merge`: Merge commit

**Scopes:**
- Phase numbers (e.g., `31-01`, `phase-31`)
- Component names (e.g., `codebase`, `state`, `v1.5`)
- Module names (e.g., `31-tool-migration`)

**Examples:**
```
docs(codebase): map Modular Fortress codebase structure
feat(31-01): insert 75 tool definitions into area_content
fix(31-tool-migration): revise plans based on checker feedback
docs(phase-31): complete phase execution
merge: resolve conflicts from parallel wave 1 execution (31-01 + 31-02)
```

**Subject line:**
- Lowercase after colon
- No period at end
- Present tense imperative (e.g., "add", not "added" or "adds")

## ASDF System Organization

**Pattern:**
- Files: `project-name.asd` in project root
- Explicit `:depends-on` for each component (avoid `:serial t` for complex systems)
- Separate test system: `project-name/tests` depends on main system
- Pathname directives: `:pathname "src/"` or `:pathname "tests/"`

**Example from `innatescript.asd`:**
```lisp
(defsystem "innatescript"
  :pathname "src/"
  :serial nil
  :components
  ((:file "packages")
   (:file "types" :depends-on ("packages"))
   (:module "parser"
    :depends-on ("packages")
    :components
    ((:file "tokenizer")
     (:file "parser" :depends-on ("tokenizer"))))))

(defsystem "innatescript/tests"
  :depends-on ("innatescript")
  :pathname "tests/"
  :components
  ((:file "packages")
   (:file "test-framework" :depends-on ("packages"))))
```

## Special Conventions

**Dynamic Variables (Common Lisp):**

Use for implicit context threading:
```lisp
(defvar *current-test* nil "Name of currently executing test.")
(defvar *db-pool* nil "PostgreSQL connection pool.")
(defvar *api-key* nil "API authentication key.")
```

Pattern: bind dynamically in entry point, access without passing

**Environment Variables (AF64 Runtime):**

All runtime configuration uses env vars with `AF64_*` prefix:
- `AF64_PRIMARY_USER_HANDLE` — wiki handle reference
- `AF64_PERSONA_DIR` — persona files location
- `AF64_MEMORY_TABLE` — target table for memories
- `COGNITION_PROVIDER_CONFIG` — LLM provider chain config

**Struct vs Class (Common Lisp):**

Use `defstruct` by default:
- Faster than CLOS classes
- Accessors auto-generated
- Works for AST nodes, result types, job structs

Use `defclass` + `defmethod` when:
- Need multiple dispatch
- Building extensible protocols (e.g., resolver protocol)

---

*Convention analysis: 2026-04-04*
