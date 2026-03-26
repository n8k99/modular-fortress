# Phase 2: Perception Pipeline - Research

**Researched:** 2026-03-26
**Domain:** Rust/Axum API enhancement, PostgreSQL query modification, Lisp runtime integration
**Confidence:** HIGH

## Summary

Phase 2 enhances the existing perception endpoint (`/api/perception/:agent_id`) in `/opt/dpn-api/src/handlers/af64_perception.rs` (401 lines). The endpoint already works and returns tasks, projects, messages, relationships, memories, team activity, and proactive eligibility. The work is surgical: (1) add GSD fields to task SELECT/serialization, (2) migrate WHERE clauses from `assignee` (varchar) to `assigned_to` (text[]), (3) include `scheduled_at` in the response without filtering (Lisp tick engine already handles filtering client-side), and (4) verify the urgency boost fires end-to-end.

The Lisp consumer (`perception.lisp`, `tick-engine.lisp`, `action-planner.lisp`) already reads `:tasks` and `:projects` from the perception response. The JSON parser converts underscores to hyphens automatically (`project_id` becomes `:project-id`). The tick engine already has `filter-scheduled-tasks` which reads `:scheduled-at` from each task hash and the project boost `(* 15 (length projects))` is already coded. The action-planner reads `:assignee` from task hashes for delegation logic -- this will need to also read `:assigned-to` for the new GSD tasks.

**Primary recommendation:** Make minimal, targeted changes to the three SQL queries (triage/exec/staff) and the serialization block in `af64_perception.rs`. Add `assigned_to` as an additional field alongside `assignee` (don't remove `assignee` -- legacy tasks still use it). Verify with a curl-based end-to-end test against the live API.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** `/api/perception/:agent_id` is fully implemented in `af64_perception.rs` (401 lines). Route registered in main.rs.
- **D-02:** This is an ENHANCEMENT phase, not a build-from-scratch. Modify existing queries, don't rewrite the handler.
- **D-03:** Add ALL GSD fields to task query responses: `project_id`, `source`, `context`, `parent_id`, `priority`, `assigned_to`, `scheduled_at`.
- **D-04:** The task serialization block (lines 123-134) needs expansion to include these additional columns in both the SELECT and the JSON output.
- **D-05:** Migrate ALL perception task queries from `assignee` (varchar) to `assigned_to` (text[]). Use `$1 = ANY(assigned_to)` for filtering.
- **D-06:** Executive query: `WHERE $1 = ANY(assigned_to) OR (department = $2 AND assigned_to IS NULL)`. Staff query: `WHERE $1 = ANY(assigned_to)`.
- **D-07:** Existing tasks with only `assignee` set won't appear unless they also have `assigned_to`. Acceptable -- GSD tasks use `assigned_to`.
- **D-08:** NO scheduling/wave filtering in perception. Show all tasks regardless of wave.
- **D-09:** `scheduled_at` column included in response for informational purposes but NOT used as a WHERE filter.
- **D-10:** End-to-end test: dispatch a project owned by an executive, call the perception endpoint, verify the urgency score includes the +15/project boost.
- **D-11:** The tick engine code in `tick-engine.lisp` already has `(* 15 (length projects))`. Verify this code path fires when perception returns a non-empty projects array.

### Claude's Discretion
- Whether to add project `goals` and `current_context` to the perception project response (currently returns id, name, status, description, goals, blockers, current_context, open_tasks, completed_tasks -- **RESEARCH FINDING: goals and current_context are ALREADY included** in the project query at lines 320-343)
- Error handling for malformed `assigned_to` arrays
- Whether to truncate `context` JSON in perception response or return full

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PERC-01 | /api/perception/:agent_id returns dispatched project-linked tasks for the agent | Requires adding `project_id` to SELECT + serialization, and migrating WHERE from `assignee` to `assigned_to` (D-03, D-05) |
| PERC-02 | Executive agents perceive projects they own with current status and goals | **Already working** -- project query at lines 318-343 already returns owned active projects with goals, status, description, blockers, current_context, open_tasks, completed_tasks. Just needs verification. |
| PERC-03 | Staff agents perceive tasks assigned to them with project context and must_haves | Requires adding `context` (contains must_haves JSON) and `project_id` to task serialization (D-03, D-04) |
| PERC-04 | Project ownership triggers urgency boost (+15/project) in tick engine ranking | **Already coded** in tick-engine.lisp line 157-159. Needs end-to-end verification that dispatched projects appear in perception and the boost fires (D-10, D-11) |
| PERC-05 | Perception filters tasks by scheduled_at so ghosts only see ready work | **Already coded** in Lisp `filter-scheduled-tasks` (task-scheduler.lisp:47). Requires `scheduled_at` in API response (D-09). Tick engine calls filter client-side (tick-engine.lisp:120-124). **NOTE: D-08 says NO server-side filtering -- Lisp handles it.** |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| axum | 0.7.x | HTTP framework for dpn-api | Already in use, route registered |
| sqlx | 0.8.x | Async PostgreSQL driver | Already in use, all queries use sqlx |
| serde_json | (bundled) | JSON response building | Already in use via `serde_json::json!()` |
| chrono | (bundled) | Timestamp handling | Already in use for perception timestamps |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| curl | system | End-to-end API testing | Verify perception responses manually |
| psql | system | Direct DB inspection | Verify task data before/after |
| jq | system | JSON response parsing in tests | Parse curl output for assertions |

No new dependencies needed. All changes are within existing code.

## Architecture Patterns

### Existing File Structure (No Changes Needed)
```
/opt/dpn-api/src/
  handlers/
    af64_perception.rs   # PRIMARY TARGET - 401 lines
    af64_tasks.rs         # Reference for task query patterns
    af64_agents.rs        # Reference for text[] array handling
  main.rs                 # Route already registered (line 126)
  error.rs                # ApiError type

/opt/project-noosphere-ghosts/lisp/runtime/
  perception.lisp          # API caller - reads response into hash tables
  tick-engine.lisp         # Phase 1: perceive, Phase 2: rank (urgency boost)
  task-scheduler.lisp      # filter-scheduled-tasks, deadline-urgency-boost
  action-planner.lisp      # Reads :assignee from tasks for delegation logic
```

### Pattern 1: Task Query Modification
**What:** Each role-based query (triage/exec/staff) selects specific columns. All three must be updated consistently.
**When to use:** Every task query in the perception endpoint.
**Current pattern (lines 100-121):**
```rust
// Triage (line 100-103)
"SELECT id, text, status, assignee, department, stage, goal_id, stage_notes FROM tasks WHERE ..."

// Executive (line 107-108)
"SELECT id, text, status, assignee, department, stage, goal_id, stage_notes FROM tasks WHERE ..."

// Staff (line 116-118)
"SELECT id, text, status, assignee, department, stage, goal_id, stage_notes FROM tasks WHERE ..."
```
**New pattern -- add columns to all three:**
```rust
"SELECT id, text, status, assignee, assigned_to, department, stage, goal_id, stage_notes,
        project_id, source, context, parent_id, priority, scheduled_at
 FROM tasks WHERE ..."
```

### Pattern 2: PostgreSQL text[] Array Filtering
**What:** Use `$1 = ANY(assigned_to)` for array membership testing.
**Verified from:** `af64_agents.rs` already uses `Option<Vec<String>>` for text[] columns (lines 33-36). sqlx 0.8 handles text[] natively.
**Example for executive query:**
```rust
sqlx::query(
    r#"SELECT id, text, status, assignee, assigned_to, department, stage, goal_id, stage_notes,
              project_id, source, context, parent_id, priority, scheduled_at
       FROM tasks
       WHERE status IN ('open', 'pending', 'in-progress')
         AND ($1 = ANY(assigned_to) OR (department = $2 AND assigned_to IS NULL))
       ORDER BY CASE WHEN $1 = ANY(assigned_to) THEN 0 ELSE 1 END, id DESC LIMIT 10"#
)
.bind(&agent_id).bind(&agent_dept)
```

### Pattern 3: Serialization Block Extension
**What:** The task serialization (lines 123-135) maps DB rows to JSON. Must add new fields.
**Current (lines 123-135):**
```rust
let tasks: Vec<Value> = task_rows.iter().map(|r| {
    let text: String = r.get("text");
    let truncated_text: String = text.chars().take(300).collect();
    serde_json::json!({
        "id": r.get::<i32, _>("id"),
        "text": truncated_text,
        "status": r.get::<String, _>("status"),
        "assignee": r.get::<Option<String>, _>("assignee"),
        "department": r.get::<Option<String>, _>("department"),
        "stage": r.get::<Option<String>, _>("stage"),
        "goal_id": r.get::<Option<i32>, _>("goal_id"),
        "stage_notes": r.get::<Option<String>, _>("stage_notes"),
    })
}).collect();
```
**New -- add GSD fields:**
```rust
serde_json::json!({
    // ... existing fields ...
    "assigned_to": r.get::<Option<Vec<String>>, _>("assigned_to"),
    "project_id": r.get::<Option<i32>, _>("project_id"),
    "source": r.get::<Option<String>, _>("source"),
    "context": r.get::<Option<String>, _>("context"),
    "parent_id": r.get::<Option<i32>, _>("parent_id"),
    "priority": r.get::<Option<String>, _>("priority"),
    "scheduled_at": r.get::<Option<chrono::DateTime<chrono::Utc>>, _>("scheduled_at")
        .map(|dt| dt.to_rfc3339()),
})
```

### Pattern 4: Lisp JSON Key Mapping
**What:** The Lisp JSON parser converts underscores to hyphens automatically.
**Impact on new fields:**

| Rust JSON key | Lisp hash key |
|---------------|---------------|
| `project_id` | `:project-id` |
| `assigned_to` | `:assigned-to` |
| `parent_id` | `:parent-id` |
| `scheduled_at` | `:scheduled-at` |
| `stage_notes` | `:stage-notes` |
| `goal_id` | `:goal-id` |

**Already consumed by Lisp:**
- `task-scheduler.lisp` reads `:scheduled-at` (line 15) -- already working
- `tick-engine.lisp` reads `:tasks` and `:projects` from perception -- already working
- `action-planner.lisp` reads `:assignee` (lines 513, 529, 586-588) -- still needs `:assignee` in response

### Anti-Patterns to Avoid
- **Removing `assignee` from response:** The action-planner.lisp reads `:assignee` extensively for delegation and classification logic. MUST keep `assignee` in the response alongside `assigned_to`. GSD tasks have both set; legacy tasks only have `assignee`.
- **Server-side scheduled_at filtering:** D-08 explicitly forbids this. The Lisp tick engine already does client-side filtering via `filter-scheduled-tasks`. Including `scheduled_at` in the response is sufficient.
- **Rewriting the handler structure:** D-02 says enhance, not rebuild. Touch only the SQL queries and serialization block.
- **Truncating context field aggressively:** The `context` JSON contains `must_haves` which ghosts need for task verification. Truncation could break JSON parsing. Recommendation: return full context (it's typically <2KB per task).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| text[] array queries | Custom SQL string building | `$1 = ANY(assigned_to)` PostgreSQL syntax | Native array support, index-backed (GIN index exists) |
| Timestamp serialization | Manual string formatting | `chrono::DateTime<Utc>::to_rfc3339()` | ISO8601 compliance, Lisp `parse-iso8601` expects this format |
| JSON response building | Custom serializer | `serde_json::json!()` macro | Already the established pattern, type-safe |
| Scheduled task filtering | API-side WHERE clause | Lisp `filter-scheduled-tasks` | Already implemented and tested client-side |

## Common Pitfalls

### Pitfall 1: Inconsistent SELECT Columns Across Role Queries
**What goes wrong:** Adding new columns to the exec query but forgetting the triage or staff query. The serialization block tries to `.get()` a column that wasn't selected, causing a runtime panic.
**Why it happens:** Three separate SQL strings (triage line 101, exec line 107, staff line 117) that must all have identical column lists.
**How to avoid:** Update all three queries in a single pass. Use the same column list string or extract to a constant.
**Warning signs:** `column not found` panic on specific agent types only.

### Pitfall 2: NULL assigned_to Breaking ANY() Check
**What goes wrong:** `$1 = ANY(NULL)` returns NULL (not FALSE) in PostgreSQL. Tasks with `assigned_to IS NULL` need explicit handling.
**Why it happens:** The `ANY()` operator doesn't match against NULL arrays.
**How to avoid:** Executive query already handles this with `OR (department = $2 AND assigned_to IS NULL)`. Triage query should use `assigned_to IS NULL` (they see unassigned tasks). Staff query won't match NULL arrays by design.
**Warning signs:** Executives not seeing unassigned department tasks.

### Pitfall 3: Lisp Assignee Field Dependency
**What goes wrong:** Removing `assignee` from the JSON response breaks the action-planner's delegation logic, which reads `(gethash :assignee task)` at lines 513, 529, 586-588.
**Why it happens:** Migration to `assigned_to` in WHERE clauses doesn't mean removing `assignee` from the response.
**How to avoid:** Keep BOTH `assignee` and `assigned_to` in the serialized response. They serve different consumers.
**Warning signs:** Delegation and classification actions silently fail (assignee reads as NIL).

### Pitfall 4: context Field Type Mismatch
**What goes wrong:** The `context` column is `text` type containing JSON. Reading it as `Option<String>` is correct. Reading it as `Option<Value>` would fail because it's not a JSONB column.
**Why it happens:** `context` stores JSON but as plain text, not PostgreSQL JSON/JSONB type.
**How to avoid:** Use `r.get::<Option<String>, _>("context")` -- return the raw JSON string and let the Lisp consumer parse it.
**Warning signs:** Deserialization errors on the `context` column.

### Pitfall 5: scheduled_at Timestamp Format
**What goes wrong:** Lisp `parse-iso8601` expects a specific ISO8601 format. If the Rust serialization produces a different format, `filter-scheduled-tasks` breaks silently (treats unparseable timestamps as "ready").
**Why it happens:** `chrono::to_rfc3339()` produces `2026-03-26T12:00:00+00:00` which is valid ISO8601. The Lisp parser handles this.
**How to avoid:** Use `.to_rfc3339()` for timestamp serialization. Verify by checking `task-ready-p` behavior with the actual output format.
**Warning signs:** All tasks appearing as "ready" regardless of scheduled_at.

### Pitfall 6: ORDER BY with ANY() in CASE Expression
**What goes wrong:** Using `CASE WHEN assignee = $1 THEN 0 ELSE 1 END` in ORDER BY needs to change to `CASE WHEN $1 = ANY(assigned_to) THEN 0 ELSE 1 END` for consistent sorting.
**Why it happens:** Easy to update the WHERE clause but forget the ORDER BY clause.
**How to avoid:** Search for ALL occurrences of `assignee` in each query, not just WHERE.
**Warning signs:** Assigned tasks not appearing before department tasks in executive perception.

## Code Examples

### Example 1: Updated Executive Task Query
```rust
// Source: derived from af64_perception.rs lines 104-113, adapted per D-05/D-06
sqlx::query(
    r#"SELECT id, text, status, assignee, assigned_to, department, stage, goal_id, stage_notes,
              project_id, source, context, parent_id, priority, scheduled_at
       FROM tasks
       WHERE status IN ('open', 'pending', 'in-progress')
         AND ($1 = ANY(assigned_to) OR (department = $2 AND assigned_to IS NULL))
       ORDER BY CASE WHEN $1 = ANY(assigned_to) THEN 0 ELSE 1 END, id DESC LIMIT 10"#
)
.bind(&agent_id).bind(&agent_dept)
.fetch_all(&pool).await.unwrap_or_default()
```

### Example 2: Updated Staff Task Query
```rust
// Source: derived from af64_perception.rs lines 114-120, adapted per D-05/D-06
sqlx::query(
    r#"SELECT id, text, status, assignee, assigned_to, department, stage, goal_id, stage_notes,
              project_id, source, context, parent_id, priority, scheduled_at
       FROM tasks
       WHERE $1 = ANY(assigned_to) AND status IN ('open', 'pending', 'in-progress')
       ORDER BY id DESC LIMIT 5"#
)
.bind(&agent_id)
.fetch_all(&pool).await.unwrap_or_default()
```

### Example 3: Updated Triage Task Query
```rust
// Source: derived from af64_perception.rs lines 98-103, adapted per D-05
sqlx::query(
    r#"SELECT id, text, status, assignee, assigned_to, department, stage, goal_id, stage_notes,
              project_id, source, context, parent_id, priority, scheduled_at
       FROM tasks
       WHERE status IN ('open', 'pending', 'in-progress') AND assigned_to IS NULL
       ORDER BY id DESC LIMIT 15"#
)
.fetch_all(&pool).await.unwrap_or_default()
```

### Example 4: Updated Serialization Block
```rust
// Source: derived from af64_perception.rs lines 123-136
let tasks: Vec<Value> = task_rows.iter().map(|r| {
    let text: String = r.get("text");
    let truncated_text: String = text.chars().take(300).collect();
    serde_json::json!({
        "id": r.get::<i32, _>("id"),
        "text": truncated_text,
        "status": r.get::<String, _>("status"),
        "assignee": r.get::<Option<String>, _>("assignee"),
        "assigned_to": r.get::<Option<Vec<String>>, _>("assigned_to"),
        "department": r.get::<Option<String>, _>("department"),
        "stage": r.get::<Option<String>, _>("stage"),
        "goal_id": r.get::<Option<i32>, _>("goal_id"),
        "stage_notes": r.get::<Option<String>, _>("stage_notes"),
        "project_id": r.get::<Option<i32>, _>("project_id"),
        "source": r.get::<Option<String>, _>("source"),
        "context": r.get::<Option<String>, _>("context"),
        "parent_id": r.get::<Option<i32>, _>("parent_id"),
        "priority": r.get::<Option<String>, _>("priority"),
        "scheduled_at": r.get::<Option<chrono::DateTime<chrono::Utc>>, _>("scheduled_at")
            .map(|dt| dt.to_rfc3339()),
    })
}).collect();
```

### Example 5: End-to-End Verification Script
```bash
# Verify GSD tasks appear in Eliana's perception (project #51 owner)
curl -s http://localhost:8080/api/perception/eliana?tier=working | jq '.tasks[] | select(.source == "gsd") | {id, project_id, assigned_to, source}'

# Verify projects appear (urgency boost prerequisite)
curl -s http://localhost:8080/api/perception/eliana?tier=working | jq '.projects[] | {id, name, status, open_tasks}'

# Verify staff sees assigned tasks
curl -s http://localhost:8080/api/perception/devin?tier=working | jq '.tasks[] | {id, assigned_to, project_id}'
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `assignee` varchar for single assignment | `assigned_to` text[] for multi-assignment | Phase 1 (2026-03-26) | GSD dispatch writes to `assigned_to`; perception must read from it |
| Legacy tasks from Obsidian sync | GSD-dispatched tasks with full context | Phase 1 (2026-03-26) | New fields: project_id, source, context, parent_id need to flow through |

**Key finding: PERC-05 is already satisfied by existing code.** The Lisp tick engine's `filter-scheduled-tasks` (task-scheduler.lisp:47-56) already filters tasks by `scheduled_at` on the client side. All Phase 2 needs to do is include `scheduled_at` in the API response. The REQUIREMENTS.md says "Perception filters tasks by scheduled_at" -- this filtering happens in the Lisp layer, not the API layer, per D-08.

**Key finding: PERC-02 is already satisfied.** The project query (lines 318-343) already returns goals, status, description, blockers, current_context, open_tasks, completed_tasks for owned active projects. No modification needed -- just verification.

**Key finding: PERC-04 is already coded.** The tick engine (line 157-159) already calculates `(* 15 (length projects))`. All that's needed is verification that dispatched projects actually appear in the perception response for the owner.

## Discretion Recommendations

### Project goals/current_context in response
**Recommendation: No change needed.** Research shows these fields are ALREADY included in the project query (lines 320-321) and serialization (lines 337, 340). The CONTEXT.md noted the response "currently returns id, name, status, owner, open_tasks, completed_tasks" but this is outdated -- the actual code already includes description, goals, blockers, current_context.

### Error handling for malformed assigned_to arrays
**Recommendation: Use `Option<Vec<String>>` with `.unwrap_or_default()`.** If `assigned_to` is NULL, it becomes `None`. If it's a valid array, it deserializes. Malformed arrays would cause a sqlx deserialization error -- but PostgreSQL enforces text[] type constraint, so malformed data can't exist. No special error handling needed.

### Whether to truncate context JSON
**Recommendation: Return full context.** The context JSON from dispatch contains `must_haves`, `wave`, `requirements`, and `depends_on`. Typical size is <2KB (verified from live DB: task #12664's context is ~380 bytes). Truncation risks breaking JSON structure. The ghosts need `must_haves` for task verification (Phase 5). Return full.

## Open Questions

1. **Triage query: assignee IS NULL vs assigned_to IS NULL**
   - What we know: Current triage query uses `assignee IS NULL`. D-05 says migrate to `assigned_to`.
   - What's unclear: Should triage see tasks where `assigned_to IS NULL` (new GSD convention) or `assignee IS NULL` (legacy convention)? Or both?
   - Recommendation: Use `assigned_to IS NULL` per D-05. Triage agents route GSD pipeline tasks. Legacy unassigned tasks without `assigned_to` set will also have `assigned_to IS NULL`, so both sets are captured.

2. **Action planner assignee dependency**
   - What we know: `action-planner.lisp` reads `:assignee` at 3+ locations for delegation logic (lines 513, 529, 586-588).
   - What's unclear: Should the Lisp code be updated to also check `:assigned-to`?
   - Recommendation: Out of scope for Phase 2 (this is Phase 3 territory -- executive planning). For now, keep `assignee` in the response. GSD dispatch sets both `assignee` and `assigned_to[0]` to the same value, so existing Lisp code works for GSD tasks.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust/cargo | Building dpn-api | Yes | (compiling confirmed) | -- |
| PostgreSQL | Live database | Yes | 15+ | -- |
| dpn-api service | Testing endpoint | Yes | Port 8080 | -- |
| curl | E2E testing | Yes | system | -- |
| jq | JSON parsing in tests | Yes | system | -- |

No missing dependencies.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (Rust) + shell script (E2E) |
| Config file | /opt/dpn-api/Cargo.toml |
| Quick run command | `cd /opt/dpn-api && cargo build` (compile check) |
| Full suite command | `curl -s http://localhost:8080/api/perception/eliana \| jq .` (E2E) |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PERC-01 | Perception returns project-linked GSD tasks | E2E | `curl -s http://localhost:8080/api/perception/eliana?tier=working \| jq '.tasks[] \| select(.source=="gsd") \| .project_id'` | No -- Wave 0 |
| PERC-02 | Executives perceive owned projects with goals | E2E | `curl -s http://localhost:8080/api/perception/eliana?tier=working \| jq '.projects[] \| {id,name,goals}'` | No -- Wave 0 |
| PERC-03 | Staff perceive tasks with context and must_haves | E2E | `curl -s http://localhost:8080/api/perception/devin?tier=working \| jq '.tasks[] \| select(.source=="gsd") \| .context'` | No -- Wave 0 |
| PERC-04 | Project ownership triggers +15 urgency boost | E2E + manual | Verify tick engine logs show project_boost > 0 for project-owning exec | No -- Wave 0 |
| PERC-05 | Tasks filtered by scheduled_at (client-side) | E2E | `curl -s http://localhost:8080/api/perception/eliana?tier=working \| jq '.tasks[] \| .scheduled_at'` (verify field present) | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** `cd /opt/dpn-api && cargo build` (compile check)
- **Per wave merge:** Full E2E curl tests against live API
- **Phase gate:** All PERC-01 through PERC-05 verified via API responses

### Wave 0 Gaps
- [ ] Shell test script for perception endpoint E2E validation
- [ ] Verification that existing GSD tasks (project #51) appear correctly after changes

## Sources

### Primary (HIGH confidence)
- `/opt/dpn-api/src/handlers/af64_perception.rs` -- direct code inspection of 401-line handler
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` -- urgency boost at line 157-159, filter-scheduled-tasks at line 120-124
- `/opt/project-noosphere-ghosts/lisp/runtime/task-scheduler.lisp` -- filter-scheduled-tasks implementation, task-ready-p, deadline-urgency-boost
- `/opt/project-noosphere-ghosts/lisp/runtime/perception.lisp` -- API caller, has-actionable-items checks :tasks and :projects
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` -- :assignee reads at lines 513, 529, 586-588
- Live `\d tasks` output -- confirmed all 39 columns including assigned_to text[], scheduled_at timestamptz
- Live GSD task query -- confirmed 5 GSD tasks exist for project #51 with assigned_to, project_id, source, context populated
- `/opt/dpn-api/src/handlers/af64_agents.rs` -- confirmed `Option<Vec<String>>` pattern for text[] columns (lines 33-36)

### Secondary (MEDIUM confidence)
- sqlx 0.8 documentation -- text[] maps to `Vec<String>` in sqlx (verified via existing code patterns in af64_agents.rs)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all code inspected directly, no external libraries needed
- Architecture: HIGH -- existing patterns clearly established, changes are surgical
- Pitfalls: HIGH -- identified from direct code analysis of both Rust and Lisp consumers

**Research date:** 2026-03-26
**Valid until:** 2026-04-26 (stable codebase, no external dependency changes)
