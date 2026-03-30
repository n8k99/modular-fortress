---
phase: 30-team-pipelines
plan: 01
subsystem: database, runtime
tags: [common-lisp, postgresql, jsonb, pipeline, area-content, caching]

# Dependency graph
requires:
  - phase: 27-area-content-tables
    provides: area_content table schema
  - phase: 22-ghost-sovereignty-sql
    provides: db-query direct SQL functions
provides:
  - 4 pipeline definitions in area_content with JSONB metadata
  - pipeline-definitions.lisp module with load/cache/accessor functions
  - Compound (pipeline, stage) key lookups eliminating stage name collisions
  - Edge-based DAG topology support for diamond-shaped pipelines
affects: [30-02, action-executor, action-planner, tick-engine]

# Tech tracking
tech-stack:
  added: []
  patterns: [edge-based DAG pipeline topology, order-based linear pipeline inference, terminal_stages for fork branch endpoints]

key-files:
  created:
    - /opt/project-noosphere-ghosts/lisp/runtime/pipeline-definitions.lisp
    - /opt/project-noosphere-ghosts/migrations/028_pipeline_definitions.sql
  modified:
    - /opt/project-noosphere-ghosts/lisp/packages.lisp
    - /opt/project-noosphere-ghosts/lisp/af64.asd
    - /opt/project-noosphere-ghosts/launch.sh

key-decisions:
  - "Edge-based DAG support for modular-fortress pipeline: order-based inference cannot capture skip-level and terminal fork branches"
  - "JSONB metadata returned as string from libpq, requires parse-json before hash-table access"
  - "terminal_stages array marks fork branches that go directly to done instead of next order group"

patterns-established:
  - "Dual pipeline topology: linear pipelines use order-based inference, DAG pipelines use explicit edges array"
  - "Per-tick reload from area_content: cache cleared and rebuilt each tick (~1ms cost)"

requirements-completed: [PIPE-01, PIPE-02]

# Metrics
duration: 13min
completed: 2026-03-30
---

# Phase 30 Plan 01: Pipeline Definitions Summary

**DB-sourced pipeline definitions in area_content with compound-key caching module replacing hardcoded defparameters**

## Performance

- **Duration:** 13 min
- **Started:** 2026-03-30T20:16:27Z
- **Completed:** 2026-03-30T20:29:24Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- 4 pipeline definitions stored in area_content with JSONB metadata (engineering, investment, editorial, modular-fortress)
- pipeline-definitions.lisp module with 14 functions: load, cache, and 5 accessor functions matching old defparameter shapes
- Compound (pipeline-name, stage) key lookups resolve the "research" stage collision between editorial and investment pipelines
- Edge-based DAG topology support for modular-fortress diamond pipeline (fork branches, terminal stages)

## Task Commits

Each task was committed atomically:

1. **Task 1: Insert pipeline definitions into area_content** - `865b148` (feat)
2. **Task 2: Create pipeline-definitions.lisp and update packages.lisp** - `e70c0d8` (feat)

## Files Created/Modified
- `/opt/project-noosphere-ghosts/lisp/runtime/pipeline-definitions.lisp` - New module: load/cache/accessor for DB-sourced pipelines
- `/opt/project-noosphere-ghosts/migrations/028_pipeline_definitions.sql` - Migration with 4 pipeline INSERT statements
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` - New package + imports into action-executor, action-planner, tick-engine
- `/opt/project-noosphere-ghosts/lisp/af64.asd` - Added pipeline-definitions to component list
- `/opt/project-noosphere-ghosts/launch.sh` - Added pipeline-definitions to load sequence

## Decisions Made
- Edge-based DAG support added for modular-fortress: the order-based inference approach cannot capture architecture-research -> synthesis (skips order 3) or security-standards -> done (terminal fork branch). Added explicit `edges` and `terminal_stages` fields to JSONB metadata.
- JSONB metadata from libpq returns as string, not hash-table: added parse-json step in load-pipeline-definitions.
- Engineering pipeline spec -> infra-review assignee differs between hardcoded (isaac) and plan SQL (casey): used plan SQL as canonical source since DB definitions are the authoritative data going forward.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] JSONB metadata returned as string, not hash-table**
- **Found during:** Task 2 (pipeline-definitions.lisp)
- **Issue:** db-query returns JSONB columns as raw strings from libpq, not parsed hash-tables
- **Fix:** Added `(if (stringp raw-meta) (parse-json raw-meta) raw-meta)` in load-pipeline-definitions
- **Files modified:** pipeline-definitions.lisp
- **Verification:** reload-pipeline-definitions returns 4 pipelines after fix
- **Committed in:** e70c0d8

**2. [Rule 1 - Bug] Order-based inference fails for DAG pipelines**
- **Found during:** Task 2 (pipeline-definitions.lisp)
- **Issue:** Modular-fortress diamond pipeline has skip-level edges (architecture-research -> synthesis skips order 3) and terminal fork branches (security-standards -> done) that cannot be inferred from order numbers alone
- **Fix:** Added edge-based topology support: if metadata contains `edges` array, use explicit edges instead of order inference. Added `terminal_stages` array for fork branches that end directly. Updated modular-fortress JSONB in DB.
- **Files modified:** pipeline-definitions.lisp, 028_pipeline_definitions.sql, area_content row
- **Verification:** All 28 advancement entries match hardcoded defparameter values
- **Committed in:** e70c0d8

---

**Total deviations:** 2 auto-fixed (2 bugs)
**Impact on plan:** Both fixes necessary for correct pipeline loading. Edge-based approach is more robust than plan's original order-only scheme.

## Issues Encountered
None beyond the auto-fixed deviations.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Pipeline definitions loaded and cached, ready for Plan 02 to swap action-executor and action-planner to use the new accessors
- All 5 exported accessor functions match the shapes expected by existing code
- The fork-cache and participants-cache provide scoped data that fixes the existing bug where all pipeline participants got energy rewards regardless of involvement

---
*Phase: 30-team-pipelines*
*Completed: 2026-03-30*
