---
phase: 22-conversations-tasks-direct
plan: 01
subsystem: database
tags: [postgresql, lisp, sbcl, sql-wrappers, direct-db, conversations, tasks]

# Dependency graph
requires:
  - phase: 21-direct-postgresql-foundation
    provides: "db-client.lisp with db-query, db-execute, db-escape, connection pool"
provides:
  - "db-conversations.lisp: db-insert-conversation, db-mark-read-batch, db-get-conversations"
  - "db-tasks.lisp: db-update-task, db-create-task, db-get-tasks-by-filter, db-get-task-by-id, db-complete-and-unblock"
  - "db-auxiliary.lisp: 18 SQL wrapper functions for tick-log, tick-reports, drives, decisions, documents, agents, memory, requests, projects, cognition jobs"
  - "pg-text-array and pg-int-array helper functions for PostgreSQL array construction"
affects: [22-02-PLAN, 22-03-PLAN]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Domain-separated SQL wrapper files (db-conversations, db-tasks, db-auxiliary)"
    - "handler-case per SQL call for error isolation"
    - "pg-text-array/pg-int-array for safe PostgreSQL array construction"
    - "db-query-single for INSERT...RETURNING, db-execute for UPDATE/DELETE without RETURNING"

key-files:
  created:
    - "lisp/runtime/db-conversations.lisp"
    - "lisp/runtime/db-tasks.lisp"
    - "lisp/runtime/db-auxiliary.lisp"
  modified:
    - "lisp/packages.lisp"
    - "lisp/af64.asd"
    - "launch.sh"

key-decisions:
  - "Cognition job DB functions implemented as no-ops since broker manages jobs in-memory with local file persistence"
  - "db-insert-rollup is a no-op since rollups already persist to local JSONL files"
  - "db-tasks package placed after cognition-types in load order due to generate-uuid import dependency"

patterns-established:
  - "SQL wrapper file pattern: in-package, import from db + json, handler-case per function, db-escape all values"
  - "pg-text-array returns ARRAY[escaped]::varchar(50)[] for PostgreSQL text array columns"
  - "pg-int-array returns ARRAY[n]::integer[] for PostgreSQL integer array columns"

requirements-completed: [DB-03, DB-04]

# Metrics
duration: 7min
completed: 2026-03-29
---

# Phase 22 Plan 01: SQL Wrapper Library Summary

**26 SQL wrapper functions across 3 domain files (conversations, tasks, auxiliary) providing complete DB operation library for Plans 02 and 03 to rewire HTTP calls**

## Performance

- **Duration:** 7 min
- **Started:** 2026-03-29T17:58:33Z
- **Completed:** 2026-03-29T18:05:41Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Created db-conversations.lisp with 3 functions: insert, mark-read-batch, get with filtering
- Created db-tasks.lisp with 5 functions: update, create, get-by-filter, get-by-id, complete-and-unblock (atomic transaction)
- Created db-auxiliary.lisp with 18 functions: tick-log, tick-report, drives (tick/fulfill/get), decisions, documents, agent-by-id, daily-memory upsert, request update, project-by-id, document-by-id, document search, get-decisions, rollup, and 3 cognition-job no-ops
- Added pg-text-array and pg-int-array helper functions for safe PostgreSQL array construction
- Wired 3 new packages into packages.lisp with exports, updated 10 consuming packages with imports
- Updated af64.asd and launch.sh load order
- Full SBCL system loads cleanly with all new files

## Task Commits

Each task was committed atomically:

1. **Task 1: Create db-conversations.lisp, db-tasks.lisp, and db-auxiliary.lisp** - `48aba1e` (feat)
2. **Task 2: Wire new packages into packages.lisp, af64.asd, and launch.sh** - `a62df8f` (feat)

## Files Created/Modified
- `lisp/runtime/db-conversations.lisp` - Conversation SQL operations (insert, mark-read, get)
- `lisp/runtime/db-tasks.lisp` - Task SQL operations (update, create, filter, complete-and-unblock)
- `lisp/runtime/db-auxiliary.lisp` - All other SQL operations (26 functions including helpers)
- `lisp/packages.lisp` - 3 new defpackage declarations + 10 consuming packages updated
- `lisp/af64.asd` - 3 new files added to runtime module
- `launch.sh` - 3 new files added to load order

## Decisions Made
- Cognition job functions (db-get/insert/update-cognition-job) implemented as no-ops since the broker manages jobs in-memory and persists to local files -- the original API calls were fire-and-forget (ignore-errors)
- db-insert-rollup implemented as no-op since empirical-rollups.lisp already writes to local JSONL files
- db-tasks defpackage placed after cognition-types in load order to resolve generate-uuid import dependency
- db-auxiliary loaded before db-conversations and db-tasks since both import pg-text-array/pg-int-array from it

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed package load order for db-tasks**
- **Found during:** Task 2 (package wiring)
- **Issue:** db-tasks imports generate-uuid from cognition-types, but initial placement put db-tasks before cognition-types in packages.lisp
- **Fix:** Moved db-tasks defpackage to after cognition-types; ensured af64.asd and launch.sh maintain cognition-types before db-tasks
- **Files modified:** lisp/packages.lisp
- **Verification:** SBCL loads all files without errors
- **Committed in:** a62df8f (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Package ordering fix was necessary for correct compilation. No scope creep.

## Issues Encountered
None beyond the package ordering fix documented above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All SQL wrapper functions are defined and callable from any runtime package
- Plans 02 and 03 can now substitute HTTP calls with direct SQL calls using these wrappers
- No API client imports need to be removed yet -- that happens when Plans 02/03 actually rewire the call sites

---
*Phase: 22-conversations-tasks-direct*
*Completed: 2026-03-29*
