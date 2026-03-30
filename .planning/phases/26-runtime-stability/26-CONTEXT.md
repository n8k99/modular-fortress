# Phase 26: Runtime Stability - Context

**Gathered:** 2026-03-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Fix the execute-work-task paren scope bug (STAB-01) and commit all 9 uncommitted tick engine fixes from the 2026-03-29 session (STAB-02) so subsequent v1.5 phases build on a solid, loadable, runtime-tested foundation.

</domain>

<decisions>
## Implementation Decisions

### Bug fix scope
- **D-01:** All 9 uncommitted modified files in /opt/project-noosphere-ghosts/ are in scope — commit everything before building Phase 27+ on top
- **D-02:** The STAB-01 paren bug in execute-work-task (action-executor.lisp lines 471-612) must be fixed so the json-object result returns from the correct let* scope
- **D-03:** Specific fixes identified in uncommitted changes:
  - UTF-8 pg-escape fix in pg.lisp (byte length vs char length)
  - Description column removal in db-tasks.lisp
  - Error handler-case wrapping in action-executor.lisp
  - SQL formatting cleanup in db-client.lisp
  - Changes in db-auxiliary.lisp, db-conversations.lisp, cognition-types.lisp, task-scheduler.lisp, packages.lisp

### Commit strategy
- **D-04:** Atomic commits per logical fix — not one giant commit. Group related changes (e.g., all UTF-8 fixes together, all SQL cleanup together)
- **D-05:** Follow existing commit message conventions: `fix(component): description`

### Verification approach
- **D-06:** SBCL must load the full system without compile errors or warnings after all fixes
- **D-07:** A complete tick cycle must run on the live system without runtime errors
- **D-08:** Verification is against the live noosphere-ghosts process (PM2), not a test harness

### Claude's Discretion
- Exact commit grouping (how to batch the 9 files into logical commits)
- Order of fixes (which to commit first)
- Whether to add any defensive error handling beyond what's already in the uncommitted changes

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Tick engine core
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Contains STAB-01 paren bug in execute-work-task (lines 471-612)
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — Main tick cycle orchestrator, must load cleanly
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — Action planning, recently modified

### Database layer
- `/opt/project-noosphere-ghosts/lisp/util/pg.lisp` — PostgreSQL FFI bindings, UTF-8 pg-escape fix
- `/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp` — SQL wrappers, formatting cleanup
- `/opt/project-noosphere-ghosts/lisp/runtime/db-tasks.lisp` — Task queries, description column removal
- `/opt/project-noosphere-ghosts/lisp/runtime/db-auxiliary.lisp` — Auxiliary DB ops (modified)
- `/opt/project-noosphere-ghosts/lisp/runtime/db-conversations.lisp` — Conversation storage (modified)

### Support files
- `/opt/project-noosphere-ghosts/lisp/runtime/cognition-types.lisp` — CLOS types (modified)
- `/opt/project-noosphere-ghosts/lisp/runtime/task-scheduler.lisp` — Task scheduling (modified)
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` — Package definitions (modified)

### Requirements
- `.planning/REQUIREMENTS.md` — STAB-01 and STAB-02 requirement definitions

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- handler-case pattern already used in tick-engine.lisp for error recovery — same pattern extends to action-executor
- pg-escape already handles basic string escaping — UTF-8 fix is an extension of existing code

### Established Patterns
- Atomic commits with `fix()` / `feat()` prefixes throughout project history
- SBCL loads system via ASDF — `(asdf:load-system :af64)` is the verification command
- PM2 manages noosphere-ghosts process — restart via `pm2 restart noosphere-ghosts`

### Integration Points
- All 9 files are part of the af64 ASDF system — changes must not break the load order defined in af64.asd
- pg.lisp is used by all db-*.lisp files — UTF-8 fix has wide impact
- action-executor.lisp is called by tick-engine.lisp on every tick — paren fix is critical path

</code_context>

<specifics>
## Specific Ideas

- The execute-work-task paren bug (STAB-01) has been "contained but not fixed" — it's been worked around but the root cause remains
- The 7 fixes from 2026-03-29 are described in STATE.md as: UTF-8 pg-escape, NULL handling, tilde SQL, type coercion, description column (plus additional changes found in uncommitted diff)
- Branch is em-droplet, 39 commits ahead of origin — all work is local

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 26-runtime-stability*
*Context gathered: 2026-03-30*
