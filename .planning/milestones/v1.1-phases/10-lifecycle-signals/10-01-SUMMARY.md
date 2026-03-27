---
phase: 10-lifecycle-signals
plan: 01
subsystem: api, runtime
tags: [rust, axum, jsonb, metadata, common-lisp, energy]

requires:
  - phase: 09-verification-levels
    provides: agent_state metadata column already exists in DB
provides:
  - metadata field in list_agents API response
  - metadata PATCH merge with COALESCE semantics
  - idle-transition energy reward (+12) in energy.lisp
affects: [10-02, lifecycle-signals, tick-engine]

tech-stack:
  added: []
  patterns: [JSONB merge via COALESCE || operator for non-destructive metadata updates]

key-files:
  created: []
  modified:
    - /opt/dpn-api/src/handlers/af64_agents.rs
    - /opt/project-noosphere-ghosts/lisp/runtime/energy.lisp

key-decisions:
  - "COALESCE(metadata, '{}'::jsonb) || $1::jsonb for merge semantics -- never overwrites existing metadata keys"
  - "idle-transition reward set to 12 (within D-06 range of 10-15)"

patterns-established:
  - "Metadata merge pattern: always use COALESCE + || for JSONB metadata updates to preserve existing keys"

requirements-completed: [LIFE-01, LIFE-03]

duration: 1min
completed: 2026-03-27
---

# Phase 10 Plan 01: Metadata API + Idle Energy Summary

**list_agents metadata exposure, PATCH merge with COALESCE semantics, and +12 idle-transition energy reward**

## Performance

- **Duration:** 1 min
- **Started:** 2026-03-27T00:40:53Z
- **Completed:** 2026-03-27T00:42:11Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- list_agents API now returns metadata JSONB field for each agent (enables tick engine to read lifecycle_state)
- PATCH /api/agents/:id/state merges metadata using COALESCE for non-destructive updates
- :idle-transition energy reward of +12 added to +energy-rewards+ hash table

## Task Commits

Each task was committed atomically:

1. **Task 1: Add metadata to list_agents response and metadata PATCH merge** - `7580c43` (feat)
2. **Task 2: Add :idle-transition energy reward** - `ad95a0c` (feat)

## Files Created/Modified
- `/opt/dpn-api/src/handlers/af64_agents.rs` - Added metadata to list_agents JSON response + metadata merge block in update_state handler
- `/opt/project-noosphere-ghosts/lisp/runtime/energy.lisp` - Added :idle-transition reward of +12 to +energy-rewards+ hash table

## Decisions Made
- Used COALESCE(metadata, '{}'::jsonb) || $1::jsonb for merge semantics per research Pitfall 1 -- prevents overwrite of existing metadata keys
- Set idle-transition reward to 12, within the D-06 specified range of 10-15

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Metadata API ready for Plan 02 to read/write lifecycle_state via list_agents and PATCH endpoints
- idle-transition energy reward ready for tick engine to apply during state transitions
- Both dpn-api and noosphere-ghosts repos have committed changes

## Self-Check: PASSED

All files exist, all commits verified.

---
*Phase: 10-lifecycle-signals*
*Completed: 2026-03-27*
