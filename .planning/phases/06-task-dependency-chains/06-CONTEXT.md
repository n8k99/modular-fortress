# Phase 6: Task Dependency Chains - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Wire the existing `blocked_by` column into perception filtering so ghosts only see unblocked tasks, auto-unblock dependents when a task completes, and enable dependency-aware task creation by both executives (CREATE_TASK) and dispatch (wave ordering).

</domain>

<decisions>
## Implementation Decisions

### Dependency Cardinality
- **D-01:** Migrate `blocked_by` from `INTEGER` to `INTEGER[]` (Postgres array). A task is unblocked when ALL referenced task IDs in the array are complete.
- **D-02:** Migration must preserve existing data — convert single INTEGER values to single-element arrays. No data loss.

### Perception Filtering
- **D-03:** Filter blocked tasks at the SQL level in dpn-api. Tasks where `blocked_by` contains any incomplete task ID are excluded from the perception response. Ghosts only see actionable work.
- **D-04:** Executives see blocked tasks in their project review context as a separate informational section (not in their actionable task list). This gives executives visibility into the full dependency graph when reviewing a project.

### Auto-Unblock Mechanism
- **D-05:** Extend the existing `on_task_completed_after()` DB trigger. When task N completes, find all tasks where `blocked_by` array contains N, remove N from the array. When the array becomes empty, the task is fully unblocked.
- **D-06:** Same pattern as wave advancement (Phase 5) — all completion side-effects live in DB triggers for consistency.

### Claude's Discretion
- Whether to change task status when fully unblocked (e.g., set to 'open') or leave status unchanged and rely on perception filtering (empty blocked_by = perceivable). Decide based on how wave advancement trigger and perception filtering interact.
- How to handle non-existent task IDs in blocked_by references from CREATE_TASK — pick the most robust approach based on existing error patterns in action-executor.lisp.

### CREATE_TASK Syntax Extension
- **D-07:** Extend CREATE_TASK with inline `blocked_by` parameter: `CREATE_TASK: description assignee=agent-id blocked_by=#123,#456`. Follows existing `key=value` parsing pattern.
- **D-08:** Parser extracts comma-separated task IDs from `blocked_by=` and passes them as INTEGER[] to the API.

### Dispatch Wave-to-Dependency Mapping
- **D-09:** `dispatch_to_db.py` must set `blocked_by` for wave 2+ subtasks based on wave ordering. Wave 2 tasks are blocked by wave 1 completion. Uses the existing `depends_on` from context JSON to populate the actual `blocked_by` column.

### Carried From Prior Phases
- Phase 1: `blocked_by INTEGER` column exists (will be migrated to INTEGER[])
- Phase 1: Context JSON stores `{"wave": N, "depends_on": "..."}` — informational only, not enforced
- Phase 3: CREATE_TASK supports `assignee=`, `project_id`, `parent_id`, `department`
- Phase 5: `on_task_completed_after()` trigger handles wave advancement — dependency unblocking extends this
- Phase 5: Completion reports post to executive conversations with GSD context

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Tasks Schema & Migration
- `/root/gotcha-workspace/tools/db/migrate_to_postgres.sql` — Tasks table definition (blocked_by INTEGER currently)
- `/root/gotcha-workspace/tools/engineering/migrate_schema.py` — Schema migration tool (ALTER TABLE pattern)

### Perception Endpoint
- `/opt/dpn-api/src/handlers/af64_perception.rs` — Perception handler SQL queries. Add blocked_by filtering here.

### Action Executor (CREATE_TASK + COMPLETE)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Key: `parse-create-task-lines` (line 575), CREATE_TASK execution (line 689), `parse-complete-lines` (line 559), completion execution (line 637)

### DB Triggers (Wave Advancement)
- `/root/.planning/phases/05-feedback-reporting/migrations/001_wave_advancement_trigger.sql` — `on_task_completed_after()` trigger. Extend with dependency unblocking.

### Dispatch
- `/root/gotcha-workspace/tools/gsd/dispatch_to_db.py` — Wave handling (line 219+), context JSON construction (line 237). Must populate blocked_by column for wave 2+ tasks.

### Perception Lisp Layer
- `/opt/project-noosphere-ghosts/lisp/runtime/perception.lisp` — `perceive` function (line 15). No changes needed if filtering is at API level.

### Task API
- `/opt/dpn-api/src/handlers/af64_tasks.rs` — Task CRUD endpoints. Must accept blocked_by as INTEGER[] in create/update.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `on_task_completed_after()` trigger — already fires on task completion. Extend with array element removal logic.
- `parse-create-task-lines` — existing key=value parser for CREATE_TASK. Add `blocked_by=` as new recognized key.
- `dispatch_to_db.py` wave handling — already stores wave number and depends_on in context JSON. Convert to populate blocked_by column.
- Postgres `array_remove()` function — native for removing elements from INTEGER[].
- `cascade_reporter.py` — already queries blocked_by, will need updating for array type.

### Established Patterns
- Completion side-effects via DB triggers (wave advancement, set_completed_date, notify)
- Key=value parsing in CREATE_TASK (`assignee=agent-id`)
- Perception SQL filtering in dpn-api handlers
- Context JSON for metadata storage alongside typed columns

### Integration Points
- `af64_perception.rs` — WHERE clause modification to exclude blocked tasks
- `on_task_completed_after()` — new logic: remove completed task ID from all blocked_by arrays
- `action-executor.lisp` — parse `blocked_by=#id,#id` from CREATE_TASK lines
- `dispatch_to_db.py` — populate blocked_by column from wave ordering
- `af64_tasks.rs` — accept INTEGER[] for blocked_by in task create/update API

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

*Phase: 06-task-dependency-chains*
*Context gathered: 2026-03-26*
