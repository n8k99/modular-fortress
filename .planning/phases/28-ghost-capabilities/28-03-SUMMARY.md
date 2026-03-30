---
phase: 28-ghost-capabilities
plan: 03
subsystem: ghost-runtime
tags: [innate, yaml, ghost-capabilities, responsibility-mutation, executive-authorization, self-modification]

# Dependency graph
requires:
  - phase: 28-ghost-capabilities (28-01)
    provides: "Ghost YAML loading, parse-simple-yaml, ghost-yaml-path, load-ghost-capabilities"
  - phase: 28-ghost-capabilities (28-02)
    provides: "YAML-first capability injection into cognition prompts"
provides:
  - "Responsibility self-modification: ghosts add/remove/edit own InnateScipt YAML via LLM output"
  - "Executive authorization for subordinate capability modification"
  - "Atomic YAML write with temp-file + rename (D-15)"
  - "Parse-round-trip validation on all mutations (CAP-07)"
  - "Capability mutation instructions in LLM system prompts"
affects: [ghost-capabilities, action-executor, action-planner, tick-engine]

# Tech tracking
tech-stack:
  added: []
  patterns: [extract-json-array-from-llm-output, find-symbol-soft-dependency, atomic-file-write, executive-authorization]

key-files:
  created: []
  modified:
    - /opt/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp
    - /opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp
    - /opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp
    - /opt/project-noosphere-ghosts/lisp/packages.lisp

key-decisions:
  - "Refactored process-responsibility-mutations to accept JSON hash-tables (not plists) matching LLM output format"
  - "Split mutation processing into process-single-mutation helper with block/return-from for clean control flow"
  - "Used find-symbol soft dependency pattern for cross-package calls (ghost-capabilities may not be loaded)"
  - "Added *capability-mutation-instructions* as separate defparameter from *innate-generation-instructions*"

patterns-established:
  - "extract-*-from-llm: Search raw string for key, bracket-match JSON array, parse and coerce"
  - "find-symbol soft dependency: funcall via find-symbol to avoid hard package dependency"
  - "Atomic YAML write: write temp file, rename-file to target path"

requirements-completed: [CAP-04, CAP-05, CAP-06, CAP-07]

# Metrics
duration: 6min
completed: 2026-03-30
---

# Phase 28 Plan 03: Responsibility Self-Modification Summary

**Ghost self-modification via LLM cognition output: add/remove/edit InnateScipt responsibilities with executive authorization and parse-round-trip validation**

## Performance

- **Duration:** 6 min
- **Started:** 2026-03-30T17:38:18Z
- **Completed:** 2026-03-30T17:44:32Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Ghosts can add, remove, and edit their own InnateScipt responsibilities via cognition output
- Executives can modify subordinate capabilities via target_agent field with department-based authorization
- All mutations validated via parse-round-trip before YAML write (no invalid Innate persisted)
- Atomic YAML writes via temp-file + rename prevent corruption
- LLM system prompts include capability mutation schema for both work-task and project-review paths

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement mutation processing and atomic YAML write in ghost-capabilities.lisp** - `055ec4c` (feat)
2. **Task 2: Wire responsibility mutation extraction into action-executor cognition output processing** - `6b3cd72` (feat)

## Files Created/Modified
- `lisp/runtime/ghost-capabilities.lisp` - Atomic write-ghost-yaml, agent-is-executive-p, validate-executive-target, process-responsibility-mutations with add/remove/edit
- `lisp/runtime/action-executor.lisp` - extract-responsibility-mutations function, wired into execute-work-task and execute-proactive-work
- `lisp/runtime/action-planner.lisp` - *capability-mutation-instructions* defparameter, appended to work-task and project-review system prompts
- `lisp/packages.lisp` - Added db-get-agent-by-id import and new exports to ghost-capabilities package

## Decisions Made
- Refactored the mutation interface from plist-based (`:type`, `:expression`, `:old-expression`) to hash-table-based (`:ACTION`, `:EXPRESSION`, `:OLD`, `:NEW`, `:TARGET-AGENT`) matching the AF64 JSON parser's output format
- Split process-responsibility-mutations into a per-mutation helper (process-single-mutation) using block/return-from instead of tagbody/go for cleaner early-exit control flow
- Kept *capability-mutation-instructions* as a separate defparameter from *innate-generation-instructions* for modularity

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added db-get-agent-by-id import to ghost-capabilities package**
- **Found during:** Task 1 (executive authorization functions)
- **Issue:** ghost-capabilities package needed db-get-agent-by-id for agent-is-executive-p and validate-executive-target, but the import was missing from packages.lisp
- **Fix:** Added `:import-from :af64.runtime.db-auxiliary #:db-get-agent-by-id` to the ghost-capabilities package definition
- **Files modified:** lisp/packages.lisp
- **Verification:** SBCL loads without style-warnings about undefined DB-GET-AGENT-BY-ID
- **Committed in:** 055ec4c (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Essential for correct package imports. No scope creep.

## Issues Encountered
- DPN_API_URL and DPN_API_KEY environment variables required for SBCL load (sourced from config/af64.env)

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 28 (ghost-capabilities) is now complete: all 3 plans executed
- Ghost YAML capabilities fully operational: load, inject, validate, and self-modify
- Ready for team pipeline definitions and area-scoped content in subsequent phases

## Known Stubs
None - all functions are fully implemented with real data paths.

---
*Phase: 28-ghost-capabilities*
*Completed: 2026-03-30*
