---
phase: 25-ghost-expression-generation
plan: 02
subsystem: runtime
tags: [lisp, sbcl, innate, action-executor, action-planner, cognition-pipeline, templates]

# Dependency graph
requires:
  - phase: 25-ghost-expression-generation/01
    provides: "innate-builder module with builder functions, validation, slug generation, template CRUD"
  - phase: 24-template-evaluation-execution
    provides: "template evaluation in cognition pipeline, evaluate-template-for-project path"
provides:
  - "Expression extraction from LLM output via extract-innate-expressions"
  - "Validation-before-persistence via process-innate-expressions (CREATE + UPDATE paths)"
  - "Innate generation instructions in project-review and work-task system prompts"
  - "Complete read-write loop: ghosts evaluate (Phase 24) and generate (Phase 25) Innate expressions"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns: ["LLM output JSON extraction with bracket-depth matching", "additive cognition pipeline integration via handler-case wrapping"]

key-files:
  created: []
  modified:
    - /opt/project-noosphere-ghosts/lisp/packages.lisp
    - /opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp
    - /opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp

key-decisions:
  - "JSON parser converts underscores to hyphens, so innate_expressions keys accessed as :NAME :BODY :UPDATE :CATEGORY :DESCRIPTION"
  - "Innate generation instructions appended to both project-review and generic work-task system prompts"
  - "Expression generation is additive -- handler-case wrapping ensures no impact on existing cognition paths"

patterns-established:
  - "extract-innate-expressions uses bracket-depth matching to extract JSON arrays from mixed LLM output"
  - "process-innate-expressions validates before persistence, logs [innate-gen-invalid] for rejected expressions"

requirements-completed: [INNATE-03]

# Metrics
duration: 4min
completed: 2026-03-30
---

# Phase 25 Plan 02: Cognition Pipeline Integration Summary

**Innate expression generation wired into ghost cognition: action-planner adds syntax instructions to LLM prompts, action-executor extracts/validates/persists generated expressions from LLM output to templates table**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-30T01:33:14Z
- **Completed:** 2026-03-30T01:37:00Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Action-executor extracts innate_expressions JSON from LLM output, validates each via parse-round-trip, persists valid ones to templates table
- Action-planner system prompts include Innate syntax reference and JSON schema for both project-review and work-task jobs
- Full SBCL compilation of all AF64 runtime files passes without errors
- Template CRUD round-trip verified via SQL INSERT/SELECT/DELETE
- Complete read-write loop closed: Phase 24 reads/evaluates templates, Phase 25 generates/persists them

## Task Commits

Each task was committed atomically:

1. **Task 1: Add innate-builder imports and expression extraction/validation/persistence** - `a39be7e` (feat) -- in project-noosphere-ghosts repo
2. **Task 2: Add template generation instructions to system prompts and verify full compilation** - `0063f0b` (feat) -- in project-noosphere-ghosts repo

## Files Created/Modified
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` - Added conditional innate-builder imports for action-executor package
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` - Added extract-innate-expressions, process-innate-expressions, integrated into execute-work-task and execute-project-review
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` - Added *innate-generation-instructions* defparameter, appended to project-review and work-task system prompts

## Decisions Made
- JSON parser converts underscores to hyphens, so expression hash-table keys accessed as :NAME :BODY :UPDATE :CATEGORY :DESCRIPTION (not lowercase string keys)
- Innate generation instructions appended to both project-review and generic work-task system prompts (not just project-review)
- Expression generation is fully additive: handler-case wrapping ensures no impact on existing cognition paths when innate_expressions key is absent

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Adjusted hash-table key access for JSON parser convention**
- **Found during:** Task 1 (process-innate-expressions implementation)
- **Issue:** Plan used dual gethash with both :name and :NAME, but AF64 JSON parser always converts to uppercase keyword with hyphens
- **Fix:** Used only :NAME, :BODY, :UPDATE, :CATEGORY, :DESCRIPTION (the actual keys produced by parse-json)
- **Files modified:** action-executor.lisp
- **Verification:** Confirmed via json.lisp json-keyword function that all keys are uppercased keywords

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Minor adjustment for correctness based on known JSON parser behavior. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Known Stubs
None - all functions are fully implemented and wired.

## Next Phase Readiness
- Complete Innate expression generation pipeline operational
- Ghosts can now both evaluate (Phase 24) and generate (Phase 25) Innate expressions
- The template ecosystem is fully bidirectional: ghosts read, evaluate, create, and update templates

---
*Phase: 25-ghost-expression-generation*
*Completed: 2026-03-30*
