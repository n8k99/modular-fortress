# Milestones

## v1.3 PARAT Noosphere Schema (Shipped: 2026-03-29)

**Phases completed:** 15 phases, 33 plans, 64 tasks

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
- POST /api/conversations/mark-read endpoint with read_by perception filtering, GIN index, and historical cleanup of 2229 stale messages
- Tick engine marks all perceived messages as read after cognition via api-post to /api/conversations/mark-read, closing the perception-cognition-mark-read loop
- JSONB schedule column on projects, PATCH API endpoint, perception schedule metadata, and Lisp cron matcher for 5-field cron expressions
- Cron schedule evaluation wired into tick engine ranking with +50 boost, double-fire prevention, and Standing Orders Fired prompt enrichment for executive project reviews
- 6 operational Python scripts registered in ghost tool-registry with operations scope, process-tool-calls wired into execute-project-review, Podcast Watch schedule added to Project #14
- 1. [Rule 1 - Bug] Corrected ops_daily_note tool name
- editorial_nightly registered with dynamic per-label tool mapping so Sylvia executes nightly editorial via standing order, not OpenClaw cron
- Trading briefing and calendar sync tools wired into ghost standing orders under Kathryn's Project #10 with 4 label-to-tool mappings
- 4 PARAT tables (areas/archives/resources/templates) with 3 DB-level enforcement triggers, 5 seeded areas, and ApiError::Conflict for 409 responses
- Rust CRUD modules for all 4 PARAT tables (areas, archives, resources, templates) with struct definitions, async query functions, full-text search, and lib.rs re-exports
- REST handlers for areas/archives/resources/templates with frozen-resource 409, metadata-only archive PATCH, and template version history endpoint
- Lifestage lifecycle column on projects with forward-only trigger, area_id FK linking projects to areas, and project_id FK migrating goals from wikilink text to integer references
- dpn-api project handlers updated with lifestage/area_id support and perception endpoint enriched with LEFT JOIN areas for ghost lifecycle awareness
- vault_notes table renamed to memories with INSTEAD OF view bridge, compression metadata columns, and departments lookup table with FK backfill
- Renamed vault_notes module to memories across 18 dpn-core files with Memory/MemoryLight structs including compression_tier and compressed_from fields, cargo check passes clean
- dpn-api handlers migrated from vault_notes to memories, release binary built, PM2 restarted and serving live traffic
- 5 new tables (teams, team_members, ghost_relationships, agent_areas, routines) and agents.aliases column created with all FK constraints, CHECK constraints, and 18 indexes
- 13 teams seeded, 500 relationships migrated from text arrays, 67 area mappings with cross-functional agents, 11 routines from standing orders, Nova aliased as T.A.S.K.S.
- 64 EM Staff documents enriched with YAML frontmatter linking agent_id, memory_column, department, team, area; all agents backfilled with document_path
- Deduplicated 1984 ChatGPT conversations across two archive paths into 990 canonical entries, imported all into archives table with extracted dates, topics, and trivial flags
- 822 ChatGPT conversations summarized with LLM-generated domain classification, cascaded into 28 temporal memories (18 monthly, 7 quarterly, 3 yearly) with 111 ghost perspective narratives for Nova, LRM, Vincent, and Sylvia
- 316 daily notes and 65 weekly notes linked to imported Nexus archive conversations via wikilinks in ## Nexus Imports sections

---

## v1.2 Operational Readiness (Shipped: 2026-03-28)

**Phases completed:** 5 phases, 8 plans, 16 tasks

**Key accomplishments:**

- POST /api/conversations/mark-read endpoint with read_by perception filtering, GIN index, and historical cleanup of 2229 stale messages
- Tick engine marks all perceived messages as read after cognition via api-post to /api/conversations/mark-read, closing the perception-cognition-mark-read loop
- JSONB schedule column on projects, PATCH API endpoint, perception schedule metadata, and Lisp cron matcher for 5-field cron expressions
- Cron schedule evaluation wired into tick engine ranking with +50 boost, double-fire prevention, and Standing Orders Fired prompt enrichment for executive project reviews
- 6 operational Python scripts registered in ghost tool-registry with operations scope, process-tool-calls wired into execute-project-review, Podcast Watch schedule added to Project #14
- 1. [Rule 1 - Bug] Corrected ops_daily_note tool name
- editorial_nightly registered with dynamic per-label tool mapping so Sylvia executes nightly editorial via standing order, not OpenClaw cron
- Trading briefing and calendar sync tools wired into ghost standing orders under Kathryn's Project #10 with 4 label-to-tool mappings

---

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
