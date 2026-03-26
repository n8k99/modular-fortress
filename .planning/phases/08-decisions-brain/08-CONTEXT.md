# Phase 8: Decisions Brain - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Wire the existing decisions table into the executive cognition loop: create API endpoints for decisions CRUD, enhance decision detection in the action executor to persist decisions to the DB, and inject prior decisions into executive project review prompts so executives act consistently.

</domain>

<decisions>
## Implementation Decisions

### Decision Capture Flow
- **D-01:** Enhance the existing `DECISION:` keyword detection in action-executor.lisp (line ~1081). When detected, parse content and POST to decisions table via API. Minimal change to executive behavior — they already naturally output `DECISION:` lines.
- **D-02:** Keep existing logging to agent memory/vault_notes alongside the new DB persistence. The decisions table is the source of truth for project decisions; memory is the agent's personal record.

### Decision Context Injection
- **D-03:** Query `GET /api/decisions?project_id=X&limit=10&order=desc` for each project during `build-project-review-job`. Include as a "Recent Decisions" section in the review prompt, after project summary and before task breakdown.
- **D-04:** Last 10 decisions per project, most recent first. Bounded context to avoid prompt bloat.

### Decisions API Design
- **D-05:** Create `GET /api/decisions` with `project_id` filter (required or optional), `limit`, `order` params. Also support `department` filter for department-wide decisions.
- **D-06:** Create `POST /api/decisions` for new decisions. Fields: decision (required), rationale (optional), project_id (optional), department (optional), owner (required), stakeholders (optional JSONB).
- **D-07:** Decisions are append-only — no PUT or DELETE. Historical record cannot be altered.

### Decision Scope & Attribution
- **D-08:** Decisions can be project-scoped (project_id set) OR department-scoped (department set, project_id null). Both are optional but at least one should be present.
- **D-09:** Add optional `department VARCHAR(256)` column to decisions table for department-wide decisions.
- **D-10:** Auto-populate owner from agent_id and stakeholders from project team members when logging a decision during project review. Executive doesn't need to specify these manually.

### Claude's Discretion
- Whether to parse rationale from the DECISION: line (e.g., `DECISION: X because Y`) or let the LLM output structured fields
- How to format decisions in the review prompt (table vs bullet list)
- Whether to add a `tags` or `category` field to decisions for future filtering

### Carried From Prior Phases
- Phase 3: Executive LLM cognition for project decomposition and delegation
- Phase 5: Completion reports and blocker escalation to executives
- Existing: `DECISION:` keyword detection already in action-executor.lisp
- Existing: decisions table schema (id, project_id, decision, rationale, owner, stakeholders, date, created_at)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Decisions Table Schema
- `/root/gotcha-workspace/tools/db/migrate_to_postgres.sql` — decisions table definition (lines 122-136)

### Action Executor (Decision Detection)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Key: `DECISION:` keyword detection (lines 1081-1097), decision logging to memory (lines 1101-1112)

### Action Planner (Project Review)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — Key: `build-project-review-job` (lines 759-821), `format-project-tasks` (line 692), `format-team-roster` (line 735), executive prompt template (line 809)

### API Router
- `/opt/dpn-api/src/main.rs` — Route definitions. No decisions routes exist yet — add them here.

### Existing Handler Patterns
- `/opt/dpn-api/src/handlers/af64_tasks.rs` — Example of CRUD handler with JSONB fields (pattern to follow for decisions handler)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `DECISION:` keyword detection — already scans executive LLM output for decision keywords. Extend to POST to API.
- `build-project-review-job` — already builds rich project context. Add decisions section.
- dpn-api handler pattern — existing CRUD handlers for tasks, projects, conversations. Follow same pattern for decisions.
- decisions table — already created in migration with indexes on owner and date.

### Established Patterns
- Keyword detection → action in action-executor: `DECISION:` detected → currently logs to memory. Extend to API POST.
- Project review context: project summary → tasks → team roster. Insert decisions between summary and tasks.
- API CRUD: handler file in `/opt/dpn-api/src/handlers/`, route registration in main.rs, sqlx queries.

### Integration Points
- `action-executor.lisp` decision detection block (line 1081) — add API POST call
- `action-planner.lisp` `build-project-review-job` — add decisions fetch and prompt injection
- `dpn-api/src/main.rs` — register new decisions routes
- New file: `dpn-api/src/handlers/decisions.rs` — CRUD handler for decisions table

</code_context>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 08-decisions-brain*
*Context gathered: 2026-03-26*
