---
phase: 12-standing-orders
verified: 2026-03-28T04:15:00Z
status: passed
score: 9/9 must-haves verified
re_verification: false
---

# Phase 12: Standing Orders Verification Report

**Phase Goal:** Ghosts execute recurring project work on a cron schedule without manual dispatch
**Verified:** 2026-03-28T04:15:00Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Projects table has a schedule JSONB column with cron objects | VERIFIED | DB query: projects 10/12/14 have schedule_count 3/1/5 respectively |
| 2 | API PATCH /projects/:id can set schedule on a project | VERIFIED | `curl -X PATCH` returns 200 with schedule field in response |
| 3 | Perception endpoint returns schedule metadata for owned projects | VERIFIED | `/api/perception/kathryn` returns project with 3 schedule entries |
| 4 | Projects #10, #12, #14 have correct owners and schedules in the DB | VERIFIED | DB: kathryn/3, sylvia/1, nova/5 schedule entries confirmed |
| 5 | A Lisp cron matcher parses and evaluates standard 5-field cron expressions | VERIFIED | SBCL tests pass: wildcard, exact, range, list, step, dow-7 alias |
| 6 | When a project schedule matches the current tick time, the owning executive gets a cognition job | VERIFIED | `schedule-boost` (+50) in `phase-rank` calls `cron-matches-p` with UTC time and populates `*schedule-fired-labels*` |
| 7 | The cognition prompt includes the schedule label so the executive knows which standing order fired | VERIFIED | `build-project-review-job` reads `*schedule-fired-labels*` and appends "## Standing Orders Fired" section |
| 8 | Standing order execution produces conversation output attributed to the executive ghost, not system | VERIFIED | `execute-project-review` sets `:from-agent agent-id` (cognition-result-agent-id) on API post |
| 9 | Double-firing within the same cron minute window is prevented | VERIFIED | `*last-schedule-fire*` hash table tracks `pid:label` keys with 60-second deduplication window |

**Score:** 9/9 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `/opt/project-noosphere-ghosts/lisp/runtime/cron-matcher.lisp` | Cron expression parser and matcher | VERIFIED | 85-line implementation with `cron-matches-p`, `parse-cron-field`, `split-cron-string` |
| `/root/dpn-core/src/db/projects.rs` | Project struct with schedule field + update_project with schedule support | VERIFIED | `pub schedule: Option<serde_json::Value>` on struct; schedule in all SELECT queries; dynamic update_project |
| `/opt/dpn-api/src/handlers/projects.rs` | PATCH endpoint for project updates including schedule | VERIFIED | `UpdateProjectRequest` struct with schedule field; `update_project` handler registered |
| `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` | Schedule check in ranking phase with +50 boost | VERIFIED | `schedule-boost` in `phase-rank` at line 204; `clrhash` in `run-tick` at line 511 |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` | Schedule label in project review cognition prompt | VERIFIED | `schedule-context` reads `*schedule-fired-labels*` at line 847; appended to prompt at line 898 |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `/opt/dpn-api/src/handlers/af64_perception.rs` | projects table | `SELECT p.schedule` in projects query | VERIFIED | Line 425: `p.schedule` in SELECT; line 448: `"schedule": r.get::<Option<serde_json::Value>, _>("schedule")` |
| `/opt/dpn-api/src/handlers/projects.rs` | `/root/dpn-core/src/db/projects.rs` | `dpn_core::update_project` call with schedule param | VERIFIED | Line 92: `dpn_core::update_project(...)` with `req.schedule.as_ref()` at line 99 |
| `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` | `cron-matcher.lisp` | `cron-matches-p` call in ranking phase | VERIFIED | Line 222: `(cron-matches-p expr min hr date mon dow)` with UTC time via `decode-universal-time ... 0` |
| `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` | `action-planner.lisp` | `schedule-fired-labels` passed via exported var | VERIFIED | Line 235: `(setf (gethash aid *schedule-fired-labels*) fired-labels)`; action-planner reads via `af64.runtime.tick-engine:*schedule-fired-labels*` |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` | conversations table | Cognition result posted as executive agent conversation | VERIFIED | `execute-project-review` in action-executor.lisp line 927: `:from-agent agent-id` |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|-------------------|--------|
| `af64_perception.rs` (schedule in response) | `schedule` JSON field | `SELECT p.schedule FROM projects` | Yes — DB JSONB column with seeded cron arrays | FLOWING |
| `tick-engine.lisp` (schedule-boost) | `fired-labels` / `*schedule-fired-labels*` | `cron-matches-p` against live UTC time + perception schedule data | Yes — evaluates real schedule entries from perception | FLOWING |
| `action-planner.lisp` (schedule-context) | `fired` from `*schedule-fired-labels*` | Populated by tick-engine during ranking | Yes — real labels from matching schedules | FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| DB schedules seeded for projects 10/12/14 | `psql ... SELECT id, owner, jsonb_array_length(schedule)` | 10:kathryn:3, 12:sylvia:1, 14:nova:5 | PASS |
| API GET returns schedule field | `curl GET /api/projects/10` | schedule array with 3 entries present | PASS |
| API PATCH endpoint works | `curl -X PATCH /api/projects/10 {"status":"active"}` | 200 with schedule field in response | PASS |
| Perception includes schedule metadata | `curl GET /api/perception/kathryn` | Project Complete Success with 3 schedule entries | PASS |
| SBCL loads AF64 system cleanly | `sbcl --load test_af64_load.lisp` | "LOAD OK" (one benign redefine warning) | PASS |
| cron-matches-p functional tests | SBCL: weekday/sunday/step/list/Tokyo/dow-7 | "CRON TESTS PASSED" | PASS |
| tick-engine symbols exported | SBCL: `*schedule-fired-labels*`, `*last-schedule-fire*` | "TICK ENGINE SYMBOL TESTS PASSED" | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| STAND-01 | 12-01-PLAN.md | Projects table supports a schedule field that triggers ghost perception at scheduled times | SATISFIED | `schedule JSONB` column exists; perception endpoint returns it; 3 projects seeded |
| STAND-02 | 12-02-PLAN.md | Tick engine recognizes scheduled projects and creates cognition jobs for the owning executive at the scheduled time | SATISFIED | `schedule-boost` (+50) in `phase-rank` via `cron-matches-p`; owning executive enters acting-set |
| STAND-03 | 12-02-PLAN.md | Standing order execution produces conversation output attributed to the responsible ghost | SATISFIED | `execute-project-review` sets `:from-agent (cognition-result-agent-id result)` — the executive's agent-id, not a system account |

No orphaned requirements — all three STAND-0x IDs claimed by plans and verified in codebase.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `tick-engine.lisp` | 511 | `clrhash *schedule-fired-labels*` clears per-tick state but `*last-schedule-fire*` is never cleared | Info | Intentional — last-fire persists across ticks for deduplication. Not a stub. |

No blocking or warning-level anti-patterns found. All data paths are populated with real DB values. No `return null` / placeholder patterns in phase-modified files.

### Human Verification Required

#### 1. Live Schedule Fire Test

**Test:** Temporarily set a 1-minute schedule on project #14 matching current UTC time, start `pm2 start noosphere-ghosts`, watch `pm2 logs noosphere-ghosts` for `[schedule] nova: standing orders fired: ...`, then stop ghosts and restore schedule.
**Expected:** Within 2 tick cycles, Nova's urgency gets a +50 boost, she enters the acting-set, and a project review job fires with "## Standing Orders Fired" in the prompt.
**Why human:** Requires live ghost execution with a timed cron match — cannot verify tick-to-cognition flow without running the full system.

### Gaps Summary

No gaps found. All 9 observable truths are verified against the actual codebase. Schedule infrastructure (DB, API, perception, Lisp parser) and tick engine integration (ranking boost, deduplication, prompt enrichment, attribution) are fully wired end-to-end.

---

_Verified: 2026-03-28T04:15:00Z_
_Verifier: Claude (gsd-verifier)_
