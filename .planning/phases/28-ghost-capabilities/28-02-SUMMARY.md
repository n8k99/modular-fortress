---
phase: 28-ghost-capabilities
plan: 02
subsystem: runtime
tags: [common-lisp, sbcl, action-planner, ghost-capabilities, yaml, innatescript, cognition-prompts]

# Dependency graph
requires:
  - phase: 28-ghost-capabilities plan 01
    provides: ghost-capabilities.lisp module with load-ghost-capabilities and format-capabilities-for-prompt
provides:
  - YAML capability injection in all 4 action-planner prompt builders
  - YAML-first with tool-registry.json fallback in pipeline task builder
  - Debug logging showing yaml vs tool-registry source
affects: [28-ghost-capabilities plan 03, tick-engine, action-executor]

# Tech tracking
tech-stack:
  added: []
  patterns: [find-symbol soft dependency for cross-package calls, YAML-first with JSON fallback]

key-files:
  created: []
  modified:
    - /opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp

key-decisions:
  - "YAML-first strategy: try ghost-capabilities YAML, fall back to tool-registry.json only when YAML returns nil (D-11)"
  - "find-symbol pattern maintained for soft dependency: ghost-capabilities package may not be loaded"
  - "effective-prompt binding unifies YAML capabilities and tool-registry prompts in pipeline builder"

patterns-established:
  - "YAML capability injection pattern: load-ghost-capabilities -> format-capabilities-for-prompt -> inject into system-prompt"
  - "Conditional format directive ~@[~%~%~a~] for optional prompt sections"

requirements-completed: [CAP-02, CAP-03]

# Metrics
duration: 3min
completed: 2026-03-30
---

# Phase 28 Plan 02: Capability Injection Summary

**YAML ghost capabilities wired into all 4 action-planner prompt builders with tool-registry.json fallback**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-30T17:32:11Z
- **Completed:** 2026-03-30T17:35:30Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments
- All 4 prompt builder functions (build-pipeline-task-job, build-task-job, build-proactive-job, build-message-job) now inject YAML ghost capabilities into LLM cognition prompts
- build-pipeline-task-job uses YAML-first with explicit tool-registry.json fallback when YAML returns nil
- Debug logging distinguishes "yaml" vs "tool-registry" source for capability/tool injection
- SBCL loads the full file chain without errors

## Task Commits

Each task was committed atomically:

1. **Task 1: Replace tool injection with capability injection in all 4 prompt builders** - `3e4afb2` (feat)

## Files Created/Modified
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` - YAML capability injection in build-pipeline-task-job, build-task-job, build-proactive-job, build-message-job

## Decisions Made
- YAML-first strategy per D-11: ghosts with YAML files get InnateScipt capabilities; ghosts without fall back to tool-registry.json
- find-symbol soft dependency maintained: handler-case protects against ghost-capabilities package not being loaded
- effective-prompt binding in pipeline builder unifies both paths cleanly

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Known Stubs
None - all capability injection is fully wired to ghost-capabilities.lisp functions.

## Next Phase Readiness
- Capability injection complete; ghosts will see their YAML-declared InnateScipt capabilities in every cognition prompt
- Ready for Plan 03 (whatever the next capability phase step is)
- tool-registry.json continues to serve as fallback for ghosts without YAML configuration files

---
*Phase: 28-ghost-capabilities*
*Completed: 2026-03-30*

## Self-Check: PASSED
