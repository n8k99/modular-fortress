# Phase 8: Decisions Brain - Research

**Researched:** 2026-03-26
**Domain:** API CRUD (Rust/Axum), Lisp action-executor integration, LLM prompt enrichment
**Confidence:** HIGH

## Summary

Phase 8 wires the existing but empty `decisions` table into the executive cognition loop. Three integration points: (1) a new Rust handler for decisions CRUD via dpn-api, (2) enhancement of the Lisp action-executor to POST decisions to the API when `DECISION:` is detected in LLM output, and (3) injection of recent decisions into the executive project review prompt in action-planner.lisp.

All infrastructure exists. The decisions table is live in PostgreSQL with the correct schema (minus a `department` column per D-09). The dpn-api has well-established handler patterns (af64_tasks.rs is the reference). The Lisp api-client provides `api-get` and `api-post` functions. The `execute-project-review` function already extracts `:project-id` from metadata, so routing decisions to the correct project is straightforward.

**Primary recommendation:** Follow the existing af64_tasks.rs handler pattern exactly for the Rust decisions handler, then make minimal Lisp changes -- add an `api-post` call in the decision detection block and a decisions fetch in `build-project-review-job`.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Enhance existing `DECISION:` keyword detection in action-executor.lisp (line ~1081). When detected, parse content and POST to decisions table via API.
- **D-02:** Keep existing logging to agent memory/vault_notes alongside new DB persistence. Decisions table is source of truth for project decisions; memory is agent's personal record.
- **D-03:** Query `GET /api/decisions?project_id=X&limit=10&order=desc` for each project during `build-project-review-job`. Include as "Recent Decisions" section in review prompt.
- **D-04:** Last 10 decisions per project, most recent first. Bounded context to avoid prompt bloat.
- **D-05:** Create `GET /api/decisions` with `project_id` filter, `limit`, `order` params. Support `department` filter for department-wide decisions.
- **D-06:** Create `POST /api/decisions` for new decisions. Fields: decision (required), rationale (optional), project_id (optional), department (optional), owner (required), stakeholders (optional JSONB).
- **D-07:** Decisions are append-only -- no PUT or DELETE.
- **D-08:** Decisions can be project-scoped OR department-scoped. Both optional but at least one should be present.
- **D-09:** Add optional `department VARCHAR(256)` column to decisions table.
- **D-10:** Auto-populate owner from agent_id and stakeholders from project team members when logging a decision during project review.

### Claude's Discretion
- Whether to parse rationale from the DECISION: line (e.g., `DECISION: X because Y`) or let the LLM output structured fields
- How to format decisions in the review prompt (table vs bullet list)
- Whether to add a `tags` or `category` field to decisions for future filtering

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DEC-01 | Executive project review prompt includes recent decisions from decisions table | `build-project-review-job` (action-planner.lisp:759-821) is the injection point; api-get available; project-id available in context |
| DEC-02 | When executive makes a decision during project review, it is logged to decisions table with project_id and agent attribution | `write-agent-daily-memory` (action-executor.lisp:1067) detects `DECISION:` keyword; metadata contains `:project-id`; api-post available |
| DEC-03 | Decisions queryable via /api/decisions with project_id filter | New handler file `decisions.rs` following af64_tasks.rs pattern; route registration in main.rs |
</phase_requirements>

## Standard Stack

### Core (Already Installed -- No New Dependencies)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| axum | 0.7 | HTTP handler for decisions endpoints | Already used by dpn-api |
| sqlx | 0.8 | PostgreSQL queries for decisions CRUD | Already used by dpn-api |
| serde / serde_json | 1 | Request/response serialization | Already used by dpn-api |
| chrono | 0.4 | Date handling for decision dates | Already used by dpn-api |

### Supporting (Lisp Side)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| af64 api-client | custom | HTTP calls to dpn-api | api-get for fetching decisions, api-post for creating them |
| af64 json utilities | custom | json-object, json-array, encode-json | Building payloads for API calls |

**Installation:** None required. All dependencies already present in both dpn-api and project-noosphere-ghosts.

## Architecture Patterns

### Recommended Changes (3 Files Modified, 1 File Created, 1 SQL Migration)

```
/opt/dpn-api/
├── src/handlers/
│   ├── mod.rs                    # ADD: pub mod decisions;
│   └── decisions.rs              # NEW: GET + POST handlers
├── src/main.rs                   # ADD: 2 route lines

/opt/project-noosphere-ghosts/
├── lisp/runtime/
│   ├── action-executor.lisp      # MODIFY: decision detection block (~line 1082)
│   └── action-planner.lisp       # MODIFY: build-project-review-job (~line 787)

SQL Migration:
  ALTER TABLE decisions ADD COLUMN department VARCHAR(256);
  CREATE INDEX idx_decisions_department ON decisions (department);
  CREATE INDEX idx_decisions_project_id ON decisions (project_id);
```

### Pattern 1: Rust Handler (decisions.rs)

**What:** A new handler file following the af64_tasks.rs pattern exactly.
**When to use:** This is the only approach. Follow established conventions.

Key struct definitions needed:
```rust
// Source: Pattern from /opt/dpn-api/src/handlers/af64_tasks.rs
#[derive(Deserialize)]
pub struct DecisionQuery {
    pub project_id: Option<i32>,
    pub department: Option<String>,
    pub owner: Option<String>,
    pub limit: Option<i64>,
    pub order: Option<String>,  // "asc" or "desc", default "desc"
}

#[derive(Deserialize)]
pub struct NewDecision {
    pub decision: String,
    pub rationale: Option<String>,
    pub project_id: Option<i32>,
    pub department: Option<String>,
    pub owner: String,
    pub stakeholders: Option<serde_json::Value>,
    pub date: Option<String>,  // ISO date, defaults to today
}
```

Handler signatures:
```rust
pub async fn list_decisions(
    State(pool): State<DbPool>,
    Query(q): Query<DecisionQuery>,
) -> Result<Json<Value>, ApiError>

pub async fn create_decision(
    State(pool): State<DbPool>,
    Json(body): Json<NewDecision>,
) -> Result<Json<Value>, ApiError>
```

### Pattern 2: Lisp API POST for Decision Capture

**What:** When `has-decision` is true in `write-agent-daily-memory`, also POST to `/api/decisions`.
**When to use:** Every time a decision is detected in LLM output.

The metadata (input-context from the job) contains `:project-id` and `:department`. The `result` struct exposes this via `(cognition-result-metadata result)`.

```lisp
;; In write-agent-daily-memory, after existing has-decision block (line 1089-1090):
(when has-decision
  ;; Existing: write to memory
  (setf (gethash "decisions_made" payload) summary)
  ;; NEW: persist to decisions table via API
  (handler-case
    (let* ((meta (cognition-result-metadata result))
           (project-id (when (hash-table-p meta) (gethash :project-id meta)))
           (department (when (hash-table-p meta) (gethash :department meta)))
           (decision-text (extract-decision-text content)))
      (when decision-text
        (api-post "/api/decisions"
          (json-object
            :decision decision-text
            :owner agent-id
            :project-id (or project-id :null)
            :department (or department :null)
            :date (today-iso)))))
    (error (e) (format t "  [decision-persist-error] ~a~%" e))))
```

**Critical issue:** `write-agent-daily-memory` currently receives `(result action)` but NOT `metadata` separately. However, metadata IS accessible via `(cognition-result-metadata result)` since the struct stores it. Verified: cognition-types.lisp line 120 shows metadata as a slot.

### Pattern 3: Decision Context Injection in Project Review

**What:** Fetch recent decisions and include in executive prompt.
**When to use:** In `build-project-review-job` before building the prompt.

```lisp
;; In build-project-review-job, after team-roster binding (~line 789):
(decisions-context
  (handler-case
    (let* ((pid (gethash :id (elt projects 0)))
           (resp (api-get "/api/decisions"
                   (list :project-id pid :limit 10 :order "desc"))))
      (if (and resp (vectorp resp) (> (length resp) 0))
        (with-output-to-string (s)
          (format s "~%### Recent Decisions~%")
          (loop for d across resp do
            (format s "- [~a] ~a: ~a~%"
              (or (gethash :date d) "?")
              (or (gethash :owner d) "?")
              (gethash :decision d))))
        ""))
    (error () "")))
```

Then inject into the user message content (line 813), between project-summaries and team-roster.

### Anti-Patterns to Avoid
- **Do NOT add a separate decisions fetch per project in the loop.** Fetch for the first project only (current behavior reviews one project at a time per job).
- **Do NOT parse "decided" keyword for DB persistence.** The "decided" detection is too loose for structured persistence -- only `DECISION:` keyword lines should create DB records (D-01).
- **Do NOT add PUT/DELETE endpoints.** Decisions are append-only (D-07).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Decision text extraction | Complex NLP parsing | Simple `DECISION:` prefix extraction | The keyword is already standardized; LLMs output it consistently |
| Date handling | Manual date string formatting | `CURRENT_DATE` in SQL, chrono in Rust | Avoids timezone bugs |
| JSON response building | Custom serializers | `serde_json::json!()` macro + `r.get::<Type, _>()` | Established dpn-api pattern |

## Common Pitfalls

### Pitfall 1: Lisp JSON Underscore-to-Hyphen Conversion
**What goes wrong:** Lisp JSON parser converts `project_id` to `:project-id`. When building payloads, keywords auto-convert back. But string keys in hash-tables do NOT convert.
**Why it happens:** AF64 custom JSON parser behavior (CLAUDE.md: "parser converts underscores to hyphens").
**How to avoid:** Use keyword keys (`:project-id`) for json-object payloads -- they auto-convert to `project_id` in the JSON output. Never use string keys like `"project_id"` in json-object calls.
**Warning signs:** API returns 400 or field is null when it should have a value.

### Pitfall 2: NULL vs Missing project_id in Decisions
**What goes wrong:** A decision with `project_id = null` is department-scoped. But if the Lisp code sends `:null` for project_id, SQL receives NULL correctly. If it sends nothing, the field is missing from the JSON body.
**Why it happens:** json-object with `:null` value encodes to JSON `null`. Omitting the key entirely means the Rust deserializer uses `None` (which is correct for `Option<i32>`).
**How to avoid:** Always include project_id in the POST body, using `:null` when not project-scoped. This is explicit and matches the Rust `Option<i32>` deserialization.
**Warning signs:** Decisions created without project_id when one was expected.

### Pitfall 3: Decision Detection False Positives
**What goes wrong:** The word "decided" (line 1082) triggers `has-decision` for casual mentions like "the team decided to meet later" -- these are not formal decisions.
**Why it happens:** Current detection is intentionally broad for memory logging. But DB persistence needs higher precision.
**How to avoid:** For the new API POST, only trigger on the explicit `DECISION:` keyword prefix, NOT on the looser "decided" match. Keep the existing broad match for memory logging unchanged (D-02).
**Warning signs:** Decision table filled with non-decisions.

### Pitfall 4: Missing project_id Index
**What goes wrong:** `GET /api/decisions?project_id=X` does a sequential scan on the decisions table.
**Why it happens:** The migration script (line 135-136) creates indexes on `owner` and `date` but NOT on `project_id`.
**How to avoid:** Add `CREATE INDEX idx_decisions_project_id ON decisions (project_id);` in the migration.
**Warning signs:** Slow queries as decisions table grows.

### Pitfall 5: Rust UTF-8 in Decision Text
**What goes wrong:** Decision text from LLM output may contain emoji or unicode. Byte slicing panics.
**Why it happens:** CLAUDE.md guardrail: "Never mix character positions with byte indices."
**How to avoid:** Use `.chars().take(N).collect::<String>()` if truncating decision text. The current handler pattern uses `r.get::<String, _>()` which is safe.
**Warning signs:** Runtime panic on decision creation/retrieval.

## Code Examples

### Rust: GET /api/decisions Handler
```rust
// Source: Pattern from /opt/dpn-api/src/handlers/af64_tasks.rs
pub async fn list_decisions(
    State(pool): State<DbPool>,
    Query(q): Query<DecisionQuery>,
) -> Result<Json<Value>, ApiError> {
    let limit = q.limit.unwrap_or(20).min(100);
    let order = q.order.as_deref().unwrap_or("desc");
    let order_clause = if order == "asc" { "ASC" } else { "DESC" };

    let rows = if let Some(project_id) = q.project_id {
        sqlx::query(&format!(
            "SELECT id, project_id, decision, rationale, owner, stakeholders, department, date, created_at \
             FROM decisions WHERE project_id = $1 ORDER BY created_at {} LIMIT $2", order_clause))
            .bind(project_id).bind(limit)
            .fetch_all(&pool).await
    } else if let Some(ref department) = q.department {
        sqlx::query(&format!(
            "SELECT id, project_id, decision, rationale, owner, stakeholders, department, date, created_at \
             FROM decisions WHERE department = $1 ORDER BY created_at {} LIMIT $2", order_clause))
            .bind(department).bind(limit)
            .fetch_all(&pool).await
    } else {
        sqlx::query(&format!(
            "SELECT id, project_id, decision, rationale, owner, stakeholders, department, date, created_at \
             FROM decisions ORDER BY created_at {} LIMIT $1", order_clause))
            .bind(limit)
            .fetch_all(&pool).await
    };

    let rows = rows.map_err(|e| ApiError::Database(e.to_string()))?;
    let decisions: Vec<Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.get::<i32, _>("id"),
            "project_id": r.get::<Option<i32>, _>("project_id"),
            "decision": r.get::<String, _>("decision"),
            "rationale": r.get::<Option<String>, _>("rationale"),
            "owner": r.get::<Option<String>, _>("owner"),
            "stakeholders": r.get::<Option<Value>, _>("stakeholders"),
            "department": r.get::<Option<String>, _>("department"),
            "date": r.get::<Option<chrono::NaiveDate>, _>("date"),
            "created_at": r.get::<Option<chrono::DateTime<chrono::Utc>>, _>("created_at"),
        })
    }).collect();

    Ok(Json(serde_json::json!(decisions)))
}
```

### Rust: POST /api/decisions Handler
```rust
pub async fn create_decision(
    State(pool): State<DbPool>,
    Json(body): Json<NewDecision>,
) -> Result<Json<Value>, ApiError> {
    let date = body.date
        .as_deref()
        .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .unwrap_or_else(|| chrono::Utc::now().date_naive());

    let row = sqlx::query(
        "INSERT INTO decisions (decision, rationale, project_id, department, owner, stakeholders, date) \
         VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, created_at")
        .bind(&body.decision)
        .bind(&body.rationale)
        .bind(&body.project_id)
        .bind(&body.department)
        .bind(&body.owner)
        .bind(&body.stakeholders)
        .bind(&date)
        .fetch_one(&pool).await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "id": row.get::<i32, _>("id"),
        "created_at": row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("created_at"),
    })))
}
```

### Lisp: Decision Text Extraction
```lisp
;; Discretion recommendation: parse "DECISION: X because Y" into decision + rationale
(defun extract-decision-text (content)
  "Extract decision text from content containing DECISION: keyword.
   Returns (values decision rationale) or NIL if no DECISION: found."
  (let ((pos (search "DECISION:" content)))
    (when pos
      (let* ((start (+ pos 9))  ;; length of "DECISION:"
             (rest (string-trim '(#\Space) (subseq content start)))
             (end (or (position #\Newline rest) (length rest)))
             (line (subseq rest 0 end))
             (because-pos (search " because " line)))
        (if because-pos
            (values (string-trim '(#\Space) (subseq line 0 because-pos))
                    (string-trim '(#\Space) (subseq line (+ because-pos 9))))
            (values line nil))))))
```

### Lisp: Decisions Section in Review Prompt
```lisp
;; Insert between project-summaries and team-roster in build-project-review-job
;; In the :content format string (line 813):
(format nil "Review your project portfolio:~%~a~%~a~%~a"
        project-summaries decisions-context team-roster)
```

### Route Registration (main.rs)
```rust
// Add to protected_routes in main.rs after existing routes:
.route("/decisions", get(handlers::decisions::list_decisions)
                    .post(handlers::decisions::create_decision))
```

## Discretion Recommendations

### Rationale Parsing
**Recommendation:** Parse `DECISION: X because Y` into separate decision and rationale fields. This is cheap, LLMs naturally use "because" as a delimiter, and it enriches the data without requiring LLM behavior changes. If no "because" is found, put the full text in decision and leave rationale NULL.

### Prompt Formatting
**Recommendation:** Use a bullet list, not a table. Tables consume more tokens and are harder to read in the system prompt. Format: `- [2026-03-26] nova: Use Redis for caching (because: lower latency than PostgreSQL polling)`

### Tags/Category Field
**Recommendation:** Skip for now. The decisions table is empty and we don't yet know what categories would be useful. Can be added later with a simple ALTER TABLE. Avoid premature abstraction.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (Rust), manual curl (API), SBCL REPL (Lisp) |
| Config file | /opt/dpn-api/Cargo.toml |
| Quick run command | `cd /opt/dpn-api && cargo check` |
| Full suite command | `cd /opt/dpn-api && cargo build` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DEC-01 | Decisions injected into project review prompt | manual | Inspect `build-project-review-job` output via tick log | N/A |
| DEC-02 | Decision detected and POSTed to API | smoke | `curl -s http://localhost:8080/api/decisions -H "X-API-Key: dpn-nova-2026" \| jq length` | N/A |
| DEC-03 | GET /api/decisions?project_id=X returns decisions | smoke | `curl -s "http://localhost:8080/api/decisions?project_id=1" -H "X-API-Key: dpn-nova-2026"` | N/A |

### Sampling Rate
- **Per task commit:** `cd /opt/dpn-api && cargo check` (type-checks without full build)
- **Per wave merge:** `cd /opt/dpn-api && cargo build` + curl smoke tests
- **Phase gate:** Full build + manual curl tests for all 3 requirements

### Wave 0 Gaps
- None -- no test framework to set up. Rust compilation IS the type-check. Smoke tests via curl against live API.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Decision detection writes to memory only | Decision detection writes to memory + decisions table | Phase 8 | Structured, queryable decisions |
| No decisions in project review context | Recent decisions injected into review prompt | Phase 8 | Executives see prior decisions, act consistently |

## Open Questions

1. **Multiple DECISION: lines per response**
   - What we know: LLM output can contain multiple `DECISION:` lines in one response
   - What's unclear: Should each create a separate DB record?
   - Recommendation: Yes, split on `DECISION:` and create one record per occurrence. Simple loop.

2. **First project only vs all projects**
   - What we know: `build-project-review-job` sends ALL projects in the summary but metadata only carries the FIRST project's ID (line 806)
   - What's unclear: Should decisions be fetched for all owned projects?
   - Recommendation: Fetch for the first project only (matches current metadata pattern). Multi-project decision context would bloat the prompt. Executives review one project deeply per tick cycle anyway.

## Sources

### Primary (HIGH confidence)
- Live DB schema: `\d decisions` -- verified 8 columns, 0 rows, no department column
- `/opt/dpn-api/src/handlers/af64_tasks.rs` -- verified CRUD handler pattern
- `/opt/dpn-api/src/handlers/mod.rs` -- verified 22 handler modules
- `/opt/dpn-api/src/main.rs` -- verified route registration pattern
- `/opt/dpn-api/src/error.rs` -- verified ApiError enum (5 variants)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` -- verified DECISION: detection at line 1082, project-id in metadata
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` -- verified build-project-review-job at line 759, prompt template at line 809
- `/opt/project-noosphere-ghosts/lisp/runtime/api-client.lisp` -- verified api-get, api-post, api-put, api-patch functions
- `/opt/project-noosphere-ghosts/lisp/runtime/cognition-types.lisp` -- verified cognition-result struct with metadata slot (line 120)
- `/opt/project-noosphere-ghosts/lisp/runtime/cognition-broker.lisp` -- verified metadata = input-context (line 198)

### Secondary (MEDIUM confidence)
- None needed -- all findings from direct code inspection

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - no new dependencies, all verified in codebase
- Architecture: HIGH - all integration points verified with line numbers
- Pitfalls: HIGH - JSON quirk, index gap, and false positive issues verified in code

**Research date:** 2026-03-26
**Valid until:** 2026-04-26 (stable -- internal codebase, no external dependency drift)
