---
phase: 11-message-hygiene
plan: 02
subsystem: runtime
tags: [common-lisp, tick-engine, mark-read, perception, cognition-broker]

# Dependency graph
requires:
  - phase: 11-01
    provides: "POST /api/conversations/mark-read endpoint and perception read_by filtering"
provides:
  - "Mark-read integration in tick engine — messages marked as read after cognition completes"
  - "End-to-end message hygiene: perceive(filtered) -> cognize -> mark-read -> next perceive(excluded)"
affects: [12-standing-orders, ghost-token-costs, agent-idle-detection]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Post-cognition mark-read via api-post in phase-process-cognition"]

key-files:
  created: []
  modified: ["/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp"]

key-decisions:
  - "Mark-read fires after ALL cognition results, not just those with action-detail (cached results need it too)"
  - "All perceived message IDs marked as read (not just responded-to), per Research pitfall #4"

patterns-established:
  - "Post-cognition side-effects: place outside (when action-detail) to cover cached broker results"

requirements-completed: [SPAM-02, SPAM-03]

# Metrics
duration: 13min
completed: 2026-03-27
---

# Phase 11 Plan 02: Mark-Read Integration Summary

**Tick engine marks all perceived messages as read after cognition via api-post to /api/conversations/mark-read, closing the perception-cognition-mark-read loop**

## Performance

- **Duration:** 13 min
- **Started:** 2026-03-27T10:45:35Z
- **Completed:** 2026-03-27T10:58:41Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Integrated mark-read API call into tick engine phase-process-cognition
- Messages marked as read after cognition completes (both fresh and cached results)
- Agents with 0 unread messages now correctly idle (no cognition job, no token spend)
- End-to-end verified: perception returns 0 messages for agents that processed everything

## Task Commits

Each task was committed atomically:

1. **Task 1: Add mark-read call to phase-process-cognition** - `c271c3d` (feat)
2. **Task 2: Restart ghosts and verify end-to-end + fix cached result handling** - `87a2016` (fix)

## Files Created/Modified
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` - Added perceptions parameter to phase-process-cognition, mark-read api-post after cognition, error handling

## Decisions Made
- Mark-read fires outside the `(when action-detail ...)` guard because cached broker results have nil action-detail but still represent processed cognition
- All message IDs from the perception snapshot are marked as read (not just the specific responded-to message), preventing re-processing

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Mark-read inside action-detail guard skipped for cached results**
- **Found during:** Task 2 (verification)
- **Issue:** Plan placed mark-read inside `(when action-detail ...)` but cached broker results return nil action-detail, meaning mark-read never fired for cache hits (the majority of results)
- **Fix:** Moved mark-read block outside `(when action-detail ...)` but still inside the `(let ((action-detail ...)))` form, so it fires for all cognition results
- **Files modified:** /opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp
- **Verification:** DB shows read_by populated for recent messages; perception returns 0 msgs for processed agents
- **Committed in:** 87a2016

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Essential fix for correctness. Without it, mark-read would never fire since broker cache serves most results.

## Issues Encountered
- Pre-existing perception errors for eliana, sarah, sylvia (`:NULL is not of type SEQUENCE`) -- not related to this plan, pre-existing issue with nil field handling in perception data
- Nova still has 1 unread message after verification (likely a message created during the tick itself) -- expected behavior, will be marked on next tick

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Message hygiene pipeline complete (SPAM-01, SPAM-02, SPAM-03 all resolved)
- Phase 11 done -- ready for Phase 12 (standing orders) or other v1.2 work
- Pre-existing perception errors for 3 agents should be investigated separately

---
*Phase: 11-message-hygiene*
*Completed: 2026-03-27*
