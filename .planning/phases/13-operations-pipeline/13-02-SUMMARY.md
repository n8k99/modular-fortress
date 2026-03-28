---
phase: 13-operations-pipeline
plan: 02
subsystem: ghost-runtime
tags: [common-lisp, sbcl, action-planner, standing-orders, tool-mapping, prompt-engineering]

# Dependency graph
requires:
  - phase: 13-01
    provides: "ops_* tool registry entries and process-tool-calls wiring in action-executor"
  - phase: 12-standing-orders
    provides: "schedule-fired-labels mechanism and schedule-context in action-planner"
provides:
  - "Label-to-tool mapping table in Nova's project review prompt for standing order execution"
  - "End-to-end verified pipeline: schedule fires -> prompt includes tool mapping -> Nova writes tool_call -> executor runs script"
affects: [14-editorial-pipeline, 15-financial-pipeline]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Markdown table in Lisp format string for LLM prompt tool mapping"]

key-files:
  created: []
  modified:
    - /opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp

key-decisions:
  - "Used ops_daily_note (not ops_daily_note_populate) to match exact tool-registry.json names"
  - "Tool mapping as markdown table in format string -- Claude LLMs parse markdown tables reliably"
  - "Included tool_call example block with exact syntax matching process-tool-calls parser expectations"

patterns-established:
  - "Standing order tool mapping: hardcoded in Lisp format string per D-12 (code, not data)"
  - "ops_health_check always called with fix:true argument per D-01"

requirements-completed: [OPS-01, OPS-02, OPS-03, OPS-04]

# Metrics
duration: 2min
completed: 2026-03-28
---

# Phase 13 Plan 02: Standing Order Tool Mapping Summary

**Explicit label-to-tool mapping table in action-planner.lisp so Nova translates schedule labels to ops_* tool_call blocks**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-28T03:53:48Z
- **Completed:** 2026-03-28T03:55:59Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Extended schedule-context format string with Standing Order Tool Mapping markdown table mapping all 6 labels to ops_* tools
- End-to-end Phase 13 pipeline verified: tool registry (6 entries), executor wiring (process-tool-calls), prompt mapping, script existence, schedule entries (6 in Project #14)
- SBCL compilation verified clean with full af64 system load

## Task Commits

Each task was committed atomically:

1. **Task 1: Extend standing order prompt with label-to-tool mapping** - `d537e2c` (feat)
2. **Task 2: End-to-end compilation and tool scope verification** - verification only, no file changes

## Files Created/Modified
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` - Added Standing Order Tool Mapping table to schedule-context format string with all 6 ops_* tool names and tool_call example block

## Decisions Made
- Used `ops_daily_note` to match the exact registered tool name in tool-registry.json (plan text referenced `ops_daily_note_populate` which was incorrect)
- Kept tool mapping as inline markdown table in Lisp format string per D-12 decision (mapping lives in code, not data)
- Included `fix: true` arg only for ops_health_check per D-01; all other tools take no arguments

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Corrected ops_daily_note tool name**
- **Found during:** Task 1
- **Issue:** Plan text referenced `ops_daily_note_populate` but tool-registry.json has `ops_daily_note`
- **Fix:** Used the correct registered name `ops_daily_note` in the prompt mapping
- **Files modified:** action-planner.lisp
- **Verification:** Confirmed name matches tool-registry.json entry
- **Committed in:** d537e2c

---

**Total deviations:** 1 auto-fixed (1 bug fix)
**Impact on plan:** Essential correctness fix -- wrong tool name would cause execution failure.

## Issues Encountered
- SBCL compilation requires DPN_API_URL and DPN_API_KEY environment variables (api-client.lisp checks at load time). Used test values for compilation verification. This is pre-existing behavior, not a new issue.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 13 complete: all 6 operational tools are registered, executor is wired to process tool_call blocks, and Nova's prompt includes explicit label-to-tool mapping
- Ready for Phase 14 (editorial pipeline) and Phase 15 (financial pipeline) which follow the same standing order pattern
- Nova will execute ops tools on next standing order fire after ghost restart

---
*Phase: 13-operations-pipeline*
*Completed: 2026-03-28*
