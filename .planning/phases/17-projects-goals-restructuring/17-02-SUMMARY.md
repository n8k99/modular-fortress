---
phase: 17-projects-goals-restructuring
plan: 02
subsystem: api
tags: [rust, axum, dpn-api, perception, lifestage, areas, parat]

# Dependency graph
requires:
  - phase: 17-01
    provides: "dpn-core Project struct with lifestage/area_id fields and updated create/update functions"
provides:
  - "dpn-api project handlers accept lifestage and area_id in create/update requests"
  - "Perception endpoint returns lifestage and area_name for each project"
affects: [ghost-perception, project-lifecycle, parat-areas]

# Tech tracking
tech-stack:
  added: []
  patterns: ["LEFT JOIN for optional FK enrichment in perception queries"]

key-files:
  created: []
  modified:
    - /opt/dpn-api/src/handlers/projects.rs
    - /opt/dpn-api/src/handlers/af64_perception.rs

key-decisions:
  - "Release build required for PM2 (PM2 runs target/release/dpn-api, not debug)"

patterns-established:
  - "Perception enrichment via LEFT JOIN for optional FK references (areas)"
  - "serde default functions for required NOT NULL fields with sensible defaults (default_lifestage)"

requirements-completed: [API-05]

# Metrics
duration: 6min
completed: 2026-03-28
---

# Phase 17 Plan 02: API Handlers for Lifestage and Area Summary

**dpn-api project handlers updated with lifestage/area_id support and perception endpoint enriched with LEFT JOIN areas for ghost lifecycle awareness**

## Performance

- **Duration:** 6 min
- **Started:** 2026-03-28T21:13:06Z
- **Completed:** 2026-03-28T21:19:33Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Project create/update API endpoints now accept lifestage and area_id fields
- Perception endpoint returns lifestage (String) and area_name (Option via LEFT JOIN) for each active project
- Backward lifestage transitions properly rejected by DB trigger (verified via API)
- dpn-api compiled in release mode and running cleanly in production

## Task Commits

Each task was committed atomically:

1. **Task 1: Update dpn-api project handlers with lifestage and area_id** - `22129d7` (feat)
2. **Task 2: Enrich perception endpoint with lifestage and area_name** - `4804fd3` (feat)

## Files Created/Modified
- `/opt/dpn-api/src/handlers/projects.rs` - Added lifestage/area_id to CreateProjectRequest and UpdateProjectRequest, wired to dpn_core functions
- `/opt/dpn-api/src/handlers/af64_perception.rs` - Added p.lifestage, LEFT JOIN areas, area_name to perception SQL and JSON response

## Decisions Made
- Release build required for PM2 deployment (PM2 runs target/release/dpn-api, debug build insufficient)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Built release binary instead of debug**
- **Found during:** Task 2 (perception endpoint verification)
- **Issue:** PM2 runs /opt/dpn-api/target/release/dpn-api but `cargo build` only builds debug. Perception changes were invisible.
- **Fix:** Ran `cargo build --release` and restarted PM2
- **Files modified:** None (build artifact)
- **Verification:** Perception endpoint returns lifestage and area_name correctly
- **Committed in:** 4804fd3 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary for deployment. No scope creep.

## Issues Encountered
- Perception endpoint requires X-API-Key header for authentication (not mentioned in plan smoke test commands)
- Test PATCH to lifestage=Harvest on project 14 required superuser trigger bypass to restore original Tree value

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 17 complete: both dpn-core schema (Plan 01) and dpn-api handlers (Plan 02) support lifestage and area_id
- Ghosts now perceive project lifecycle stage and area context in their perception snapshots
- Ready for Phase 18 (memories/vault_notes restructuring)

---
*Phase: 17-projects-goals-restructuring*
*Completed: 2026-03-28*
