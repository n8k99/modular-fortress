# Milestones

## v1.0 Noosphere Dispatch Pipeline (Shipped: 2026-03-26)

**Phases completed:** 5 phases, 11 plans, 21 tasks

**Key accomplishments:**

- Fixed dispatch_project() with H1 name extraction, owner column persistence, department lookup from agents table, and PG_CONFIG integration -- 6 tests all GREEN
- Hierarchical parent+subtask dispatch with must_have truth extraction, assigned_to text[], department routing, and enhanced status reporting -- 12 tests all GREEN
- Perception endpoint enhanced with GSD task fields (project_id, source, context, assigned_to, priority, scheduled_at) using assigned_to array queries replacing legacy assignee string matching
- All 5 PERC requirements verified via automated E2E test script (curl+jq) against live dpn-api, with human-approved urgency boost path confirmation
- POST /api/af64/tasks extended with task_id/parent_id/source fields and project_id filter for ghost task creation and prompt enrichment
- Enriched build-project-review-job with per-task GSD context (wave numbers, must_haves) and dynamic team roster for wave-aware executive decomposition and delegation
- parse-create-task-lines extracts task descriptions from LLM output and POSTs them to /api/af64/tasks with ghost source, project linkage, and optional assignee
- Fixed wildcard scope bug making memory tools invisible, registered claude_code tool with bash wrapper for engineering ghosts
- 18-test E2E smoke script covering DB/API/code/memory tools plus D-08 stage_notes, with agent tool_scope corrections for 4 agents per D-10/D-11
- Wave advancement trigger, enriched completion reports, blocker/escalation routing via conversations table
- Wave-level progress in dispatch --status and comprehensive E2E script verifying all 6 REPT requirements against live DB triggers

---
