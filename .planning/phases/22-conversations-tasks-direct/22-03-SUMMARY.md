---
phase: 22-conversations-tasks-direct
plan: 03
subsystem: database
tags: [postgresql, lisp, sbcl, sql-wrappers, http-elimination]

requires:
  - phase: 22-conversations-tasks-direct plan 01
    provides: SQL wrapper functions (db-get-conversations, db-get-tasks-by-filter, db-get-agent-by-id, etc.)
  - phase: 22-conversations-tasks-direct plan 02
    provides: action-executor.lisp and tick-engine.lisp already rewired to SQL
provides:
  - Zero HTTP calls remain in any ghost tick engine file
  - action-planner.lisp reads conversations, tasks, agents, documents, decisions via SQL
  - All 7 auxiliary files (drive, tick-reporting, cognition-broker, tool-socket, task-scheduler, user-profile, empirical-rollups) use SQL wrappers
  - Phase 22 success criterion met -- dpn-api serves frontends only
affects: [tick-engine, ghost-runtime, dpn-api]

tech-stack:
  added: []
  patterns: [direct-sql-from-lisp, db-wrapper-functions, zero-http-tick-path]

key-files:
  created: []
  modified:
    - lisp/runtime/action-planner.lisp
    - lisp/runtime/tick-reporting.lisp
    - lisp/runtime/drive.lisp
    - lisp/runtime/cognition-broker.lisp
    - lisp/runtime/tool-socket.lisp
    - lisp/runtime/task-scheduler.lisp
    - lisp/runtime/user-profile.lisp
    - lisp/runtime/empirical-rollups.lisp
    - lisp/runtime/db-tasks.lisp
    - lisp/packages.lisp
    - lisp/runtime/action-executor.lisp

key-decisions:
  - "Removed api-post calls from empirical-rollups instead of wrapping -- no DB routes existed, they were silently failing"
  - "Added project-id and scheduled-at to db-tasks.lisp to support action-planner and task-scheduler callers"

patterns-established:
  - "All ghost tick engine files use only db-* wrapper functions for noosphere access"

requirements-completed: [DB-03, DB-04]

duration: 10min
completed: 2026-03-29
---

# Phase 22 Plan 03: Remaining Files HTTP-to-SQL Summary

**Rewired 22 HTTP calls across action-planner.lisp and 7 auxiliary files to direct SQL, completing zero-HTTP tick engine**

## Performance

- **Duration:** 10 min
- **Started:** 2026-03-29T18:08:17Z
- **Completed:** 2026-03-29T18:18:17Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments
- Replaced all 11 api-get calls in action-planner.lisp with SQL wrapper functions (db-get-conversations, db-get-tasks-by-filter, db-get-agent-by-id, db-get-document-by-id, db-search-documents, db-get-decisions, db-fetch-agents)
- Replaced all 11 HTTP calls across 7 auxiliary files with SQL wrappers (db-insert-tick-report, db-tick-drives, db-fulfill-drive, db-get-drives, db-insert-cognition-job, db-update-cognition-job, db-get-agent-by-id, db-get-tasks-by-filter, db-update-task)
- Combined with Plan 02: zero HTTP calls remain in any ghost tick engine file (verified via grep across all 10 files)
- SBCL loads cleanly with all changes

## Task Commits

Each task was committed atomically:

1. **Task 1: Rewire action-planner.lisp** - `ad7e0dd` (feat)
2. **Task 2: Rewire 7 auxiliary files** - `d4aa16e` (feat)

## Files Created/Modified
- `lisp/runtime/action-planner.lisp` - 11 HTTP reads replaced with SQL wrapper calls
- `lisp/runtime/tick-reporting.lisp` - api-post replaced with db-insert-tick-report
- `lisp/runtime/drive.lisp` - 3 HTTP calls replaced with db-tick-drives/db-fulfill-drive/db-get-drives
- `lisp/runtime/cognition-broker.lisp` - 3 HTTP calls replaced with db-insert/update-cognition-job
- `lisp/runtime/tool-socket.lisp` - 3 HTTP calls replaced with db-get-agent-by-id/db-get-tasks-by-filter
- `lisp/runtime/task-scheduler.lisp` - api-patch replaced with db-update-task
- `lisp/runtime/user-profile.lisp` - api-get replaced with db-get-agent-by-id
- `lisp/runtime/empirical-rollups.lisp` - 5 silently-failing api-post calls removed (no DB routes existed)
- `lisp/runtime/db-tasks.lisp` - Added project-id filter and scheduled-at parameter
- `lisp/packages.lisp` - Updated imports for action-planner and action-executor packages
- `lisp/runtime/action-executor.lisp` - Fixed extra closing paren in execute-work-task

## Decisions Made
- Removed api-post calls from empirical-rollups.lisp entirely instead of routing through db-insert-rollup no-op, because the API routes never existed and rollups persist to JSONL files directly
- Extended db-update-task with scheduled-at parameter to support task-scheduler recurrence
- Extended db-get-tasks-by-filter with project-id parameter to support action-planner project task queries

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added project-id filter to db-get-tasks-by-filter**
- **Found during:** Task 1 (action-planner rewiring)
- **Issue:** format-project-tasks calls api-get with project_id filter, but db-get-tasks-by-filter lacked project-id support
- **Fix:** Added :project-id keyword arg and WHERE clause to db-get-tasks-by-filter
- **Files modified:** lisp/runtime/db-tasks.lisp
- **Committed in:** ad7e0dd (Task 1 commit)

**2. [Rule 1 - Bug] Fixed extra closing paren in action-executor.lisp execute-work-task**
- **Found during:** Task 1 (SBCL load verification)
- **Issue:** Pre-existing bug from Plan 22-02: execute-work-task had an extra closing paren at line 420, causing the function to close prematurely and leaving lines 421+ as orphaned code with unbound variables
- **Fix:** Removed one closing paren to keep the function body intact
- **Files modified:** lisp/runtime/action-executor.lisp
- **Committed in:** ad7e0dd (Task 1 commit)

**3. [Rule 3 - Blocking] Added scheduled-at parameter to db-update-task**
- **Found during:** Task 2 (task-scheduler rewiring)
- **Issue:** task-scheduler handle-task-recurrence sets scheduled_at via api-patch, but db-update-task lacked scheduled-at support
- **Fix:** Added :scheduled-at keyword arg and SET clause to db-update-task
- **Files modified:** lisp/runtime/db-tasks.lisp
- **Committed in:** d4aa16e (Task 2 commit)

**4. [Rule 3 - Blocking] Added db-get-agent-by-id import to action-executor package**
- **Found during:** Task 2 (tool-socket rewiring)
- **Issue:** tool-socket.lisp (in action-executor package) uses db-get-agent-by-id but the package didn't import it
- **Fix:** Added :db-get-agent-by-id to (:import-from :af64.runtime.db-auxiliary ...) in packages.lisp
- **Files modified:** lisp/packages.lisp
- **Committed in:** d4aa16e (Task 2 commit)

---

**Total deviations:** 4 auto-fixed (1 bug, 3 blocking)
**Impact on plan:** All auto-fixes necessary for correctness and completing the migration. No scope creep.

## Issues Encountered
None beyond the deviations documented above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 22 is complete: zero HTTP calls remain in the ghost-to-noosphere tick path
- dpn-api now serves frontends (dpn-kb, em-site, n8k99-site) only
- Ghost tick engine communicates directly with PostgreSQL via libpq FFI
- Ready for Phase 23+ (Innate integration / noosphere resolver)

---
*Phase: 22-conversations-tasks-direct*
*Completed: 2026-03-29*
