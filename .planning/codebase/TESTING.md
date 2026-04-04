# Testing Patterns

**Analysis Date:** 2026-04-03

## Test Framework

**Common Lisp (innatescript, project-noosphere-ghosts):**

Runner:
- Hand-rolled test harness (zero external dependencies)
- Config: No config file — tests defined inline with `deftest` macro
- Pattern: Practical Common Lisp chapter 9 (Seibel's test framework)
- Location: `innatescript/tests/test-framework.lisp`

Core macros:
```lisp
(deftest test-name
  (assert-equal expected actual "description")
  (assert-true form "description")
  (assert-nil form "description")
  (assert-signals condition-type form "description"))
```

Test registry:
- Global `*test-registry*` alist
- Auto-registered by `deftest`
- Filtered runs: `(run-tests "prefix")`

Run Commands:
```bash
./run-tests.sh              # Run all tests
./run-tests.sh parser       # Run tests matching "parser"
sbcl --eval '(asdf:load-system :innatescript/tests)' \
     --eval '(innate.tests:run-tests)'
```

**Rust (dpn-api, dpn-core):**

Runner:
- `cargo test` (standard Rust testing)
- Integration tests: `test_integration.sh` shell script
- Unit tests: inline with `#[cfg(test)]` modules

Run Commands:
```bash
cargo test                  # Run all unit tests
./test_integration.sh       # Run API integration tests
cargo test --lib            # Library tests only
cargo test --test integration  # Named integration test
```

## Test File Organization

**Common Lisp:**

Location:
- Separate `tests/` directory
- Co-located with source via ASDF system, not filesystem

Naming:
- `test-*.lisp` — all test files use `test-` prefix
- `test-framework.lisp` — test harness implementation
- `smoke-test.lisp` — quick validation tests
- `test-evaluator.lisp` — module-specific tests

Structure:
```
innatescript/
├── src/
│   ├── packages.lisp
│   ├── types.lisp
│   └── parser/
│       ├── tokenizer.lisp
│       └── parser.lisp
└── tests/
    ├── packages.lisp
    ├── test-framework.lisp
    ├── test-types.lisp
    ├── test-tokenizer.lisp
    └── test-parser.lisp
```

**Rust:**

Location:
- Unit tests: inline `#[cfg(test)]` modules in source files
- Integration tests: `tests/` directory (standard Cargo convention)
- Script tests: `*.sh` files in project root

Naming:
- No prefix needed — Cargo recognizes `tests/` directory
- Integration scripts: `test_*.sh` with underscore

Structure:
```
dpn-api/
├── src/
│   ├── main.rs
│   └── handlers/
│       └── af64_agents.rs  (may contain #[cfg(test)] mod tests)
├── tests/
│   └── integration.rs
└── test_integration.sh
```

## Test Structure

**Common Lisp:**

Suite Organization:
```lisp
;;;; tests/test-evaluator.lisp — Two-pass evaluator tests

(in-package :innate.tests.evaluator)

;;; EVL-01: Two-pass - decree collected in pass 1
(deftest test-decree-collected-in-pass-1
  (let* ((env (make-eval-env :resolver (make-stub-resolver)))
         (ast (make-node :kind :program :children
                (list (make-node :kind :decree :value "greeting"
                        :children (list (make-node :kind :string-lit
                                                    :value "hello")))))))
    (evaluate ast env)
    (assert-true (gethash "greeting" (eval-env-decrees env)))))

;;; EVL-02: Reference resolution - decree before resolver
(deftest test-reference-resolves-decree-first
  ...)
```

Patterns:
- Setup: `let*` bindings for test data
- Assertions: inline with test logic
- Teardown: automatic (lexical scope cleanup)
- Each `deftest` is self-contained

**Rust:**

Suite Organization:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_from_anyhow() {
        let err = anyhow::anyhow!("test error");
        let api_err = ApiError::from(err);

        match api_err {
            ApiError::Internal(msg) => assert_eq!(msg, "test error"),
            _ => panic!("Expected Internal error"),
        }
    }
}
```

Integration tests:
```rust
// tests/integration.rs
use dpn_api::*;

#[tokio::test]
async fn test_health_endpoint() {
    let response = test_client().get("/health").send().await;
    assert!(response.status().is_success());
}
```

## Mocking

**Common Lisp:**

Framework: Hand-rolled stub resolver

Pattern:
```lisp
;; innatescript/src/eval/stub-resolver.lisp
(defclass stub-resolver ()
  ((entities :initform (make-hash-table :test #'equal))
   (wikilinks :initform (make-hash-table :test #'equal))
   (bundles :initform (make-hash-table :test #'equal))
   (contexts :initform (make-hash-table :test #'equal))
   (commissions :initform '())))

(defmethod resolve-reference ((resolver stub-resolver) name qualifiers)
  (let ((entity (gethash name (slot-value resolver 'entities))))
    (if entity
        (make-innate-result :value entity :context :query)
        (make-resistance :message (format nil "No stub for ~a" name)
                         :source name))))
```

Usage:
```lisp
(deftest test-with-stub
  (let* ((resolver (make-stub-resolver))
         (env (make-eval-env :resolver resolver)))
    (stub-add-entity resolver "user" "Alice")
    (let ((result (evaluate (parse "@user") env)))
      (assert-equal "Alice" (first result)))))
```

What to Mock:
- External substrate (resolver protocol)
- Database queries (stub-resolver)
- API responses (hand-rolled stubs)

What NOT to Mock:
- Pure functions (parser, AST construction)
- Data structures (nodes, results)
- Core logic (evaluator dispatch)

**Rust:**

Framework: No mocking library used (test against real DB or manual stubs)

Pattern (manual stubs):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn mock_pool() -> DbPool {
        // Return in-memory SQLite pool for tests
        sqlx::sqlite::SqlitePoolOptions::new()
            .connect(":memory:")
            .await
            .unwrap()
    }
}
```

Integration tests use live database or Docker containers.

## Fixtures and Factories

**Common Lisp:**

Test Data:
```lisp
;; Helper functions in test files
(defun make-test-ast ()
  (make-node :kind :program
             :children (list (make-node :kind :prose :value "test"))))

(defun make-test-env ()
  (make-eval-env :resolver (make-stub-resolver)))
```

Location:
- Inline helpers in test files
- No separate fixtures directory
- Construct fresh data per test (no global state)

**Rust:**

Test Data:
```rust
#[cfg(test)]
mod tests {
    fn sample_agent() -> Agent {
        Agent {
            id: "test-agent".into(),
            full_name: Some("Test Agent".into()),
            role: Some("Tester".into()),
            ..Default::default()
        }
    }
}
```

Database fixtures (integration tests):
```bash
# test_integration.sh sets up state
API_URL="${DPN_API_URL:-http://localhost:8080}"
API_KEY="${DPN_API_KEY:-test-key}"
```

## Coverage

**Common Lisp:**

Requirements: No enforced coverage target

Measurement:
- No automated coverage tool
- Manual verification: run tests, check SBCL compilation warnings

Approach:
- Write tests for each specification requirement (e.g., EVL-01, EVL-02)
- Test-driven development for parser and evaluator
- Smoke tests for basic integration

**Rust:**

Requirements: No enforced coverage target

Measurement:
```bash
cargo tarpaulin --out Html  # Requires tarpaulin installation
```

Approach:
- Unit tests for error handling
- Integration tests for HTTP endpoints
- No TDD pattern observed — tests added after implementation

## Test Types

**Common Lisp:**

Unit Tests:
- Scope: Single function or module
- Approach: Pure function testing with controlled inputs
- Examples: `test-tokenizer.lisp`, `test-parser.lisp`, `test-types.lisp`

Integration Tests:
- Scope: Multi-module interactions
- Approach: Test evaluator + parser + resolver together
- Examples: `test-evaluator.lisp` (uses parser + stub resolver)

E2E Tests:
- Scope: REPL or full script execution
- Framework: `smoke-test.lisp`
- Approach: Run actual `.dpn` scripts, verify output

Example smoke test:
```lisp
(deftest smoke-test-basic-parsing
  (let* ((input "# Hello World
This is prose.
")
         (ast (parse input)))
    (assert-equal :program (node-kind ast))))
```

**Rust:**

Unit Tests:
- Scope: Function-level, inline with source
- Pattern: `#[test]` attribute on functions
- Location: `#[cfg(test)] mod tests` at bottom of source file

Integration Tests:
- Scope: API endpoint testing
- Pattern: Shell script with curl commands
- Location: `test_integration.sh`

Example from `test_integration.sh`:
```bash
test_endpoint() {
    local name="$1"
    local method="$2"
    local endpoint="$3"
    local expected_field="$4"

    TESTS_RUN=$((TESTS_RUN + 1))
    echo -n "Testing $name... "

    response=$(api_request "$method" "$endpoint")

    if echo "$response" | grep -q "$expected_field"; then
        echo -e "${GREEN}✓ PASS${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ FAIL${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

test_endpoint "Health Check" "GET" "/health" "status"
test_endpoint "List Documents" "GET" "/api/documents?limit=5" "documents"
```

E2E Tests:
- Not currently implemented
- Would use: live database + full server startup

## Common Patterns

**Common Lisp:**

Async Testing:
- Not applicable — AF64 runtime uses synchronous tick model
- No async/await in Common Lisp tests

Error Testing:
```lisp
(deftest test-parse-error-signals
  (assert-signals innate-parse-error
                  (parse "!invalid syntax")
                  "Parser should signal error on invalid input"))

(deftest test-resistance-signaled
  (let* ((resolver (make-stub-resolver))
         (env (make-eval-env :resolver resolver)))
    (handler-case
        (progn
          (evaluate (parse "@missing") env)
          (error "Should have signaled resistance"))
      (innate-resistance (c)
        (assert-true (search "missing"
                             (resistance-condition-message c)))))))
```

Boundary Testing:
```lisp
(deftest test-empty-input
  (let ((ast (parse "")))
    (assert-equal :program (node-kind ast))
    (assert-nil (node-children ast))))

(deftest test-large-decree-body
  (let* ((large-string (make-string 10000 :initial-element #\x))
         (ast (parse (format nil "!x = \"~a\"" large-string))))
    (assert-equal :program (node-kind ast))))
```

**Rust:**

Async Testing:
```rust
#[tokio::test]
async fn test_async_endpoint() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

Error Testing:
```rust
#[test]
fn test_error_conversion() {
    let err = anyhow::anyhow!("test");
    let api_err = ApiError::from(err);

    match api_err {
        ApiError::Internal(msg) => assert_eq!(msg, "test"),
        _ => panic!("Wrong error variant"),
    }
}
```

## Test Execution

**Common Lisp:**

Shell script wrapper (`run-tests.sh`):
```bash
#!/usr/bin/env bash
set -e

# Wipe FASL cache for clean load
CACHE_ROOT="${HOME}/.cache/common-lisp"
find "${CACHE_ROOT}" -type d -path "*/${PROJECT_CACHE_SUFFIX}" -prune -exec rm -rf {} +

# Run tests
sbcl --non-interactive \
  --eval "(require :asdf)" \
  --eval "(asdf:load-system :innatescript/tests)" \
  --eval "(let ((result (innate.tests:run-tests)))
            (sb-ext:exit :code (if result 0 1)))"
```

Exit codes:
- `0` = all tests pass
- `1` = any test fails

Filtering:
```bash
./run-tests.sh parser    # Only run tests matching "parser"
```

**Rust:**

Standard Cargo:
```bash
cargo test                      # All tests
cargo test --lib                # Library only
cargo test integration          # Match "integration"
cargo test -- --nocapture       # Show output
cargo test -- --test-threads=1  # Serial execution
```

Integration script:
```bash
./test_integration.sh
# Outputs:
# Testing Health Check... ✓ PASS
# Testing List Documents... ✓ PASS
# ...
# Total Tests: 15
# Passed: 15
# Failed: 0
```

## CI/CD Integration

**Common Lisp:**

Not configured (manual testing workflow).

Expected pattern:
```yaml
# .github/workflows/test.yml
- name: Run Innate tests
  run: |
    sudo apt-get install sbcl
    cd innatescript
    ./run-tests.sh
```

**Rust:**

Not configured (manual testing workflow).

Expected pattern:
```yaml
# .github/workflows/test.yml
- name: Run Rust tests
  run: |
    cd dpn-api
    cargo test --all-features
    ./test_integration.sh
```

## Test Coverage Gaps

**innatescript:**

What's tested:
- Tokenizer: basic token types, whitespace handling
- Parser: all node types, nesting, error cases
- Evaluator: two-pass, decree hoisting, reference resolution
- Conditions: error signaling, resistance propagation

What's NOT tested:
- REPL: interactive mode, error recovery
- File loading: `run-file` function
- Performance: large input handling
- Edge cases: deeply nested structures (>100 levels)

Priority: Medium — core interpreter is covered, REPL and file I/O are thin wrappers

**dpn-api/dpn-core:**

What's tested (integration only):
- HTTP endpoints: health, documents, tasks, events, projects
- Query parameters: limit, filtering
- Authentication: API key validation

What's NOT tested:
- Unit tests: handler logic, error paths
- Database edge cases: NULL handling, constraint violations
- Concurrent requests: race conditions
- WebSocket endpoints (if any)

Priority: High — integration tests are brittle, need unit test foundation

**project-noosphere-ghosts:**

What's tested:
- Basic PG client: `tests/test-pg.lisp`

What's NOT tested:
- Tick engine: perception, action planning, execution
- Cognition broker: provider chain, caching, winter/thaw
- Tool registry: tool execution, argument validation
- Memory rollups: daily/weekly/monthly aggregation

Priority: Critical — zero test coverage for core runtime

---

*Testing analysis: 2026-04-03*
