# Phase 22: Conversations & Tasks Direct - Research

**Researched:** 2026-03-29
**Domain:** Direct PostgreSQL from Common Lisp (SBCL) -- converting all remaining HTTP calls to SQL
**Confidence:** HIGH

## Summary

Phase 22 completes the removal of HTTP from the ghost tick engine path, building on Phase 21's foundation (pg.lisp FFI bindings, db-client.lisp wrappers, connection pool). The scope is converting approximately 80+ remaining `api-get`, `api-post`, `api-patch`, and `api-put` calls across 10 Lisp source files to direct SQL using the existing `db-query`, `db-execute`, and `db-escape` infrastructure.

The work divides into three natural tiers: (1) conversations operations (DB-03: ~13 POST, mark-read, reads), (2) task mutations (DB-04: ~12 PATCH, 1 POST create, ~4+ GET), and (3) auxiliary operations (documents, decisions, drives, tick-log, tick-reports, memory, agent reads, requests, cognition telemetry). The auxiliary tier (D-08 through D-15) is larger than conversations+tasks combined -- it includes ~20 additional HTTP calls across 8 files.

**Primary recommendation:** Organize new SQL wrapper functions into separate domain files (db-conversations.lisp, db-tasks.lisp, db-auxiliary.lisp) rather than extending db-client.lisp further (already 772 lines). Implement conversations first (most frequent operation), then tasks, then auxiliary. Use `db-query` (not `db-execute`) for any INSERT/UPDATE with RETURNING clause.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Convert all `api-post "/api/conversations"` calls in action-executor.lisp (~20 calls) to direct SQL INSERT into the `conversations` table. Payload fields: from_agent, to_agent (array), channel, thread_id, content, metadata.
- **D-02:** Convert mark-as-read in tick-engine.lisp to SQL using `array_append()` on the `read_by` column (integer array) or `jsonb_set()` if column is JSONB. Must match the existing API behavior of appending agent IDs without duplicates.
- **D-03:** Convert conversation reads in action-planner.lisp to SQL SELECT with the same filtering (by thread_id, agent_id, unread status).
- **D-04:** Convert all `api-patch "/api/af64/tasks/:id"` calls in action-executor.lisp (~15 calls) to direct SQL UPDATE. Fields: status, stage_notes, output_artifact, completed_at, blocked_by, assigned_to.
- **D-05:** Convert `api-post "/api/af64/tasks"` (task creation) to SQL INSERT with all required fields: title, description, project_id, assigned_to, priority, status, goal_id, blocked_by.
- **D-06:** Convert `api-get "/api/af64/tasks..."` reads in action-executor.lisp and action-planner.lisp to SQL SELECT with equivalent filtering (by assigned_to, status, goal_id, limit).
- **D-07:** blocked_by management: when a task completes, query `SELECT id FROM tasks WHERE blocked_by @> ARRAY[completed_task_id]` and remove the completed ID from blocked_by arrays, auto-unblocking dependent tasks.
- **D-08:** Convert tick-log batch persistence (tick-engine.lisp) to SQL INSERT into tick_log table.
- **D-09:** Convert tick-reports (tick-reporting.lisp) to SQL INSERT.
- **D-10:** Convert drives tick/fulfill/get (drive.lisp) to SQL.
- **D-11:** Convert decisions POST (action-executor.lisp) to SQL INSERT into decisions table.
- **D-12:** Convert documents POST (action-executor.lisp) to SQL INSERT into documents table.
- **D-13:** Convert cognition telemetry and job management (cognition-broker.lisp) to SQL.
- **D-14:** Convert agent reads and user-profile reads (action-planner.lisp, tool-socket.lisp, user-profile.lisp) to SQL SELECT.
- **D-15:** Convert task-scheduler PATCH (task-scheduler.lisp) to SQL UPDATE.
- **D-16:** Big bang replacement -- same as Phase 21. Remove all api-get/api-post/api-patch calls, no dual-path fallback.
- **D-17:** Reuse Phase 21's db-client.lisp infrastructure (db-query, db-execute, db-escape, connection pool). Add new wrapper functions for each operation type.

### Claude's Discretion
- Exact SQL for each operation (Claude reads the API handlers in dpn-api to derive equivalent SQL)
- Whether to group related SQL functions into separate files or extend db-client.lisp
- Transaction boundaries (whether multi-step operations like task completion + unblock need transactions)
- Error handling for SQL failures in mid-tick operations
- Whether projects reads (api-get "/api/projects/:id") need conversion (action-executor.lisp uses them)

### Deferred Ideas (OUT OF SCOPE)
- Removing api-client.lisp entirely -- verify no other code uses it first
- LISTEN/NOTIFY for real-time event triggers -- Phase 23+ scope
- Connection pool scaling beyond 2 connections -- monitor after full SQL migration
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DB-03 | Conversations (read, write, mark-read) executed via SQL from Lisp without HTTP -- including read_by array operations | Full SQL patterns documented from af64_conversations.rs; conversations table schema verified (varchar(50)[] arrays); mark-read SQL with array_append + duplicate prevention verified |
| DB-04 | Task mutations (create, update status, complete, blocked_by management) executed via SQL from Lisp | Full SQL patterns documented from af64_tasks.rs; tasks table schema verified (integer[] blocked_by, text[] assigned_to); create/update/blocked_by SQL verified |
</phase_requirements>

## Standard Stack

### Core (Reuse from Phase 21)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| pg.lisp (FFI) | Phase 21 | libpq.so.5 bindings: PQexec, PQescapeLiteral, connection pool | Zero-deps AF64 convention; already proven in Phase 21 |
| db-client.lisp | Phase 21 | db-query, db-execute, db-escape, db-query-single, db-coerce-row | Higher-level wrappers; all new functions build on these |

### No New Dependencies
This phase adds zero new dependencies. All SQL wrapper functions use the existing pg.lisp FFI and db-client.lisp infrastructure from Phase 21.

## Architecture Patterns

### Recommended File Organization
```
lisp/runtime/
  db-client.lisp           # Existing: pool, escape, perception, agent state (772 lines)
  db-conversations.lisp    # NEW: db-insert-conversation, db-mark-read, db-get-conversations
  db-tasks.lisp            # NEW: db-update-task, db-create-task, db-get-tasks, db-unblock-dependents
  db-auxiliary.lisp         # NEW: db-insert-tick-log, db-insert-tick-report, db-tick-drives,
                            #      db-fulfill-drive, db-get-drives, db-insert-decision,
                            #      db-insert-document, db-get-agent, db-update-request,
                            #      db-upsert-memory, db-get-decisions
```

### Pattern 1: SQL Wrapper Function (from Phase 21)
**What:** One function per SQL operation, using db-escape for safe interpolation, db-query for SELECTs, db-execute for INSERT/UPDATE without RETURNING.
**When to use:** Every HTTP-to-SQL conversion.
**Example:**
```lisp
;; Source: Phase 21 db-client.lisp established pattern
(defun db-insert-conversation (from-agent to-agents message &key channel thread-id metadata)
  "Insert a conversation message. Returns the new row ID as integer."
  (let* ((escaped-from (db-escape from-agent))
         (escaped-msg (db-escape message))
         (escaped-channel (db-escape (or channel "noosphere")))
         ;; Build PostgreSQL text array literal: '{agent1,agent2}'
         (to-array (format nil "'{~{~a~^,~}}'" to-agents))
         (escaped-thread (if thread-id (db-escape (princ-to-string thread-id)) "gen_random_uuid()"))
         (escaped-meta (if metadata (db-escape (encode-json metadata)) "'{}'::jsonb"))
         (sql (format nil
                "INSERT INTO conversations (from_agent, to_agent, message, channel, ~
                 message_type, thread_id, metadata) ~
                 VALUES (~a, ~a, ~a, ~a, 'chat', ~a, ~a::jsonb) RETURNING id"
                escaped-from to-array escaped-msg escaped-channel
                escaped-thread escaped-meta))
         ;; MUST use db-query (not db-execute) because RETURNING produces tuples
         (row (db-query-single sql)))
    (when row
      (let ((id-val (gethash :id row)))
        (if (stringp id-val) (parse-integer id-val :junk-allowed t) id-val)))))
```

### Pattern 2: PostgreSQL Array Handling in Lisp
**What:** Building PostgreSQL text/integer array literals from Lisp lists.
**When to use:** Any column with `text[]` or `integer[]` type.
**Example:**
```lisp
;; Text array (conversations.to_agent is varchar(50)[])
;; Input: list of strings like ("nova" "eliana")
;; Output: ARRAY['nova','eliana']::varchar(50)[]
(defun pg-text-array (items)
  "Build a PostgreSQL text array literal from a list of strings."
  (if (or (null items) (and (vectorp items) (= (length items) 0)))
      "'{}'::varchar(50)[]"
      (let ((item-list (if (vectorp items) (coerce items 'list) items)))
        (format nil "ARRAY[~{~a~^,~}]::varchar(50)[]"
                (mapcar #'db-escape item-list)))))

;; Integer array (tasks.blocked_by is integer[])
(defun pg-int-array (items)
  "Build a PostgreSQL integer array literal from a list of integers."
  (if (or (null items) (and (vectorp items) (= (length items) 0)))
      "'{}'::integer[]"
      (let ((item-list (if (vectorp items) (coerce items 'list) items)))
        (format nil "ARRAY[~{~d~^,~}]::integer[]" item-list))))
```

### Pattern 3: Transaction for Multi-Step Operations
**What:** Using PostgreSQL transaction via BEGIN/COMMIT for atomic multi-step operations.
**When to use:** Task completion with auto-unblock (D-07), and any operation that modifies multiple rows atomically.
**Example:**
```lisp
;; Task completion with automatic unblocking of dependent tasks
(defun db-complete-task-and-unblock (task-id &key stage-notes completed-date)
  "Complete a task and remove it from blocked_by arrays of dependent tasks.
   Uses a transaction for atomicity."
  (let* ((escaped-notes (if stage-notes (db-escape (encode-json stage-notes)) "NULL"))
         (escaped-date (db-escape (or completed-date (utc-now-iso))))
         (sql (format nil
                "BEGIN; ~
                 UPDATE tasks SET status = 'done', completed_date = ~a, ~
                   stage_notes = ~a ~
                 WHERE id = ~d; ~
                 UPDATE tasks SET blocked_by = array_remove(blocked_by, ~d) ~
                 WHERE ~d = ANY(blocked_by); ~
                 COMMIT;"
                escaped-date escaped-notes task-id task-id task-id)))
    ;; NOTE: PQexec handles multi-statement SQL; result is for the last statement
    (db-execute sql)))
```

### Pattern 4: Replacing api-post in action-executor.lisp (Bulk Conversion)
**What:** The 13 `api-post "/api/conversations"` calls in action-executor all follow the same pattern: build a payload hash-table, POST it. Replace with a single db-insert-conversation call.
**When to use:** Each conversation POST site in action-executor.lisp.
**Example transformation:**
```lisp
;; BEFORE (HTTP):
(api-post "/api/conversations"
  (json-object :from-agent agent-id
               :to-agent (vector target-id)
               :message msg
               :channel "noosphere"
               :metadata (json-object :source "handoff")))

;; AFTER (SQL):
(db-insert-conversation agent-id (list target-id) msg
  :channel "noosphere"
  :metadata (json-object :source "handoff"))
```

### Anti-Patterns to Avoid
- **Using db-execute for INSERT...RETURNING:** pg-execute checks for COMMAND_OK (status 1), but RETURNING produces TUPLES_OK (status 2). Use db-query/db-query-single instead.
- **Building array literals without escaping:** Never interpolate user-controlled strings directly into array literals. Always use db-escape for each element.
- **Multi-statement SQL without transaction awareness:** PQexec with multiple semicolon-separated statements executes them all but only returns the result of the last one. Wrap in BEGIN/COMMIT if atomicity matters.
- **Forgetting the JSON keyword hyphenation quirk:** The Lisp JSON parser converts underscores to hyphens. Column `from_agent` becomes `:from-agent` in hash-table keys. All new code must use hyphenated keywords.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| SQL escaping | Custom string replacement | `db-escape` (PQescapeLiteral) | SQL injection prevention; handles all edge cases |
| Connection management | Manual connect/disconnect | `*db-pool*` (pg-pool) | Pool handles reconnection, in-use tracking |
| Result parsing | Custom PG wire protocol parsing | `pg-query` -> `result-to-vectors` | Already handles NULL, column names, type coercion |
| Array building | Manual string concatenation | `pg-text-array` / `pg-int-array` helpers | Proper escaping of array elements |

## Common Pitfalls

### Pitfall 1: INSERT RETURNING vs db-execute
**What goes wrong:** Using `db-execute` for `INSERT ... RETURNING id` causes a "pg-execute failed (status 2)" error because RETURNING produces TUPLES_OK, not COMMAND_OK.
**Why it happens:** Phase 21 only needed db-execute for UPDATE (no RETURNING). Phase 22 needs RETURNING for conversation and task creation.
**How to avoid:** Use `db-query-single` for all INSERT...RETURNING statements. Use `db-execute` only for pure INSERT/UPDATE/DELETE without RETURNING.
**Warning signs:** "pg-execute failed (status 2)" error in logs.

### Pitfall 2: PostgreSQL Array Literal Syntax
**What goes wrong:** Building `'{nova,eliana}'` text array literal when values contain special characters, or confusing text array syntax with ARRAY[] constructor syntax.
**Why it happens:** PostgreSQL has two array syntaxes: string literal `'{a,b}'` and constructor `ARRAY['a','b']`. The constructor is safer because each element can be independently escaped.
**How to avoid:** Use `ARRAY[escaped1,escaped2]::type[]` constructor syntax with `db-escape` for each element. Never use the string literal syntax for user-provided values.
**Warning signs:** SQL syntax errors involving arrays; conversations with broken to_agent arrays.

### Pitfall 3: read_by Column Type Mismatch (D-02)
**What goes wrong:** Passing an integer to `array_append(read_by, ...)` when `read_by` is `varchar(50)[]`, not `integer[]`.
**Why it happens:** CONTEXT.md mentions "integer array" for read_by, but the actual schema is `character varying(50)[]` (stores agent string IDs like 'nova').
**How to avoid:** Use agent string IDs (not integers) in mark-read SQL: `UPDATE conversations SET read_by = array_append(read_by, 'nova') WHERE id = 42 AND NOT ('nova' = ANY(read_by))`.
**Warning signs:** Type mismatch errors from PostgreSQL.

### Pitfall 4: Documents POST Goes to memories Table (D-12)
**What goes wrong:** Inserting into the `documents` table when the API actually writes to `memories` table (via dpn_core::db::memories::create).
**Why it happens:** The documents.rs handler was refactored in M2.2 to write to `memories` as primary storage, not the legacy `documents` table.
**How to avoid:** Check what the ghost actually uses documents POST for. If creating daily notes/operational docs, INSERT into `memories`. If creating world docs, INSERT into `documents`. Read the action-executor context to determine which table is correct.
**Warning signs:** Documents appearing in wrong table; search not finding newly created docs.

### Pitfall 5: Connection Pool Exhaustion with 2 Connections
**What goes wrong:** Long-running multi-statement transactions hold a connection, and a concurrent db-query needs the second connection. If both are in use, pool-acquire signals an error.
**Why it happens:** Pool size is 2. A transaction blocks one connection for its duration. The tick engine is single-threaded but handler-case might retry on a different code path.
**How to avoid:** Keep transactions short. Never nest db-query calls inside an already-acquired connection's scope. The tick engine is single-threaded, so contention should be minimal -- but be careful with handler-case retry paths.
**Warning signs:** "No available PG connections in pool" error.

### Pitfall 6: Missing Tick-Reports and Cognition Endpoints
**What goes wrong:** The existing `api-post "/api/tick-reports"` and `api-post "/api/cognition/telemetry"` calls already fail silently (no matching routes in dpn-api). Converting them to SQL is new functionality, not a replacement.
**Why it happens:** These endpoints were never implemented in dpn-api (not found in main.rs routes). The Lisp code has handler-case fallbacks (write to local file).
**How to avoid:** For tick-reports, write directly to the `tick_reports` table (schema confirmed). For cognition telemetry, it currently writes to a local file as primary and the API POST as secondary. Convert to SQL INSERT -- but note this is additive, not a simple replacement.
**Warning signs:** None visible -- these already fail silently. But after conversion, data will actually persist to the database.

### Pitfall 7: Multi-Statement PQexec Behavior
**What goes wrong:** PQexec with `"BEGIN; UPDATE...; UPDATE...; COMMIT;"` only returns the result of the LAST statement. If a middle statement fails, you don't get its error -- you get the COMMIT result (or the first error that aborts).
**Why it happens:** This is how libpq works with multi-statement strings.
**How to avoid:** For critical transactions, either: (1) use multi-statement and check the final result, or (2) execute each statement separately and handle errors per-statement. Option 1 is fine for atomicity since PostgreSQL will roll back on error within a transaction.
**Warning signs:** Silent data corruption where part of a transaction succeeds and part fails.

## Comprehensive HTTP Call Inventory

### action-executor.lisp (1234 lines) -- 39 HTTP calls
| Type | Endpoint | Count | SQL Operation |
|------|----------|-------|---------------|
| api-post | /api/conversations | 13 | INSERT INTO conversations |
| api-patch | /api/af64/tasks/:id | 12 | UPDATE tasks SET ... WHERE id = |
| api-get | /api/af64/tasks... | 4 | SELECT FROM tasks WHERE ... |
| api-get | /api/projects/:id | 3 | SELECT FROM projects WHERE id = |
| api-post | /api/documents | 2 | INSERT INTO memories (or documents) |
| api-post | /api/af64/tasks | 1 | INSERT INTO tasks |
| api-post | /api/decisions | 1 | INSERT INTO decisions |
| api-put | /api/agents/memory | 1 | INSERT/UPDATE agent_daily_memory |
| api-put | /api/agents/requests/:id | 2 | UPDATE agent_requests SET ... |

### action-planner.lisp (969 lines) -- 11 HTTP calls
| Type | Endpoint | Count | SQL Operation |
|------|----------|-------|---------------|
| api-get | /api/agents/:id | 1 | SELECT FROM agents WHERE id = |
| api-get | /api/documents/:id | 1 | SELECT FROM documents WHERE id = |
| api-get | /api/conversations | 1 | SELECT FROM conversations WHERE ... |
| api-get | /api/documents/search | 1 | SELECT FROM memories/documents WHERE path LIKE |
| api-get | /api/af64/tasks | 4 | SELECT FROM tasks WHERE ... |
| api-get | /api/agents (list all) | 1 | SELECT FROM agents (already in db-client.lisp as db-fetch-agents) |
| api-get | /api/decisions | 1 | SELECT FROM decisions WHERE ... |

### tick-engine.lisp (557 lines) -- 2 HTTP calls
| Type | Endpoint | Count | SQL Operation |
|------|----------|-------|---------------|
| api-post | /api/conversations/mark-read | 1 | UPDATE conversations SET read_by = array_append(...) |
| api-post | /api/tick-log/batch | 1 | INSERT INTO tick_log (batch) |

### Other files -- 11 HTTP calls
| File | Type | Endpoint | Count |
|------|------|----------|-------|
| tick-reporting.lisp | api-post | /api/tick-reports | 1 |
| drive.lisp | api-post | /api/drives/tick | 1 |
| drive.lisp | api-post | /api/drives/:id/fulfill | 1 |
| drive.lisp | api-get | /api/agents/:id/drives | 1 |
| cognition-broker.lisp | api-post | /api/cognition/telemetry | 1 |
| cognition-broker.lisp | api-post | /api/cognition/jobs | 1 |
| tool-socket.lisp | api-get | /api/agents/:id | 1 |
| tool-socket.lisp | api-get | /api/af64/tasks | 2 |
| task-scheduler.lisp | api-patch | /api/af64/tasks/:id | 1 |
| user-profile.lisp | api-get | /api/agents/:id | 1 |
| empirical-rollups.lisp | api-post | /api/rollups/* | ~5 |

**Total: ~63 HTTP call sites across 10 files**

## SQL Patterns from dpn-api Handlers

### Conversation INSERT (from af64_conversations.rs)
```sql
INSERT INTO conversations (from_agent, to_agent, message, channel, message_type, thread_id, metadata)
VALUES ($1, $2, $3, $4, 'chat', $5, $6::jsonb)
RETURNING id
```
- `to_agent` is `varchar(50)[]` -- use ARRAY constructor
- `thread_id` is `uuid` -- either provide a UUID string or let `gen_random_uuid()` default
- `metadata` is `jsonb` -- escape JSON string and cast

### Mark Read (from af64_conversations.rs)
```sql
UPDATE conversations
SET read_by = array_append(read_by, $1)
WHERE id = ANY($2) AND NOT ($1 = ANY(read_by))
```
- `$1` is agent_id (varchar), `$2` is array of message IDs (integer[])
- In Lisp: iterate message IDs and update each, or build `WHERE id IN (...)` clause

### Task UPDATE (from af64_tasks.rs)
```sql
-- The Rust handler does one UPDATE per field. In Lisp, combine into a single UPDATE:
UPDATE tasks SET
  status = 'done',
  completed_date = '2026-03-29',
  stage_notes = '{"issues":[]}'::jsonb
WHERE id = 42
```

### Task CREATE (from af64_tasks.rs)
```sql
INSERT INTO tasks (task_id, text, status, assignee, department, doc_path, line_number, raw_line,
                   due_date, project_id, parent_id, source, blocked_by)
VALUES ($1, $2, $3, $4, $5, $6, 0, $2, $7, $8, $9, $10, $11)
RETURNING id, task_id
```
- `doc_path` defaults to 'af64/generated', `source` defaults to 'ghost'
- `task_id` defaults to format `ghost-{uuid}`
- `line_number` always 0, `raw_line` = text

### Drives Tick (from af64_drives.rs)
```sql
UPDATE agent_drives d
SET satisfaction = GREATEST(0, d.satisfaction - d.decay_rate),
    pressure = LEAST(100, 100 - GREATEST(0, d.satisfaction - d.decay_rate)),
    frustration = CASE
        WHEN (100 - GREATEST(0, d.satisfaction - d.decay_rate)) > 70
             AND COALESCE((SELECT s.energy FROM agent_state s WHERE s.agent_id = d.agent_id), 50) < 20
        THEN d.frustration + 1
        ELSE d.frustration
    END
```

### Drives Fulfill (from af64_drives.rs)
```sql
UPDATE agent_drives
SET satisfaction = LEAST(100, satisfaction + $1),
    pressure = GREATEST(0, 100 - LEAST(100, satisfaction + $1)),
    frustration = 0
WHERE agent_id = $2 AND drive_name = $3
```

### Drives GET (from af64_agents.rs)
```sql
SELECT id, agent_id, drive_name, description, satisfaction, pressure, frustration, decay_rate
FROM agent_drives WHERE agent_id = $1 ORDER BY pressure DESC
```

### Decision INSERT (from decisions.rs)
```sql
INSERT INTO decisions (decision, rationale, project_id, department, owner, stakeholders, date)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING id, created_at
```

### Tick Log Batch INSERT (from af64_tick_log.rs)
```sql
INSERT INTO tick_log (tick_number, agent_id, action_taken, action_detail, energy_before, energy_after, tier, model_used, llm_called)
VALUES ($1, $2, $3, $4::jsonb, $5, $6, $7, $8, $9)
```

### Tick Report INSERT (table schema verified, no API handler exists)
```sql
INSERT INTO tick_reports (tick_number, total_agents, active_agents, idle_agents, dormant_agents,
                          llm_calls, budget_used, budget_max, top_actors, energy_snapshot,
                          drive_snapshot, notable_events)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::jsonb, $10::jsonb, $11::jsonb, $12::jsonb)
```

### Agent Daily Memory UPSERT (from af64_memory.rs)
```sql
INSERT INTO agent_daily_memory (agent_id, log_date, actions_taken, decisions_made, knowledge_gained, blockers, handoffs, plan_tomorrow)
VALUES ($1, $2::date, $3, $4, $5, $6, $7, $8)
ON CONFLICT (agent_id, log_date) DO UPDATE SET
  actions_taken = CASE WHEN EXCLUDED.actions_taken IS NOT NULL THEN
    COALESCE(agent_daily_memory.actions_taken || E'\n', '') || EXCLUDED.actions_taken
    ELSE agent_daily_memory.actions_taken END,
  -- ... (same pattern for other fields)
  updated_at = NOW()
```

### Agent Request UPDATE (from agent_requests.rs)
```sql
UPDATE agent_requests SET status = $1, response = $2, resolved_at = NOW()
WHERE id = $3
```

## Database Schema Summary

### conversations (target of DB-03)
| Column | Type | Notes |
|--------|------|-------|
| id | integer | SERIAL, PK |
| thread_id | uuid | DEFAULT gen_random_uuid() |
| from_agent | varchar(50) | NOT NULL |
| to_agent | varchar(50)[] | Array of agent IDs |
| channel | varchar(50) | |
| message | text | NOT NULL |
| message_type | varchar(20) | DEFAULT 'chat' |
| metadata | jsonb | DEFAULT '{}' |
| read_by | varchar(50)[] | DEFAULT '{}' -- agent IDs who read it |
| created_at | timestamptz | DEFAULT now() |

**Triggers on conversations:** `conversation_new_trigger`, `conversation_notify`, `trg_feedback_message` -- these fire on INSERT, so direct SQL will trigger them just like HTTP did.

### tasks (target of DB-04)
| Column | Type | Notes |
|--------|------|-------|
| id | integer | SERIAL, PK |
| task_id | varchar(256) | NOT NULL, unique-ish |
| text | text | NOT NULL |
| status | varchar(32) | NOT NULL |
| assignee | varchar(256) | |
| assigned_to | text[] | Array |
| department | varchar(64) | |
| project_id | integer | FK to projects |
| goal_id | integer | |
| blocked_by | integer[] | Array of task IDs |
| stage_notes | jsonb | |
| stage | varchar(32) | DEFAULT 'open' |
| source | varchar(32) | DEFAULT 'obsidian' |
| doc_path | varchar(1024) | NOT NULL |
| line_number | integer | NOT NULL |
| raw_line | text | NOT NULL |
| scheduled_at | timestamptz | |
| deadline | timestamptz | |
| recurrence | varchar(256) | |

## Discretion Recommendations

### File Organization: Separate domain files
**Recommendation:** Create 3 new files rather than extending db-client.lisp (already 772 lines with perception code).
- `db-conversations.lisp` -- db-insert-conversation, db-mark-read-batch, db-get-conversations
- `db-tasks.lisp` -- db-update-task, db-create-task, db-get-tasks-by-filter, db-complete-and-unblock
- `db-auxiliary.lisp` -- everything else (drives, tick-log, tick-reports, decisions, documents, memory, agent reads, requests)

### Transaction Boundaries
**Recommendation:** Use transactions for:
1. Task completion + unblock (D-07) -- atomicity prevents orphaned blocked tasks
2. Drives tick (affects all agents atomically) -- already a single UPDATE, no transaction needed

Do NOT use transactions for:
- Individual conversation inserts (independent, fire-and-forget)
- Individual task patches (single-row updates)

### Error Handling
**Recommendation:** Follow Phase 21's pattern: `handler-case` per SQL call with error logging and continuation. The tick engine must never crash due to a single SQL failure. Pattern:
```lisp
(handler-case (db-insert-conversation ...)
  (error (e)
    (format t "  [action-executor] conversation insert failed: ~a~%" e)))
```

### Projects Reads: Yes, Convert
**Recommendation:** The 3 `api-get "/api/projects/:id"` calls in action-executor.lisp should be converted. They fetch project status/name for task completion reporting. Simple SELECT:
```sql
SELECT id, name, status, owner, description FROM projects WHERE id = $1
```

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual smoke test (no automated Lisp test suite) |
| Quick run command | Start ghosts, observe 1 tick cycle |
| Full suite command | `pm2 start noosphere-ghosts && sleep 120 && pm2 logs noosphere-ghosts --lines 200` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DB-03 | Conversations read/write/mark-read via SQL | smoke | Trigger ghost tick, verify conversation rows in DB | N/A |
| DB-04 | Task create/update/complete/unblock via SQL | smoke | Trigger ghost tick with active tasks, verify task mutations | N/A |

### Sampling Rate
- **Per task commit:** Compile check: `cd /opt/project-noosphere-ghosts && sbcl --load launch.lisp --eval '(quit)'` (syntax validation)
- **Per wave merge:** Start ghosts, let 1-2 ticks run, verify no errors in PM2 logs
- **Phase gate:** Full tick cycle with at least one cognition job executing, verifying conversation insert + task mutation + tick-log

### Wave 0 Gaps
- [ ] No automated test framework for Lisp tick engine -- testing is manual smoke testing
- [ ] Verification queries: prepare SQL queries to validate data after tick (e.g., `SELECT * FROM conversations ORDER BY id DESC LIMIT 5`)

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| HTTP via curl to dpn-api | Direct SQL via libpq FFI | Phase 21 (2026-03-29) | Eliminated HTTP latency for perception + state updates |
| All operations via api-client.lisp | Mixed: perception SQL + operations HTTP | Phase 21 | Partial migration; this phase completes it |

## Open Questions

1. **Documents POST target table**
   - What we know: The dpn-api handler routes POST /api/documents to `memories` table (not `documents`). Action-executor has 2 `api-post "/api/documents"` calls.
   - What's unclear: What content are these POSTs creating? Daily notes? Operational docs? The target table depends on the content type.
   - Recommendation: Read the specific call sites in action-executor (lines 210-217) to determine whether to INSERT into `memories` or `documents`.

2. **Cognition jobs table existence**
   - What we know: cognition-broker.lisp POSTs to `/api/cognition/jobs` but no matching dpn-api route exists. The broker stores jobs in-memory (struct cognition-broker).
   - What's unclear: Is there a `cognition_jobs` table? Or is the API POST purely telemetry?
   - Recommendation: Check if the table exists; if not, treat the POST as telemetry-only and write to the local telemetry file (already the fallback).

3. **Rollups API endpoints**
   - What we know: empirical-rollups.lisp POSTs to `/api/rollups/{daily,weekly,monthly,quarterly,yearly}` but no matching dpn-api routes exist.
   - What's unclear: Whether rollup data should go to a database table or remain file-only.
   - Recommendation: Keep rollups file-based (they already persist to local JSONL files). Remove the silently-failing API POST calls.

## Sources

### Primary (HIGH confidence)
- `/opt/dpn-api/src/handlers/af64_conversations.rs` -- exact SQL for conversation CRUD
- `/opt/dpn-api/src/handlers/af64_tasks.rs` -- exact SQL for task CRUD
- `/opt/dpn-api/src/handlers/af64_drives.rs` -- exact SQL for drives tick/fulfill
- `/opt/dpn-api/src/handlers/af64_tick_log.rs` -- exact SQL for tick log batch insert
- `/opt/dpn-api/src/handlers/decisions.rs` -- exact SQL for decision INSERT
- `/opt/dpn-api/src/handlers/af64_memory.rs` -- exact SQL for memory upsert
- `/opt/dpn-api/src/handlers/agent_requests.rs` -- exact SQL for request update
- `/opt/dpn-api/src/handlers/documents.rs` -- documents POST routes to memories table
- PostgreSQL `\d` schema output for: conversations, tasks, decisions, documents, tick_reports, tick_log, agent_drives, agent_daily_memory, agent_requests, projects, memory_entries
- `/opt/project-noosphere-ghosts/lisp/util/pg.lisp` -- FFI bindings, pool, pg-query vs pg-execute behavior
- `/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp` -- Phase 21 wrapper functions

### Secondary (MEDIUM confidence)
- HTTP call counts from grep across all Lisp source files (verified by line-by-line review)
- CONTEXT.md decisions D-01 through D-17

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- reusing Phase 21 infrastructure, no new dependencies
- Architecture: HIGH -- all SQL patterns verified against dpn-api Rust handlers and live DB schema
- Pitfalls: HIGH -- INSERT RETURNING behavior verified against pg.lisp source; array types verified against DB schema

**Research date:** 2026-03-29
**Valid until:** 2026-04-28 (30 days -- stable infrastructure, no upstream changes expected)
