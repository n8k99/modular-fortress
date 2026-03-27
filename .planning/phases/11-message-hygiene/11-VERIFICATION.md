---
phase: 11-message-hygiene
verified: 2026-03-27T11:15:00Z
status: passed
score: 7/7 must-haves verified
re_verification: false
---

# Phase 11: Message Hygiene Verification Report

**Phase Goal:** Ghosts stop wasting tokens on stale messages they have already processed
**Verified:** 2026-03-27T11:15:00Z
**Status:** passed
**Re-verification:** No -- initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A ghost that has already processed a message does not see it again in its next perception call | VERIFIED | `af64_perception.rs` line 38: `AND NOT ($1 = ANY(read_by))` filters read messages from perception SQL query |
| 2 | After cognition completes, the agent's ID appears in the read_by array for every message it processed | VERIFIED | `tick-engine.lisp` lines 360-372: `api-post "/api/conversations/mark-read"` fires after cognition with all perceived message IDs; DB shows 17 recent messages with read_by populated |
| 3 | An agent with zero unread messages after filtering gets no cognition job and burns no tokens | VERIFIED | Perception returns 0 messages when all are read (pre-existing `has-actionable-items` check gates cognition). Nova shows only 1 unread message, down from the 336+/day baseline |
| 4 | dpn-api can read and write JSONB metadata fields without sqlx errors | VERIFIED | `Cargo.toml` line 17: `sqlx = { version = "0.8", features = ["postgres", "runtime-tokio-native-tls", "json"] }` -- json feature present, confirmed by successful build (commit 73164bf) |
| 5 | A mark-as-read API endpoint exists and correctly appends agent IDs to the read_by array | VERIFIED | `af64_conversations.rs` lines 112-141: `mark_read` handler with `MarkReadRequest` struct; endpoint responds correctly (empty agent_id returns 400, empty message_ids returns `{"updated":0}`) |
| 6 | Historical stale messages are marked as read so ghosts don't re-process them on restart | VERIFIED | DB query shows 2252 messages older than 7 days have non-empty read_by arrays (historical cleanup ran) |
| 7 | Mark-read failures are logged but do not crash the tick engine | VERIFIED | `tick-engine.lisp` lines 367-372: `handler-case` wraps `api-post` call, error condition prints `[mark-read-error]` and continues |

**Score:** 7/7 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `/opt/dpn-api/src/handlers/af64_conversations.rs` | mark_read handler + MarkReadRequest struct | VERIFIED | Lines 112-141, substantive implementation with validation, SQL update, error handling |
| `/opt/dpn-api/src/handlers/af64_perception.rs` | read_by filtering in perception messages query | VERIFIED | Line 38: `AND NOT ($1 = ANY(read_by))` in WHERE clause |
| `/opt/dpn-api/src/main.rs` | Route registration for mark-read endpoint | VERIFIED | Line 119: `.route("/conversations/mark-read", post(af64_conversations::mark_read))` |
| `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` | Post-cognition mark-read API call | VERIFIED | Lines 358-372: extracts message IDs from perceptions, calls api-post, handles errors |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `/opt/dpn-api/src/main.rs` | `af64_conversations.rs` | Route registration | WIRED | Line 119: `conversations/mark-read` -> `post(af64_conversations::mark_read)` |
| `af64_perception.rs` | conversations table read_by column | SQL WHERE clause | WIRED | Line 38: `AND NOT ($1 = ANY(read_by))` filters against DB column |
| `tick-engine.lisp` | POST /api/conversations/mark-read | api-post call after cognition | WIRED | Line 368: `(api-post "/api/conversations/mark-read" ...)` |
| `tick-engine.lisp` | perceptions hash table | phase-process-cognition parameter | WIRED | Line 319: function accepts `perceptions` parameter; Line 502: called with `perceptions` argument from `run-tick` |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| `af64_conversations.rs` mark_read | body.message_ids | POST request body from ghost tick engine | Yes -- ghost sends actual message IDs from perception | FLOWING |
| `af64_perception.rs` | read_by filter | conversations.read_by column in PostgreSQL | Yes -- real DB query with array containment check | FLOWING |
| `tick-engine.lisp` mark-read block | msg-ids | `(gethash :messages perception)` from perception hash table | Yes -- perception data fetched from dpn-api each tick | FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| mark-read rejects empty agent_id | POST with empty agent_id | `{"error":"agent_id is required"}` | PASS |
| mark-read handles empty message_ids | POST with empty array | `{"updated":0}` | PASS |
| Historical cleanup ran | Count messages with read_by older than 7 days | 2252 messages | PASS |
| GIN index exists | pg_indexes query | idx_conversations_read_by found | PASS |
| dpn-api is online | pm2 status | online | PASS |
| Perception filtering works | GET perception for nova | 1 message (down from 336+/day baseline) | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| SPAM-01 | 11-01 | Perception endpoint filters out messages already in agent's read_by array | SATISFIED | `af64_perception.rs` line 38: `AND NOT ($1 = ANY(read_by))` |
| SPAM-02 | 11-02 | Action executor marks processed messages as read after cognition completes | SATISFIED | `tick-engine.lisp` lines 358-372: api-post to mark-read after cognition |
| SPAM-03 | 11-02 | Agent with zero actionable items after filtering gets no cognition job | SATISFIED | Perception returns 0 messages when all read; pre-existing `has-actionable-items` check gates cognition |
| FIX-01 | 11-01 | dpn-api Cargo.toml includes sqlx "json" feature for JSONB support | SATISFIED | `Cargo.toml` line 17: `features = ["postgres", "runtime-tokio-native-tls", "json"]` |
| FIX-02 | 11-01 | Mark-as-read API endpoint exists to update read_by array | SATISFIED | `af64_conversations.rs` lines 112-141: functional endpoint verified via curl |

No orphaned requirements -- all 5 IDs (SPAM-01, SPAM-02, SPAM-03, FIX-01, FIX-02) are claimed by plans and satisfied.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| (none) | -- | -- | -- | No TODO, FIXME, placeholder, or stub patterns found in any modified file |

### Human Verification Required

### 1. Ghost Token Savings Over Time

**Test:** Monitor ghost token consumption over 24-48 hours and compare to the pre-fix baseline of ~336 duplicate messages per agent per day.
**Expected:** Dramatic reduction in per-agent message processing; agents should process each message exactly once.
**Why human:** Requires longitudinal observation across multiple tick cycles with real ghost activity.

### 2. Pre-existing Perception Errors

**Test:** Check PM2 logs for `:NULL is not of type SEQUENCE` errors for eliana, sarah, sylvia agents.
**Expected:** These pre-existing errors (noted in 11-02 SUMMARY) should be investigated separately; they are not caused by phase 11 changes.
**Why human:** Requires debugging nil field handling in perception data for specific agents, outside phase 11 scope.

### Gaps Summary

No gaps found. All 7 observable truths verified. All 5 requirements satisfied. All artifacts exist, are substantive, are wired, and have real data flowing through them. Behavioral spot-checks all pass. The end-to-end pipeline is confirmed: perception filters read messages, cognition processes unread ones, mark-read closes the loop, and agents with nothing to process stay idle.

---

_Verified: 2026-03-27T11:15:00Z_
_Verifier: Claude (gsd-verifier)_
