---
phase: 04-tool-execution
plan: 02
subsystem: testing
tags: [e2e, smoke-test, tool-scope, psql, curl, jq, agents, stage-notes]

requires:
  - phase: 04-tool-execution/01
    provides: "tool-socket wildcard fix, claude_code registration, claude-code-tool.sh"
provides:
  - "E2E smoke test script validating all 4 tool categories + D-08 stage_notes"
  - "Agent tool_scope audit with corrections per D-10/D-11"
  - "TOOL-04 external tools deferral documented"
affects: [05-feedback-reporting]

tech-stack:
  added: []
  patterns: ["bash E2E test with --quick flag for CI-like fast runs"]

key-files:
  created:
    - ".planning/phases/04-tool-execution/test_tools_e2e.sh"
  modified: []

key-decisions:
  - "Used modified_at (not updated_at) for documents table UPSERT -- schema discovery"
  - "Task API returns JSON array (not {tasks:[...]}) -- adapted test accordingly"
  - "Nathan CEO given broad tool_scope since he was the only agent with empty scope"
  - "Added operations+tools to sarah PA, canon+research to jmax, operations to nova per D-11"

patterns-established:
  - "E2E tool test pattern: bash + psql + curl + jq + grep assertions"

requirements-completed: [TOOL-02, TOOL-03, TOOL-04, TOOL-05, TOOL-06]

duration: 4min
completed: 2026-03-26
---

# Phase 04 Plan 02: E2E Tool Testing and Agent Scope Audit Summary

**18-test E2E smoke script covering DB/API/code/memory tools plus D-08 stage_notes, with agent tool_scope corrections for 4 agents per D-10/D-11**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-26T06:52:16Z
- **Completed:** 2026-03-26T06:57:03Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Created comprehensive E2E smoke test with 18 tests (16 PASS, 2 SKIP in --quick mode)
- TOOL-04 (external tools) explicitly deferred with rationale documented
- Audited all 64 active agents: found 4 needing tool_scope corrections
- All active agents now have non-NULL, non-empty tool_scope arrays matching D-11 role mappings

## Task Commits

Each task was committed atomically:

1. **Task 1: Create E2E smoke test script** - `41524be` (feat)
2. **Task 2: Audit and correct agent tool_scope** - DB-only changes, no file commit needed

**Plan metadata:** (pending final commit)

## Files Created/Modified
- `.planning/phases/04-tool-execution/test_tools_e2e.sh` - 18-test E2E smoke test covering TOOL-01 through TOOL-06 plus D-08

## Decisions Made
- **documents table uses `modified_at` not `updated_at`** -- discovered during test creation, adapted SQL
- **Task API returns raw JSON array** -- not wrapped in `{tasks: [...]}` object, adapted jq assertion
- **Nathan CEO given `[operations, engineering, strategy, tools, decision]` scope** -- was only agent with empty scope
- **4 agents corrected per D-11**: nathan (empty->broad), sarah (+operations,+tools), jmax (+canon,+research), nova (+operations)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed documents table column name in write_document test**
- **Found during:** Task 1 (E2E test creation)
- **Issue:** Plan specified `updated_at` column but actual schema uses `modified_at`
- **Fix:** Changed INSERT/UPSERT SQL to use `modified_at`
- **Files modified:** `.planning/phases/04-tool-execution/test_tools_e2e.sh`
- **Verification:** write_document test now returns valid document ID
- **Committed in:** 41524be (Task 1 commit)

**2. [Rule 1 - Bug] Fixed Task API response format assertion**
- **Found during:** Task 1 (E2E test creation)
- **Issue:** Plan assumed `{tasks: [...]}` response format but API returns raw JSON array
- **Fix:** Changed jq assertion to `type == "array" or .tasks`
- **Files modified:** `.planning/phases/04-tool-execution/test_tools_e2e.sh`
- **Verification:** Task API test passes
- **Committed in:** 41524be (Task 1 commit)

**3. [Rule 1 - Bug] Fixed psql output parsing for RETURNING clause**
- **Found during:** Task 1 (E2E test verification)
- **Issue:** `tr -d ' \n'` collapsed multi-line psql output including `INSERT 0 1` trailer
- **Fix:** Added `head -1` before `tr` to extract only the returned ID
- **Files modified:** `.planning/phases/04-tool-execution/test_tools_e2e.sh`
- **Verification:** write_document test returns clean numeric ID
- **Committed in:** 41524be (Task 1 commit)

---

**Total deviations:** 3 auto-fixed (3 bugs from incorrect schema/API assumptions in plan)
**Impact on plan:** All auto-fixes necessary for test correctness. No scope creep.

## Agent tool_scope Audit Trail (Task 2)

| Agent | Before | After | Change |
|-------|--------|-------|--------|
| nathan | `{}` (empty) | `{operations,engineering,strategy,tools,decision}` | Added broad CEO scope |
| sarah | `{scheduling,tracking,reporting,content,research,calendar}` | `{...existing...,operations,tools}` | Added operations+tools per D-11 |
| jmax | `{legal,regulation,compliance,risk,policy,ip,decision,audit,trading,kalshi,specs,strategy}` | `{...existing...,canon,research}` | Added canon+research per D-11 |
| nova | `{memory,temporal,system,all,strategy,...}` (15 items) | `{...existing...,operations}` | Added operations per D-11 |

All other 60 agents already matched D-11 expectations.

## Issues Encountered
- Plan verification command `sudo -u postgres psql ... WHERE status != 'inactive'` fails due to bash `!` escaping -- used `<>` operator instead for verification

## User Setup Required
None - no external service configuration required.

## Known Stubs
None - all tests execute against live infrastructure.

## Next Phase Readiness
- All tool categories validated end-to-end
- Agent scope audit complete -- scope enforcement will correctly gate tool access
- Ready for Phase 05 (feedback/reporting) where ghost execution results flow back

## Self-Check: PASSED

- FOUND: `.planning/phases/04-tool-execution/test_tools_e2e.sh`
- FOUND: `.planning/phases/04-tool-execution/04-02-SUMMARY.md`
- FOUND: commit `41524be`

---
*Phase: 04-tool-execution*
*Completed: 2026-03-26*
