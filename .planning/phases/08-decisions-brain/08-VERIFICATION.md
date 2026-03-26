---
phase: 08-decisions-brain
verified: 2026-03-26T23:34:34Z
status: passed
score: 5/5 must-haves verified
re_verification: false
---

# Phase 8: Decisions Brain Verification Report

**Phase Goal:** Executives have shared memory of project decisions so they act consistently and don't contradict prior choices
**Verified:** 2026-03-26T23:34:34Z
**Status:** PASSED
**Re-verification:** No — initial verification

---

## Goal Achievement

### Observable Truths (from ROADMAP Success Criteria)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | When an executive reviews a project, the LLM prompt includes recent decisions from the decisions table | VERIFIED | `action-planner.lisp` line 791-810: `decisions-context` binding fetches last 10 decisions via `api-get "/api/decisions"` and injects into 3-arg format string at line 834 |
| 2 | When an executive makes a decision during project review, it appears in decisions table with project_id and agent attribution | VERIFIED | `action-executor.lisp` lines 1067-1132: `extract-decision-lines` parses DECISION: prefixed lines; each POSTed to `/api/decisions` with owner=agent-id, project-id, department from cognition-result-metadata |
| 3 | GET /api/decisions?project_id=X returns all decisions for that project in chronological order | VERIFIED | `decisions.rs` lines 34-43: parameterized SQL with `WHERE project_id = $1 ORDER BY created_at {ASC|DESC} LIMIT $2`; live endpoint returned HTTP 200 with JSON array in round-trip test |

**Score: 3/3 success criteria verified**

### Additional Must-Have Truths (from PLAN frontmatter)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 4 | POST /api/decisions creates a new decision record and returns its id | VERIFIED | `decisions.rs` lines 112-141: INSERT ... RETURNING id, created_at; live test returned `{"id":5,"created_at":"2026-03-26T23:33:51.279815Z"}` |
| 5 | GET /api/decisions?department=X returns department-scoped decisions | VERIFIED | `decisions.rs` lines 45-55: `WHERE department = $1`; live test returned HTTP 200 with JSON array |
| 6 | Decisions table has a department column and project_id index | VERIFIED | DB schema confirmed: `department VARCHAR(256)`, `idx_decisions_department` btree, `idx_decisions_project_id` btree |
| 7 | Multiple DECISION: lines in one LLM response each create separate DB records | VERIFIED | `action-executor.lisp` lines 1117-1132: `dolist (d decision-lines)` iterates every extracted line and POSTs each individually |
| 8 | Existing memory/vault_notes logging is preserved alongside new DB persistence | VERIFIED | `api-put "/api/agents/memory" payload` at line 1133 and `write-vault-note-memory` at line 1135 both preserved; `has-decision` block at lines 1109-1110 unchanged |

**Overall Score: 5/5 must-have areas verified**

---

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `/opt/dpn-api/src/handlers/decisions.rs` | GET and POST handlers for decisions CRUD | VERIFIED | 142 lines; exports `list_decisions` and `create_decision`; full DB query implementation, no stubs |
| `/opt/dpn-api/src/handlers/mod.rs` | Module registration for decisions handler | VERIFIED | Line 23: `pub mod decisions;` present |
| `/opt/dpn-api/src/main.rs` | Route registration behind auth middleware | VERIFIED | Line 145: `.route("/decisions", get(decisions::list_decisions).post(decisions::create_decision))` in protected_routes; line 20: `decisions` in handler use statement |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` | Decision detection that POSTs to /api/decisions | VERIFIED | Lines 1067-1085: `extract-decision-lines` function; lines 1115-1133: api-post block in `write-agent-daily-memory` |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` | Decision context injection in project review prompt | VERIFIED | Lines 791-810: `decisions-context` binding with `api-get "/api/decisions"`; line 834: injected as 3rd arg in format string |

---

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `/opt/dpn-api/src/main.rs` | `/opt/dpn-api/src/handlers/decisions.rs` | route registration | WIRED | Line 145: `.route("/decisions", ...)` with both handlers |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` | `/api/decisions` | `api-post` call on DECISION: detection | WIRED | Line 1124: `(api-post "/api/decisions" ...)` inside `dolist` over `extract-decision-lines` results |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` | `/api/decisions` | `api-get` call in `build-project-review-job` | WIRED | Line 794: `(api-get "/api/decisions" (list :project-id pid :limit 10 :order "desc"))` |

---

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| `decisions.rs` list_decisions | `rows` (Vec of DB rows) | `sqlx::query(...)FROM decisions WHERE...` | Yes — live DB query, no static fallback | FLOWING |
| `action-planner.lisp` review prompt | `decisions-context` | `api-get "/api/decisions"` -> decisions.rs -> DB | Yes — fetches from live DB; graceful empty-string fallback when no decisions exist | FLOWING |
| `action-executor.lisp` decision capture | `decision-lines` | `extract-decision-lines(content)` parses LLM output | Yes — POSTs each parsed line to DB; error handler preserves tick on API failure | FLOWING |

---

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| POST /api/decisions returns id | `curl -X POST .../api/decisions -d {...}` | HTTP 200, `{"id":5,"created_at":"..."}` | PASS |
| GET /api/decisions?project_id=1 returns array | `curl .../api/decisions?project_id=1` | HTTP 200, JSON array | PASS |
| GET /api/decisions?department=operations returns array | `curl .../api/decisions?department=operations` | HTTP 200, JSON array | PASS |
| Round-trip: POST then GET retrieves created record | POST id=5, GET project_id=1 count=1 | Count incremented, record visible | PASS |
| dpn-api online | `pm2 list | grep dpn-api` | online, 0 restarts in session | PASS |
| No PUT/DELETE endpoints (append-only) | grep for put/delete in decisions.rs | No matches | PASS |

---

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| DEC-01 | 08-02-PLAN.md | Executive project review prompt includes recent decisions from decisions table for the project being reviewed | SATISFIED | `action-planner.lisp` lines 791-834: decisions-context fetched and injected into every `build-project-review-job` prompt |
| DEC-02 | 08-02-PLAN.md | When an executive makes a decision during project review, it is logged to the decisions table with project_id and agent attribution | SATISFIED | `action-executor.lisp` lines 1067-1132: DECISION: prefix parsed, each line POSTed with owner=agent-id, project-id from metadata |
| DEC-03 | 08-01-PLAN.md | Decisions are queryable via /api/decisions with project_id filter | SATISFIED | `decisions.rs` GET handler with project_id WHERE clause; live endpoint confirmed HTTP 200 |

All 3 phase requirements (DEC-01, DEC-02, DEC-03) are SATISFIED. No orphaned requirements found — REQUIREMENTS.md maps exactly DEC-01/DEC-02/DEC-03 to Phase 8, matching the plan frontmatter declarations.

---

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| None found | — | — | — | — |

No TODOs, stubs, placeholder returns, or empty implementations found in any phase-modified file. The `extract-decision-lines` function handles edge cases (missing "because", empty content) gracefully. Both API call sites wrap errors with `handler-case` to prevent tick crashes.

---

### Human Verification Required

#### 1. End-to-End Executive Decision Flow

**Test:** Start `noosphere-ghosts` (`pm2 restart noosphere-ghosts`), wait for a tick where an executive ghost performs a project review. Inspect the `cognition_jobs` table or tick logs to confirm the review prompt contains a "### Recent Decisions" section when decisions exist for the project.
**Expected:** The LLM prompt sent to Claude includes the decisions context block with date, owner, and decision text.
**Why human:** Requires running ghosts + actual tick execution with an executive perceiving a project that has existing decisions in the DB. Cannot verify prompt content programmatically without running the full tick cycle.

#### 2. Multi-DECISION: line capture in one response

**Test:** Manually craft a mock LLM response with 3 `DECISION:` lines (including one with "because" rationale). Confirm 3 separate rows appear in the decisions table.
**Expected:** Exactly 3 new rows in decisions table, each with distinct `decision` text; the "because" line has `rationale` populated.
**Why human:** Requires simulating a cognition result flowing through `write-agent-daily-memory` — no test harness in place for the Lisp runtime without starting the full tick engine.

---

### Gaps Summary

No gaps. All automated checks passed at all four levels (exists, substantive, wired, data-flowing). Phase goal is achieved: executives have shared memory of project decisions via the decisions API, decision capture is wired in the tick engine executor, and decision context is injected into project review prompts. Two human verification items are noted for completeness but do not block the phase goal assessment.

---

_Verified: 2026-03-26T23:34:34Z_
_Verifier: Claude (gsd-verifier)_
