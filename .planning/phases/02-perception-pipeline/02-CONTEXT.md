# Phase 2: Perception Pipeline - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Verify and enhance the existing perception endpoint (`/api/perception/:agent_id`) so ghosts see GSD-dispatched projects and tasks with full metadata. The endpoint already exists at 401 lines — this phase adds GSD fields to task queries, migrates from `assignee` to `assigned_to`, and verifies the urgency boost fires end-to-end.

</domain>

<decisions>
## Implementation Decisions

### Critical Finding: Perception Endpoint Already Exists
- **D-01:** `/api/perception/:agent_id` is fully implemented in `af64_perception.rs` (401 lines). Route registered in main.rs. Returns tasks, projects, messages, relationships, memories, team activity, and proactive eligibility.
- **D-02:** This is an ENHANCEMENT phase, not a build-from-scratch. Modify existing queries, don't rewrite the handler.

### Task Field Enrichment
- **D-03:** Add ALL GSD fields to task query responses: `project_id`, `source`, `context`, `parent_id`, `priority`, `assigned_to`, `scheduled_at`. Ghosts get the full picture.
- **D-04:** The task serialization block (lines 123-134) needs expansion to include these additional columns in both the SELECT and the JSON output.

### Assignment Column Migration
- **D-05:** Migrate ALL perception task queries from `assignee` (varchar) to `assigned_to` (text[]). Use `$1 = ANY(assigned_to)` for filtering.
- **D-06:** Executive query: `WHERE $1 = ANY(assigned_to) OR (department = $2 AND assigned_to IS NULL)`. Staff query: `WHERE $1 = ANY(assigned_to)`.
- **D-07:** This means existing tasks with only `assignee` set won't appear unless they also have `assigned_to`. This is acceptable — GSD tasks use `assigned_to`, legacy tasks use `assignee`, and we're building the GSD pipeline.

### Scheduling & Wave Filtering
- **D-08:** NO scheduling/wave filtering in perception. Show all tasks regardless of wave. Executives manage wave ordering during delegation (Phase 3). Simplest approach.
- **D-09:** `scheduled_at` column included in response for informational purposes but NOT used as a WHERE filter.

### Urgency Boost Verification
- **D-10:** End-to-end test: dispatch a project owned by an executive, call the perception endpoint, verify the urgency score includes the +15/project boost in tick engine logs.
- **D-11:** The tick engine code in `tick-engine.lisp` already has `(* 15 (length projects))`. Verify this code path fires when perception returns a non-empty projects array.

### Carried From Phase 1
- D-04 (Phase 1): Project name from H1 heading
- D-05 (Phase 1): --owner required, department from agents table
- D-08 (Phase 1): Hierarchical parent+subtask model with parent_id

### Claude's Discretion
- Whether to add project `goals` and `current_context` to the perception project response (currently returns id, name, status, owner, open_tasks, completed_tasks)
- Error handling for malformed `assigned_to` arrays
- Whether to truncate `context` JSON in perception response or return full

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Perception Endpoint (Primary Target)
- `/opt/dpn-api/src/handlers/af64_perception.rs` — 401-line handler to modify. Key sections: task queries (lines 95-121), task serialization (lines 123-134), project queries (lines 318-342), response assembly (lines 389-396)
- `/opt/dpn-api/src/main.rs` — Route registration: `.route("/perception/:agent_id", get(af64_perception::get_perception))` (line 126)

### Tick Engine (Urgency Boost)
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — Project boost calculation: `(* 15 (length projects))`
- `/opt/project-noosphere-ghosts/lisp/runtime/perception.lisp` — How ghosts call the perception API

### Phase 1 Outputs (Dependencies)
- `/root/gotcha-workspace/tools/gsd/dispatch_to_db.py` — Writes tasks with assigned_to, project_id, source, context, department
- `/root/gotcha-workspace/tools/gsd/test_dispatch.py` — Integration tests proving dispatch works

### Database Schema
- Live `tasks` table — 39 columns including assigned_to (text[]), project_id (FK), source, context, parent_id, scheduled_at

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `af64_perception.rs` is fully functional — modify queries, don't restructure
- Existing task role routing (triage/exec/staff) is correct — just update column references
- Project query already joins with task counts — just add fields
- UTF-8 safe text truncation already done: `text.chars().take(300).collect()` (line 125)

### Established Patterns
- SQLx async queries with `.fetch_all(&pool).await.unwrap_or_default()`
- `serde_json::json!()` for building response objects
- `r.get::<Type, _>("column")` pattern for row extraction
- `Option<T>` wrapping for nullable columns

### Integration Points
- Lisp perception.lisp parses JSON response — field names must match what Lisp expects (underscore→hyphen conversion: `project_id` becomes `:project-id` in Lisp)
- Tick engine reads `:projects` key from perception for urgency boost
- dpn-core project queries may need alignment if shared

</code_context>

<specifics>
## Specific Ideas

- The Lisp JSON parser converts underscores to hyphens — new fields like `project_id` will appear as `:project-id` in Lisp. Verify tick engine handles this correctly.
- End-to-end test should dispatch the Noosphere Dispatch Pipeline project (already exists in DB as project #51) and verify perception returns it for Eliana.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 02-perception-pipeline*
*Context gathered: 2026-03-26*
