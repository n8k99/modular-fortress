---
phase: 12-standing-orders
plan: 02
subsystem: runtime
tags: [cron, lisp, tick-engine, action-planner, standing-orders, schedule-boost]

# Dependency graph
requires:
  - phase: 12-01
    provides: cron-matcher.lisp module, schedule JSONB on projects, perception schedule metadata
provides:
  - tick engine schedule evaluation with +50 urgency boost on cron match
  - double-fire prevention via last-fire tracking per project:label
  - project review prompt enrichment with Standing Orders Fired section
  - *schedule-fired-labels* exported for cross-module access
affects: [13-operations, 14-editorial, 15-financial]

# Tech tracking
tech-stack:
  added: []
  patterns: [cron-schedule-tick-integration, schedule-boost-ranking, standing-order-prompt-enrichment]

key-files:
  created: []
  modified:
    - /opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp
    - /opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp
    - /opt/project-noosphere-ghosts/lisp/packages.lisp
    - /opt/project-noosphere-ghosts/launch.sh

key-decisions:
  - "Schedule boost uses +50 urgency, same weight as message boost, ensuring executives enter acting set when standing orders fire"
  - "Double-fire prevention uses universal-time comparison with 60-second window, not minute-boundary alignment"
  - "STAND-03 satisfied by existing flow -- conversation output already attributed to acting executive agent-id"

patterns-established:
  - "Schedule-fired-labels pattern: tick-engine populates hash table, action-planner reads it for prompt enrichment"
  - "Error handling on cron parse: handler-case wraps cron-matches-p call so bad expressions log warnings without crashing the tick"

requirements-completed: [STAND-02, STAND-03]

# Metrics
duration: 2min
completed: 2026-03-28
---

# Phase 12 Plan 02: Standing Orders Tick Engine Integration Summary

**Cron schedule evaluation wired into tick engine ranking with +50 boost, double-fire prevention, and Standing Orders Fired prompt enrichment for executive project reviews**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-28T03:19:09Z
- **Completed:** 2026-03-28T03:21:22Z
- **Tasks:** 2 (1 auto + 1 checkpoint pre-approved)
- **Files modified:** 4

## Accomplishments
- Tick engine evaluates cron schedules each tick using UTC time components and injects matching project owners into acting-set with +50 schedule-boost
- Double-fire prevention tracks last fire time per project:label pair with 60-second deduplication window
- Project review cognition prompt includes "Standing Orders Fired" section listing which labels triggered the review
- STAND-03 confirmed satisfied by existing conversation attribution flow (from-agent set to cognition-result agent-id)
- Full system loads and passes all integration checks (SBCL compilation, symbol exports, hash table state)

## Task Commits

Each task was committed atomically:

1. **Task 1: Schedule check in tick engine ranking + action planner prompt enrichment** - `cfb4f4f` (feat) -- in project-noosphere-ghosts sub-repo
2. **Task 2: E2E verification checkpoint** - pre-approved by user (DB schedules on projects #10, #12, #14 confirmed; API returns schedule data; perception includes schedule arrays)

## Files Created/Modified
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` - Added *schedule-fired-labels*, *last-schedule-fire*, schedule-boost in phase-rank, clrhash in run-tick
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` - Added schedule-context lookup and Standing Orders Fired section in build-project-review-job prompt
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` - Added cron-matches-p import to tick-engine, exported *schedule-fired-labels*
- `/opt/project-noosphere-ghosts/launch.sh` - Added cron-matcher to load order

## Decisions Made
- Schedule boost uses +50 urgency (same as message boost) -- ensures standing orders reliably trigger executive cognition even when other boosts are low
- Double-fire prevention uses 60-second universal-time window rather than minute-boundary alignment, avoiding edge cases at tick boundaries
- No changes needed for STAND-03 -- verified that existing action-executor conversation posting already sets from-agent to the cognition result's agent-id

## Deviations from Plan

None - plan executed exactly as written. Code was already committed in the sub-repo as part of atomic execution.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Standing orders framework is complete: DB schedules, API management, perception metadata, cron matching, tick engine integration, and prompt enrichment all wired end-to-end
- Phases 13 (Operations), 14 (Editorial), and 15 (Financial) can now proceed -- they depend on this framework to schedule their respective pipeline work
- Projects #10 (Kathryn/Financial), #12 (Sylvia/Editorial), #14 (Nova/Operations) all have schedules seeded and ready

## Self-Check: PASSED

All files verified present. Commit cfb4f4f verified in project-noosphere-ghosts repo.

---
*Phase: 12-standing-orders*
*Completed: 2026-03-28*
