# Phase 9: Verification Levels - Research

**Researched:** 2026-03-26
**Domain:** Common Lisp (action-executor, tick-engine) + Rust (dpn-api perception/conversations)
**Confidence:** HIGH

## Summary

Phase 9 adds quality severity classification (CRITICAL/WARNING/SUGGESTION) to the task completion flow. The changes are well-scoped: the Phase 7 structured artifact already contains an `issues` array with severity+description fields, so this phase extracts that data during completion, enriches the executive notification conversation, and adds a new urgency boost in the tick engine.

Three files need modification: `action-executor.lisp` (extract issues from artifact at completion time, include in executive notification), `tick-engine.lisp` (add `quality-issue-boost` to the urgency formula), and `af64_perception.rs` (add a query for tasks with CRITICAL issues in the executive's owned projects). No new database columns or API endpoints are needed -- everything flows through existing `stage_notes` JSONB, `conversations` metadata JSONB, and the perception response.

**Primary recommendation:** Implement in two waves: Wave 1 modifies the completion flow in action-executor.lisp to extract issues and enrich the notification; Wave 2 adds the urgency boost in tick-engine.lisp and the perception query in af64_perception.rs.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Leverage the existing Phase 7 `issues` array in structured artifacts. When a ghost completes a task, the action executor extracts issues from the artifact's `issues` field and includes them in the completion report. No new command or parser needed.
- **D-02:** The `issues` array already has `{severity: string, description: string}` structure from Phase 7. Severity values are: "CRITICAL", "WARNING", "SUGGESTION".
- **D-03:** If the artifact has no issues array or it's empty, the completion report indicates "no quality issues" -- this is the happy path.
- **D-04:** Self-assessed by the completing ghost. The LLM naturally includes severity in its issues array based on the work context. No external validation or rule-based classification.
- **D-05:** Valid severity values: CRITICAL (blocking issue, must-haves potentially unmet), WARNING (non-blocking issue, partial completion), SUGGESTION (improvement opportunity, non-essential).
- **D-06:** Add a new `quality_issue_boost` to the urgency formula in tick-engine.lisp. The tick engine checks for unresolved CRITICAL issues across the agent's tasks and applies an urgency boost separate from the regular message boost.
- **D-07:** The boost applies to the supervising executive's urgency when they have tasks with CRITICAL verification issues pending review.
- **D-08:** Enhance the existing completion report (conversation POST) to include a quality assessment section. Format: severity counts + individual CRITICAL/WARNING items listed. SUGGESTION items summarized as count only.
- **D-09:** Completion conversation metadata includes `severity_level` field set to the highest severity found (CRITICAL > WARNING > SUGGESTION > none).

### Claude's Discretion
- Exact urgency boost value for CRITICAL issues (should be significant -- perhaps +35 to +45, between task boost and message boost)
- Whether to store quality assessment summary in a task field or only in the conversation message
- How to surface quality issues in the perception endpoint for executive review

### Deferred Ideas (OUT OF SCOPE)
None
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| VER-01 | Task completion reports include severity classification (CRITICAL/WARNING/SUGGESTION) for quality issues found | Completion flow at action-executor.lisp lines 692-741 already posts conversation; extend to extract issues from artifact and format severity summary |
| VER-02 | Executive perceives tasks with CRITICAL verification issues at elevated urgency | Urgency formula at tick-engine.lisp lines 144-168 is additive; add quality-issue-boost alongside existing msg-boost, deadline-boost etc. |
| VER-03 | Staff ghost outputs structured quality assessment alongside COMPLETE: command | Already handled by Phase 7 artifact structure -- issues array in stage_notes JSONB. No new parser needed, just extraction at completion time |
</phase_requirements>

## Standard Stack

No new libraries needed. All changes use existing infrastructure:

### Core (Already Present)
| Library/Tool | Location | Purpose | Role in Phase 9 |
|-------------|----------|---------|-----------------|
| Custom JSON (json.lisp) | `/opt/project-noosphere-ghosts/lisp/util/json.lisp` | JSON encode/decode with keyword->underscore conversion | Build enriched completion messages, parse artifact issues |
| dpn-api (Axum) | `/opt/dpn-api/` | REST API | Perception endpoint modification for quality issue visibility |
| sqlx 0.8 | dpn-api Cargo.toml | PostgreSQL driver | Query tasks with CRITICAL issues for perception |
| serde_json | dpn-api Cargo.toml | JSON handling | Parse stage_notes JSONB for severity data |

## Architecture Patterns

### Modification Map
```
/opt/project-noosphere-ghosts/lisp/runtime/
  action-executor.lisp    # Lines 692-741: enrich completion report with issues
  tick-engine.lisp        # Lines 144-168: add quality-issue-boost to urgency formula

/opt/dpn-api/src/handlers/
  af64_perception.rs      # Lines 149-181: add critical_issues query for executives
```

### Pattern 1: Issue Extraction from Artifact (action-executor.lisp)

**What:** At completion time, after fetching `task-data`, parse `stage_notes` JSONB to extract the `issues` array. Classify by severity and format into the completion report message.

**When to use:** Inside the existing completion block (lines 692-741), after `stage-notes` is retrieved.

**Key data flow:**
```
task-data → :STAGE-NOTES → parse-json → :ISSUES array → extract severity/description
  → format into completion message
  → add :severity-level to conversation metadata
```

**Example (Lisp):**
```lisp
;; Extract issues from structured artifact in stage_notes
(let* ((stage-notes-raw (gethash :STAGE-NOTES task-data))
       (artifact (when (hash-table-p stage-notes-raw) stage-notes-raw))
       ;; stage_notes might be a string (legacy) or hash-table (structured)
       (artifact (or artifact
                     (when (stringp stage-notes-raw)
                       (handler-case (parse-json stage-notes-raw)
                         (error () nil)))))
       (issues-raw (when (hash-table-p artifact)
                     (gethash :ISSUES artifact)))
       (issues (when issues-raw
                 (if (vectorp issues-raw)
                     (coerce issues-raw 'list)
                     issues-raw))))
  ;; issues is now a list of hash-tables with :SEVERITY and :DESCRIPTION
  ;; Determine highest severity
  (let* ((has-critical (some (lambda (i) (string-equal (gethash :SEVERITY i) "CRITICAL")) issues))
         (has-warning (some (lambda (i) (string-equal (gethash :SEVERITY i) "WARNING")) issues))
         (has-suggestion (some (lambda (i) (string-equal (gethash :SEVERITY i) "SUGGESTION")) issues))
         (severity-level (cond (has-critical "CRITICAL")
                               (has-warning "WARNING")
                               (has-suggestion "SUGGESTION")
                               (t "none"))))
    ;; Use severity-level in message and metadata
    ...))
```

### Pattern 2: Urgency Boost for Quality Issues (tick-engine.lisp)

**What:** Add a `quality-issue-boost` variable to the urgency calculation. For executives, check their perception tasks for any with CRITICAL issues in `stage_notes`.

**When to use:** In `phase-rank` function, alongside existing msg-boost, task-boost, etc.

**Key insight:** The perception data already includes `stage_notes` for each task. The tick engine can inspect this in-memory without additional API calls.

**Example (Lisp):**
```lisp
;; Inside phase-rank, after computing tasks
(quality-issue-boost
  (if (and tasks (> (length tasks) 0))
      (let ((critical-count 0))
        (loop for tk across tasks
              do (let* ((sn (gethash :stage-notes tk))
                        (parsed (cond ((hash-table-p sn) sn)
                                      ((stringp sn) (handler-case (parse-json sn) (error () nil)))
                                      (t nil)))
                        (issues (when (hash-table-p parsed) (gethash :ISSUES parsed))))
                   (when issues
                     (loop for issue across (if (vectorp issues) issues (coerce issues 'vector))
                           when (and (hash-table-p issue)
                                     (string-equal (gethash :SEVERITY issue) "CRITICAL"))
                             do (incf critical-count)))))
        (if (> critical-count 0) 40 0))  ;; +40 for any CRITICAL issues
      0))
```

**Recommended boost value:** +40 (between task-boost of +25 and msg-boost of +50). This ensures CRITICAL issues get executive attention without trumping direct messages.

### Pattern 3: Perception Endpoint Enhancement (af64_perception.rs)

**What:** Add a `critical_issues` array to the executive perception response. Query recently completed tasks in the executive's owned projects that have CRITICAL issues in their `stage_notes`.

**When to use:** In the perception handler, alongside existing `blocked_tasks` executive-only query.

**Example (Rust):**
```rust
// Query tasks with CRITICAL severity issues for executive review
let critical_tasks: Vec<Value> = if is_exec {
    let rows = sqlx::query(
        r#"SELECT t.id, t.text, t.stage_notes, t.project_id, t.assignee
           FROM tasks t
           WHERE t.project_id IN (
               SELECT id FROM projects WHERE owner = $1
           )
           AND t.status IN ('done', 'completed')
           AND t.stage_notes IS NOT NULL
           AND t.stage_notes::jsonb -> 'issues' IS NOT NULL
           AND EXISTS (
               SELECT 1 FROM jsonb_array_elements(t.stage_notes::jsonb -> 'issues') AS issue
               WHERE issue->>'severity' = 'CRITICAL'
           )
           ORDER BY t.updated_at DESC LIMIT 10"#
    )
    .bind(&agent_id)
    .fetch_all(&pool).await.unwrap_or_default();
    // ... map to JSON values
} else { vec![] };
```

### Anti-Patterns to Avoid
- **Parsing stage_notes as string:** `stage_notes` is JSONB. The Lisp runtime may receive it as a hash-table OR a string depending on the API response. Always handle both cases.
- **Adding new columns to tasks table:** Per D-01, leverage existing `stage_notes` JSONB. No schema migration needed.
- **Creating a new command:** Per D-01, no new parser or command. The existing COMPLETE: flow plus the artifact issues array is sufficient.
- **Checking issues on non-structured artifacts:** `schema_version: 0` (legacy) artifacts have no issues array. Only `schema_version: 1` artifacts have issues. Check `schema-version` or simply check if `issues` key exists.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Severity classification | Custom rule engine | LLM self-assessment in issues array (D-04) | Ghost already classifies naturally when generating artifacts |
| Quality data storage | New DB column | Existing `stage_notes` JSONB + `conversations` metadata JSONB | Phase 7 already established the pattern |
| Issue format | New JSON schema | Existing `{severity, description}` from Phase 7 artifacts | Already validated by `validate-artifact-base` |

## Common Pitfalls

### Pitfall 1: Lisp JSON Keyword Case
**What goes wrong:** Accessing `:severity` (lowercase) when the parser produces `:SEVERITY` (uppercase)
**Why it happens:** The custom JSON parser in json.lisp converts all JSON keys to uppercase keywords with hyphens replacing underscores. `"severity"` becomes `:SEVERITY`, `"stage_notes"` becomes `:STAGE-NOTES`.
**How to avoid:** Always use uppercase keywords: `:SEVERITY`, `:DESCRIPTION`, `:ISSUES`, `:SCHEMA-VERSION`
**Warning signs:** `nil` values when accessing issue fields

### Pitfall 2: stage_notes Type Ambiguity
**What goes wrong:** Assuming `stage_notes` is always a hash-table when it might be a string
**Why it happens:** The dpn-api returns JSONB as a parsed JSON value, but the Lisp HTTP client may receive it as a string that needs parsing, OR as an already-parsed hash-table depending on response handling
**How to avoid:** Always check type: `(if (hash-table-p sn) sn (handler-case (parse-json sn) (error () nil)))`
**Warning signs:** Type errors when accessing hash-table keys on a string value

### Pitfall 3: Empty vs Missing Issues Array
**What goes wrong:** `build-stage-artifact` creates `"issues": []` (empty array). But legacy tasks have no issues field at all.
**Why it happens:** Phase 7 artifacts always include `:issues (json-array)` but older tasks have `schema_version: 0` with only `legacy_text`
**How to avoid:** Check both: `(when (and issues (> (length issues) 0)) ...)`. Treat nil and empty-vector the same as "no issues".
**Warning signs:** Errors iterating over nil issues arrays

### Pitfall 4: Urgency Boost Applied to Wrong Agent
**What goes wrong:** Quality issue boost applied to the staff agent who completed the task rather than the supervising executive
**Why it happens:** The boost needs to apply to the executive who will REVIEW the completion, not the agent who did the work
**How to avoid:** In tick-engine, the boost should check tasks in projects owned by the executive being ranked, filtering for done tasks with CRITICAL issues
**Warning signs:** Staff agents getting boosted urgency for their own completed work

### Pitfall 5: JSON Path in PostgreSQL Query
**What goes wrong:** `stage_notes::jsonb -> 'issues'` fails when stage_notes is already JSONB (no cast needed) or when issues key doesn't exist
**Why it happens:** `stage_notes` column is already `jsonb` type. The `->` operator returns NULL for missing keys which is fine, but `jsonb_array_elements` will error on NULL.
**How to avoid:** Use the null-check pattern: `AND t.stage_notes -> 'issues' IS NOT NULL` before using `jsonb_array_elements`
**Warning signs:** SQL errors on tasks without issues in stage_notes

## Code Examples

### Current Completion Report (Lines 727-740 of action-executor.lisp)
```lisp
;; Source: /opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp
(api-post "/api/conversations"
  (json-object
   :from-agent agent-id
   :to-agent (json-array executive)
   :message (format nil "[Task Complete] ~a finished task #~a~%~%**Project:** ~a~%**Must-haves:** ~{~a~^, ~}~%**Summary:** ~a"
                    agent-id task-id
                    (or project-name "unknown")
                    (if (and must-haves (listp must-haves)) must-haves '("none"))
                    (subseq stage-notes 0 (min 500 (length stage-notes))))
   :channel "noosphere"
   :metadata (json-object
              :source "task_completion"
              :task-id task-id
              :project-id (or project-id :null))))
```

### Current Urgency Formula (Lines 144-168 of tick-engine.lisp)
```lisp
;; Source: /opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp
(urgency (+ (* pressure (/ energy 100.0))
            msg-boost      ;; +50 for pending messages
            req-boost      ;; +40 for pending requests
            task-boost     ;; +25 for pending tasks
            project-boost  ;; +15 per active project
            deadline-boost ;; 0-50 based on deadline proximity
            ))
```

### Artifact Issues Structure (from Phase 7)
```json
{
  "schema_version": 1,
  "summary": "...",
  "key_outputs": [...],
  "issues": [
    {"severity": "CRITICAL", "description": "Must-have X not met because..."},
    {"severity": "WARNING", "description": "Partial implementation of..."},
    {"severity": "SUGGESTION", "description": "Could improve by..."}
  ],
  "metadata": {"stage": "build", "agent_id": "marcus", "timestamp": "..."}
}
```

### Lisp JSON Keyword Mapping Reference
```
JSON key         → Lisp keyword (parsing)    → JSON key (encoding)
"severity"       → :SEVERITY                  → "severity"
"description"    → :DESCRIPTION               → "description"
"stage_notes"    → :STAGE-NOTES               → "stage_notes"
"schema_version" → :SCHEMA-VERSION            → "schema_version"
"severity_level" → :SEVERITY-LEVEL            → "severity_level"
"issues"         → :ISSUES                    → "issues"
```

### Conversations Metadata Schema (Current + Phase 9 Addition)
```json
{
  "source": "task_completion",
  "task_id": 1234,
  "project_id": 56,
  "severity_level": "CRITICAL"
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Text-only stage_notes (schema_version 0) | Structured JSONB artifact (schema_version 1) | Phase 7 | Issues array available for quality classification |
| Simple completion notification | Enriched completion report with project/must-haves | Phase 5 | Foundation for adding quality assessment section |
| Fixed urgency formula | Extensible additive boosts | Phase 6 (deadline_boost) | Can add quality_issue_boost as new term |

## Open Questions

1. **Perception Query Performance**
   - What we know: `jsonb_array_elements` with a WHERE clause inside EXISTS is efficient for small result sets. The LIMIT 10 bounds the output.
   - What's unclear: How many tasks in practice will have CRITICAL issues? If rare, the query is fast. If common, may need an index.
   - Recommendation: Start without index. If perception latency increases, add a GIN index on `stage_notes` later.

2. **Quality Assessment Storage Location**
   - What we know: D-09 says metadata gets `severity_level`. The message text gets the formatted assessment.
   - What's unclear: Should a summary also be stored back into the task's `stage_notes`? This would make the perception query simpler (just check a top-level field).
   - Recommendation: Store `severity_level` as a top-level key in `stage_notes` JSONB when completing the task. This simplifies the perception query from needing `jsonb_array_elements` to a simple `stage_notes->>'severity_level' = 'CRITICAL'` check.

3. **Boost Value Calibration**
   - What we know: Current boosts: msg=50, req=40, task=25, project=15*N, deadline=0-50
   - What's unclear: Optimal value for quality_issue_boost
   - Recommendation: Use +40 (same as request boost). CRITICAL quality issues are as important as direct requests -- they indicate potentially broken deliverables that need executive review.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual validation (no automated test infrastructure for Lisp runtime) |
| Config file | None |
| Quick run command | Manual: restart ghosts, trigger completion, check conversation output |
| Full suite command | `cargo test` for dpn-api Rust changes only |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| VER-01 | Completion report includes severity classification | manual | Trigger task completion, verify conversation message format | N/A |
| VER-02 | Executive perceives CRITICAL issues at elevated urgency | manual | Check tick-engine urgency output with CRITICAL task present | N/A |
| VER-03 | Structured quality assessment in COMPLETE output | manual | Verify stage_notes has issues array after ghost completes task | N/A |

### Sampling Rate
- **Per task commit:** Manual verification via PM2 logs and DB queries
- **Per wave merge:** `cargo build` for Rust changes, restart ghosts for Lisp changes
- **Phase gate:** End-to-end test: ghost completes task with issues -> executive gets enriched notification -> urgency formula reflects boost

### Wave 0 Gaps
None -- no automated test infrastructure exists for the Lisp runtime. All validation is manual via PM2 logs and DB inspection.

## Project Constraints (from CLAUDE.md)

- **Stack**: Rust (dpn-api), Common Lisp/SBCL (ghosts), PostgreSQL -- no new languages
- **DB is the OS**: All state in master_chronicle. Quality assessments flow through existing JSONB columns.
- **Lisp JSON quirk**: Parser converts underscores to hyphens. Use uppercase keywords (`:SEVERITY`, `:ISSUES`).
- **UTF-8 Rule**: Any Rust string manipulation must use `.chars().take(N).collect()`, never byte slicing.
- **Single droplet**: Resource-conscious. No new services or heavy queries.
- **Ghost LLM budget**: $0.50/request. Quality self-assessment adds no extra LLM calls (issues are part of existing artifact output).

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` -- completion flow (lines 592-741), artifact validation (lines 78-104), build-stage-artifact (lines 142-160)
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` -- urgency formula (lines 130-180)
- `/opt/project-noosphere-ghosts/lisp/runtime/task-scheduler.lisp` -- deadline-urgency-boost pattern (lines 23-45)
- `/opt/dpn-api/src/handlers/af64_perception.rs` -- perception response structure (lines 140-205), executive blocked_tasks pattern (lines 149-181)
- `/opt/dpn-api/src/handlers/af64_conversations.rs` -- conversation POST with metadata (lines 79-104)
- `/opt/project-noosphere-ghosts/lisp/util/json.lisp` -- json-object, keyword->json-key (lines 19-30, 190-247)
- PostgreSQL schema inspection: `conversations` table (metadata JSONB), `tasks` table (stage_notes JSONB)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - no new libraries, all existing infrastructure verified via source code
- Architecture: HIGH - all integration points inspected, code patterns extracted from canonical files
- Pitfalls: HIGH - identified from actual code inspection (JSON keyword quirk, type ambiguity, empty array handling)

**Research date:** 2026-03-26
**Valid until:** 2026-04-26 (stable -- all changes to existing Lisp/Rust codebase)
