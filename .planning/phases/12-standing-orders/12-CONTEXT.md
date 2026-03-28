# Phase 12: Standing Orders - Context

**Gathered:** 2026-03-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Enable ghosts to execute recurring project work on a cron schedule without manual dispatch. Projects in master_chronicle get a `schedule` field with cron expressions. The tick engine evaluates schedules each tick and injects the project owner into the acting-set when a schedule fires, so the executive gets a cognition job and can review/delegate work for that project.

</domain>

<decisions>
## Implementation Decisions

### Schedule Storage
- **D-01:** Add `schedule JSONB` column to the `projects` table. No separate schedules table — projects are the unit of work.
- **D-02:** Schedule format is a JSON array of cron objects: `[{"expr": "0 8 * * 1-5", "label": "NYC Session"}, {"expr": "0 2 * * 1-5", "label": "London Session"}]`. Each fires independently.
- **D-03:** Empty array or null means no schedule (project only runs when dispatched or when owner has other actionable items).

### Cron Evaluation
- **D-04:** Cron expression evaluation happens in Lisp (tick engine), not in the API. The tick engine already runs on a timer — natural place to check schedules.
- **D-05:** Simple cron matcher in Lisp — parse `minute hour day-of-month month day-of-week` fields, match against current time. No external dependencies. Supports standard cron syntax including ranges, lists, and `*/N` step values.
- **D-06:** Schedule check happens once per tick in the ranking phase. If current time matches any of a project's schedules, the project owner gets boosted into the acting-set.

### Trigger Mechanism
- **D-07:** When a schedule fires, inject the project owner into the acting-set (same as message boost). The executive gets a cognition job with the project in their perception context and the specific schedule label that triggered.
- **D-08:** The cognition prompt should include the schedule label (e.g., "NYC Session") so the executive knows which standing order fired and can act accordingly.
- **D-09:** No automatic task creation — the executive reviews the project and decides what to do, same as any project review. Standing orders are just scheduled project reviews.

### API Support
- **D-10:** API needs: GET/PATCH for project schedule field (already partially supported via existing project CRUD), plus a way to set schedules via the dispatch bridge or direct API call.
- **D-11:** Perception endpoint should include schedule metadata when returning projects, so the executive knows the project is schedule-triggered (not just ownership boost).

### Claude's Discretion
- Exact Lisp cron parser implementation (match function signature, edge cases)
- How to handle missed schedules (if ghost was down during scheduled time)
- Whether to add a `last_fired_at` timestamp to prevent double-firing within the same cron window

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Database
- `/opt/dpn-api/src/handlers/af64_perception.rs` — perception endpoint, projects section (needs schedule metadata)
- `/opt/dpn-api/src/main.rs` — router for any new endpoints

### Tick Engine
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — ranking phase where schedule check should happen (lines 135-218)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — build-project-review-job (line 799), needs schedule label in prompt

### Existing Cron Jobs (migration source)
- `/root/.openclaw/cron/jobs.json` — 14 active OpenClaw cron jobs with expressions and descriptions

### Prior Phase
- `.planning/phases/11-message-hygiene/11-CONTEXT.md` — message hygiene decisions (read_by, ghost chat filter)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `projects` table already has `owner`, `status`, `goals`, `description` — just needs `schedule` column
- Perception already returns projects with ownership boost (+15 per project)
- `build-project-review-job` already constructs project review cognition jobs
- `api-get` / `api-post` Lisp helpers for HTTP calls to dpn-api

### Established Patterns
- JSONB columns used throughout (tasks.context, tasks.stage_notes, agents.metadata)
- Tick engine ranking phase filters actionable items then selects top-N for acting-set
- Project ownership already boosts urgency — schedule trigger is conceptually similar

### Integration Points
- Ranking phase in tick-engine.lisp — add schedule check alongside existing urgency calculation
- Perception endpoint — include schedule info in project response
- DB migration — add schedule column to projects table
- OpenClaw jobs.json — source of truth for schedule expressions to migrate

</code_context>

<specifics>
## Specific Ideas

- OpenClaw's 14 cron jobs map to 3 existing DB projects: #10 (financial), #12 (editorial), #14 (operations)
- Trading briefings have 3 separate schedules (Tokyo 18:00 ET, London 02:00 ET, NYC 08:00 ET Mon-Fri)
- Nightly editorial is a single schedule (21:00 ET daily)
- Operations has mixed schedules (daily health 09:00 ET, nightly synthesis 00:05 ET, weekly Fri 00:30 ET, monthly 1st 01:00 ET)
- The schedule label in cognition prompt is critical — Kathryn needs to know it's "Tokyo Session" not just "Project Complete Success"

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 12-standing-orders*
*Context gathered: 2026-03-28*
