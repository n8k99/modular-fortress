---
phase: 22-conversations-tasks-direct
plan: 02
subsystem: database
tags: [sbcl, lisp, postgresql, direct-sql, action-executor, tick-engine]

requires:
  - phase: 22-conversations-tasks-direct (plan 01)
    provides: SQL wrapper functions (db-insert-conversation, db-update-task, db-mark-read-batch, etc.)
provides:
  - action-executor.lisp fully migrated from HTTP to direct SQL (39 calls)
  - tick-engine.lisp fully migrated from HTTP to direct SQL (2 calls)
  - ~65% of all HTTP calls eliminated from ghost tick path
affects: [22-03-PLAN, tick-engine, ghost-runtime]

tech-stack:
  added: []
  patterns:
    - db-insert-conversation replaces api-post /api/conversations (to-agents as list, not vector)
    - db-update-task replaces api-patch /api/af64/tasks/:id (keyword args, not json-object)
    - db-get-tasks-by-filter replaces api-get query string filtering
    - Direct db-execute for classify mutations (department field not in db-update-task)

key-files:
  created: []
  modified:
    - lisp/runtime/action-executor.lisp
    - lisp/runtime/tick-engine.lisp

key-decisions:
  - "Used db-execute direct SQL for CLASSIFY mutations since db-update-task lacks department keyword"
  - "Converted vector to-agents to list format for db-insert-conversation compatibility"
  - "Used (when (hash-table-p response) (gethash :id response)) for safe id extraction from SQL returns"

patterns-established:
  - "Pattern: db-insert-conversation returns hash-table (not integer), use hash-table-p guard for :id extraction"
  - "Pattern: Direct db-execute for one-off field updates not covered by wrapper functions"

requirements-completed: [DB-03, DB-04]

duration: 8min
completed: 2026-03-29
---

# Phase 22 Plan 02: Action Executor + Tick Engine Direct SQL Summary

**Replaced all 41 HTTP calls in action-executor.lisp (39) and tick-engine.lisp (2) with direct PostgreSQL via SQL wrapper functions**

## Performance

- **Duration:** 8 min
- **Started:** 2026-03-29T18:08:13Z
- **Completed:** 2026-03-29T18:16:49Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- action-executor.lisp: zero HTTP calls remain; 13 db-insert-conversation, 12 db-update-task, 3 db-get-project-by-id, 2 db-get-tasks-by-filter, 2 db-get-task-by-id, 2 db-insert-document, 2 db-update-request, 1 db-create-task, 1 db-insert-decision, 1 db-upsert-daily-memory
- tick-engine.lisp: zero HTTP calls remain; 1 db-mark-read-batch, 1 db-insert-tick-log
- Full ASDF system loads cleanly (LOAD OK)

## Task Commits

Each task was committed atomically:

1. **Task 1: Rewire action-executor.lisp** - `ad7e0dd` (feat) -- 39 HTTP calls replaced with SQL wrappers; committed alongside 22-03 parallel agent changes
2. **Task 2: Rewire tick-engine.lisp** - `6a3faa7` (feat) -- 2 HTTP calls replaced with SQL wrappers

## Files Created/Modified
- `lisp/runtime/action-executor.lisp` - All 39 HTTP calls replaced with direct SQL wrapper function calls
- `lisp/runtime/tick-engine.lisp` - Mark-read and tick-log HTTP calls replaced with db-mark-read-batch and db-insert-tick-log

## Decisions Made
- Used direct db-execute SQL for CLASSIFY task mutations because db-update-task wrapper doesn't support department field updates
- Converted json-array to-agents to list format since db-insert-conversation expects a Lisp list (not a vector/json-array)
- Added (when (hash-table-p response) ...) guards around response id extraction since SQL wrappers return hash-tables instead of the API's JSON objects

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Direct SQL for CLASSIFY department field**
- **Found during:** Task 1 (apply-task-mutations CLASSIFY section)
- **Issue:** db-update-task doesn't have a :department keyword argument, but CLASSIFY needs to set department
- **Fix:** Used direct db-execute SQL for the department+assignee update in classify mutations
- **Files modified:** lisp/runtime/action-executor.lisp
- **Verification:** SBCL loads cleanly, grep confirms zero HTTP calls
- **Committed in:** ad7e0dd (Task 1 commit)

**2. [Rule 1 - Bug] Safe hash-table extraction for response ids**
- **Found during:** Task 1 (multiple functions)
- **Issue:** Old code used (gethash :id response) assuming response was always a hash-table; SQL wrappers may return nil on error
- **Fix:** Added (when (hash-table-p response) (gethash :id response)) guards everywhere response ids are extracted
- **Files modified:** lisp/runtime/action-executor.lisp
- **Verification:** SBCL loads cleanly
- **Committed in:** ad7e0dd (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** Both fixes necessary for correctness. No scope creep.

## Issues Encountered
- Task 1 commit was absorbed into parallel agent's 22-03 commit (ad7e0dd) since both agents modified action-executor.lisp concurrently. The parallel agent also fixed a closing paren issue in execute-work-task. All 39 HTTP replacements are correctly committed.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- action-executor.lisp and tick-engine.lisp are HTTP-free
- Plan 03 (action-planner.lisp) was committed in parallel by another agent
- All critical tick-path files now use direct SQL
- System loads cleanly, ready for production testing

---
*Phase: 22-conversations-tasks-direct*
*Completed: 2026-03-29*
