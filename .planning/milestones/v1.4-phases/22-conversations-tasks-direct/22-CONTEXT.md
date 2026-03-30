# Phase 22: Conversations & Tasks Direct - Context

**Gathered:** 2026-03-29
**Status:** Ready for planning
**Source:** Auto-mode (--auto)

<domain>
## Phase Boundary

Convert ALL remaining HTTP calls from the Lisp tick engine to direct SQL, completing the removal of the HTTP middleman from the ghost-to-noosphere path. After Phase 21 eliminated perception and state updates, this phase converts: conversation read/write, message mark-as-read, task CRUD (create, status update, completion, blocked_by management), tick-log persistence, tick-reports, drives, decisions, documents, cognition telemetry, and action-planner reads. dpn-api continues serving frontends — only the ghost tick engine path changes.

</domain>

<decisions>
## Implementation Decisions

### Conversation Operations (DB-03)
- **D-01:** Convert all `api-post "/api/conversations"` calls in action-executor.lisp (~20 calls) to direct SQL INSERT into the `conversations` table. Payload fields: from_agent, to_agent (array), channel, thread_id, content, metadata.
- **D-02:** Convert mark-as-read in tick-engine.lisp to SQL using `array_append()` on the `read_by` column (integer array) or `jsonb_set()` if column is JSONB. Must match the existing API behavior of appending agent IDs without duplicates.
- **D-03:** Convert conversation reads in action-planner.lisp to SQL SELECT with the same filtering (by thread_id, agent_id, unread status).

### Task Mutations (DB-04)
- **D-04:** Convert all `api-patch "/api/af64/tasks/:id"` calls in action-executor.lisp (~15 calls) to direct SQL UPDATE. Fields: status, stage_notes, output_artifact, completed_at, blocked_by, assigned_to.
- **D-05:** Convert `api-post "/api/af64/tasks"` (task creation) to SQL INSERT with all required fields: title, description, project_id, assigned_to, priority, status, goal_id, blocked_by.
- **D-06:** Convert `api-get "/api/af64/tasks..."` reads in action-executor.lisp and action-planner.lisp to SQL SELECT with equivalent filtering (by assigned_to, status, goal_id, limit).
- **D-07:** blocked_by management: when a task completes, query `SELECT id FROM tasks WHERE blocked_by @> ARRAY[completed_task_id]` and remove the completed ID from blocked_by arrays, auto-unblocking dependent tasks.

### Auxiliary Operations (zero HTTP goal)
- **D-08:** Convert tick-log batch persistence (tick-engine.lisp) to SQL INSERT into tick_log or equivalent table.
- **D-09:** Convert tick-reports (tick-reporting.lisp) to SQL INSERT.
- **D-10:** Convert drives tick/fulfill/get (drive.lisp) to SQL.
- **D-11:** Convert decisions POST (action-executor.lisp) to SQL INSERT into decisions table.
- **D-12:** Convert documents POST (action-executor.lisp) to SQL INSERT into documents table.
- **D-13:** Convert cognition telemetry and job management (cognition-broker.lisp) to SQL.
- **D-14:** Convert agent reads and user-profile reads (action-planner.lisp, tool-socket.lisp, user-profile.lisp) to SQL SELECT.
- **D-15:** Convert task-scheduler PATCH (task-scheduler.lisp) to SQL UPDATE.

### Migration Strategy
- **D-16:** Big bang replacement — same as Phase 21. Remove all api-get/api-post/api-patch calls, no dual-path fallback.
- **D-17:** Reuse Phase 21's db-client.lisp infrastructure (db-query, db-execute, db-escape, connection pool). Add new wrapper functions for each operation type.

### Claude's Discretion
- Exact SQL for each operation (Claude reads the API handlers in dpn-api to derive equivalent SQL)
- Whether to group related SQL functions into separate files or extend db-client.lisp
- Transaction boundaries (whether multi-step operations like task completion + unblock need transactions)
- Error handling for SQL failures in mid-tick operations
- Whether projects reads (api-get "/api/projects/:id") need conversion (action-executor.lisp uses them)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Ghost Tick Engine (remaining HTTP calls)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — ~50 api-post/api-patch/api-get calls for conversations, tasks, documents, decisions
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — ~12 api-get calls for agents, conversations, documents, tasks, decisions
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — 2 remaining: mark-read (api-post) and tick-log batch (api-post)
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-reporting.lisp` — 1 api-post for tick-reports
- `/opt/project-noosphere-ghosts/lisp/runtime/drive.lisp` — 3 calls: drives tick, fulfill, get
- `/opt/project-noosphere-ghosts/lisp/runtime/cognition-broker.lisp` — 3 calls: telemetry, job create, job update
- `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` — 3 api-get calls for agent/task data
- `/opt/project-noosphere-ghosts/lisp/runtime/task-scheduler.lisp` — 1 api-patch for task status
- `/opt/project-noosphere-ghosts/lisp/runtime/user-profile.lisp` — 1 api-get for user agent
- `/opt/project-noosphere-ghosts/lisp/runtime/empirical-rollups.lisp` — 1 api-post generic

### Phase 21 Foundation (reuse these)
- `/opt/project-noosphere-ghosts/lisp/util/pg.lisp` — libpq FFI bindings, connection pool
- `/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp` — db-query, db-execute, db-escape wrappers + perception SQL

### dpn-api Handlers (SQL reference)
- `/opt/dpn-api/src/handlers/` — Rust handlers that define the SQL for each endpoint, reference for query logic

### Requirements
- `.planning/REQUIREMENTS.md` — DB-03 (conversations SQL), DB-04 (task mutations SQL)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `db-query` / `db-execute` / `db-escape` from Phase 21 — all new SQL functions build on these
- `pg-query` / `pg-execute` from pg.lisp — low-level FFI already handles connection pool, result parsing
- Connection pool initialized at startup in launch.sh

### Established Patterns
- Phase 21 pattern: one SQL wrapper function per operation (e.g., db-perceive-messages, db-update-energy)
- Results returned as lists of alists with keyword keys
- NULL handling via PQgetisnull check before PQgetvalue
- `array_to_json()` SQL casting for PG arrays

### Integration Points
- action-executor.lisp is the largest file (~1200 lines) with the most HTTP calls
- action-planner.lisp is second (~900 lines) with read-heavy HTTP calls
- Both files use `api-post`/`api-get`/`api-patch` from api-client.lisp
- After this phase, api-client.lisp becomes dead code for the tick engine (still used by nothing)

</code_context>

<specifics>
## Specific Ideas

- The conversation INSERT must handle the `to_agent` array field — PG text array literal format `'{agent1,agent2}'`
- Mark-as-read needs `UPDATE conversations SET read_by = array_append(read_by, $1) WHERE id = $2 AND NOT ($1 = ANY(read_by))` to prevent duplicates
- Task completion with auto-unblock is a natural transaction boundary: UPDATE task SET status='done' + UPDATE dependent tasks SET blocked_by = array_remove(blocked_by, task_id)
- Projects reads in action-executor (for project status checks) should also convert to SQL

</specifics>

<deferred>
## Deferred Ideas

- Removing api-client.lisp entirely — verify no other code uses it first
- LISTEN/NOTIFY for real-time event triggers — Phase 23+ scope
- Connection pool scaling beyond 2 connections — monitor after full SQL migration

</deferred>

---

*Phase: 22-conversations-tasks-direct*
*Context gathered: 2026-03-29 via auto-mode*
