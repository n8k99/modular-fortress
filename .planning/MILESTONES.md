# Milestones

## v1.1 Ghost Coordination Patterns (Shipped: 2026-03-27)

**Phases completed:** 5 phases, 12 plans, 22 tasks

**Key accomplishments:**

- Migrated tasks.blocked_by from INTEGER to INTEGER[] with GIN index and auto-unblock trigger via array_remove
- SQL-level blocked_by filtering in all perception queries with executive blocked task visibility and INTEGER[] task API support
- CREATE_TASK parser extended with blocked_by=#id,#id syntax and dispatch_to_db.py auto-populates blocked_by from wave ordering via two-pass approach
- Migrated stage_notes from TEXT to JSONB with legacy data wrapping, updated Rust API to serve/accept JSON objects
- JSON schema validation replacing keyword matching in validate-stage-output, structured artifact storage in stage_notes, and final deliverable persistence to documents table per D-07
- Replaced disk-file predecessor loading with DB-sourced stage_notes query, formatting schema v0/v1 artifacts into LLM prompts
- Append-only decisions CRUD API (GET/POST /api/decisions) with project_id, department, and owner filters following af64_tasks.rs pattern
- Decision capture from DECISION: prefix lines via API POST, and prior-decisions context injection into executive project review prompts
- Quality issue extraction from structured artifacts into executive completion reports with CRITICAL/WARNING/SUGGESTION severity classification
- +40 urgency boost for CRITICAL quality issues in tick engine and critical_issues array in executive perception endpoint
- list_agents metadata exposure, PATCH merge with COALESCE semantics, and +12 idle-transition energy reward
- Idle transition detection in tick engine Phase 5 with one-time energy boost and enriched team roster showing agent availability

---

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
