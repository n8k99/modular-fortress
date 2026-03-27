---
phase: 09-verification-levels
plan: 02
subsystem: api, tick-engine
tags: [lisp, rust, perception, urgency, quality-issues, jsonb]

requires:
  - phase: 09-01
    provides: extract-artifact-issues with CRITICAL/WARNING/SUGGESTION classification in stage_notes
provides:
  - quality-issue-boost (+40) in tick engine urgency formula for CRITICAL issues
  - critical_issues array in executive perception response
affects: [10-lifecycle-signals]

tech-stack:
  added: []
  patterns: [jsonb_array_elements EXISTS pattern for severity filtering, vectorp/listp coerce pattern for Lisp collections]

key-files:
  created: []
  modified:
    - /opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp
    - /opt/project-noosphere-ghosts/lisp/packages.lisp
    - /opt/dpn-api/src/handlers/af64_perception.rs

key-decisions:
  - "Used +40 boost value for CRITICAL issues (between task-boost +25 and msg-boost +50)"
  - "parse-json import added to tick-engine package for string fallback in stage_notes parsing"

patterns-established:
  - "quality-issue-boost pattern: scan perception tasks for CRITICAL severity in STAGE-NOTES -> ISSUES"
  - "Executive-scoped critical_issues query following blocked_tasks pattern in perception"

requirements-completed: [VER-02]

duration: 3min
completed: 2026-03-27
---

# Phase 09 Plan 02: Quality Issue Urgency Escalation Summary

**+40 urgency boost for CRITICAL quality issues in tick engine and critical_issues array in executive perception endpoint**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-27T00:04:20Z
- **Completed:** 2026-03-27T00:07:21Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Added quality-issue-boost to tick engine urgency formula: executives with CRITICAL issues in owned-project tasks get +40 urgency
- Added critical_issues query and response field to perception endpoint, scoped to executive's owned projects with done/completed tasks containing CRITICAL severity issues
- Both implementations follow existing patterns (deadline-boost for Lisp, blocked_tasks for Rust)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add quality-issue-boost to tick engine urgency formula** - `46f999e` (feat)
2. **Task 2: Add critical_issues to executive perception response** - `60cbb67` (feat)

## Files Created/Modified
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` - quality-issue-boost in phase-rank urgency formula
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` - parse-json import for tick-engine package
- `/opt/dpn-api/src/handlers/af64_perception.rs` - critical_issues query and response field for executives

## Decisions Made
- Used +40 boost value for CRITICAL issues, positioned between task-boost (+25) and msg-boost (+50) per research recommendation
- Added parse-json import to tick-engine package for handling string-type stage_notes as safety fallback
- Followed exact blocked_tasks pattern for critical_issues query (executive-scoped, owned projects, LIMIT 10)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added parse-json import to tick-engine package**
- **Found during:** Task 1 (quality-issue-boost implementation)
- **Issue:** Plan's Lisp code calls parse-json but tick-engine package did not import it
- **Fix:** Added :parse-json to :import-from :af64.utils.json in packages.lisp
- **Files modified:** /opt/project-noosphere-ghosts/lisp/packages.lisp
- **Verification:** SBCL loads tick-engine.lisp without errors
- **Committed in:** 46f999e (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary import for compilation. No scope creep.

## Issues Encountered
- Pre-existing SBCL compilation error in action-executor.lisp (CONTENT unbound variable) prevents full system load. Verified tick-engine.lisp compiles correctly by loading without action-executor. This is out of scope for this plan.

## Known Stubs
None - all implementations are fully wired.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 09 complete: verification severity levels implemented across action-executor (Plan 01) and tick-engine + perception (Plan 02)
- Ready for Phase 10: lifecycle signals (staff idle/ready-for-next signaling)

---
*Phase: 09-verification-levels*
*Completed: 2026-03-27*
