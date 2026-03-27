---
phase: 10-lifecycle-signals
verified: 2026-03-27T00:52:00Z
status: gaps_found
score: 5/7 must-haves verified
re_verification: false
gaps:
  - truth: "list_agents API response includes metadata JSONB for each agent"
    status: failed
    reason: "sqlx 'json' feature missing from dpn-api Cargo.toml — r.get::<Option<Value>, _>(\"metadata\") returns null for all agents even when DB has content"
    artifacts:
      - path: "/opt/dpn-api/src/handlers/af64_agents.rs"
        issue: "metadata field is in SQL SELECT and in serde_json::json! block, but sqlx cannot decode JSONB to serde_json::Value without the 'json' feature"
      - path: "/opt/dpn-api/Cargo.toml"
        issue: "sqlx = { version = \"0.8\", features = [\"postgres\", \"runtime-tokio-native-tls\"] } — missing \"json\" feature"
    missing:
      - "Add \"json\" to sqlx features in /opt/dpn-api/Cargo.toml: features = [\"postgres\", \"runtime-tokio-native-tls\", \"json\"]"
      - "Rebuild and redeploy dpn-api (cargo build --release + pm2 restart dpn-api)"
  - truth: "PATCH /api/agents/:id/state with metadata merges into existing metadata (does not overwrite)"
    status: failed
    reason: "Same sqlx 'json' feature gap — .bind(metadata) silently does not execute the JSONB merge; API returns 200 OK but DB is unchanged"
    artifacts:
      - path: "/opt/dpn-api/src/handlers/af64_agents.rs"
        issue: "COALESCE merge SQL is correct but sqlx cannot bind serde_json::Value to $1::jsonb without the 'json' feature"
      - path: "/opt/dpn-api/Cargo.toml"
        issue: "Missing \"json\" sqlx feature prevents JSONB parameter binding"
    missing:
      - "Add \"json\" to sqlx features — same fix as above gap, same root cause"
  - truth: "After a staff ghost completes its last task and is classified idle, its agent_state.metadata contains lifecycle_state = idle"
    status: failed
    reason: "Depends on the PATCH merge working. The Lisp tick engine correctly calls api-patch with lifecycle_state in metadata, but the Rust handler silently fails to write it. The log shows '[lifecycle] eliana transitioning to idle' on EVERY tick — the one-time transition guard (prev-lifecycle check) never triggers because metadata stays {}"
    artifacts:
      - path: "/opt/dpn-api/Cargo.toml"
        issue: "Root cause: missing sqlx 'json' feature — PATCH metadata never persists"
    missing:
      - "Same fix: add 'json' to sqlx features — all three gap truths resolve from this one change"
---

# Phase 10: Lifecycle Signals Verification Report

**Phase Goal:** Executives know which staff are available for new work so they can delegate immediately instead of waiting for the next tick cycle
**Verified:** 2026-03-27T00:52:00Z
**Status:** gaps_found — one root-cause defect blocks three must-haves
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | list_agents API response includes metadata JSONB for each agent | FAILED | API returns `metadata: null` for all 64 agents even when DB has content — confirmed via curl + psql |
| 2 | PATCH /api/agents/:id/state with metadata merges (not overwrites) | FAILED | PATCH returns 200 OK but DB unchanged; confirmed with before/after psql queries |
| 3 | energy.lisp has an :idle-transition reward of +12 | VERIFIED | Line 27: `(setf (gethash :idle-transition table) 12)` — exact value in +energy-rewards+ hash |
| 4 | After idle classification, agent_state.metadata contains lifecycle_state = idle | FAILED | Depends on PATCH working — metadata stays `{}` after every tick; log shows agents re-triggering idle transition every tick |
| 5 | Executive project review shows team roster with IDLE/ACTIVE, energy, open task count | PARTIAL | format-team-roster code is correct and wired to project review, but reads metadata from /api/agents which returns null — all agents show "UNKNOWN" status |
| 6 | Idle agent transition gets one-time +12 energy boost (not repeated) | PARTIAL | Boost IS firing (log: "[lifecycle] eliana transitioning to idle — +12 energy boost") but fires every tick, not once — because prev-lifecycle check reads from metadata which is never persisted |
| 7 | Idle agents sorted first in team roster by energy descending | VERIFIED | Sorting logic in format-team-roster checks :lifecycle-state from metadata, defaults correctly when null — implementation is correct, blocked by metadata read |

**Score:** 5/7 truths verified (Truths 3, 7 fully verified; Truths 5, 6 partial — code correct but data missing; Truths 1, 2, 4 failed)

### Required Artifacts

| Artifact | Expected | Exists | Substantive | Wired | Status |
|----------|----------|--------|-------------|-------|--------|
| `/opt/dpn-api/src/handlers/af64_agents.rs` | metadata in list_agents + PATCH merge | Yes | Yes | Yes | HOLLOW — code present, sqlx feature missing |
| `/opt/dpn-api/Cargo.toml` | sqlx with json feature | Yes | No | N/A | STUB — missing `"json"` feature flag |
| `/opt/project-noosphere-ghosts/lisp/runtime/energy.lisp` | :idle-transition reward +12 | Yes | Yes | Yes | VERIFIED |
| `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` | Lifecycle state detection in Phase 5 | Yes | Yes | Yes | VERIFIED (code correct; data flow blocked upstream) |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` | Enriched team roster with availability | Yes | Yes | Yes | VERIFIED (code correct; data null due to API gap) |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `af64_agents.rs` | `agent_state.metadata` (read) | SQL SELECT + `r.get::<Option<Value>, _>("metadata")` | NOT_WIRED | Returns null — sqlx missing 'json' feature; confirmed empirically |
| `af64_agents.rs` | `agent_state.metadata` (write) | `COALESCE(metadata, '{}') \|\| $1::jsonb` | NOT_WIRED | SQL is correct but `.bind(metadata)` is a no-op — sqlx missing 'json' feature |
| `tick-engine.lisp Phase 5` | `/api/agents/:id/state` | `api-patch` with `:metadata` key | PARTIAL | Call reaches API and gets 200 OK; DB not updated |
| `action-planner.lisp` | `/api/agents` | `api-get "/api/agents"` reads metadata.lifecycle-state | NOT_WIRED | Reads null metadata; lifecycle-state never populated |
| `tick-engine.lisp` | `energy.lisp` | `update-energy` with `:idle-transition` reward | WIRED | Confirmed: boost fires correctly (log evidence), but fires every tick due to metadata persistence failure |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| `action-planner.lisp format-team-roster` | `(gethash :lifecycle-state meta)` | `/api/agents` metadata field | No — returns null | HOLLOW — wired but data disconnected at API layer |
| `tick-engine.lisp phase-update-state` | `prev-lifecycle` from `(gethash :lifecycle-state prev-metadata)` | Prior tick's PATCH to agent_state.metadata | No — metadata never written | HOLLOW — reads correct field but prior write never succeeds |
| `energy.lisp update-energy` | `:idle-transition` reward | `+energy-rewards+` hash table | Yes — value 12 confirmed | FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| list_agents returns metadata JSONB | `curl /api/agents` — check metadata field for any agent with DB content | `metadata: null` for all 64 agents (eliana has `{"lifecycle_state":"idle"}` in DB) | FAIL |
| PATCH metadata merges into DB | PATCH `{"metadata":{"lifecycle_state":"idle"}}` then SELECT agent_state.metadata | metadata unchanged `{}` after PATCH | FAIL |
| :idle-transition reward exists | `grep ':idle-transition' energy.lisp` | Line 27: `(setf (gethash :idle-transition table) 12)` | PASS |
| Lifecycle transition fires | pm2 logs noosphere-ghosts for `[lifecycle]` lines | 3 agents fire idle transition per tick (should be once) | PARTIAL — fires but repeats |
| SBCL loads af64 system | `sbcl ... (asdf:load-system :af64)` with registry path set | Warning: redefining RUN-TICK (expected on reload), no errors | PASS |
| Rust API compiles | `cargo check` in dpn-api | Finished dev profile, 12 warnings (no errors) | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|-------------|-------------|--------|----------|
| LIFE-01 | 10-01, 10-02 | Staff ghost signals IDLE after completing assigned work | BLOCKED | Lisp code detects idle and calls PATCH, but PATCH does not persist lifecycle_state — signal never reaches DB |
| LIFE-02 | 10-02 | Executive perceives staff availability in project review context | BLOCKED | format-team-roster is in project review prompt, but all agents show "UNKNOWN" because metadata is null from API |
| LIFE-03 | 10-01, 10-02 | Energy system reflects lifecycle state (idle agents have energy for new work) | PARTIAL | +12 boost fires on idle transition (energy reaches 100) but fires every tick, not once — repeated boost is wasteful and indicates the signal loop is broken |

No orphaned requirements — LIFE-01, LIFE-02, LIFE-03 are the only Phase 10 requirements per REQUIREMENTS.md traceability table.

### Anti-Patterns Found

| File | Location | Pattern | Severity | Impact |
|------|----------|---------|----------|--------|
| `/opt/dpn-api/Cargo.toml` | line 17 | `sqlx` missing `"json"` feature | BLOCKER | JSONB read/write via `serde_json::Value` silently fails — all three LIFE requirements blocked |
| `tick-engine.lisp Phase 5` | line 391 | `(update-energy ... :idle-transition ...)` fires every tick | WARNING | One-time boost becomes every-tick boost — 3 agents gain +12 energy on every tick until LIFE-01 gap is fixed |

### Human Verification Required

#### 1. Team Roster Display After Fix

**Test:** After adding `"json"` to sqlx features and redeploying dpn-api, trigger a project review tick for an executive (e.g., nova or eliana) and read the conversation output.
**Expected:** Team roster in the project review prompt should show lines like `- casey (systems-engineer) -- IDLE, energy: 65, tasks: 0`
**Why human:** Requires reading the LLM conversation output and verifying it contains the roster with correct IDLE/ACTIVE labels.

#### 2. One-Time Boost Verification After Fix

**Test:** After fix, monitor two consecutive ticks for an agent known to be idle.
**Expected:** First tick after transition: `[lifecycle] casey transitioning to idle — +12 energy boost`. Second tick: no lifecycle log line for casey.
**Why human:** Requires watching live pm2 logs across two tick cycles.

### Root Cause Summary

**Single root cause, three blocked requirements:**

The `sqlx` crate in `/opt/dpn-api/Cargo.toml` is declared with `features = ["postgres", "runtime-tokio-native-tls"]` but is missing the `"json"` feature. In sqlx 0.8, binding or reading `serde_json::Value` to/from JSONB columns requires the `"json"` feature to be enabled. Without it:

- `r.get::<Option<Value>, _>("metadata")` returns `None` (null) even when the column has content
- `.bind(metadata)` where metadata is `serde_json::Value` silently produces no SQL effect; the execute() call returns Ok but the row is not updated

The Rust handler code itself is structurally correct — the SQL query uses the right COALESCE merge pattern, and the metadata field is correctly added to both list_agents and the StateUpdate struct. The Lisp code is also correct — phase-update-state builds the lifecycle-state payload and calls api-patch with it. All the logic is right; the single missing `"json"` feature flag in Cargo.toml severs the data path.

**Fix:** Add `"json"` to sqlx features in `/opt/dpn-api/Cargo.toml`, rebuild, and redeploy.

---

_Verified: 2026-03-27T00:52:00Z_
_Verifier: Claude (gsd-verifier)_
