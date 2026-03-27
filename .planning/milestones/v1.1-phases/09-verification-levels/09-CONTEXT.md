# Phase 9: Verification Levels - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Add quality severity classification (CRITICAL/WARNING/SUGGESTION) to task completion by leveraging the existing Phase 7 structured artifact issues array. Extract issues from completed artifacts, include severity in completion reports to executives, and add a new urgency modifier in the tick engine so CRITICAL issues get elevated perception priority.

</domain>

<decisions>
## Implementation Decisions

### Quality Assessment Format
- **D-01:** Leverage the existing Phase 7 `issues` array in structured artifacts. When a ghost completes a task, the action executor extracts issues from the artifact's `issues` field and includes them in the completion report. No new command or parser needed.
- **D-02:** The `issues` array already has `{severity: string, description: string}` structure from Phase 7. Severity values are: "CRITICAL", "WARNING", "SUGGESTION".
- **D-03:** If the artifact has no issues array or it's empty, the completion report indicates "no quality issues" — this is the happy path.

### Severity Classification
- **D-04:** Self-assessed by the completing ghost. The LLM naturally includes severity in its issues array based on the work context. No external validation or rule-based classification.
- **D-05:** Valid severity values: CRITICAL (blocking issue, must-haves potentially unmet), WARNING (non-blocking issue, partial completion), SUGGESTION (improvement opportunity, non-essential).

### Executive Urgency Escalation
- **D-06:** Add a new `quality_issue_boost` to the urgency formula in tick-engine.lisp. The tick engine checks for unresolved CRITICAL issues across the agent's tasks and applies an urgency boost separate from the regular message boost.
- **D-07:** The boost applies to the supervising executive's urgency when they have tasks with CRITICAL verification issues pending review.

### Completion Report Enhancement
- **D-08:** Enhance the existing completion report (conversation POST) to include a quality assessment section. Format: severity counts + individual CRITICAL/WARNING items listed. SUGGESTION items summarized as count only.
- **D-09:** Completion conversation metadata includes `severity_level` field set to the highest severity found (CRITICAL > WARNING > SUGGESTION > none).

### Claude's Discretion
- Exact urgency boost value for CRITICAL issues (should be significant — perhaps +35 to +45, between task boost and message boost)
- Whether to store quality assessment summary in a task field or only in the conversation message
- How to surface quality issues in the perception endpoint for executive review

### Carried From Prior Phases
- Phase 5: Completion reports to executives with project, must_haves, stage_notes summary
- Phase 7: Structured artifacts with `issues` array (severity, description) in JSONB stage_notes
- Existing: Urgency formula: pressure*energy + msg_boost(50) + req_boost(40) + task_boost(25) + project_boost(15*N) + deadline_boost(0-50)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Action Executor (Completion Flow)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Key: `parse-complete-lines` (lines 592-606), completion execution and executive notification (lines 692-741), `validate-artifact-base` (lines 78-104), `build-stage-artifact` (line 142)

### Tick Engine (Urgency Formula)
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — Key: urgency calculation (lines 144-168), component boosts (msg_boost, req_boost, task_boost, project_boost, deadline_boost)

### Task Scheduler (Deadline Boost)
- `/opt/project-noosphere-ghosts/lisp/runtime/task-scheduler.lisp` — Key: `deadline-urgency-boost` (lines 23-45)

### Perception Endpoint
- `/opt/dpn-api/src/handlers/af64_perception.rs` — How tasks and messages are returned to ghosts

### Conversations API
- `/opt/dpn-api/src/handlers/af64_conversations.rs` — POST /api/conversations for completion reports

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `issues` array in structured artifacts — already has severity + description fields from Phase 7
- Completion report format — existing conversation POST with project, must_haves, stage_notes summary
- `validate-artifact-base` — validates issues array if present (must be array)
- Urgency formula in tick-engine — extensible with new boost components

### Established Patterns
- Completion flow: COMPLETE: parsed → task marked done → recurrence check → executive notification
- Urgency scoring: additive boosts from different sources (messages, tasks, projects, deadlines)
- Conversation metadata: already carries source, task-id, project-id — add severity_level

### Integration Points
- `action-executor.lisp` completion execution block (lines 692-741) — extract issues from artifact, include in completion report
- `tick-engine.lisp` urgency calculation (lines 144-168) — add quality_issue_boost
- Completion conversation metadata — add severity_level field
- Perception endpoint — potentially add quality issue visibility for executives

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

*Phase: 09-verification-levels*
*Context gathered: 2026-03-26*
