---
phase: 08-decisions-brain
plan: 02
subsystem: runtime
tags: [common-lisp, decisions-api, tick-engine, executive-cognition]

# Dependency graph
requires:
  - phase: 08-01
    provides: "POST /api/decisions and GET /api/decisions endpoints in dpn-api"
provides:
  - "Decision capture from LLM output via DECISION: prefix detection and API POST"
  - "Decision context injection in executive project review prompts"
affects: [09-verify-severity, 10-lifecycle-signals]

# Tech tracking
tech-stack:
  added: []
  patterns: ["DECISION: prefix parsing with because-rationale split", "Strict prefix match for DB persistence vs loose match for memory logging"]

key-files:
  created: []
  modified:
    - "/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp"
    - "/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp"

key-decisions:
  - "Strict DECISION: prefix for DB persistence, loose 'decided' match preserved for memory logging only"
  - "Single project decisions fetch (first project) to avoid per-project API loop"
  - "Decisions placed between project summaries and team roster in review prompt"

patterns-established:
  - "extract-decision-lines: parse structured prefix lines from LLM output into typed pairs"
  - "handler-case wrapping API calls with descriptive error tags (decision-persist-error, decisions-fetch-error)"

requirements-completed: [DEC-01, DEC-02]

# Metrics
duration: 2min
completed: 2026-03-26
---

# Phase 08 Plan 02: Decisions Brain - Lisp Integration Summary

**Decision capture from DECISION: prefix lines via API POST, and prior-decisions context injection into executive project review prompts**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-26T23:29:27Z
- **Completed:** 2026-03-26T23:31:18Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Executives now see last 10 project decisions (date, owner, text, rationale) before reviewing projects
- Each DECISION: line in LLM output creates a separate record in decisions table with project_id and agent attribution
- Existing memory/vault_notes logging preserved unchanged alongside new DB persistence
- All API interactions wrapped in error handlers to prevent tick crashes

## Task Commits

Each task was committed atomically:

1. **Task 1: Decision capture -- POST to API on DECISION: detection** - `7c062a8` (feat)
2. **Task 2: Decision context injection in project review prompt** - `20244a1` (feat)

## Files Created/Modified
- `lisp/runtime/action-executor.lisp` - Added extract-decision-lines helper and API POST block in write-agent-daily-memory
- `lisp/runtime/action-planner.lisp` - Added decisions-context fetch and injection in build-project-review-job prompt

## Decisions Made
- Used strict DECISION: prefix match for DB persistence (not the looser "decided" match used for memory logging) per research Pitfall 3
- Single fetch for first project only in project review (avoids N+1 API calls per research recommendation)
- Decisions section placed between project summaries and team roster in the review prompt format string

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Decisions brain fully wired: capture (write) and context (read) both operational
- Ready for Phase 09 (verification severity levels) or Phase 10 (lifecycle signals)
- End-to-end flow: executive reviews project -> sees prior decisions -> makes new DECISION: -> persisted to DB -> visible on next review

---
*Phase: 08-decisions-brain*
*Completed: 2026-03-26*

## Self-Check: PASSED
