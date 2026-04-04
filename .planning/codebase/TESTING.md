# Testing Patterns

**Analysis Date:** 2026-04-04

## Test Framework

**Runner (Common Lisp):**
- Hand-rolled test harness (91 lines in `innatescript/tests/test-framework.lisp`)
- Pattern: Practical Common Lisp chapter 9 (`deftest`/`check`/`combine-results`)
- Config: None — pure Lisp macros

**Runner (Rust):**
- `cargo test` (built-in test framework)
- `tokio-test` for async test utilities
- Config: `[dev-dependencies]` section in `Cargo.toml`

**Assertion Library (Lisp):**
- Custom macros: `assert-equal`, `assert-true`, `assert-nil`, `assert-signals`
- No external dependencies

**Assertion Library (Rust):**
- Standard `assert!`, `assert_eq!`, `assert_ne!` macros
- No additional libraries

**Run Commands:**
```bash
# Common Lisp (innatescript, af64)
./run-tests.sh                # Run all tests
./run-tests.sh tokenizer      # Run tests matching "tokenizer"

# Rust (dpn-core, dpn-api)
cargo test                    # Run all tests
cargo test --lib              # Library tests only
cargo test integration        # Tests matching "integration"
```

## Test File Organization

**Location (Common Lisp):**
- Co-located in `tests/` directory at project root
- Separate ASDF system (e.g., `:innatescript/tests` depends on `:innatescript`)
- Pattern: One test file per source module (e.g., `test-tokenizer.lisp` for `parser/tokenizer.lisp`)

**Location (Rust):**
- Inline: `#[cfg(test)]` modules at bottom of source files
- Integration tests: `tests/` directory (not currently used in these projects)

**Naming (Lisp):**
- Prefix `test-` for test files (e.g., `test-parser.lisp`, `test-evaluator.lisp`)
- Test packages mirror source packages with `.tests` suffix (e.g., `:innate.tests.tokenizer`)

**Naming (Rust):**
- No separate test files — tests inline in source

**Structure (Lisp):**
```
innatescript/
├── src/
│   ├── packages.lisp
│   ├── types.lisp
│   ├── parser/
│   │   ├── tokenizer.lisp
│   │   └── parser.lisp
│   └── eval/
│       ├── resolver.lisp
│       └── evaluator.lisp
└── tests/
    ├── packages.lisp          # Test package definitions
    ├── test-framework.lisp    # Test harness
    ├── smoke-test.lisp        # Harness verification
    ├── test-tokenizer.lisp
    ├── test-parser.lisp
    ├── test-resolver.lisp
    └── test-evaluator.lisp
```

**Structure (Rust):**
- No separate test directory structure — tests live inline

## Test Structure

**Suite Organization (Lisp):**
```lisp
;;;; test-tokenizer.lisp — tests for the Innate tokenizer (Phase 3)
;;;; Covers TOK-01 through TOK-18

(in-package :innate.tests.tokenizer)

;;; ─── Task 1: Single-character token tests (TOK-01 through TOK-10) ───

(deftest test-single-bracket-tokens
  (let ((lbrak (tokenize "[")))
    (assert-equal 1 (length lbrak) "lbracket: one token")
    (assert-equal :lbracket (token-type (first lbrak)) "lbracket type")))

(deftest test-single-punctuation-tokens
  (assert-equal :colon (token-type (first (tokenize ":"))) "colon")
  (assert-equal :comma (token-type (first (tokenize ","))) "comma"))
```

**Patterns (Lisp):**
- File header comments document phase and task coverage
- Section dividers group related tests (e.g., `;;; ─── Task 1: ... ───`)
- `deftest` macro defines named tests
- `let` bindings for setup (e.g., tokenizing input)
- Multiple assertions per test allowed
- Descriptive string labels for each assertion

**Suite Organization (Rust):**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection() {
        let pool = create_pool(DEFAULT_DATABASE_URL).await.unwrap();
        let result = test_connection(&pool).await;
        assert!(result.is_ok());
    }
}
```

**Patterns (Rust):**
- `#[cfg(test)]` module at file bottom
- `use super::*;` imports parent module
- `#[test]` attribute for sync tests
- `#[tokio::test]` attribute for async tests
- Inline setup in test body

## Mocking

**Framework (Lisp):** Stub resolver pattern

**Patterns:**
- `stub-resolver` struct with in-memory hash tables
- Provides `stub-add-entity`, `stub-add-wikilink`, `stub-add-bundle` functions
- Fulfills resolver protocol without external substrate
- Used for testing evaluator in isolation

**Pattern (Lisp stub resolver):**
```lisp
(let ((resolver (make-stub-resolver)))
  (stub-add-entity resolver "count" "42")
  (let* ((env (make-eval-env :resolver resolver))
         (result (evaluate ast env)))
    (assert-equal "42" (innate-result-value result))))
```

**Framework (Rust):** Not currently used

**What to Mock:**
- Lisp: External substrate calls (database, API) via resolver protocol
- Rust: Not actively mocked — tests use real database connections

**What NOT to Mock:**
- Lisp: Core interpreter logic (tokenizer, parser, evaluator)
- Rust: Core business logic

## Fixtures and Factories

**Test Data (Lisp):**
- Inline strings for small inputs (e.g., `(tokenize "[foo]")`)
- File reading for larger inputs (e.g., `burg_pipeline.dpn`)
- Helper functions prefixed `%` (e.g., `%read-file-to-string`)

**Pattern (Lisp file fixtures):**
```lisp
(defun %read-file-to-string (path)
  "Read entire file at PATH into a string."
  (with-open-file (stream path :direction :input)
    (let ((contents (make-string (file-length stream))))
      (read-sequence contents stream)
      contents)))

(deftest test-burg-pipeline-tokenizes
  (let* ((source (%read-file-to-string "burg_pipeline.dpn"))
         (tokens (tokenize source)))
    (assert-true (> (length tokens) 0) "token list is non-empty")))
```

**Location (Lisp):**
- Test data files in project root (e.g., `burg_pipeline.dpn`)
- No separate `fixtures/` directory

**Location (Rust):**
- No fixtures currently used

## Coverage

**Requirements:** No enforced coverage targets

**View Coverage:**
- Lisp: Manual review — no coverage tooling
- Rust: `cargo tarpaulin` (not configured in these projects)

## Test Types

**Unit Tests (Lisp):**
- Scope: Individual functions and modules
- Approach: Test tokenizer, parser, resolver, evaluator in isolation
- Example: `test-tokenizer.lisp` tests `tokenize` function with various inputs

**Unit Tests (Rust):**
- Scope: Individual functions
- Approach: Inline `#[test]` functions in source files
- Example: `test_connection` in `db/connection.rs`

**Integration Tests (Lisp):**
- Scope: End-to-end interpreter pipeline
- Approach: Parse and evaluate complete `.dpn` files
- Example: `test-burg-pipeline-tokenizes` reads full file and verifies token output

**Integration Tests (Rust):**
- Scope: Not actively used
- Approach: Would live in `tests/` directory (not present)

**E2E Tests:**
- Not used in either Lisp or Rust projects

## Common Patterns

**Async Testing (Rust):**
```rust
#[tokio::test]
async fn test_create_pool() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await;
    assert!(pool.is_ok());
}
```

**Error Testing (Lisp):**
```lisp
(deftest test-string-unterminated
  (assert-signals innate-parse-error
    (tokenize "\"hello")
    "unterminated string signals parse error"))
```

**Pattern:** Use `assert-signals` macro to verify condition is signaled

**Positive/Negative Testing (Lisp):**
```lisp
(deftest test-wikilink-vs-nested-brackets
  ;; Positive case: [[Burg]] is wikilink
  (let ((toks (tokenize "[[Burg]]")))
    (assert-equal :wikilink (token-type (first toks))))
  ;; Negative case: [[sylvia[command]]] is nested brackets
  (let ((toks (tokenize "[[sylvia[command]]]")))
    (assert-equal 8 (length toks) "not a wikilink")))
```

## Test Harness Implementation

**Hand-Rolled Framework (91 lines):**
- `innatescript/tests/test-framework.lisp`
- Three core macros: `assert-equal`, `assert-true`, `assert-nil`, `assert-signals`
- One registration macro: `deftest`
- One runner function: `run-tests`

**Key Features:**
- Global test registry: `*test-registry*` alist
- Failure counter: `*test-failures*` per test
- Selective execution: `(run-tests "prefix")` runs matching tests
- Exit code: Returns `T` if all pass, `NIL` if any fail

**Pattern (deftest macro):**
```lisp
(defmacro deftest (name &body body)
  "Define a named test. Registers it in *test-registry*.
   NAME is a symbol. BODY is a sequence of assertions."
  `(progn
     (defun ,name ()
       (let ((*current-test* ,(symbol-name name)))
         (format t "  ~a ... " ,(symbol-name name))
         (let ((*test-failures* 0))
           ,@body
           (if (zerop *test-failures*)
               (format t "PASS~%")
               (format t "FAIL (~a failure~:p)~%" *test-failures*))
           (zerop *test-failures*))))
     (pushnew (cons ,(symbol-name name) #',name) *test-registry*
              :key #'car :test #'string=)
     ',name))
```

## ASDF System Organization

**Test System (Lisp):**
- Separate system definition (e.g., `:innatescript/tests`)
- Depends on main system (e.g., `:depends-on ("innatescript")`)
- Pathname pointing to `tests/` directory

**Pattern (ASDF):**
```lisp
(defsystem "innatescript/tests"
  :description "Test suite for the Innate interpreter"
  :depends-on ("innatescript")
  :pathname "tests/"
  :components
  ((:file "packages")
   (:file "test-framework"  :depends-on ("packages"))
   (:file "smoke-test"      :depends-on ("packages" "test-framework"))
   (:file "test-tokenizer"  :depends-on ("packages" "test-framework"))))
```

## Shell Integration

**Test Runner Script:**
- `innatescript/run-tests.sh`
- Wipes project FASL cache before run (cold-load guarantee)
- Loads test system via ASDF
- Calls `(innate.tests:run-tests)` with optional filter
- Exit code matches test result (0 = pass, 1 = fail)

**Pattern (run-tests.sh):**
```bash
#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CACHE_ROOT="${HOME}/.cache/common-lisp"

# Wipe project FASL cache
find "${CACHE_ROOT}" -type d -path "*/${SCRIPT_DIR#/}" -prune -exec rm -rf {} +

# Run tests
sbcl --non-interactive \
  --eval "(require :asdf)" \
  --eval "(asdf:load-system :innatescript/tests)" \
  --eval "(let ((result (innate.tests:run-tests)))
            (sb-ext:exit :code (if result 0 1)))"
```

## Test Coverage Gaps

**innatescript:**

What's tested:
- Tokenizer: basic token types, whitespace handling, position tracking
- Parser: all node types, nesting, error cases
- Evaluator: two-pass, decree hoisting, reference resolution
- Conditions: error signaling, resistance propagation
- Resolver protocol: stub implementation, entity resolution

What's NOT tested:
- REPL: interactive mode, error recovery
- File loading: `run-file` function
- Performance: large input handling
- Edge cases: deeply nested structures (>100 levels)

Priority: Medium — core interpreter is covered, REPL and file I/O are thin wrappers

**dpn-api/dpn-core:**

What's tested:
- Basic connection tests inline in source

What's NOT tested:
- Unit tests: handler logic, error paths
- Database edge cases: NULL handling, constraint violations
- Concurrent requests: race conditions
- HTTP endpoints: health, documents, tasks, events (no integration tests found)

Priority: High — minimal test coverage for production API

**project-noosphere-ghosts:**

What's tested:
- Basic PG client: `tests/test-pg.lisp`

What's NOT tested:
- Tick engine: perception, action planning, execution
- Cognition broker: provider chain, caching, winter/thaw
- Tool registry: tool execution, argument validation
- Memory rollups: daily/weekly/monthly aggregation
- Standing orders: cron matching, task injection

Priority: Critical — zero test coverage for core runtime loops

---

*Testing analysis: 2026-04-04*
