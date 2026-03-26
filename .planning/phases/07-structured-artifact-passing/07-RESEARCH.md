# Phase 7: Structured Artifact Passing - Research

**Researched:** 2026-03-26
**Domain:** Pipeline stage output schemas, JSONB migration, Lisp JSON validation, Rust API type changes
**Confidence:** HIGH

## Summary

Phase 7 replaces freeform text `stage_notes` with typed, validated JSON artifacts so pipeline handoffs carry structured context. The work spans three layers: (1) PostgreSQL migration from TEXT to JSONB with legacy data wrapping, (2) Lisp validation rewrite in `action-executor.lisp` replacing keyword/length checks with JSON schema validation, and (3) Rust API type changes in `af64_tasks.rs` and `af64_perception.rs` to handle JSONB instead of String.

The current codebase has 374 tasks with existing stage_notes data (all freeform text). The migration must wrap these in `{"legacy_text": "<content>", "schema_version": 0}` per D-05. The existing `validate-stage-output` function (118 lines, 15 stage-specific keyword checks) will be rewritten to check JSON structure. The `load-previous-stage-output` disk-file function in `action-planner.lisp` will be replaced with a DB query for the predecessor task's stage_notes JSONB.

**Primary recommendation:** Execute in three waves: (1) DB migration + Rust JSONB type changes, (2) Lisp validation rewrite + structured output generation in advance-pipeline, (3) action-planner context injection from predecessor stage_notes.

<user_constraints>

## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Universal base schema + stage-specific extension fields. Base fields required on every stage output; extensions optional and vary by stage/pipeline type.
- **D-02:** Base schema fields: `{summary: string, key_outputs: [{name: string, content: string}], issues: [{severity: string, description: string}], metadata: {stage: string, agent_id: string, timestamp: string, duration_ms: number}}`.
- **D-03:** Stage-specific extensions defined per-stage but validated only if present (not required).
- **D-04:** Migrate `stage_notes` column from TEXT to JSONB.
- **D-05:** Existing freeform text data wrapped during migration: `{"legacy_text": "<original content>", "schema_version": 0}`.
- **D-06:** Disk file loading pattern deprecated. New stages read/write structured output from stage_notes JSONB only.
- **D-07:** Final pipeline results must persist to appropriate DB table (documents, vault_notes, decisions, etc.).
- **D-08:** Required base fields + optional extensions. Base fields MUST be present and valid JSON.
- **D-09:** Existing rejection/retry pattern preserved (3 attempts, then task blocked). Rejection fires on: not valid JSON, missing base fields, or base field type mismatch.
- **D-10:** Replace keyword + length checks in `validate-stage-output` with JSON schema validation. Keep minimum length check on `summary` field.
- **D-11:** action-planner reads predecessor task's `stage_notes` JSONB from DB and injects structured output into LLM prompt. Replaces disk-file function.
- **D-12:** Hard prompts stay in documents table, separate from stage schema.

### Claude's Discretion
- Whether to add a `schema_version` field to the base schema for future evolution
- Exact JSON structure of stage-specific extensions
- Whether `advance-pipeline` should validate JSON before storing or rely on `validate-stage-output` alone
- How to handle the transition period where some tasks have legacy text and others have structured JSON

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.

</user_constraints>

<phase_requirements>

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ART-01 | Pipeline stages have typed output schemas stored in stage_notes as structured JSON | D-01/D-02/D-03 define the schema; DB migration (D-04/D-05) enables JSONB storage; validate-stage-output rewrite (D-10) enforces schema |
| ART-02 | When task advances to next stage, output schema from current stage is passed as input context to next assignee | D-11 defines the mechanism: action-planner reads predecessor stage_notes JSONB from DB; replaces load-previous-stage-output disk function |
| ART-03 | Action executor validates stage output matches expected schema before marking stage complete | D-08/D-09/D-10 define validation: base fields required, extensions optional, 3-attempt rejection preserved |

</phase_requirements>

## Standard Stack

No new libraries needed. All work uses existing stack components.

### Core (Already in Project)
| Component | Version | Purpose | Role in Phase 7 |
|-----------|---------|---------|------------------|
| PostgreSQL | (production) | master_chronicle | JSONB column migration for stage_notes |
| sqlx | 0.8 | Rust DB driver | Handle JSONB type in queries (serde_json::Value) |
| serde_json | 1 | Rust JSON | Parse/emit JSONB values in task handlers |
| SBCL + AF64 json.lisp | custom | Lisp JSON | Build/validate structured JSON artifacts |
| axum | 0.7 | HTTP framework | Task update/create endpoints |

### No New Dependencies
This phase requires zero new crates or libraries. PostgreSQL natively supports JSONB. sqlx already handles `serde_json::Value` for JSONB columns. The Lisp `json.lisp` utility already has `json-object`, `json-array`, `parse-json`, and `encode-json`.

## Architecture Patterns

### Pattern 1: JSONB Column Migration (TEXT to JSONB)
**What:** ALTER COLUMN with data preservation, following Phase 6 migration pattern.
**When to use:** Changing stage_notes from TEXT to JSONB.
**Key concern:** 374 existing rows with freeform text must be wrapped, not lost.

```sql
-- Migration: stage_notes TEXT -> JSONB
-- Wrap existing text data, preserve NULLs
ALTER TABLE tasks
  ALTER COLUMN stage_notes TYPE JSONB
  USING CASE
    WHEN stage_notes IS NOT NULL AND stage_notes != ''
    THEN jsonb_build_object('legacy_text', stage_notes, 'schema_version', 0)
    ELSE NULL
  END;
```

**Confidence:** HIGH -- follows identical pattern from Phase 6 blocked_by migration. PostgreSQL ALTER...USING is well-documented for type conversions with data transformation.

### Pattern 2: Base Schema + Optional Extensions
**What:** Every stage output has required base fields; stage-specific extensions validated only if present.
**Schema (from D-02):**

```json
{
  "schema_version": 1,
  "summary": "string (min 50 chars)",
  "key_outputs": [
    {"name": "string", "content": "string"}
  ],
  "issues": [
    {"severity": "critical|warning|info", "description": "string"}
  ],
  "metadata": {
    "stage": "string",
    "agent_id": "string",
    "timestamp": "ISO 8601 string",
    "duration_ms": 0
  }
}
```

**Recommendation (Claude's Discretion):** Include `schema_version: 1` in the base schema. Cost is one integer field. Benefit is future-proofing: version 0 = legacy wrapped text, version 1 = structured output. The transition-period handler checks this field to decide whether to parse structured or legacy data.

### Pattern 3: Lisp JSON Schema Validation
**What:** Replace keyword string search with hash-table key/type checking.
**Current approach (to be replaced):** 118 lines of `(search "keyword" content-lower)` pattern matching.
**New approach:** Parse JSON, check required keys exist and have correct types.

```lisp
;; Validate base schema fields from a parsed JSON hash-table
(defun validate-artifact-base (artifact)
  "Validate base schema. Returns (T . nil) or (NIL . reason)."
  (cond
    ((not (hash-table-p artifact))
     (cons nil "Stage output must be valid JSON object"))
    ((not (gethash :SUMMARY artifact))
     (cons nil "Missing required field: summary"))
    ((not (stringp (gethash :SUMMARY artifact)))
     (cons nil "Field 'summary' must be a string"))
    ((< (length (gethash :SUMMARY artifact)) 50)
     (cons nil "Field 'summary' must be at least 50 characters"))
    ((not (gethash :KEY-OUTPUTS artifact))
     (cons nil "Missing required field: key_outputs"))
    ((not (or (vectorp (gethash :KEY-OUTPUTS artifact))
              (listp (gethash :KEY-OUTPUTS artifact))))
     (cons nil "Field 'key_outputs' must be an array"))
    ((not (gethash :METADATA artifact))
     (cons nil "Missing required field: metadata"))
    ((not (hash-table-p (gethash :METADATA artifact)))
     (cons nil "Field 'metadata' must be an object"))
    (t (cons t nil))))
```

**Critical Lisp JSON quirk:** The parser converts underscores to hyphens in keys. So `key_outputs` becomes `:KEY-OUTPUTS`, `agent_id` becomes `:AGENT-ID`, `schema_version` becomes `:SCHEMA-VERSION`. All validation code must use hyphenated keyword symbols.

### Pattern 4: Rust JSONB Handling
**What:** Change stage_notes from `Option<String>` to `Option<serde_json::Value>` in sqlx queries.
**Current code reads:** `r.get::<Option<String>, _>("stage_notes")`
**New code reads:** `r.get::<Option<serde_json::Value>, _>("stage_notes")`

For the PATCH handler, the update query changes from binding a String to binding a `serde_json::Value`:
```rust
// In TaskUpdate struct:
pub stage_notes: Option<serde_json::Value>,  // was Option<String>

// In update handler:
if let Some(ref stage_notes) = body.stage_notes {
    sqlx::query("UPDATE tasks SET stage_notes = $1 WHERE id = $2")
        .bind(stage_notes).bind(id).execute(&pool).await
        .map_err(|e| ApiError::Database(e.to_string()))?;
}
```

**Confidence:** HIGH -- sqlx 0.8 natively maps `serde_json::Value` to PostgreSQL JSONB.

### Pattern 5: Predecessor Context Injection
**What:** Replace `load-previous-stage-output` (disk file reader) with DB query for predecessor's stage_notes.
**Current flow:** action-planner looks up `~/gotcha-workspace/tools/{tool}/{STAGE}.md` files.
**New flow:** Query tasks with same `goal_id` and the predecessor stage, extract stage_notes JSONB.

```lisp
(defun load-predecessor-stage-output (goal-id current-stage)
  "Load structured output from the previous pipeline stage via DB query."
  (let* ((prev-stage-map '(("infra-review" . "spec")
                            ("design" . "infra-review")
                            ("build" . "design")
                            ("security-review" . "build")
                            ("test" . "security-review")
                            ("deploy" . "test")
                            ;; Investment pipeline
                            ("research" . "thesis")
                            ("analysis" . "research")
                            ("compliance" . "analysis")
                            ("documentation" . "compliance")
                            ("approval" . "documentation")))
         (prev-stage (cdr (assoc current-stage prev-stage-map :test #'string-equal))))
    (when (and prev-stage goal-id)
      (handler-case
          (let ((tasks (api-get (format nil "/api/af64/tasks?goal_id=~a&limit=50" goal-id))))
            (when tasks
              (let ((task-list (if (vectorp tasks) (coerce tasks 'list) tasks)))
                (dolist (t task-list)
                  (when (and (hash-table-p t)
                             (string-equal (or (gethash :STAGE t) "") prev-stage)
                             (gethash :STAGE-NOTES t))
                    (return (gethash :STAGE-NOTES t)))))))
        (error () nil)))))
```

### Pattern 6: Structured Output Construction in advance-pipeline
**What:** When `advance-pipeline` stores stage_notes, construct the structured JSON artifact instead of truncating raw content.
**Current code:** `(subseq content 0 (min 2000 (length content)))` -- just truncates raw text.
**New code:** Build a proper artifact JSON object from the content.

```lisp
(defun build-stage-artifact (stage agent-id content)
  "Build structured artifact JSON from stage output content."
  (let ((summary (subseq content 0 (min 500 (length content)))))
    (json-object
     :schema-version 1
     :summary summary
     :key-outputs (json-array
                   (json-object :name "stage_output"
                                :content (subseq content 0 (min 4000 (length content)))))
     :issues (json-array)
     :metadata (json-object
                :stage stage
                :agent-id agent-id
                :timestamp (format-iso-timestamp)
                :duration-ms 0))))
```

**Note:** The Lisp `encode-json` converts `:schema-version` to `"schema_version"` in the output (hyphens become underscores via `keyword->json-key`). This means the JSON stored in DB will have correct underscore keys, but when parsed back by Lisp it will be accessed with hyphenated keywords.

### Recommended Project Structure (changes only)
```
/opt/dpn-api/src/handlers/
  af64_tasks.rs          # Change stage_notes type: String -> serde_json::Value
  af64_perception.rs     # Change stage_notes type: String -> serde_json::Value

/opt/project-noosphere-ghosts/lisp/runtime/
  action-executor.lisp   # Rewrite validate-stage-output for JSON schema validation
                         # Update advance-pipeline to build structured artifacts
                         # Update execute-work-task stage_notes writes
  action-planner.lisp    # Replace load-previous-stage-output with DB query
                         # Update build-pipeline-task-job to inject structured context

/root/.planning/phases/07-structured-artifact-passing/migrations/
  001_stage_notes_jsonb_migration.sql  # ALTER stage_notes TEXT -> JSONB
```

### Anti-Patterns to Avoid
- **Sending raw strings as JSONB:** After migration, the Rust PATCH handler must accept `serde_json::Value` not `String`. Sending a bare string to a JSONB column will fail or store it as a JSON string literal.
- **Assuming Lisp hash-table keys are lowercase:** The Lisp JSON parser converts `key_outputs` to `:KEY-OUTPUTS`. All validation must use uppercase hyphenated keywords.
- **Validating before parsing:** Do not try to validate JSON structure on the raw string. Parse first with `parse-json`, then validate the resulting hash-table.
- **Breaking the Lisp encode/decode roundtrip:** `json-object` uses keywords like `:stage-notes` which `encode-json` converts to `"stage_notes"`. When parsed back, `"stage_notes"` becomes `:STAGE-NOTES`. Code must be consistent about which representation it uses.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JSON schema validation library | Full JSONSchema spec validator in Lisp | Simple key/type checking functions | Only 4 base fields to validate; a full JSONSchema library is overkill for this use case |
| JSONB migration tooling | Custom migration runner | Raw SQL in `.sql` file (same as Phase 6) | PostgreSQL ALTER...USING handles the conversion natively |
| JSON serialization | Custom serializer | Existing `json.lisp` encode-json/parse-json | Already battle-tested across the codebase |

**Key insight:** The schema is simple enough (4 base fields, simple types) that hand-written validation functions are cleaner and more maintainable than importing a JSON Schema library into Common Lisp.

## Common Pitfalls

### Pitfall 1: Lisp Keyword Case Mismatch
**What goes wrong:** Validation checks `:key-outputs` but the parsed JSON has `:KEY-OUTPUTS`.
**Why it happens:** `json-keyword` in json.lisp upcases and interns: `(intern (string-upcase (substitute #\- #\_ string)) :keyword)`.
**How to avoid:** Always use uppercase keywords in hash-table lookups: `(gethash :KEY-OUTPUTS artifact)`, not `(gethash :key-outputs artifact)`.
**Warning signs:** Validation always fails with "missing required field" even when the field is present.

### Pitfall 2: Rust String/Value Type Mismatch After Migration
**What goes wrong:** After migrating to JSONB, `r.get::<Option<String>, _>("stage_notes")` throws a runtime error because sqlx expects String but gets JSONB.
**Why it happens:** PostgreSQL JSONB and TEXT are different types; sqlx type-checks at query time.
**How to avoid:** Change ALL sqlx reads from `Option<String>` to `Option<serde_json::Value>` in both `af64_tasks.rs` and `af64_perception.rs`. Search for every occurrence.
**Warning signs:** API returns 500 errors on any task query after migration.

### Pitfall 3: Lisp api-patch Sends Encoded JSON String for stage_notes
**What goes wrong:** `api-patch` calls `encode-json` on the payload. If stage_notes value is already a string, it gets double-encoded: `"stage_notes": "\"the content\""`.
**Why it happens:** Currently `advance-pipeline` passes `(subseq content 0 (min 2000 (length content)))` as stage-notes value -- a plain string. After this phase, stage_notes must be a hash-table (JSON object) that `encode-json` serializes properly.
**How to avoid:** Always pass a hash-table (from `json-object`) as the stage-notes value, never a raw string. The `encode-json` function handles hash-tables correctly.
**Warning signs:** stage_notes in DB contains escaped string literals instead of JSON objects.

### Pitfall 4: Transition Period -- Legacy Tasks Break Perception
**What goes wrong:** Existing 374 tasks have `{"legacy_text": "...", "schema_version": 0}` after migration. Code that expects structured artifacts (schema_version 1) fails on these.
**Why it happens:** Migration wraps old data but doesn't restructure it.
**How to avoid:** All code that reads stage_notes must check `schema_version`. If 0, treat as legacy (extract `legacy_text` for display). If 1, parse as structured artifact. If null/missing, treat as empty.
**Warning signs:** Older tasks cause crashes or display garbled JSON wrapper in prompts.

### Pitfall 5: stage_notes Truncation in Multiple Places
**What goes wrong:** Content gets truncated to 2000 chars (advance-pipeline) or 4000 chars (execute-work-task) BEFORE structured artifact is built.
**Why it happens:** Current code truncates early for safety. After this phase, the structured artifact must be built first, then the entire artifact stored.
**How to avoid:** Build the artifact first, then truncate `key_outputs[].content` within the artifact if needed. Never truncate the outer JSON structure.
**Warning signs:** Malformed JSON in stage_notes because truncation cut mid-field.

### Pitfall 6: Multiple stage_notes Writes Per Tick
**What goes wrong:** `execute-work-task` writes stage_notes at least twice: once at entry (line 358-360) as raw content, and again after tool execution (lines 374-376). Then `advance-pipeline` writes it again (line 209). The last write wins.
**Why it happens:** Current code progressively appends tool results to stage_notes.
**How to avoid:** Restructure to build the artifact once after all processing is complete, then write it in a single PATCH call. The intermediate "in-progress" writes can remain as raw text (schema_version 0); only the final validated write needs to be structured.
**Warning signs:** Structured artifact gets overwritten by raw text from an earlier write in the same execution flow.

## Code Examples

### Example 1: Complete validate-stage-output Replacement

```lisp
;; Source: analysis of current validate-stage-output (action-executor.lisp lines 78-195)
(defun validate-stage-output (stage content &optional (tools-executed 0))
  "Validate that stage output meets schema requirements.
   Returns (T . artifact) if valid, (NIL . reason) if rejected."
  (handler-case
      (let ((artifact (parse-json content)))
        ;; Must be a JSON object
        (unless (hash-table-p artifact)
          (return-from validate-stage-output
            (cons nil "Output must be a valid JSON object with base schema fields.")))
        ;; Validate base schema
        (let ((base-result (validate-artifact-base artifact)))
          (unless (car base-result)
            (return-from validate-stage-output base-result)))
        ;; Tool-execution check for non-engineering stages (preserved from current logic)
        (when (member stage '("thesis" "research" "analysis" "compliance"
                              "documentation" "approval" "collection" "curation"
                              "composition" "editing" "polish" "publish"
                              "discovery" "pattern-analysis" "architecture-research"
                              "security-audit" "synthesis" "tool-audit"
                              "module-standards" "security-standards")
                     :test #'string-equal)
          (when (= tools-executed 0)
            (return-from validate-stage-output
              (cons nil (format nil "~a stage: 0 tools executed. Tool calls required." stage)))))
        ;; Passed all checks -- return artifact for storage
        (cons t artifact))
    ;; JSON parse error
    (error (e)
      (cons nil (format nil "Invalid JSON: ~a. Output must be a JSON object with summary, key_outputs, issues, metadata fields." e)))))
```

### Example 2: Rust Type Change Pattern

```rust
// Source: af64_tasks.rs current code + JSONB migration requirements
// In TaskUpdate struct:
#[derive(Deserialize)]
pub struct TaskUpdate {
    // ... other fields unchanged ...
    pub stage_notes: Option<serde_json::Value>,  // Changed from Option<String>
}

// In list_tasks and perception handlers:
// Before: "stage_notes": r.get::<Option<String>, _>("stage_notes"),
// After:
"stage_notes": r.get::<Option<serde_json::Value>, _>("stage_notes"),
```

### Example 3: Predecessor Context in LLM Prompt

```lisp
;; Source: action-planner.lisp build-pipeline-task-job analysis
;; Replace prev-output binding (currently from load-previous-stage-output)
(let* ((prev-output (load-predecessor-stage-output
                     (gethash :goal-id task) stage))
       ;; Extract useful context from structured artifact
       (prev-context (when (and prev-output (hash-table-p prev-output))
                       (let ((ver (gethash :SCHEMA-VERSION prev-output)))
                         (if (and ver (>= ver 1))
                             ;; Structured: extract summary + key outputs
                             (format nil "Previous stage summary: ~a~%~%Key outputs:~%~{- ~a: ~a~%~}"
                                     (gethash :SUMMARY prev-output)
                                     (let ((outputs (gethash :KEY-OUTPUTS prev-output)))
                                       (when (vectorp outputs)
                                         (loop for o across outputs
                                               append (list (gethash :NAME o)
                                                            (gethash :CONTENT o))))))
                             ;; Legacy: use raw text
                             (or (gethash :LEGACY-TEXT prev-output) ""))))))
  ;; ... inject prev-context into system prompt ...
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Disk file loading (SPEC.md, ARCHITECTURE.md) | DB-stored stage_notes | Phase 4 (v1.0) | Stage output in DB but as freeform text |
| Keyword/length validation | Keyword/length validation | Phase 4 (v1.0) | Works but brittle, no schema enforcement |
| TEXT column for stage_notes | TEXT column | Original schema | About to change to JSONB in this phase |

**Current state:** stage_notes is TEXT, validation is keyword-based, predecessor context loads from disk files that may not exist. This phase upgrades all three.

## Open Questions

1. **Final deliverable persistence (D-07)**
   - What we know: Final pipeline results must go to appropriate DB tables (documents, vault_notes, decisions)
   - What's unclear: The exact mapping of pipeline type to target table, and whether this should happen inside `advance-pipeline` when `next-stage = "done"`, or as a separate post-pipeline step
   - Recommendation: Handle in `advance-pipeline` when `next-stage = "done"`. Map: engineering pipeline -> vault_notes, editorial pipeline -> documents, investment pipeline -> documents, modular fortress -> vault_notes. This can be a follow-up task within the phase.

2. **Editorial/investment pipeline stage extensions**
   - What we know: D-03 says extensions are optional and stage-specific
   - What's unclear: What specific extension fields each non-engineering stage might need
   - Recommendation: Start with base schema only for all stages. Extensions can be added later per-stage as ghosts actually produce structured output. This avoids over-engineering.

3. **Intermediate stage_notes writes during execution**
   - What we know: execute-work-task writes stage_notes multiple times (raw content, then with tool results)
   - What's unclear: Whether intermediate writes should also be structured JSON
   - Recommendation: Intermediate writes during execution can remain raw text or use `{"schema_version": 0, "legacy_text": "in-progress..."}`. Only the final validated write (after `validate-stage-output` passes) should be the full structured artifact.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual E2E testing via psql + curl + API calls |
| Config file | None -- this is Lisp + Rust + SQL, no unit test framework in use for AF64 |
| Quick run command | `curl -s http://localhost:8080/api/af64/tasks?limit=5 \| jq .` |
| Full suite command | `cd /root/dpn-core && cargo test` (Rust only) |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| ART-01 | stage_notes contains structured JSON matching schema | integration | `psql -c "SELECT stage_notes FROM tasks WHERE stage = 'spec' AND stage_notes IS NOT NULL LIMIT 1;" \| jq .` | N/A -- DB query |
| ART-02 | Next assignee perception includes predecessor structured output | integration | `curl -s http://localhost:8080/api/perception/{agent_id} \| jq '.tasks[0].stage_notes'` | N/A -- API call |
| ART-03 | Executor rejects non-schema output | smoke | Test by sending raw text to a pipeline task and confirming rejection in PM2 logs | Manual -- requires ghost tick |

### Sampling Rate
- **Per task commit:** `curl -s http://localhost:8080/api/af64/tasks?limit=3 | jq '.[0].stage_notes'` (verify JSONB returns correctly)
- **Per wave merge:** `cd /root/dpn-core && cargo build` + `curl` perception endpoint
- **Phase gate:** Full pipeline test: create a goal task with spec stage, observe structured artifact in stage_notes after completion

### Wave 0 Gaps
- [ ] Migration SQL file: `migrations/001_stage_notes_jsonb_migration.sql`
- [ ] No Lisp unit tests exist for validate-stage-output -- testing is via live ghost execution

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` -- Current validate-stage-output (lines 78-195), advance-pipeline (lines 197-318), execute-work-task (lines 351-481)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` -- build-pipeline-task-job (lines 310-397), load-previous-stage-output (lines 296-308)
- `/opt/project-noosphere-ghosts/lisp/util/json.lisp` -- JSON utilities: json-object, json-array, parse-json, encode-json, keyword->json-key
- `/opt/dpn-api/src/handlers/af64_tasks.rs` -- Task CRUD handlers, stage_notes as Option<String>
- `/opt/dpn-api/src/handlers/af64_perception.rs` -- Perception task serialization, stage_notes as Option<String>
- PostgreSQL `\d tasks` output -- Current schema showing stage_notes as TEXT
- DB query: 374 tasks with existing stage_notes data, 2165 with NULL

### Secondary (MEDIUM confidence)
- Phase 6 migration pattern (`001_blocked_by_array_migration.sql`) -- Verified ALTER...USING pattern works
- sqlx 0.8 documentation -- serde_json::Value maps to JSONB (verified via existing codebase usage of JSONB for blocked_by)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- No new dependencies, all existing tools sufficient
- Architecture: HIGH -- All integration points inspected, code patterns verified in source
- Pitfalls: HIGH -- Identified from direct code reading, especially the Lisp JSON keyword quirk and multi-write pattern

**Research date:** 2026-03-26
**Valid until:** 2026-04-26 (stable -- internal codebase, no external API changes expected)
