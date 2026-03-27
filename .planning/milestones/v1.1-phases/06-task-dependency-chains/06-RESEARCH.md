# Phase 6: Task Dependency Chains - Research

**Researched:** 2026-03-26
**Domain:** PostgreSQL schema migration, Rust API endpoints, Common Lisp parser extension, Python dispatch tooling
**Confidence:** HIGH

## Summary

Phase 6 wires the existing `blocked_by INTEGER` column into a functional dependency system. The column must be migrated to `INTEGER[]` to support multi-dependency chains, then perception filtering must exclude blocked tasks at the SQL level, the DB trigger must auto-unblock downstream tasks on completion, CREATE_TASK must accept `blocked_by=` syntax, and dispatch must populate dependencies from wave ordering.

All integration points have been verified against live source code and the running database. The current `blocked_by` column is `INTEGER`, has no data (0 rows with non-null values), and has no index. The `on_task_completed_after()` trigger is confirmed live and matches the Phase 5 migration SQL exactly. The perception endpoint has 4 SQL query branches (triage, exec, staff, toolless) that all need the blocked_by filter. The CREATE_TASK parser uses a straightforward `search "key=" after` pattern that can be extended for `blocked_by=`.

**Primary recommendation:** Use PostgreSQL `INTEGER[]` with `array_remove()` for dependency tracking, filter at SQL level in all perception queries, and extend the existing trigger with array element removal logic.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Migrate `blocked_by` from `INTEGER` to `INTEGER[]` (Postgres array). A task is unblocked when ALL referenced task IDs in the array are complete.
- **D-02:** Migration must preserve existing data -- convert single INTEGER values to single-element arrays. No data loss.
- **D-03:** Filter blocked tasks at the SQL level in dpn-api. Tasks where `blocked_by` contains any incomplete task ID are excluded from the perception response. Ghosts only see actionable work.
- **D-04:** Executives see blocked tasks in their project review context as a separate informational section (not in their actionable task list). This gives executives visibility into the full dependency graph when reviewing a project.
- **D-05:** Extend the existing `on_task_completed_after()` DB trigger. When task N completes, find all tasks where `blocked_by` array contains N, remove N from the array. When the array becomes empty, the task is fully unblocked.
- **D-06:** Same pattern as wave advancement (Phase 5) -- all completion side-effects live in DB triggers for consistency.
- **D-07:** Extend CREATE_TASK with inline `blocked_by` parameter: `CREATE_TASK: description assignee=agent-id blocked_by=#123,#456`. Follows existing `key=value` parsing pattern.
- **D-08:** Parser extracts comma-separated task IDs from `blocked_by=` and passes them as INTEGER[] to the API.
- **D-09:** `dispatch_to_db.py` must set `blocked_by` for wave 2+ subtasks based on wave ordering. Wave 2 tasks are blocked by wave 1 completion. Uses the existing `depends_on` from context JSON to populate the actual `blocked_by` column.

### Claude's Discretion
- Whether to change task status when fully unblocked (e.g., set to 'open') or leave status unchanged and rely on perception filtering (empty blocked_by = perceivable). Decide based on how wave advancement trigger and perception filtering interact.
- How to handle non-existent task IDs in blocked_by references from CREATE_TASK -- pick the most robust approach based on existing error patterns in action-executor.lisp.

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DEP-01 | Perception endpoint filters out tasks where blocked_by references an incomplete task | Verified: 4 SQL query branches in af64_perception.rs (triage line 100, exec line 110, staff line 122, toolless skipped). Each needs a WHERE clause addition. Use `NOT EXISTS (SELECT 1 FROM unnest(blocked_by) bid WHERE NOT EXISTS (SELECT 1 FROM tasks WHERE id = bid AND status IN ('done','completed')))` or simpler `(blocked_by IS NULL OR blocked_by = '{}')` approach. |
| DEP-02 | When a task completes, all tasks with blocked_by pointing to it are automatically unblocked | Verified: `on_task_completed_after()` trigger is live on `task_completed_trigger` (AFTER UPDATE). Extend with `UPDATE tasks SET blocked_by = array_remove(blocked_by, NEW.id) WHERE NEW.id = ANY(blocked_by)`. |
| DEP-03 | Executives can set blocked_by when creating tasks via CREATE_TASK | Verified: `parse-create-task-lines` at line 575 of action-executor.lisp uses `search "assignee=" after` pattern. Add same pattern for `blocked_by=`. API POST `/api/af64/tasks` needs `blocked_by` field in `NewTask` struct. |
| DEP-04 | dispatch_to_db.py sets blocked_by for subtasks based on wave ordering | Verified: dispatch_to_db.py line 219 reads wave from frontmatter, line 237 builds context_json. Currently does NOT set blocked_by column. Must add: wave 2+ parent tasks blocked by all wave (N-1) parent task IDs. |
</phase_requirements>

## Standard Stack

No new libraries required. This phase extends the existing stack in place:

### Core (existing, no changes)
| Component | Version | Purpose | Status |
|-----------|---------|---------|--------|
| PostgreSQL | 15+ | `INTEGER[]` type, `array_remove()`, `ANY()` operator | Verified available |
| sqlx | 0.8 | Rust PostgreSQL driver, supports `Vec<i32>` as `INTEGER[]` | In dpn-api Cargo.toml |
| axum | 0.7 | HTTP framework for API endpoints | In dpn-api |
| SBCL | -- | Common Lisp runtime for action-executor | Running |
| psycopg2 | -- | Python PostgreSQL driver, auto-adapts lists to arrays | In gotcha-workspace |

### PostgreSQL Array Functions (verified)
| Function | Purpose | Example |
|----------|---------|---------|
| `array_remove(arr, elem)` | Remove element from array | `array_remove(ARRAY[1,2,3], 2)` returns `{1,3}` |
| `ANY(arr)` | Check if value is in array | `5 = ANY(blocked_by)` |
| `@>` | Array contains | `blocked_by @> ARRAY[5]` |
| `unnest(arr)` | Expand array to rows | For complex filtering |
| `ARRAY[]::INTEGER[]` | Empty array literal | Default for unblocked tasks |

**Confidence:** HIGH -- all PostgreSQL array operations are stable, well-documented features available since PG 9.1+.

## Architecture Patterns

### Recommended Approach: SQL-Level Filtering + Trigger-Based Unblocking

The dependency system follows the same architectural pattern established in Phase 5: DB triggers handle state transitions, SQL queries handle filtering, and the API is the gateway.

```
Task completion flow:
1. Ghost sends COMPLETE: #123 → action-executor.lisp
2. api-patch /api/af64/tasks/123 → status = 'done'
3. on_task_completed_after() trigger fires:
   a. [existing] vault_notes update
   b. [existing] pg_notify task_completed
   c. [existing] wave advancement
   d. [NEW] array_remove(blocked_by, 123) for all dependents
   e. [existing] project completion check
4. Next tick: perception query excludes blocked tasks
5. Unblocked tasks appear in ghost perception
```

### Pattern 1: Perception Filtering via SQL Subquery
**What:** Add WHERE clause to exclude tasks with unresolved dependencies
**When to use:** All 3 active perception query branches (triage, exec, staff)

The simplest correct approach:
```sql
-- Add to WHERE clause of each perception query:
AND (blocked_by IS NULL OR blocked_by = '{}' OR NOT EXISTS (
  SELECT 1 FROM unnest(blocked_by) AS dep_id
  WHERE dep_id NOT IN (SELECT id FROM tasks WHERE id = dep_id AND status IN ('done', 'completed'))
))
```

Simpler alternative (recommended for performance with 871 active tasks):
```sql
AND (blocked_by IS NULL OR blocked_by = '{}'
     OR NOT EXISTS (
       SELECT 1 FROM unnest(blocked_by) AS dep_id
       JOIN tasks t ON t.id = dep_id
       WHERE t.status NOT IN ('done', 'completed')
     ))
```

This reads as: "show this task if it has no blocked_by, or if ALL tasks in its blocked_by array are complete."

### Pattern 2: Executive Blocked Tasks Visibility (D-04)
**What:** Separate query for blocked tasks in executive project review context
**When to use:** Only for executive perception when projects are present

```sql
-- Additional query in exec perception, keyed by project_id
SELECT id, text, status, blocked_by, assignee
FROM tasks
WHERE project_id = $1
  AND blocked_by IS NOT NULL AND blocked_by != '{}'
  AND EXISTS (
    SELECT 1 FROM unnest(blocked_by) AS dep_id
    JOIN tasks t ON t.id = dep_id
    WHERE t.status NOT IN ('done', 'completed')
  )
```

### Pattern 3: Trigger Extension for Auto-Unblock
**What:** Remove completed task ID from all blocked_by arrays
**When to use:** Inside `on_task_completed_after()` after the existing logic

```sql
-- Add before project completion check:
UPDATE tasks
SET blocked_by = array_remove(blocked_by, NEW.id),
    updated_at = NOW()
WHERE NEW.id = ANY(blocked_by);
```

### Pattern 4: CREATE_TASK Parser Extension (Lisp)
**What:** Parse `blocked_by=#123,#456` from CREATE_TASK lines
**When to use:** In `parse-create-task-lines` function

The existing parser extracts key=value pairs by searching for `"assignee="`. The same pattern works for `blocked_by=`:
```lisp
;; After extracting assignee, also extract blocked_by
(blocked-by-pos (search "blocked_by=" after))
(blocked-by-str (when blocked-by-pos
                  (let* ((start (+ blocked-by-pos 11))
                         (end (or (position #\Space after :start start) (length after))))
                    (subseq after start end))))
;; Parse "#123,#456" into list of integers
(blocked-by-ids (when blocked-by-str
                  (mapcar (lambda (s) (parse-integer (string-trim '(#\#) s) :junk-allowed t))
                          (uiop:split-string blocked-by-str :separator '(#\,)))))
```

### Anti-Patterns to Avoid
- **Application-level dependency checking:** Do NOT check dependencies in Lisp/Rust code. SQL filtering is authoritative and atomic.
- **Status changes on unblock:** Do NOT automatically set status to 'open' when blocked_by becomes empty. The wave advancement trigger already handles status transitions. Changing status in the dependency trigger could conflict with wave-based status management. Instead, rely on perception filtering: empty `blocked_by` = perceivable.
- **Foreign key on blocked_by elements:** Do NOT add FK constraints on array elements. PostgreSQL does not support FK on array elements natively, and invalid IDs will simply never match a completed task, making the dependency permanent (safe failure mode). The ghost will eventually notice via escalation.

### Claude's Discretion Recommendations

**Status change on unblock:** Do NOT change status. Rationale: The wave advancement trigger already sets status to 'open' for wave N+1 tasks when wave N completes. Adding status changes in the dependency unblock would create a race condition where both triggers try to set status. Instead, perception filtering handles visibility: `blocked_by IS NULL OR blocked_by = '{}'` means perceivable regardless of status value. This is simpler and avoids trigger ordering issues.

**Non-existent task IDs in blocked_by:** Fail silently (accept the ID). Rationale: The existing `action-executor.lisp` pattern wraps all API calls in `handler-case` and logs errors but continues (see lines 628-629, 635-636). The `api-post` for CREATE_TASK at line 707 follows this pattern. If a ghost references `blocked_by=#99999` (non-existent), the task will remain permanently blocked until an executive notices and removes the dependency. This is safer than silently dropping the dependency (which could cause premature unblocking). Log a warning in the API response.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Array element removal | Custom PL/pgSQL loop | `array_remove()` | Built-in, atomic, handles edge cases (empty arrays, missing elements) |
| Dependency resolution | Recursive CTE graph walker | Simple WHERE clause | Tasks only need to know "am I blocked?" not "what's the full graph" |
| Array type in Rust | Custom serialization | sqlx `Vec<i32>` binding | sqlx natively maps `Vec<i32>` to PostgreSQL `INTEGER[]` |
| Python list-to-array | Manual SQL array syntax | psycopg2 list adaptation | `psycopg2` automatically converts Python lists to PostgreSQL arrays |

## Common Pitfalls

### Pitfall 1: Trigger Ordering with Wave Advancement
**What goes wrong:** The dependency unblock and wave advancement both fire in `on_task_completed_after()`. If dependency unblock runs AFTER wave advancement, wave N+1 tasks might still have blocked_by entries from wave N tasks that haven't been removed yet.
**Why it happens:** Both are in the same trigger function, but order matters.
**How to avoid:** Place the dependency unblock (`array_remove`) BEFORE the wave advancement logic in the trigger. This ensures blocked_by is cleaned up before wave status checks run.
**Warning signs:** Wave N+1 tasks get status='open' but still have non-empty blocked_by arrays.

### Pitfall 2: Empty Array vs NULL
**What goes wrong:** `blocked_by IS NULL` and `blocked_by = '{}'` are different conditions. A task migrated from INTEGER NULL becomes `NULL`, while a task that had all dependencies removed becomes `'{}'`.
**Why it happens:** `array_remove()` on a single-element array returns `'{}'` (empty array), not NULL.
**How to avoid:** Always check BOTH conditions in WHERE clauses: `(blocked_by IS NULL OR blocked_by = '{}')`. Or normalize: after `array_remove`, if the array is empty, SET to NULL. Recommendation: check both -- it's defensive and costs nothing.
**Warning signs:** Tasks that should be unblocked not appearing in perception.

### Pitfall 3: Lisp JSON Underscore-to-Hyphen Conversion
**What goes wrong:** The Lisp JSON parser converts `blocked_by` to `:BLOCKED-BY` in hash tables. If API responses include `blocked_by`, Lisp code must access it as `:BLOCKED-BY`.
**Why it happens:** Documented quirk in CLAUDE.md: "parser converts underscores to hyphens".
**How to avoid:** In Lisp code, always use `:BLOCKED-BY` when reading JSON fields. In API payloads sent FROM Lisp, use the `json-object` helper which handles serialization.
**Warning signs:** NIL values when trying to read blocked_by from API responses in Lisp.

### Pitfall 4: Dispatch Wave Dependency Mapping
**What goes wrong:** `dispatch_to_db.py` creates tasks in a loop. Wave 2 tasks need to reference wave 1 task IDs. But task IDs (integer PKs) are only known after INSERT.
**Why it happens:** The dispatch creates parent tasks sequentially per plan file. Wave 2 plan files need the integer IDs of wave 1 tasks.
**How to avoid:** Two-pass approach: first pass creates all tasks (collecting IDs), second pass sets `blocked_by` for wave 2+ tasks. OR: use `RETURNING id` from wave 1 inserts and collect IDs before processing wave 2.
**Warning signs:** Wave 2 tasks created with empty blocked_by.

### Pitfall 5: sqlx Type Mismatch After Migration
**What goes wrong:** After migrating `blocked_by` from `INTEGER` to `INTEGER[]`, all Rust code that reads/writes this column must use `Vec<i32>` instead of `i32`. The `TaskUpdate` struct currently has `pub blocked_by: Option<i32>`.
**Why it happens:** The Rust type must match the PostgreSQL type exactly for sqlx.
**How to avoid:** Update `TaskUpdate.blocked_by` to `Option<Vec<i32>>` and update all SQL bind calls.
**Warning signs:** sqlx runtime errors about type mismatch on the blocked_by column.

## Code Examples

### Migration SQL (verified against live schema)
```sql
-- Step 1: Migrate blocked_by from INTEGER to INTEGER[]
-- Current state: blocked_by INTEGER, 0 rows with non-null values
ALTER TABLE tasks
  ALTER COLUMN blocked_by TYPE INTEGER[]
  USING CASE WHEN blocked_by IS NOT NULL THEN ARRAY[blocked_by] ELSE NULL END;

-- Step 2: Add GIN index for array containment queries
CREATE INDEX idx_tasks_blocked_by ON tasks USING GIN (blocked_by);
```

### Trigger Extension (appends to existing on_task_completed_after)
```sql
-- Add BEFORE wave advancement section in on_task_completed_after():
-- Dependency unblock: remove completed task from all blocked_by arrays
UPDATE tasks
SET blocked_by = array_remove(blocked_by, NEW.id),
    updated_at = NOW()
WHERE NEW.id = ANY(blocked_by);
```

### Rust TaskUpdate Struct Change
```rust
// In af64_tasks.rs - change from:
pub blocked_by: Option<i32>,
// To:
pub blocked_by: Option<Vec<i32>>,
```

### Rust NewTask Struct Addition
```rust
// In af64_tasks.rs NewTask - add:
pub blocked_by: Option<Vec<i32>>,
```

### Perception SQL Addition (staff example)
```sql
-- Current (line 122 of af64_perception.rs):
WHERE $1 = ANY(assigned_to) AND status IN ('open', 'pending', 'in-progress')
-- Becomes:
WHERE $1 = ANY(assigned_to) AND status IN ('open', 'pending', 'in-progress')
  AND (blocked_by IS NULL OR blocked_by = '{}' OR NOT EXISTS (
    SELECT 1 FROM unnest(blocked_by) AS dep_id
    JOIN tasks t ON t.id = dep_id
    WHERE t.status NOT IN ('done', 'completed')
  ))
```

### Lisp Parser Extension (parse-create-task-lines)
```lisp
;; Extract blocked_by=#123,#456 from CREATE_TASK line
;; Returns list of (description assignee-or-nil blocked-by-ids-or-nil)
(let* ((blocked-by-pos (search "blocked_by=" after))
       ;; Adjust description end to exclude blocked_by= segment
       (desc-end (or assignee-pos blocked-by-pos (length after)))
       (description (string-trim '(#\Space #\Tab)
                     (subseq after 0 desc-end)))
       (blocked-by-str (when blocked-by-pos
                          (let* ((start (+ blocked-by-pos 11))
                                 (end (or (position #\Space after :start start)
                                          (length after))))
                            (subseq after start end)))))
  ;; Parse "#123,#456" into (123 456)
  (push (list description assignee
               (when blocked-by-str
                 (remove nil
                   (mapcar (lambda (s)
                             (parse-integer (string-trim '(#\# #\Space) s)
                                            :junk-allowed t))
                           (uiop:split-string blocked-by-str
                                              :separator '(#\,))))))
        results))
```

### Dispatch blocked_by Population (Python)
```python
# Two-pass approach in dispatch_phase():
# Pass 1: Create all tasks, collect wave -> [task_ids] mapping
wave_task_ids = {}  # {wave_num: [db_integer_id, ...]}

# After INSERT RETURNING id:
wave_task_ids.setdefault(wave, []).append(parent_int_id)

# Pass 2: Set blocked_by for wave 2+ tasks
for wave_num, task_ids in wave_task_ids.items():
    if wave_num <= 1:
        continue
    prev_wave_ids = wave_task_ids.get(wave_num - 1, [])
    if prev_wave_ids:
        for task_id in task_ids:
            cur.execute(
                "UPDATE tasks SET blocked_by = %s WHERE id = %s",
                (prev_wave_ids, task_id)
            )
```

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | bash + psql (integration tests against live DB) |
| Config file | none -- see Wave 0 |
| Quick run command | `bash test_dep_chains.sh` |
| Full suite command | `bash test_dep_chains.sh --full` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DEP-01 | Blocked tasks excluded from perception | integration | `curl localhost:8080/api/perception/test-agent \| jq '.tasks'` | No -- Wave 0 |
| DEP-02 | Completing blocker unblocks dependents | integration | `psql -c "UPDATE tasks SET status='done' WHERE id=X" && psql -c "SELECT blocked_by FROM tasks WHERE id=Y"` | No -- Wave 0 |
| DEP-03 | CREATE_TASK with blocked_by persists | integration | `curl -X POST localhost:8080/api/af64/tasks -d '{"text":"test","blocked_by":[1]}'` | No -- Wave 0 |
| DEP-04 | Dispatch sets blocked_by for wave 2+ | integration | `python dispatch_to_db.py --phase N && psql -c "SELECT blocked_by FROM tasks WHERE ..."` | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** Quick manual verification via psql
- **Per wave merge:** Full test script
- **Phase gate:** All 4 DEP requirements verified via test script

### Wave 0 Gaps
- [ ] `test_dep_chains.sh` -- end-to-end test script covering DEP-01 through DEP-04
- [ ] Test data setup: create test tasks with known dependency chains
- [ ] Cleanup: remove test data after verification

## Open Questions

1. **Cascade reporter update scope**
   - What we know: `gotcha-workspace/tools/cascade_reporter/cascade_reporter.py` references `blocked_by` and will need updating for array type.
   - What's unclear: Whether it needs to be updated in this phase or can be deferred.
   - Recommendation: Include a task to update cascade_reporter since it queries blocked_by directly. Low effort, prevents breakage.

2. **Kanban tool blocked_by references**
   - What we know: `gotcha-workspace/tools/kanban/` files reference blocked_by.
   - What's unclear: Whether the kanban tool is actively used.
   - Recommendation: Update if touched, but do not block phase on it. Flag for follow-up.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| PostgreSQL | Schema migration, triggers | Yes | 15+ (confirmed live) | -- |
| dpn-api (Rust) | Perception filtering, task API | Yes | Running on port 8080 | -- |
| SBCL | Action executor changes | Yes | Running via PM2 | -- |
| psycopg2 | Dispatch script | Yes | In gotcha-workspace venv | -- |
| cargo | Rust compilation | Yes | Available | -- |

**Missing dependencies:** None. All required tools are available.

## Sources

### Primary (HIGH confidence)
- Live database schema: `\d tasks` on master_chronicle (verified 2026-03-26)
- Live trigger function: `pg_proc WHERE proname = 'on_task_completed_after'` (verified 2026-03-26)
- `/opt/dpn-api/src/handlers/af64_perception.rs` -- full source read
- `/opt/dpn-api/src/handlers/af64_tasks.rs` -- full source read
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` lines 550-750
- `/root/gotcha-workspace/tools/gsd/dispatch_to_db.py` -- full source read
- `/root/.planning/phases/05-feedback-reporting/migrations/001_wave_advancement_trigger.sql`

### Secondary (MEDIUM confidence)
- PostgreSQL array functions documentation (training data, stable features since PG 9.1)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- no new dependencies, all verified against live system
- Architecture: HIGH -- all integration points inspected, SQL queries verified, trigger confirmed live
- Pitfalls: HIGH -- derived from direct code inspection and schema analysis

**Research date:** 2026-03-26
**Valid until:** 2026-04-26 (stable infrastructure, no external dependencies)

## Project Constraints (from CLAUDE.md)

- **Stack**: Rust (dpn-api), Common Lisp/SBCL (ghosts), Python (dispatch tools), PostgreSQL -- no new languages
- **DB is the OS**: All state in master_chronicle. No file-based state.
- **UTF-8 Rule**: Never mix character positions with byte indices in Rust code
- **Ghost LLM**: Claude Code CLI (`claude -p`) with `--output-format json`
- **Lisp JSON quirk**: Parser converts underscores to hyphens (`:BLOCKED-BY` not `:blocked_by`)
- **Workspace portability**: Python tools must use `tools/_config.py` for paths
- **New tools**: Must be added to `tools/manifest.md`
