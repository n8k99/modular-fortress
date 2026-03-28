---
phase: 12-standing-orders
plan: 01
subsystem: database, api, runtime
tags: [cron, jsonb, schedule, lisp, axum, perception, standing-orders]

# Dependency graph
requires:
  - phase: 11-message-hygiene
    provides: sqlx JSONB support and message read-marking
provides:
  - schedule JSONB column on projects table
  - PATCH /api/projects/:id endpoint with schedule support
  - perception endpoint returns schedule metadata for owned projects
  - cron-matcher.lisp module for evaluating 5-field cron expressions
  - seed data for projects 10, 12, 14 with cron schedules and owners
affects: [12-02, 13-editorial, 14-operations, 15-financial]

# Tech tracking
tech-stack:
  added: []
  patterns: [cron-expression-scheduling, jsonb-schedule-field]

key-files:
  created:
    - /opt/project-noosphere-ghosts/lisp/runtime/cron-matcher.lisp
  modified:
    - /opt/dpn-core/src/db/projects.rs
    - /opt/dpn-core/src/lib.rs
    - /opt/dpn-api/src/handlers/projects.rs
    - /opt/dpn-api/src/handlers/af64_perception.rs
    - /opt/dpn-api/src/main.rs
    - /opt/project-noosphere-ghosts/lisp/packages.lisp
    - /opt/project-noosphere-ghosts/lisp/af64.asd
    - /root/dpn-core/src/db/projects.rs
    - /root/dpn-core/src/lib.rs

key-decisions:
  - "Used dynamic SQL query builder pattern for schedule in update_project, consistent with existing field handling"
  - "Combined GET and PATCH on /projects/:id using axum method chaining to avoid duplicate route panics"
  - "Applied changes to both /root/dpn-core and /opt/dpn-core since dpn-api depends on /opt/dpn-core"

patterns-established:
  - "JSONB schedule array on projects: [{expr, label}] format for cron schedules"
  - "Lisp cron matcher: parse-cron-field returns lambda predicates for composable matching"

requirements-completed: [STAND-01]

# Metrics
duration: 13min
completed: 2026-03-28
---

# Phase 12 Plan 01: Schedule Infrastructure Summary

**JSONB schedule column on projects, PATCH API endpoint, perception schedule metadata, and Lisp cron matcher for 5-field cron expressions**

## Performance

- **Duration:** 13 min
- **Started:** 2026-03-28T02:58:01Z
- **Completed:** 2026-03-28T03:11:08Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments
- Added schedule JSONB column to projects table and seeded cron schedules for projects #10 (Financial/Kathryn), #12 (Editorial/Sylvia), #14 (Operations/Nova)
- Built PATCH /api/projects/:id endpoint supporting schedule, name, description, owner, and status updates
- Extended perception endpoint to include schedule metadata so ghosts can see cron schedules for their owned projects
- Created cron-matcher.lisp supporting wildcard, exact, range, list, and step cron field types with dow=7 Sunday alias

## Task Commits

Each task was committed atomically:

1. **Task 1: DB migration + dpn-core Project struct + API PATCH endpoint** - `9a0543b` (dpn-core), `785c175` (dpn-api) (feat)
2. **Task 2: Lisp cron matcher module** - `a3a80df` (noosphere-ghosts) (feat)

## Files Created/Modified
- `/opt/dpn-core/src/db/projects.rs` - Added schedule field to Project struct, updated all SELECT queries, added schedule to update_project
- `/opt/dpn-core/src/lib.rs` - Added update_project to re-exports
- `/opt/dpn-api/src/handlers/projects.rs` - Added UpdateProjectRequest struct and update_project PATCH handler
- `/opt/dpn-api/src/handlers/af64_perception.rs` - Added p.schedule to projects query and JSON response
- `/opt/dpn-api/src/main.rs` - Registered PATCH route via method chaining on /projects/:id
- `/opt/project-noosphere-ghosts/lisp/runtime/cron-matcher.lisp` - New cron expression parser and matcher
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` - Added af64.runtime.cron-matcher package
- `/opt/project-noosphere-ghosts/lisp/af64.asd` - Registered cron-matcher component
- `/root/dpn-core/src/db/projects.rs` - Mirror of /opt/dpn-core changes
- `/root/dpn-core/src/lib.rs` - Mirror of /opt/dpn-core changes

## Decisions Made
- Used axum method chaining `.route("/projects/:id", get(...).patch(...))` instead of separate route registrations to avoid duplicate route panics in axum 0.7
- Applied changes to both `/root/dpn-core` (git source) and `/opt/dpn-core` (deployment copy used by dpn-api) to ensure build consistency
- Built dpn-api in release mode since PM2 runs the release binary

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed duplicate route registration in axum**
- **Found during:** Task 1 (Step 5: Register PATCH route)
- **Issue:** axum 0.7 panics when the same path is registered twice with `.route()`, even for different HTTP methods
- **Fix:** Combined GET and PATCH handlers using method chaining: `.route("/projects/:id", get(...).patch(...))`
- **Files modified:** /opt/dpn-api/src/main.rs
- **Verification:** cargo build succeeds, PATCH endpoint responds correctly

**2. [Rule 3 - Blocking] Fixed dual dpn-core deployment path**
- **Found during:** Task 1 (Step 7: Build)
- **Issue:** dpn-api depends on `/opt/dpn-core` (via `path = "../dpn-core"`), not `/root/dpn-core` which was initially edited
- **Fix:** Applied same changes to `/opt/dpn-core/src/db/projects.rs` and `/opt/dpn-core/src/lib.rs`
- **Files modified:** /opt/dpn-core/src/db/projects.rs, /opt/dpn-core/src/lib.rs
- **Verification:** cargo build --release succeeds for dpn-api

**3. [Rule 3 - Blocking] Built release binary for production**
- **Found during:** Task 1 (Step 7: Verify)
- **Issue:** PM2 runs `/opt/dpn-api/target/release/dpn-api` but initial build was debug mode
- **Fix:** Ran `cargo build --release` instead of `cargo build`
- **Files modified:** none (binary output)
- **Verification:** pm2 restart dpn-api, API returns schedule data

---

**Total deviations:** 3 auto-fixed (all blocking)
**Impact on plan:** All auto-fixes were necessary for correct deployment. No scope creep.

## Issues Encountered
- Pre-existing test failures in dpn-core (12 tests: DB connection tests and notify tests) unrelated to schedule changes. Project-specific tests pass cleanly.

## Known Stubs
None - all data paths are wired end-to-end.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Schedule infrastructure complete, ready for Plan 02 to wire tick engine integration
- cron-matcher.lisp exports are available for import into tick-engine.lisp
- Perception already returns schedule metadata that the tick engine can use for cron-triggered cognition

---
*Phase: 12-standing-orders*
*Completed: 2026-03-28*
