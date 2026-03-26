# Phase 5: Feedback & Reporting - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Close the feedback loop: enhance task completion reports with GSD context, implement wave advancement via DB trigger, wire blocker escalation to executive urgency, and ensure Nathan only gets pulled in for unresolvable blockers and project completion. Much of the infrastructure already exists — this phase connects and enhances it.

</domain>

<decisions>
## Implementation Decisions

### Critical Finding: Feedback Infrastructure Substantially Exists
- **D-01:** `parse-complete-lines` + PATCH status→done already handles task completion in action-executor.lisp
- **D-02:** Pipeline advancement (mark done → unblock next stage, line 198+) already works for hardcoded pipelines
- **D-03:** Conversation posting on completion already exists (line 254: "[Pipeline] agent completed stage...")
- **D-04:** Blocker detection exists (line 929: searches for "BLOCKED:" in LLM output)
- **D-05:** DB triggers already fire: `task_completed_trigger` (AFTER UPDATE), `task_assigned_notify` (AFTER INSERT), `task_rejected_trigger` (BEFORE UPDATE), `set_completed_date` (BEFORE UPDATE)
- **D-06:** Energy rewards: +15 for stage complete, +30 for pipeline complete — already coded

### Completion Reporting
- **D-07:** Enhance existing completion messages with GSD context: project name, must_have satisfied, tool results summary from stage_notes. Standard format + GSD enrichment.
- **D-08:** Completion reports posted to conversations table addressed to the supervising executive (from staff, to executive).

### Wave Advancement
- **D-09:** Implement via DB trigger on `on_task_completed_after()`. When a task completes, check if all sibling tasks in the same wave (same project_id, same wave number in context JSON) are done. If so, mark next-wave tasks as 'open'. Automatic, no tick needed.
- **D-10:** Wave number stored in task context JSON field (already populated by dispatch in Phase 1). Trigger parses context to extract wave number.

### Blocker Escalation
- **D-11:** Staff posts BLOCKED: message to conversations addressed to supervising executive. Executive perceives with elevated urgency (+50 for messages). Already partially coded — enhance the blocker detection path (line 929) to ensure it posts to the right executive.
- **D-12:** No Nathan notification for routine blockers — executives handle them. Nathan only for unresolvable escalations.

### Nathan Notifications
- **D-13:** Nathan receives conversation messages ONLY when: (1) executive escalates a blocker they can't resolve (ESCALATE: @nathan), (2) project reaches completion (all tasks done). Everything else handled by ghosts.
- **D-14:** Project completion detection: when last task in project completes, post summary to Nathan's conversation channel.

### Carried From Prior Phases
- Phase 1: Hierarchical tasks with wave context in JSON
- Phase 2: Perception returns all GSD fields
- Phase 3: CREATE_TASK + executive delegation
- Phase 4: Tool results in stage_notes

### Claude's Discretion
- Whether to add a /gsd:progress query command that reads project status from DB (or rely on dispatch --status)
- Format of completion summary message (markdown vs plain text)
- Whether wave advancement trigger should log to a separate audit table

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Action Executor (Completion + Blocker Handling)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Key: `advance-pipeline-stage` (line 198), completion posting (line 254), `parse-complete-lines` (line 532), blocker detection (line 929), energy rewards (lines 292-303)

### DB Triggers
- `on_task_completed_after()` — AFTER UPDATE trigger on tasks table. This is where wave advancement logic should go.
- `notify_task_assigned()` — AFTER INSERT trigger. Already fires on new task creation.
- `set_completed_date()` — BEFORE UPDATE trigger. Auto-sets completed_date.
- `on_task_rejected()` — BEFORE UPDATE trigger. Handles rejection logic.

### Dispatch (Wave Context)
- `/root/gotcha-workspace/tools/gsd/dispatch_to_db.py` — Phase 1 output. Tasks have context JSON with wave numbers.

### Perception (Urgency)
- `/opt/dpn-api/src/handlers/af64_perception.rs` — Messages with from_agent give +50 urgency boost in tick engine

### Conversation API
- `/opt/dpn-api/src/handlers/af64_conversations.rs` — POST /api/conversations for messaging

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `advance-pipeline-stage` — existing pattern for stage→next_stage advancement. Wave advancement follows similar logic.
- `apply-task-mutations` — already parses COMPLETE: lines and PATCHes task status
- `api-post "/api/conversations"` — used throughout for posting messages
- `json-object` / `json-array` — Lisp JSON construction
- DB trigger functions can be modified via `CREATE OR REPLACE FUNCTION`

### Established Patterns
- Completion flow: LLM outputs COMPLETE: → parse → PATCH status=done → post to conversations → energy reward
- Blocker flow: LLM outputs BLOCKED: → detected in content → logged (but not escalated to executive)
- Pipeline advancement: stage N done → find stage N+1 task → unblock → notify assignee

### Integration Points
- `on_task_completed_after()` trigger fires on every task status→done. Add wave advancement logic here.
- Conversations table: from_agent, to_agent (array), channel, message_type, metadata — used for all reporting
- Task context JSON: `{"wave": N, "must_haves": [...]}` — populated by dispatch

</code_context>

<specifics>
## Specific Ideas

- Wave advancement trigger should check: `SELECT COUNT(*) FROM tasks WHERE project_id = NEW.project_id AND context::jsonb->>'wave' = current_wave AND status != 'done'`. If 0, UPDATE next-wave tasks to status='open'.
- Nathan notification on project completion: when last task in project is done, INSERT into conversations with to_agent=['nathan'], channel='noosphere', message_type='project_complete'
- Blocker escalation should use the task's assigned_by field to determine which executive to notify

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 05-feedback-reporting*
*Context gathered: 2026-03-26*
