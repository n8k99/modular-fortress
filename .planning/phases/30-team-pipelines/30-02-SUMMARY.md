---
phase: 30-team-pipelines
plan: 02
subsystem: runtime
tags: [common-lisp, pipeline, action-executor, action-planner, tick-engine, db-lookup]

# Dependency graph
requires:
  - phase: 30-team-pipelines
    plan: 01
    provides: pipeline-definitions.lisp with load/cache/accessor functions
provides:
  - DB-backed pipeline advancement replacing all hardcoded defparameters
  - Compound (pipeline, stage) key lookups in advance-pipeline
  - Scoped energy rewards per completing pipeline
  - Per-tick pipeline reload in tick engine
affects: [action-executor, action-planner, tick-engine, future pipeline changes]

# Tech tracking
tech-stack:
  added: []
  patterns: [DB-sourced pipeline data wired into runtime, compound-key stage lookups]

key-files:
  created: []
  modified:
    - /opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp
    - /opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp
    - /opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp
    - /opt/project-noosphere-ghosts/lisp/packages.lisp

key-decisions:
  - "Used get-pipeline-type-for-stage in action-planner to avoid circular dependency with action-executor"
  - "Preserved detect-pipeline-type wrapper function for backward compatibility per D-14"

patterns-established:
  - "Pipeline data accessed via compound (pipeline-name, stage) lookups, never by stage name alone"
  - "Pipeline reload happens once per tick before perception/action phases"

requirements-completed: [PIPE-03, PIPE-04]

# Metrics
duration: 5min
completed: 2026-03-30
---

# Phase 30 Plan 02: Pipeline Wiring Summary

**DB-loaded pipeline definitions wired into action-executor, action-planner, and tick-engine replacing all hardcoded pipeline data structures**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-30T20:32:15Z
- **Completed:** 2026-03-30T20:36:55Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Removed 35-line hardcoded *pipeline-advancement* defparameter from action-executor.lisp
- Replaced 20-line detect-pipeline-type function with single DB lookup call
- Replaced 28-line hardcoded prev-stage-map in action-planner with get-prev-stage DB call
- Wired reload-pipeline-definitions into tick engine perceive phase (loads 4 pipelines per tick)
- Fixed energy reward bug: now scoped to completing pipeline only instead of all pipelines globally

## Task Commits

Each task was committed atomically:

1. **Task 1: Replace hardcoded pipeline data in action-executor.lisp** - `fb4ec84` (feat)
2. **Task 2: Replace prev-stage-map in action-planner.lisp and wire reload into tick-engine.lisp** - `2ee1ef6` (feat)

## Files Created/Modified
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` - DB-backed pipeline advancement, fork handling, type detection, energy rewards
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` - DB-backed predecessor stage lookup
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` - Pipeline reload call in run-tick
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` - Added get-pipeline-type-for-stage import to action-planner

## Decisions Made
- Used `get-pipeline-type-for-stage` directly in action-planner instead of calling `detect-pipeline-type` from action-executor, avoiding circular package dependency
- Preserved `detect-pipeline-type` as a thin wrapper around `get-pipeline-type-for-stage` per D-14 (function signature stability)
- Left validate-stage-output hardcoded stage list as-is with TODO(Phase 31) comment -- affects tool execution validation only, not pipeline data

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Known Stubs
None - all pipeline data sources are fully wired to DB lookups.

## Next Phase Readiness
- All hardcoded pipeline data eliminated from runtime files
- System boots and logs "Loaded 4 pipeline definitions from DB" each tick
- Pipeline changes can now be made by updating area_content rows, no code changes needed
- validate-stage-output tool-execution stage list is the only remaining hardcoded pipeline-related data (tagged for Phase 31)

---
*Phase: 30-team-pipelines*
*Completed: 2026-03-30*
