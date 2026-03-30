---
phase: 21-direct-postgresql-foundation
plan: 01
subsystem: database
tags: [postgresql, libpq, ffi, sbcl, sb-alien, connection-pool, common-lisp]

# Dependency graph
requires:
  - phase: none (foundational layer)
    provides: n/a
provides:
  - libpq FFI bindings via SB-ALIEN (13 alien routines)
  - Connection pool with auto-reconnect (pg-pool struct)
  - pg-query and pg-execute for raw SQL
  - db-query, db-execute, db-escape high-level wrappers
  - db-coerce-row for type coercion of result columns
  - db-query-single convenience function
affects: [21-02 (perception migration), 21-03 (state update migration)]

# Tech tracking
tech-stack:
  added: [libpq.so.5 FFI via sb-alien]
  patterns: [unwind-protect for PQclear memory safety, pool-acquire/release pattern, json-keyword for column name hyphenation]

key-files:
  created:
    - /opt/project-noosphere-ghosts/lisp/util/pg.lisp
    - /opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp
    - /opt/project-noosphere-ghosts/lisp/tests/test-pg.lisp
  modified:
    - /opt/project-noosphere-ghosts/lisp/packages.lisp
    - /opt/project-noosphere-ghosts/lisp/af64.asd
    - /opt/project-noosphere-ghosts/launch.sh
    - /opt/project-noosphere-ghosts/config/provider-config.json

key-decisions:
  - "Used SB-ALIEN FFI to libpq.so.5 directly instead of Quicklisp/postmodern, maintaining AF64 zero-deps convention"
  - "Connection pool size of 2 (matching plan spec), sufficient for single-threaded tick engine"
  - "PQescapeLiteral for SQL injection prevention rather than parameterized queries (simpler FFI surface)"

patterns-established:
  - "Pool acquire/release with unwind-protect: every PQexec wrapped in unwind-protect with PQclear"
  - "Column name conversion: PQfname -> json-keyword for underscore-to-hyphen keyword mapping"
  - "Type dispatch in db-escape: nil->NULL, string->escaped, integer->unquoted, hash-table->JSON escaped"

requirements-completed: [DB-01, DB-02]

# Metrics
duration: 21min
completed: 2026-03-29
---

# Phase 21 Plan 01: libpq FFI Foundation Summary

**Direct PostgreSQL access from SBCL via libpq FFI with 2-connection pool, db-query/db-execute wrappers, and 11 passing smoke tests**

## Performance

- **Duration:** 21 min
- **Started:** 2026-03-29T16:34:27Z
- **Completed:** 2026-03-29T16:55:43Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- Created libpq FFI layer with 13 alien routines (PQconnectdb through PQerrorMessage + PQfreemem)
- Implemented connection pool with auto-reconnect, acquire/release, and shutdown
- Built high-level db-client wrappers matching api-client.lisp patterns (db-query, db-execute, db-escape, db-coerce-row)
- Full ASDF system load succeeds with new modules
- 11 end-to-end smoke tests passing against live master_chronicle (64 agents queried)

## Task Commits

Each task was committed atomically:

1. **Task 1: Create libpq FFI bindings and connection pool (pg.lisp)** - `bafe251` (feat)
2. **Task 2: Create db-client wrappers and verify end-to-end connectivity** - `5eec5cc` (feat)

## Files Created/Modified
- `lisp/util/pg.lisp` - libpq FFI bindings, connection pool, pg-query/pg-execute/pg-escape/result-to-vectors
- `lisp/runtime/db-client.lisp` - High-level wrappers: init-db-pool, db-query, db-execute, db-escape, db-coerce-row
- `lisp/tests/test-pg.lisp` - 11 smoke tests for pool, health, query, escape, NULL handling, coercion
- `lisp/packages.lisp` - Added af64.utils.pg and af64.runtime.db package definitions
- `lisp/af64.asd` - Added pg to util module, db-client to runtime module
- `launch.sh` - Added util/pg and runtime/db-client to load order
- `config/provider-config.json` - Added database connection config (gitignored)

## Decisions Made
- Used SB-ALIEN FFI to libpq.so.5 directly (zero-deps convention, no Quicklisp needed)
- Pool size of 2 connections: sufficient for single-threaded tick engine, minimal resource footprint
- PQescapeLiteral for SQL injection prevention rather than implementing PG wire protocol parameterized queries
- Column names converted via existing json-keyword (underscore-to-hyphen), maintaining AF64 convention
- provider-config.json updated locally but gitignored; connection defaults hardcoded in build-conninfo with env var overrides

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated launch.sh load order**
- **Found during:** Task 2 (db-client creation)
- **Issue:** launch.sh loads files directly (not via ASDF). New pg.lisp and db-client.lisp would not be loaded at runtime.
- **Fix:** Added "util/pg" after "util/json" and "runtime/db-client" after "runtime/api-client" in launch.sh dolist
- **Files modified:** launch.sh
- **Verification:** ASDF load succeeds; launch.sh load order verified correct
- **Committed in:** 5eec5cc (Task 2 commit)

**2. [Rule 1 - Bug] Fixed test-pg.lisp column reference**
- **Found during:** Task 2 (test creation)
- **Issue:** Test referenced avatar_url column which does not exist in agents table
- **Fix:** Changed to machine column which has known NULL values
- **Verification:** All 11 tests pass
- **Committed in:** 5eec5cc (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** Launch.sh update essential for runtime loading. Test fix was trivial column name correction. No scope creep.

## Issues Encountered
- provider-config.json is gitignored; database config applied locally but not tracked in git (security measure, acceptable)
- SBCL style warnings during load are normal forward-reference warnings from non-compiled sequential loading

## User Setup Required

None - no external service configuration required.

## Known Stubs

None - all functions are fully implemented and tested.

## Next Phase Readiness
- pg.lisp and db-client.lisp provide complete primitives for Plans 02 and 03
- db-query returns vector of hash-tables with hyphenated keywords, ready for perception migration
- db-execute ready for state update migration
- Pool initialized via init-db-pool, accessible via *db-pool* global

---
*Phase: 21-direct-postgresql-foundation*
*Completed: 2026-03-29*
