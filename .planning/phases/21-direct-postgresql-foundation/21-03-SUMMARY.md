---
phase: 21-direct-postgresql-foundation
plan: 03
subsystem: database
tags: [postgresql, sbcl, lisp, ffi, libpq, agent-state, energy, tick-engine]

# Dependency graph
requires:
  - phase: 21-direct-postgresql-foundation/02
    provides: "db-client.lisp with perception SQL, db-fetch-agents, db-fetch-fitness"
provides:
  - "db-update-energy, db-get-energy, db-set-energy, db-update-agent-state in db-client.lisp"
  - "energy.lisp using direct SQL instead of HTTP"
  - "tick-engine phase-update-state using direct SQL instead of HTTP"
  - "Full perceive-rank-classify-update cycle running over PostgreSQL"
affects: [22-action-execution, tick-engine, energy, db-client]

# Tech tracking
tech-stack:
  added: []
  patterns: ["SQL RETURNING clause for atomic read-after-write", "GREATEST/LEAST clamping in SQL", "COALESCE + || for JSONB merge"]

key-files:
  created: []
  modified:
    - "/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp"
    - "/opt/project-noosphere-ghosts/lisp/runtime/energy.lisp"
    - "/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp"
    - "/opt/project-noosphere-ghosts/lisp/packages.lisp"

key-decisions:
  - "Removed all HTTP imports (api-get, api-patch, *api-base*, *api-key*) from tick-engine package since they are no longer used"
  - "db-update-energy uses RETURNING clause for atomic read-after-write in single round-trip"
  - "db-update-agent-state combines all field updates into single UPDATE statement"

patterns-established:
  - "SQL state updates via db-client.lisp: all agent state mutations go through db-update-* functions"
  - "Energy functions accept pool parameter but global *db-pool* is used at the energy.lisp layer"

requirements-completed: [DB-02]

# Metrics
duration: 3min
completed: 2026-03-29
---

# Phase 21 Plan 03: Agent State Updates via SQL Summary

**Energy updates, tier changes, and tick counters all converted from HTTP PATCH to direct PostgreSQL SQL -- the perceive-rank-classify-update cycle now runs entirely over the noosphere**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-29T17:09:55Z
- **Completed:** 2026-03-29T17:12:59Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Added 4 new SQL functions to db-client.lisp: db-update-energy, db-get-energy, db-set-energy, db-update-agent-state
- Rewired energy.lisp to use direct SQL -- removed all api-get and api-patch calls
- Rewired tick-engine phase-update-state to use db-update-agent-state -- removed api-patch
- Cleaned up tick-engine package imports: only api-post remains (for mark-read and tick-log, Phase 22 scope)
- Full smoke test: ASDF loads, 64 agents fetched, perception works, energy SQL works

## Task Commits

Each task was committed atomically:

1. **Task 1: Add state update SQL functions to db-client.lisp and rewire energy.lisp** - `a9f56ba` (feat)
2. **Task 2: Rewire tick-engine phase-update-state to SQL and verify full tick cycle** - `c5182c2` (feat)

## Files Created/Modified
- `/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp` - Added db-update-energy, db-get-energy, db-set-energy, db-update-agent-state functions
- `/opt/project-noosphere-ghosts/lisp/runtime/energy.lisp` - Rewired update-energy and get-energy to SQL, removed agent-path helper
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` - Rewired phase-update-state to db-update-agent-state
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` - Updated exports/imports for af64.runtime.db, af64.runtime.energy, af64.runtime.tick-engine

## Decisions Made
- Removed all HTTP imports (api-get, api-patch, *api-base*, *api-key*) from tick-engine package since no code references them anymore
- Used SQL RETURNING clause in db-update-energy for atomic read-after-write in a single query
- Combined all agent state fields into a single UPDATE in db-update-agent-state rather than multiple queries

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Cleaned up unused tick-engine HTTP imports**
- **Found during:** Task 2
- **Issue:** After removing api-patch from phase-update-state, the tick-engine package still imported api-get, api-patch, *api-base*, *api-key* which were all unused
- **Fix:** Removed unused imports, keeping only api-post (still needed for mark-read and tick-log)
- **Files modified:** /opt/project-noosphere-ghosts/lisp/packages.lisp
- **Verification:** ASDF system loads cleanly
- **Committed in:** c5182c2 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 missing critical)
**Impact on plan:** Cleanup of dead imports, no scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- The perceive-rank-classify-update tick cycle runs entirely over PostgreSQL
- Only api-post calls remain in tick-engine.lisp: mark-as-read and tick-log batch (Phase 22 scope)
- action-executor.lisp still uses HTTP for conversations, task mutations, and tool execution (Phase 22 scope)
- Phase 21 is now complete: all 3 plans delivered

---
*Phase: 21-direct-postgresql-foundation*
*Completed: 2026-03-29*

## Self-Check: PASSED
- All 4 modified files exist
- Both commit hashes (a9f56ba, c5182c2) found in git log
- All 4 db-client functions verified present
- Zero api-get/api-patch references in tick-engine.lisp and energy.lisp
