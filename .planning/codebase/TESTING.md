# Testing Patterns

**Analysis Date:** 2026-04-04

## Test Framework

**Runner:**
- Rust: `cargo test` (built-in test framework)
- `tokio-test` version 0.4 for async test utilities
- Config: `Cargo.toml` `[dev-dependencies]` section

**Assertion Library:**
- Standard Rust macros: `assert!`, `assert_eq!`, `assert_ne!`
- Custom error messages supported: `assert!(cond, "message: {:?}", val)`
- No additional assertion libraries

**Run Commands:**
```bash
# Run all tests
cargo test

# Run tests in noosphere crate
cd /Volumes/Elements/Modular\ Fortress/noosphere && cargo test

# Run specific test
cargo test test_create_pool

# Run with single thread (for database tests)
cargo test -- --test-threads=1

# Show output even on success
cargo test -- --nocapture
```

## Test File Organization

**Location:**
- Co-located in module: `tests.rs` within module directories
- Pattern: `noosphere/src/core/db/tests.rs` for database tests
- Pattern: `dpn-core/src/db/tests.rs` (identical copy)
- No separate `tests/` integration directory detected

**Naming:**
- Test files: `tests.rs` within feature modules
- Test modules: `#[cfg(test)] mod tests { ... }` for inline tests

**Structure:**
```
noosphere/src/core/db/
├── connection.rs
├── memories.rs
├── documents.rs
├── tasks.rs
├── events.rs
├── projects.rs
├── mod.rs
└── tests.rs          # All db module tests (326 lines)
```

**Co-location Pattern:**
- Database tests in `src/core/db/tests.rs`
- No handler tests detected
- No cache tests detected
- No API endpoint tests detected

## Test Structure

**Suite Organization:**
```rust
//! Tests for the db module
//!
//! These tests require an active SSH tunnel to the PostgreSQL database:
//! ssh -L 5433:127.0.0.1:5432 root@144.126.251.126 -N -f
//!
//! Run tests with: cargo test -- --test-threads=1

use super::connection::{create_pool, test_connection, DEFAULT_DATABASE_URL};
use super::memories;
use super::documents;

#[tokio::test]
async fn test_create_pool() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await;
    assert!(pool.is_ok(), "Failed to create pool: {:?}", pool.err());
}
```

**Patterns:**
- File header with prerequisites (SSH tunnel instructions)
- Import from parent module: `use super::*;`
- Each test function marked `#[tokio::test]` for async
- Setup in test body (no separate setup functions)
- Multiple assertions per test allowed
- Descriptive failure messages

**Test Organization:**
- Grouped by table/feature with section comments
- Example: `// ============================================================================`
- Example: `// Documents Canonical/Dedup Tests`
- Example: `// ============================================================================`

## Mocking

**Framework:** None detected

**Patterns:**
- Tests use real PostgreSQL connections
- No mocking of database layer
- No HTTP client mocking
- No external service mocking

**What to Mock:**
- Not currently implemented

**What NOT to Mock:**
- Core business logic
- Database queries (uses real database)

## Fixtures and Factories

**Test Data:**
- No fixture files detected
- Use live database data
- Query existing data for test IDs/paths

**Pattern (query-first testing):**
```rust
#[tokio::test]
async fn test_get_memory_by_path() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");

    // First get any note to find a valid path
    let notes = memories::list_light(&pool, 1, 0).await.expect("list_light failed");
    assert!(!notes.is_empty(), "Need at least one memory to test");

    let test_path = &notes[0].path;

    // Now fetch by that path
    let note = memories::get_by_path(&pool, test_path).await;
    assert!(note.is_ok(), "get_by_path failed: {:?}", note.err());
}
```

**Location:**
- No separate fixtures directory
- No test data files
- Relies on existing database content

## Coverage

**Requirements:** No enforced coverage targets

**View Coverage:**
```bash
# Not configured, but would use:
cargo tarpaulin --out Html
cargo llvm-cov --html
```

**Current Coverage:**
- Database layer: Partial (memories, documents, tasks tested)
- API handlers: None detected
- Cache layer: None detected
- Business logic: Minimal

## Test Types

**Unit Tests:**
- Scope: Individual database functions
- Approach: Test each query function in isolation
- Location: Inline in `tests.rs` files

**Integration Tests:**
- Scope: Full database operations end-to-end
- Approach: Test query → result validation
- Example: `test_list_canonical_documents()` tests list + verify structure

**E2E Tests:**
- Not implemented
- No HTTP endpoint tests
- No full workflow tests

## Common Patterns

**Async Testing:**
```rust
#[tokio::test]
async fn test_get_tasks_due_on() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");

    let today = chrono::Local::now().date_naive();
    let tasks = tasks::get_tasks_due_on(&pool, today).await;

    assert!(tasks.is_ok(), "get_tasks_due_on failed: {:?}", tasks.err());
}
```

**Error Testing:**
```rust
#[tokio::test]
async fn test_get_versions() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");

    // This should succeed even if the column doesn't exist (returns empty vec)
    let versions = documents::get_versions(&pool, 1).await;
    assert!(versions.is_ok(), "get_versions should not error: {:?}", versions.err());
}
```

**Conditional Assertions:**
```rust
#[tokio::test]
async fn test_get_open_tasks_due_on() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    let today = chrono::Local::now().date_naive();

    let tasks = tasks::get_open_tasks_due_on(&pool, today).await;
    assert!(tasks.is_ok());

    let tasks = tasks.unwrap();
    // Verify all returned tasks are open
    for task in &tasks {
        assert!(task.is_open(), "All tasks should be open, but got status: {}", task.status);
    }
}
```

**Data Validation:**
```rust
#[tokio::test]
async fn test_get_all_titles_canonical() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");

    let titles = documents::get_all_titles_canonical(&pool).await;
    assert!(titles.is_ok());

    let titles = titles.unwrap();
    assert!(!titles.is_empty(), "Expected titles but got empty list");

    // We have ~47K documents, so should have many titles
    assert!(titles.len() > 1000, "Expected >1000 titles, got {}", titles.len());

    // Verify tuple structure
    let (id, title) = &titles[0];
    assert!(*id > 0, "ID should be positive");
    assert!(!title.is_empty(), "Title should not be empty");
}
```

**Date Testing:**
```rust
use chrono::NaiveDate;

#[tokio::test]
async fn test_get_overdue_tasks() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    let today = chrono::Local::now().date_naive();

    let tasks = tasks::get_overdue_tasks(&pool, today).await;
    assert!(tasks.is_ok());

    let tasks = tasks.unwrap();
    // Verify all returned tasks are overdue and open
    for task in &tasks {
        assert!(task.is_open(), "Overdue tasks should be open");
        if let Some(ref due) = task.due_date {
            let due_date = NaiveDate::parse_from_str(due, "%Y-%m-%d")
                .expect("Due date should be valid");
            assert!(due_date < today, "Task should be overdue");
        }
    }
}
```

## Test Database Requirements

**Prerequisites:**
- Active SSH tunnel to remote PostgreSQL
- Command: `ssh -L 5433:127.0.0.1:5432 root@144.126.251.126 -N -f`
- Local port: 5433
- Remote port: 5432
- Connection string: `postgres://chronicle:chronicle2026@127.0.0.1:5433/master_chronicle`

**Database State:**
- Tests expect existing data (~2,678 memories, ~47K documents)
- Tests query existing records for validation
- Tests do NOT create/modify/delete data
- Read-only test approach

**Known Data Volumes:**
- Memories: >100 expected, ~2,678 actual
- Documents: >40,000 expected, ~47K actual
- Titles: >1,000 expected

## Test Execution

**Threading:**
- Single-threaded recommended: `cargo test -- --test-threads=1`
- Reason: Shared database connection pool
- Default (parallel) may cause connection contention

**Output:**
- Default: Only failed tests show output
- Verbose: `cargo test -- --nocapture` shows all output

**Performance:**
- Database tests are slow (network latency)
- Typical test: 50-200ms per test
- Full suite: ~5-15 seconds

## Test Naming Conventions

**Pattern:** `test_<operation>_<target>`

**Examples:**
- `test_create_pool()` — basic setup
- `test_list_memories()` — list operation
- `test_get_memory_by_path()` — specific lookup
- `test_search_canonical_documents()` — complex query
- `test_format_tasks_for_daily_note()` — formatting function

**Descriptive Names:**
- `test_get_open_tasks_due_on()` — clear what's tested
- `test_get_all_titles_canonical()` — specific variant
- `test_get_versions()` — fallback behavior tested

## Known Test Files

**Database Tests:**
- `noosphere/src/core/db/tests.rs` (326 lines)
- `dpn-core/src/db/tests.rs` (identical copy, 326 lines)

**Test Categories:**
1. Connection tests (pool creation, connectivity)
2. Memory tests (list, get, search, count)
3. Document tests (canonical, versions, search)
4. Task tests (due dates, overdue, formatting)

**Test Count:**
- ~20-25 test functions total
- All async (`#[tokio::test]`)
- All integration-style (use real database)

## Python Testing

**Framework:**
- No formal test framework detected
- Scripts use manual validation

**Test Files:**
- `doltgres-data/test_connection.py` — connection validation
- No unittest or pytest usage detected

**Approach:**
- Scripts run and report success/failure
- No structured test suites
- Manual verification

## Test Coverage Gaps

**What's Tested:**
- Database connection pool creation
- Memory CRUD operations (list, get by ID, get by path, search, count)
- Document queries (list, canonical, search, titles, versions)
- Task queries (due dates, open tasks, overdue, formatting)
- Event queries (assumed similar pattern)
- Project queries (assumed similar pattern)

**What's NOT Tested:**
- API handlers (`noosphere/src/api/handlers/*`) — zero tests
- HTTP endpoints (health, ghosts, tasks, conversations) — zero tests
- Cache layer (`noosphere/src/core/cache/*`) — zero tests
- Hybrid store (offline-first) — zero tests
- Sync queue — zero tests
- Embeddings generation — zero tests
- Context injection — zero tests
- Deduplication logic — zero tests
- Notification webhooks — zero tests
- Pipeline automation — zero tests
- RSS reader — zero tests
- Wikilink parsing — zero tests
- Graph generation — zero tests
- Timeline building — zero tests
- Conversations messaging — zero tests
- Error handling paths — minimal tests
- Validation logic — zero tests
- Authentication — zero tests
- Concurrent request handling — zero tests

**Priority: Critical**
- API handlers completely untested
- Cache layer untested (offline-first is core feature)
- Business logic layers untested

**Priority: High**
- Error paths and edge cases
- Validation logic
- Concurrent access patterns

**Priority: Medium**
- Helper utilities (wikilinks, graph, timeline)
- Notification system
- RSS reader

## Testing Best Practices (Observed)

**Good Practices:**
1. Descriptive test names
2. Clear failure messages with context
3. Header documentation with prerequisites
4. Query-first pattern for test data
5. Validation of data structure, not just success
6. Date handling with chrono
7. Conditional assertions for optional data

**Missing Practices:**
1. Test data fixtures
2. Setup/teardown functions
3. Test isolation (tests share database)
4. Mocking external dependencies
5. Property-based testing
6. Performance benchmarks
7. Integration test coverage
8. HTTP endpoint testing
9. Error path coverage
10. Test coverage reporting

---

*Testing analysis: 2026-04-04*
