# Phase 3: Executive Cognition - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Implement CREATE_TASK parsing in the action executor, enrich the project review prompt with GSD context (waves, must_haves, phase structure), and enable same-action delegation so executives can create and assign subtasks in a single tick. The executive cognition framework substantially exists — this phase fills the gaps.

</domain>

<decisions>
## Implementation Decisions

### Critical Finding: Executive Cognition Substantially Exists
- **D-01:** `build-project-review-job` (action-planner.lisp:626) already creates LLM cognition jobs for executives to review their project portfolio.
- **D-02:** `execute-project-review` (action-executor.lisp:643) already posts to conversations, applies task mutations (CLASSIFY/DELEGATE/COMPLETE), and routes handoffs.
- **D-03:** Priority chain already correct: messages > requests > tasks > project review. Project review is the lowest-priority proactive action — fires when idle.
- **D-04:** `parse-delegate-lines` and `parse-classify-lines` already work. `apply-task-mutations` handles DELEGATE: and CLASSIFY: output.
- **D-05:** The gap is: `CREATE_TASK:` is in the prompt but has NO parser or executor in action-executor.lisp.

### CREATE_TASK Parser
- **D-06:** Use Lisp `api-post` to POST to dpn-api task creation endpoint. Direct API client approach — consistent with existing `api-post` usage in the executor.
- **D-07:** CREATE_TASK format: `CREATE_TASK: <description> assignee=<agent_id>` — single line creates AND assigns. project_id, department, and source='ghost' set automatically from the executive's current project context.
- **D-08:** New function `parse-create-task-lines` extracts description and optional assignee. Called alongside existing `parse-classify-lines` and `parse-delegate-lines` in `apply-task-mutations`.
- **D-09:** Created tasks get `parent_id` set to the parent task being reviewed (if context available), linking them into the hierarchy.

### Project Review Prompt Enrichment
- **D-10:** Include FULL GSD context in project review prompt: wave structure, must_haves per task, phase goals, parent/subtask hierarchy. Executives see everything dispatched.
- **D-11:** Modify `build-project-review-job` to query task details for each project (not just counts) and include the `context` JSON (parsed into readable format) in the prompt.
- **D-12:** Include team roster in prompt so executives know who's available to delegate to (this already partially exists in `build-task-job` for Eliana).

### Delegation Model
- **D-13:** Same-action creation + delegation: `CREATE_TASK: Build auth module assignee=casey` creates the task AND assigns it in one tick.
- **D-14:** If no assignee specified in CREATE_TASK, task is created unassigned — executive can delegate in a follow-up tick via existing DELEGATE: mechanism.

### Executive Monitoring
- **D-15:** Proactive-when-idle monitoring. No change needed — `build-project-review-job` already fires as the lowest-priority action (line 687). Executives naturally review projects when they have no messages, requests, or tasks.
- **D-16:** The existing `execute-project-review` already posts review output to conversations — Nathan and other ghosts can see review activity.

### Carried From Prior Phases
- Phase 1: Hierarchical parent+subtask model with parent_id
- Phase 2: Perception returns GSD fields (project_id, context, assigned_to)
- Phase 2: assigned_to migration complete

### Claude's Discretion
- Whether CREATE_TASK should support additional fields beyond description and assignee (e.g., priority, stage, due_date)
- Whether to add UPDATE_GOAL parsing (mentioned in prompt but like CREATE_TASK, has no executor)
- Error handling when executive references nonexistent staff agents
- Whether to add ESCALATE: parsing (posts to Nathan's conversation channel)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Action Planner (Prompt Construction)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — 700 lines. Key: `build-project-review-job` (line 626), `default-job-builder` (line 677), delegation prompt (line 531), triage prompt (line 516)

### Action Executor (Command Parsing + Execution)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — 898 lines. Key: `execute-project-review` (line 643), `parse-classify-lines` (line 488), `parse-delegate-lines` (line 512), `apply-task-mutations` (line 549)

### API Client
- `/opt/project-noosphere-ghosts/lisp/runtime/api-client.lisp` — `api-post`, `api-get`, `api-patch` functions

### Task API Endpoints
- `/opt/dpn-api/src/handlers/af64_tasks.rs` — Task CRUD endpoints for creating/updating tasks via API

### Perception (Context Source)
- `/opt/dpn-api/src/handlers/af64_perception.rs` — Returns projects + tasks with GSD fields (Phase 2 output)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `parse-delegate-lines` pattern — exact template for `parse-create-task-lines` (regex line-by-line extraction)
- `apply-task-mutations` — already calls classify + delegate parsers, just needs CREATE_TASK added
- `api-post "/api/af64/tasks"` — existing pattern for creating tasks via API
- `json-object` / `json-array` — Lisp JSON construction utilities
- Executive team roster already embedded in `build-task-job` for Eliana (line 500) — can generalize

### Established Patterns
- LLM output → parse structured commands → apply via API → log to conversations
- `handler-case` wrapping for API error resilience
- `format t "[action] ..."` for tick logging
- `cognition-result-*` accessors for job results

### Integration Points
- `apply-task-mutations` is the single point where all LLM command output gets executed — add CREATE_TASK handling here
- `build-project-review-job` prompt construction is where GSD context must be injected
- Tasks created by ghosts should have `source='ghost'` to distinguish from GSD-dispatched tasks

</code_context>

<specifics>
## Specific Ideas

- The `parse-create-task-lines` function should follow the exact same pattern as `parse-delegate-lines` (line 512): scan for prefix, extract fields after it
- Team roster should come from the agents table via API, not hardcoded — use `/api/agents` filtered by department
- CREATE_TASK output should include parent task ID when the executive is reviewing a specific task, enabling automatic hierarchy linking

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 03-executive-cognition*
*Context gathered: 2026-03-26*
