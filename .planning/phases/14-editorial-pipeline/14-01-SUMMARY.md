---
phase: 14-editorial-pipeline
plan: 01
subsystem: ghost-tools
tags: [common-lisp, python, tool-registry, action-planner, editorial, standing-orders]

# Dependency graph
requires:
  - phase: 13-operations-pipeline
    provides: "Standing order tool mapping pattern in action-planner.lisp and tool-registry.json"
  - phase: 12-standing-orders
    provides: "Schedule framework with *schedule-fired-labels* and Project #12 Nightly Editorial schedule"
provides:
  - "editorial_nightly tool registered in tool-registry.json with editorial scope"
  - "Dynamic per-label tool mapping in action-planner.lisp supporting multiple executives"
  - "nightly_editorial.py patched for ANTHROPIC_API_KEY env var auth"
  - "HEARTBEAT_OK output for no-comment days"
affects: [15-financial-pipeline]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Dynamic label-to-tool mapping via tool-mapping-for-label + build-tool-mapping-table"
    - "ANTHROPIC_API_KEY env var as primary auth with OAuth fallback"

key-files:
  created: []
  modified:
    - "/opt/project-noosphere-ghosts/config/tool-registry.json"
    - "/root/gotcha-workspace/tools/editorial/nightly_editorial.py"
    - "/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp"

key-decisions:
  - "Used cons (tool . args) return from tool-mapping-for-label for simple pair structure"
  - "Dynamic example tool_call block uses first fired label's tool instead of hardcoded ops_health_check"
  - "Auth fallback preserved for backwards compatibility with OpenClaw auth-profiles"

patterns-established:
  - "Per-label tool mapping: add new cond clause in tool-mapping-for-label for each new standing order"
  - "Dynamic tool mapping table: only shows tools relevant to fired labels, not all registered tools"

requirements-completed: [EDIT-01, EDIT-02]

# Metrics
duration: 3min
completed: 2026-03-28
---

# Phase 14 Plan 01: Editorial Pipeline Summary

**editorial_nightly registered with dynamic per-label tool mapping so Sylvia executes nightly editorial via standing order, not OpenClaw cron**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-28T05:20:31Z
- **Completed:** 2026-03-28T05:23:35Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Registered editorial_nightly tool in ghost tool-registry.json with editorial scope matching Sylvia's tool_scope
- Patched nightly_editorial.py to use ANTHROPIC_API_KEY env var (x-api-key header) as primary auth, with OAuth fallback
- Added HEARTBEAT_OK output when no reader comments found for the day
- Generalized action-planner label-to-tool mapping from hardcoded ops-only to dynamic per-label system supporting 7 mappings (6 ops + 1 editorial)

## Task Commits

Each task was committed atomically:

1. **Task 1: Register editorial tool and patch nightly_editorial.py** - `noosphere-ghosts@0514d8a` + `gotcha-workspace@8b59e0a` (feat)
2. **Task 2: Generalize action-planner label-to-tool mapping** - `noosphere-ghosts@729c7c4` (feat)

## Files Created/Modified
- `/opt/project-noosphere-ghosts/config/tool-registry.json` - Added editorial_nightly tool entry with editorial scope
- `/root/gotcha-workspace/tools/editorial/nightly_editorial.py` - Patched call_claude() auth to use ANTHROPIC_API_KEY env var, added HEARTBEAT_OK
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` - Added tool-mapping-for-label and build-tool-mapping-table functions, replaced hardcoded table

## Decisions Made
- Used cons (tool . args) return from tool-mapping-for-label for simple pair structure in Lisp
- Dynamic example tool_call block uses first fired label's tool instead of hardcoded ops_health_check
- Preserved auth-profiles OAuth fallback for backwards compatibility with non-ghost invocation
- Did NOT register delegate_editorial.py or trigger_editorial.py (OpenClaw-era tools per research recommendation)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required. Sylvia's tool_scope already includes "editorial" and Project #12 schedule already has "Nightly Editorial" label.

## Next Phase Readiness
- Phase 15 (financial pipeline) can follow the same pattern: add tool_mapping_for_label clause for trading labels
- The dynamic mapping system is ready for any number of additional standing order labels
- All verification checks pass including SBCL compilation and DB state validation

---
*Phase: 14-editorial-pipeline*
*Completed: 2026-03-28*
