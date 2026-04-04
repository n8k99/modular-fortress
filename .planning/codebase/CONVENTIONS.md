# Coding Conventions

**Analysis Date:** 2026-04-03

## Naming Patterns

**Files:**

Common Lisp:
- `packages.lisp` — package definitions (loaded first)
- `kebab-case.lisp` — all other source files
- `test-*.lisp` — test files with `test-` prefix
- System definitions: `project-name.asd`

Rust:
- `snake_case.rs` — all source files
- `lib.rs` — library entry point
- `main.rs` — binary entry point
- `mod.rs` — module definitions

**Functions:**

Common Lisp:
- `kebab-case` for all functions and methods
- Predicate suffix: `-p` (e.g., `resistance-p`, `node-p`)
- Internal helpers: prefix with `%` (e.g., `%report-failure`)
- Generic functions: `defgeneric` for protocol definitions
- Constants: `+constant+` with plus signs (e.g., `+node-program+`)

Rust:
- `snake_case` for functions and methods
- Type constructors: `new()` or `make_*` pattern
- Async functions: marked with `async fn`
- Error conversion: implement `From<T>` traits

**Variables:**

Common Lisp:
- `kebab-case` for locals
- Dynamic variables: `*asterisk-wrapped*` (e.g., `*test-registry*`, `*db-pool*`)
- Environment variables: `AF64_*` prefix (uppercase with underscores)

Rust:
- `snake_case` for all variables
- Constants: `SCREAMING_SNAKE_CASE`
- Type parameters: `CamelCase` single letters or descriptive names

**Types:**

Common Lisp:
- `defstruct` names: lowercase with hyphens (e.g., `innate-result`, `cognition-job`)
- Condition types: `-condition` or `-error` suffix (e.g., `innate-parse-error`, `innate-resistance`)
- Package names: `:package.submodule` (e.g., `:innate.parser`, `:af64.utils.json`)

Rust:
- `CamelCase` for structs, enums, traits
- Error types: `*Error` suffix (e.g., `ApiError`)
- Type aliases: `CamelCase`

## Code Style

**Formatting:**

Common Lisp:
- No external formatter — hand-formatted
- Indentation: 2 spaces per level
- Opening paren stays on same line as form name
- Closing parens stack on final line
- Comment headers: `;;;; filename — description` (4 semicolons for file headers)
- Section comments: `;;; Section Name` (3 semicolons)
- Line comments: `;; explanation` (2 semicolons, indented with code)
- Inline comments: `; inline note` (1 semicolon, after code)

Rust:
- Use `rustfmt` (standard Rust formatter)
- Edition 2021
- Line length: default (100 chars)
- No explicit config files found — uses Rust defaults

**Linting:**

Common Lisp:
- No linter — manual review and SBCL warnings
- Compile with `(declaim (optimize (safety 3) (debug 3)))` during development
- Zero external dependencies policy (no Quicklisp)

Rust:
- Standard `cargo clippy` (no custom config found)
- Warnings as errors: not enforced by default
- Use `#[allow(clippy::*)]` sparingly for legitimate cases

## Import Organization

**Common Lisp:**

Order:
1. `(in-package :package.name)` — always first
2. No `:use` except `:cl` in defpackage
3. Explicit `:import-from` for cross-package symbols
4. Export list in package definition

Pattern from `innatescript/src/packages.lisp`:
```lisp
(defpackage :innate.parser
  (:use :cl)
  (:import-from :innate.parser.tokenizer
    #:token-type #:token-value #:tokenize)
  (:import-from :innate.types
    #:make-node #:node-kind #:+node-program+)
  (:export #:parse))
```

Path Aliases:
- None — use explicit package prefixes when needed

**Rust:**

Order:
1. Standard library imports
2. External crate imports
3. Internal crate imports
4. Module declarations

Pattern from `dpn-api/src/handlers/af64_agents.rs`:
```rust
use axum::{
    extract::{Path, Query, State},
    Json,
};
use dpn_core::DbPool;
use serde::Deserialize;
use serde_json::Value;
use sqlx::Row;

use crate::error::ApiError;
```

## Error Handling

**Common Lisp:**

Patterns:
- Condition system with `define-condition`
- Parse errors: inherit from `error` — unrecoverable
- Resistance conditions: inherit from `condition` — use `signal`, not `error`
- `handler-case` for recovery
- `restart-case` for fulfillment protocol

Example from `innatescript/src/conditions.lisp`:
```lisp
(define-condition innate-resistance (innate-condition condition)
  ((message :initarg :message :reader resistance-condition-message)
   (source  :initarg :source  :reader resistance-condition-source))
  (:report (lambda (condition stream)
             (format stream "Innate resistance: ~a (from: ~a)"
                     (resistance-condition-message condition)
                     (resistance-condition-source condition)))))
```

Resistance values vs conditions:
- Return `(make-resistance :message "..." :source "...")` for soft failures
- Signal `innate-resistance` condition for propagatable errors

**Rust:**

Patterns:
- `thiserror` for error type definitions
- `anyhow` for error propagation in non-library code
- `Result<T, E>` return types
- `?` operator for propagation
- Custom `ApiError` enum with `IntoResponse` for HTTP handlers

Example from `dpn-api/src/error.rs`:
```rust
#[derive(Debug)]
pub enum ApiError {
    Database(String),
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}
```

## Logging

**Common Lisp:**

Framework: Hand-rolled or `format` to stderr

Patterns:
- Development: `(format t "Debug: ~a~%" value)`
- Production: structured writes to log files
- No external logging framework (zero-deps policy)

**Rust:**

Framework: `tracing` crate

Patterns:
- Use `tracing::info!`, `tracing::error!`, etc.
- Initialize with `tracing-subscriber` in `main.rs`
- Environment filter via `RUST_LOG`

## Comments

**Common Lisp:**

When to Comment:
- File headers: always include `;;;; filename — description`
- Function purpose: docstrings in `defun`, `defstruct`, `define-condition`
- Complex algorithms: explain why, not what
- Protocol implementations: reference protocol name

Docstring convention:
```lisp
(defun parse-json (text)
  "Parse JSON TEXT string into Lisp data structures.
Returns hash tables for objects, vectors for arrays, keywords for keys."
  ...)
```

**Rust:**

When to Comment:
- Module documentation: `//!` at top of file
- Public API documentation: `///` before public items
- Implementation notes: `//` inline
- Complex logic: explain intent

Example:
```rust
//! AF64 Agent endpoints

/// GET /api/agents/:id
pub async fn get_agent(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    ...
}
```

## Function Design

**Common Lisp:**

Size:
- Small focused functions preferred
- Test helpers: single assertion each
- Evaluator: dispatch via `etypecase` on node kind

Parameters:
- Required first, optional/keyword after
- Dynamic variables for implicit context (e.g., `*resolver*`)

Return Values:
- Multiple values: use `values` form
- Structs: return `defstruct` instances
- Lists: use for ordered collections
- Hash tables: use for key-value data

**Rust:**

Size:
- Functions stay focused and testable
- Handler functions: single responsibility (one endpoint)

Parameters:
- Use Axum extractors for HTTP handlers (`State`, `Path`, `Query`, `Json`)
- Generic types for reusable code

Return Values:
- `Result<T, E>` for fallible operations
- `impl Trait` for complex return types
- `Json<Value>` for HTTP responses

## Module Design

**Common Lisp:**

Exports:
- Explicit `:export` list in package definition
- Only export public API symbols
- Use qualified symbols for cross-package access

Package structure (innatescript):
```
:innate.types
:innate.conditions
:innate.parser.tokenizer
:innate.parser
:innate.eval.resolver
:innate.eval
:innate.repl
:innate (facade)
```

Barrel Files:
- Not applicable — Common Lisp uses package system

**Rust:**

Exports:
- `pub` for public items
- `pub use` for re-exports
- Private by default

Module structure (dpn-api):
```
src/
├── main.rs
├── error.rs
├── auth.rs
└── handlers/
    ├── mod.rs
    ├── af64_agents.rs
    └── af64_tasks.rs
```

## AF64 Zero-Dependencies Policy

**Critical Convention:**

Projects under AF64 framework follow a **strict zero external dependencies** policy:

**Common Lisp:**
- No Quicklisp
- No external libraries
- Hand-roll utilities: JSON parser, HTTP client (via curl subprocess), PostgreSQL client (via libpq FFI)
- ASDF system definition only
- Standard library only (`:use :cl`)

**Rust (dpn-core/dpn-api):**
- Rust projects are **exempt** from zero-deps policy
- Use Cargo ecosystem normally
- Standard production crates allowed: `axum`, `sqlx`, `tokio`, `serde`, `anyhow`, `thiserror`, `tracing`

**Why this matters:**
The AF64 ghosts are Common Lisp native. InnateScipt must be compatible with that ecosystem. All `.lisp` files in `project-noosphere-ghosts/lisp/` and `innatescript/` implement hand-rolled utilities to avoid external dependencies.

Example hand-rolled utilities:
- `af64.utils.json` — JSON parser/encoder (`project-noosphere-ghosts/lisp/util/json.lisp`)
- `af64.utils.pg` — PostgreSQL client via SB-ALIEN FFI (`project-noosphere-ghosts/lisp/util/pg.lisp`)
- `af64.utils.http` — HTTP via curl subprocess (`project-noosphere-ghosts/lisp/util/http.lisp`)
- `innate.tests` — Test framework (3 macros) (`innatescript/tests/test-framework.lisp`)

## ASDF System Organization

**Pattern:**

Files:
- `project-name.asd` in project root
- Explicit `:depends-on` for each component
- `:serial t` acceptable for utility modules, avoid for complex systems
- Separate test system: `project-name/tests`

Example from `innatescript/innatescript.asd`:
```lisp
(defsystem "innatescript"
  :description "Innate language interpreter"
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
   (:file "test-framework" :depends-on ("packages"))
   (:file "test-parser" :depends-on ("packages" "test-framework"))))
```

## Documentation Standards

**Common Lisp:**

Required:
- File header with `;;;; filename — description`
- Docstrings for all public functions
- System `:description` in `.asd` file
- README.md for project overview

**Rust:**

Required:
- Module docs (`//!`) in each file
- Public API docs (`///`) for public functions
- `README.md` with usage examples
- `Cargo.toml` description field

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

*Convention analysis: 2026-04-03*
