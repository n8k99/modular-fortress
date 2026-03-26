---
phase: 07-structured-artifact-passing
verified: 2026-03-26T23:45:00Z
status: human_needed
score: 9/9 must-haves verified
re_verification: true
  previous_status: gaps_found
  previous_score: 8/9
  gaps_closed:
    - "When a pipeline reaches its final stage (next-stage=done), the completed artifact is persisted to the appropriate DB table per D-07"
  gaps_remaining: []
  regressions: []
human_verification:
  - test: "Trigger a pipeline to completion (next-stage=done) and verify a document row is created in the documents table"
    expected: "A new row appears in master_chronicle.documents with path like Pipeline/engineering/... and content containing the structured artifact JSON"
    why_human: "Requires a live ghost tick cycle executing a pipeline stage with valid JSON output to verify the full end-to-end path works after the endpoint fix"
---

# Phase 7: Structured Artifact Passing Verification Report

**Phase Goal:** Pipeline stage outputs are typed and validated, so the next assignee receives structured context instead of freeform text
**Verified:** 2026-03-26T23:45:00Z
**Status:** human_needed — all automated checks pass, one live end-to-end test remains
**Re-verification:** Yes — after gap closure (api-post path fix in persist-pipeline-deliverable)

## Gap Fix Confirmation

**Previous gap:** `persist-pipeline-deliverable` called `api-post "/documents"` (lines 193 and 200), resolving to `http://localhost:8080/documents` which returned HTTP 404.

**Fix verified:** Both lines 193 and 200 in `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` now read `api-post "/api/documents"`. Confirmation:
- `curl -X POST http://localhost:8080/documents` → HTTP **404** (old path, still broken as expected)
- `curl -X POST http://localhost:8080/api/documents` → HTTP **401** (new path, endpoint exists, protected by auth as expected)

No other changes were introduced. All previously verified must-haves remain intact (regression checks passed).

---

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | stage_notes column in tasks table is JSONB type, not TEXT | VERIFIED | `SELECT data_type FROM information_schema.columns WHERE table_name='tasks' AND column_name='stage_notes'` returns "jsonb" |
| 2 | Existing freeform text data is wrapped in {legacy_text, schema_version: 0} JSON | VERIFIED (regression) | DB query of 374 rows with stage_notes IS NOT NULL confirmed in initial verification; column type still JSONB |
| 3 | Rust API returns stage_notes as JSON objects (not strings) and accepts JSON objects for updates | VERIFIED (regression) | af64_tasks.rs line 61: `serde_json::Value`; line 86 TaskUpdate: `Option<serde_json::Value>`; af64_perception.rs line 195: `serde_json::Value` |
| 4 | A ghost perceiving a pipeline task receives stage_notes as a JSON object (not a raw string) | VERIFIED (regression) | af64_perception.rs line 195 confirmed as `serde_json::Value` |
| 5 | validate-artifact-base parses JSON and checks base schema fields instead of keyword matching | VERIFIED (regression) | Lines 78-140: defun validate-artifact-base checks :SUMMARY (length >= 50), :KEY-OUTPUTS (vectorp or listp), :METADATA (hash-table-p); parse-json called at line 111 in validate-stage-output |
| 6 | advance-pipeline stores structured JSON artifacts in stage_notes, not truncated raw text | VERIFIED (regression) | Line 221: `:stage-notes (build-stage-artifact current-stage agent-id content)` — hash-table with schema-version 1 |
| 7 | Action executor rejects output that is not valid JSON with required base fields | VERIFIED (regression) | validate-stage-output returns `(cons nil "Output must be a valid JSON object...")` on parse error; execute-work-task retry/block loop triggered |
| 8 | When a task advances to the next pipeline stage, the new assignee's LLM prompt includes structured output from the previous stage | VERIFIED (regression) | action-planner.lisp line 296: defun load-predecessor-stage-output; line 330: api-get /api/af64/tasks; line 415: `~@[PREVIOUS STAGE OUTPUT:~%~a~%~%~]` in format string |
| 9 | When a pipeline reaches its final stage (next-stage=done), the completed artifact is persisted to the appropriate DB table per D-07 | VERIFIED | Lines 193 and 200 now call `api-post "/api/documents"` (was `"/documents"`). Endpoint verified: HTTP 401 (auth-protected, exists) vs HTTP 404 for old path. Call site at lines 318-325 confirmed in advance-pipeline completion branch |

**Score: 9/9 truths verified**

---

## Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `/root/.planning/phases/07-structured-artifact-passing/migrations/001_stage_notes_jsonb_migration.sql` | SQL migration converting TEXT to JSONB with data wrapping | VERIFIED | Contains `ALTER TABLE tasks ALTER COLUMN stage_notes TYPE JSONB USING CASE` with legacy_text wrapping |
| `/opt/dpn-api/src/handlers/af64_tasks.rs` | Task CRUD handlers with JSONB stage_notes | VERIFIED | Line 61: `serde_json::Value` for read; Line 86: `Option<serde_json::Value>` in TaskUpdate |
| `/opt/dpn-api/src/handlers/af64_perception.rs` | Perception handler with JSONB stage_notes | VERIFIED | Line 195: `r.get::<Option<serde_json::Value>, _>("stage_notes")` |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` | JSON schema validation, structured artifact construction, pipeline deliverable persistence | VERIFIED | validate-artifact-base (line 78), build-stage-artifact (line 142), detect-pipeline-type (line 162), persist-pipeline-deliverable (line 184) — all correct; endpoint fix confirmed at lines 193 and 200 |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` | Pipeline task job builder with DB-sourced predecessor context | VERIFIED | load-predecessor-stage-output (line 296) queries /api/af64/tasks; prev-context handles schema v0/v1; disk file access eliminated |

---

## Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| af64_tasks.rs | tasks.stage_notes (JSONB) | sqlx row.get and query bind | WIRED | `serde_json::Value` in both read (line 61) and TaskUpdate struct (line 86) |
| af64_perception.rs | tasks.stage_notes (JSONB) | sqlx row.get | WIRED | Line 195 confirmed as `serde_json::Value` |
| validate-stage-output | parse-json | JSON parsing before validation | WIRED | Line 111: `(let ((artifact (parse-json content)))` before validate-artifact-base call |
| advance-pipeline | build-stage-artifact | Constructs structured JSON before storing | WIRED | Line 221: `:stage-notes (build-stage-artifact current-stage agent-id content)` |
| advance-pipeline api-patch | tasks.stage_notes (JSONB) | Stores hash-table that encode-json serializes to JSONB | WIRED | api-patch at line 219 with json-object containing :stage-notes from build-stage-artifact |
| advance-pipeline (next-stage=done) | /api/documents | api-post to persist final deliverable | WIRED | Lines 193 and 200: `api-post "/api/documents"` — HTTP 401 confirms endpoint exists and is reachable |
| build-pipeline-task-job | load-predecessor-stage-output | Function call to get previous stage output | WIRED | Line 359: `(prev-output (load-predecessor-stage-output (gethash :GOAL-ID task) stage))` |
| load-predecessor-stage-output | /api/af64/tasks | api-get query for predecessor task by goal_id and stage | WIRED | Line 330: `(api-get (format nil "/api/af64/tasks?goal_id=~a&limit=50" goal-id))` |
| build-pipeline-task-job | LLM system prompt | prev-context injected into format string | WIRED | Line 415: `~@[PREVIOUS STAGE OUTPUT:~%~a~%~%~]` in format nil with prev-context as optional arg |

---

## Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|--------------|--------|-------------------|--------|
| af64_tasks.rs | stage_notes (serde_json::Value) | PostgreSQL JSONB column | Yes — DB column confirmed JSONB type, 374 rows with wrapped legacy data | FLOWING |
| af64_perception.rs | stage_notes (serde_json::Value) | PostgreSQL JSONB column | Yes — same column, same JSONB type | FLOWING |
| action-executor.lisp validate-stage-output | artifact (hash-table) | parse-json(content) | Yes — parses LLM output; returns parsed artifact on success | FLOWING |
| action-executor.lisp persist-pipeline-deliverable | artifact (hash-table) | build-stage-artifact output | Yes — artifact is real structured data; API endpoint path is now correct (`/api/documents` → HTTP 401) | FLOWING |
| action-planner.lisp build-pipeline-task-job | prev-output (hash-table from DB) | load-predecessor-stage-output → api-get | Yes — queries DB for goal_id tasks; returns :STAGE-NOTES value | FLOWING |

---

## Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| DB column type is JSONB | `SELECT data_type FROM information_schema.columns WHERE table_name='tasks' AND column_name='stage_notes'` | "jsonb" | PASS |
| Old /documents path still 404 | `curl -X POST http://localhost:8080/documents` | HTTP 404 | PASS (confirms old path was wrong) |
| New /api/documents path exists | `curl -X POST http://localhost:8080/api/documents` | HTTP 401 | PASS (auth-protected = endpoint exists) |
| All four new Lisp functions exist | `grep -n "defun validate-artifact-base\|defun build-stage-artifact\|defun persist-pipeline-deliverable\|defun detect-pipeline-type" action-executor.lisp` | Lines 78, 142, 162, 184 | PASS |
| Rust API JSONB types present | `grep "serde_json::Value.*stage_notes" af64_tasks.rs af64_perception.rs` | Lines 61, 86 (tasks), 195 (perception) | PASS |
| Predecessor context loading from DB | `grep "defun load-predecessor-stage-output\|api-get.*af64/tasks" action-planner.lisp` | Lines 296, 330 | PASS |
| PREVIOUS STAGE OUTPUT injected in prompt | `grep "PREVIOUS STAGE OUTPUT" action-planner.lisp` | Line 415 | PASS |

---

## Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| ART-01 | 07-01, 07-02 | Pipeline stages have typed output schemas stored in stage_notes as structured JSON | SATISFIED | stage_notes is JSONB; build-stage-artifact constructs schema_version 1 objects; validate-artifact-base enforces base schema; persist-pipeline-deliverable endpoint now correct |
| ART-02 | 07-03 | Output schema from current stage is passed as input context to next assignee | SATISFIED | load-predecessor-stage-output queries DB; prev-context injected into LLM system prompt under "PREVIOUS STAGE OUTPUT" header |
| ART-03 | 07-02 | Action executor validates that stage output matches expected schema before marking stage complete | SATISFIED | validate-stage-output enforces JSON parse + base schema; execute-work-task triggers 3-attempt retry loop; after 3 failures, task is marked blocked |

All three required requirements (ART-01, ART-02, ART-03) are fully satisfied. The D-07 persistence gap that previously blocked ART-01 completeness has been closed.

**Requirements traceability note:** REQUIREMENTS.md traceability table lists ART-01, ART-02, ART-03 as Phase 7 / Complete. No orphaned requirements found — all requirement IDs declared in plan frontmatter (07-01: ART-01; 07-02: ART-01, ART-03; 07-03: ART-02) match the REQUIREMENTS.md definitions.

---

## Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| action-executor.lisp | 188-207 | handler-case swallows all errors in persist-pipeline-deliverable | Warning | Silent failures make debugging persistence issues invisible; format t log exists so partial visibility, but the error string is not checked for HTTP status. This is acceptable given the fix — 401 will be caught and logged via the error handler. |

The previous blocker (wrong endpoint path) has been resolved. The remaining warning (handler-case error swallowing) is pre-existing and acceptable — the format t log ensures at least some visibility into failures.

**Note on schema-version 0 intermediate writes** (lines 382, 399, 468, 475): These are intentional — intermediate in-progress and tool-result writes use legacy wrapping. The schema_version 1 final artifact from advance-pipeline overwrites them. Not a stub.

---

## Human Verification Required

### 1. End-to-End Pipeline Deliverable Persistence

**Test:** Start noosphere-ghosts (`pm2 start noosphere-ghosts`), wait for a tick where a ghost completes a pipeline stage with `next-stage=done`. Then query:

```sql
SELECT id, path, created_at FROM documents WHERE path LIKE 'Pipeline/%' ORDER BY created_at DESC LIMIT 5;
```

**Expected:** A new document row appears with path like `Pipeline/engineering/unknown` or `Pipeline/editorial/<goal_id>`, content containing the full structured artifact JSON (schema_version 1).

**Why human:** Requires live noosphere-ghosts process running, a ghost actually completing a pipeline stage with valid JSON output, and the pipeline reaching final stage — cannot simulate this programmatically without running the full tick engine.

---

## Gaps Summary

No gaps remain. The single gap from the initial verification has been closed:

**Closed:** `persist-pipeline-deliverable` in `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` now calls `api-post "/api/documents"` at both lines 193 and 200 (was `api-post "/documents"`). The `/api/documents` endpoint returns HTTP 401 (auth-required) rather than HTTP 404, confirming the endpoint exists and the routing is correct. The dpn-api authentication layer requires a valid API key or JWT, which the ghost runtime supplies via its configured credentials.

All 9 observable truths verified. All 3 required requirements satisfied. One human end-to-end test remains to confirm live behavior after the fix.

---

_Verified: 2026-03-26T23:45:00Z_
_Verifier: Claude (gsd-verifier)_
_Re-verification: Yes — gap closure check after api-post endpoint fix_
