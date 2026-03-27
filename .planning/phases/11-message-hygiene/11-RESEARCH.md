# Phase 11: Message Hygiene - Research

**Researched:** 2026-03-27
**Domain:** PostgreSQL array filtering, Rust/Axum API endpoints, Common Lisp HTTP client integration
**Confidence:** HIGH

## Summary

This phase addresses a critical token-bleed problem where ghosts re-process the same messages every tick because the `conversations.read_by` column is never written to and never filtered in perception queries. The fix is end-to-end across three codebases: Rust API (new endpoint + perception query modification), PostgreSQL (array operations + cleanup migration), and Common Lisp (post-cognition mark-read call).

The existing infrastructure is well-suited for this change. The `read_by varchar(50)[]` column already exists with the right type. PostgreSQL's `ANY()` operator is already used in the codebase for array membership checks (e.g., `to_agent`, `assigned_to`, `blocked_by`). The sqlx `"json"` feature was already added in a prior commit (73164bf) so FIX-01 is resolved at the code level -- it just needs rebuild verification on the running service.

**Primary recommendation:** Implement in 3 tasks: (1) Rust API changes (mark-read endpoint + perception filter), (2) Lisp action-executor integration (call mark-read after cognition), (3) historical cleanup SQL migration. Build and restart dpn-api after task 1, then restart ghosts after task 2.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Messages are marked as read AFTER cognition completes (not after perception loads). This ensures the ghost actually processed the message before marking it. If cognition fails or times out, the message stays unread and will be retried next tick.
- **D-02:** Batch mark-read endpoint -- one POST call with an array of message IDs, appends agent_id to read_by for all in a single query. Reduces API calls from N-per-message to 1-per-tick.
- **D-03:** Endpoint path: POST /api/conversations/mark-read with body `{ agent_id, message_ids: [int] }`
- **D-04:** Perception query adds `AND NOT ($1 = ANY(read_by))` to the WHERE clause. Messages already read by the requesting agent are excluded from results.
- **D-05:** The `has-actionable-items` function in perception.lisp already works correctly -- it checks vector length. Once perception returns empty messages, agents will naturally classify as idle.
- **D-06:** One-time SQL cleanup on deploy: mark all existing stale messages as read for agents that have already responded to them. This prevents the 336+ duplicate message problem from re-triggering when ghosts restart.
- **D-07:** Add `"json"` to sqlx features in dpn-api Cargo.toml. (ALREADY DONE -- commit 73164bf)

### Claude's Discretion
- Exact SQL for the historical cleanup migration
- Whether to add an index on read_by (GIN) for performance -- decide based on table size
- Error handling approach for the batch mark-read endpoint

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SPAM-01 | Perception endpoint filters out messages already in the agent's read_by array | Perception query at af64_perception.rs:33-52 needs `AND NOT ($1 = ANY(read_by))` added to WHERE clause |
| SPAM-02 | Action executor marks processed messages as read after cognition completes | action-executor.lisp `execute-cognition-result` dispatches to action handlers; mark-read call goes in `phase-process-cognition` in tick-engine.lisp after `execute-cognition-result` succeeds |
| SPAM-03 | Agent with zero actionable items after read_by filtering is classified as idle | Already works -- `has-actionable-items` in perception.lisp checks vector length; once perception returns empty messages, agent gets no cognition job |
| FIX-01 | dpn-api Cargo.toml includes sqlx "json" feature | ALREADY DONE in commit 73164bf. Needs rebuild verification only |
| FIX-02 | Mark-as-read API endpoint exists to update read_by array | New endpoint POST /api/conversations/mark-read using PostgreSQL `array_append` |
</phase_requirements>

## Standard Stack

No new libraries needed. All changes use existing dependencies.

### Core (Already Present)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| sqlx | 0.8 | PostgreSQL queries with `json` feature | Already in dpn-api Cargo.toml |
| axum | 0.7 | HTTP endpoint for mark-read | Already in dpn-api |
| serde/serde_json | 1 | Request/response serialization | Already in dpn-api |

### Lisp (Already Present)
| Module | Location | Purpose |
|--------|----------|---------|
| api-client | `lisp/runtime/api-client.lisp` | `api-post` helper for HTTP calls to dpn-api |
| action-executor | `lisp/runtime/action-executor.lisp` | Post-cognition action dispatch |
| tick-engine | `lisp/runtime/tick-engine.lisp` | `phase-process-cognition` where mark-read integration goes |

## Architecture Patterns

### Existing Patterns to Follow

**Rust API handler pattern** (from af64_conversations.rs):
```rust
// Deserialize request body
#[derive(Deserialize)]
pub struct MarkReadRequest {
    pub agent_id: String,
    pub message_ids: Vec<i32>,
}

// Handler signature follows existing pattern
pub async fn mark_read(
    State(pool): State<DbPool>,
    Json(body): Json<MarkReadRequest>,
) -> Result<Json<Value>, ApiError> {
    // Implementation
}
```

**Router registration pattern** (from main.rs line 118):
```rust
// Current: .route("/conversations", get(...).post(...))
// Add: .route("/conversations/mark-read", post(af64_conversations::mark_read))
```

**Lisp API call pattern** (from action-executor.lisp line 31):
```lisp
(api-post "/api/conversations/mark-read"
          (json-object :agent-id agent-id
                       :message-ids message-id-vector))
```

### Key Integration Points

1. **Perception query** (af64_perception.rs:33-52): Add read_by filter to the WHERE clause of the messages query
2. **Mark-read endpoint**: New function in af64_conversations.rs, new route in main.rs
3. **Post-cognition mark-read**: In tick-engine.lisp `phase-process-cognition` (line 319-362), after `execute-cognition-result` succeeds
4. **Message ID passthrough**: The cognition result's metadata hash contains `:source-message` with `:id` -- this is how the Lisp side knows which message was processed

### Data Flow (Post-Implementation)

```
Tick Start
  |
  v
phase-perceive --> GET /api/perception/:agent_id
  |                  WHERE ... AND NOT ($1 = ANY(read_by))
  v                  (agent only sees UNREAD messages)
phase-rank --> has-actionable-items checks message count
  |              (0 unread = idle, no cognition job)
  v
phase-process-cognition --> execute-cognition-result
  |                           (processes message, posts reply)
  v
  mark-read call --> POST /api/conversations/mark-read
                       { agent_id, message_ids: [processed_id] }
                       (appends agent_id to read_by array)
```

### Anti-Patterns to Avoid
- **Marking read during perception**: Would cause messages to be skipped if cognition fails
- **Per-message mark-read calls**: Decision D-02 mandates batch endpoint (1 call per tick, not N)
- **Filtering in Lisp**: Do NOT filter read messages in perception.lisp -- filter in SQL for efficiency

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Array membership check | Custom Lisp filtering | PostgreSQL `ANY()` operator | SQL-level filtering is faster and keeps the Lisp side simple |
| Atomic array append | Read-modify-write pattern | PostgreSQL `array_append()` | Avoids race conditions between concurrent mark-read calls |
| Batch update | Loop of single-row updates | Single UPDATE with `id = ANY($2)` | One query instead of N |

## Common Pitfalls

### Pitfall 1: Race Condition on array_append
**What goes wrong:** Two concurrent mark-read calls for the same message_id could both read the old array and overwrite each other.
**Why it happens:** `UPDATE ... SET read_by = array_append(read_by, $1)` is atomic per-row in PostgreSQL -- this is actually safe. The real risk is if code does a SELECT then UPDATE instead.
**How to avoid:** Use `array_append` directly in the UPDATE statement. Never read-then-write.
**Warning signs:** Any code path that SELECTs read_by then UPDATEs it separately.

### Pitfall 2: Duplicate Entries in read_by Array
**What goes wrong:** If an agent processes a message twice (retry), `array_append` adds the agent_id again.
**Why it happens:** `array_append` does not check for duplicates.
**How to avoid:** Add a condition: `UPDATE ... SET read_by = array_append(read_by, $1) WHERE NOT ($1 = ANY(read_by))` OR accept duplicates (harmless for filtering, just wastes a few bytes).
**Recommendation:** Accept duplicates -- the WHERE filter `NOT ($1 = ANY(read_by))` works regardless. The simplicity is worth the minor space cost.

### Pitfall 3: Lisp JSON Key Mapping
**What goes wrong:** Lisp JSON parser converts underscores to hyphens. Sending `message_ids` from Lisp would encode as `message-ids`.
**Why it happens:** This is a known quirk documented in CLAUDE.md: "parser converts underscores to hyphens."
**How to avoid:** The Lisp `json-object` uses keyword arguments with hyphens (`:message-ids`) which the JSON encoder must serialize as `message_ids`. Verify the encoder handles this correctly. The existing `api-post "/api/conversations"` call works with `from-agent` -> `from_agent`, confirming this works.
**Warning signs:** 400 Bad Request errors from the mark-read endpoint during testing.

### Pitfall 4: Message IDs Not Available in Cognition Metadata
**What goes wrong:** Trying to mark messages as read but not knowing which messages were processed.
**Why it happens:** Only `respond_message` jobs store the source message ID in metadata. Other job types (work_task, proactive_work, project_review) may not have specific message IDs.
**How to avoid:** For `respond_message`: extract `:id` from `(gethash :source-message metadata)`. For other types: mark ALL messages from the perception snapshot as read, since the agent processed the entire perception context. The perception data is available in `phase-process-cognition` via the `perceptions` hash table.
**Recommendation:** After successful cognition, collect all message IDs from the agent's perception snapshot and batch-mark them as read.

### Pitfall 5: Historical Messages Causing Immediate Spam on Restart
**What goes wrong:** After deploying the read_by filter, ghosts restart and see 0 unread messages (good). But if the cleanup migration is skipped, the `since = "2026-01-01T00:00:00Z"` lookback means agents see thousands of old messages as "unread."
**How to avoid:** Run the historical cleanup migration BEFORE restarting ghosts. The cleanup must mark old messages as read for agents that have already responded to them.

### Pitfall 6: Axum Route Ordering
**What goes wrong:** `/conversations/mark-read` might not match if registered after `/conversations/:id`.
**Why it happens:** Axum routes are matched in registration order. A parameterized route like `/:id` would swallow `mark-read`.
**How to avoid:** Currently there IS no `/conversations/:id` route in main.rs (line 118 only has `/conversations` for GET+POST). So this is safe. But register `/conversations/mark-read` BEFORE any future `/:id` route.

## Code Examples

### 1. Mark-Read Endpoint (Rust)

```rust
// Source: Pattern from af64_conversations.rs
#[derive(Deserialize)]
pub struct MarkReadRequest {
    pub agent_id: String,
    pub message_ids: Vec<i32>,
}

/// POST /api/conversations/mark-read
pub async fn mark_read(
    State(pool): State<DbPool>,
    Json(body): Json<MarkReadRequest>,
) -> Result<Json<Value>, ApiError> {
    if body.message_ids.is_empty() {
        return Ok(Json(serde_json::json!({"updated": 0})));
    }

    let result = sqlx::query(
        "UPDATE conversations SET read_by = array_append(read_by, $1) WHERE id = ANY($2) AND NOT ($1 = ANY(read_by))"
    )
    .bind(&body.agent_id)
    .bind(&body.message_ids)
    .execute(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({"updated": result.rows_affected()})))
}
```

### 2. Perception Query Filter Addition (Rust)

```rust
// Source: af64_perception.rs line 33-52
// Add to WHERE clause after `AND from_agent != $1`:
//   AND NOT ($1 = ANY(read_by))
// This single line change filters out messages the agent has already processed.
```

### 3. Post-Cognition Mark-Read (Lisp)

```lisp
;; In phase-process-cognition, after execute-cognition-result succeeds:
;; Collect message IDs from perception, then mark as read
(let* ((perception (gethash agent-id perceptions))
       (msgs (when perception (gethash :messages perception)))
       (msg-ids (when (and msgs (> (length msgs) 0))
                  (coerce (map 'vector (lambda (m) (gethash :id m)) msgs) 'vector))))
  (when (and msg-ids (> (length msg-ids) 0))
    (handler-case
        (api-post "/api/conversations/mark-read"
                  (json-object :agent-id agent-id
                               :message-ids msg-ids))
      (error (e)
        (format t "  [mark-read-error] ~a: ~a~%" agent-id e)))))
```

### 4. Historical Cleanup SQL

```sql
-- Mark messages as read for agents that have already responded to them
-- Logic: if agent X has sent a reply with metadata->>'responding_to' = msg.id,
-- then msg should have X in its read_by array
UPDATE conversations c
SET read_by = array_append(c.read_by, r.from_agent)
FROM conversations r
WHERE r.metadata->>'responding_to' = c.id::text
  AND NOT (r.from_agent = ANY(c.read_by));

-- Also mark all messages older than 7 days as read by all their recipients
-- These are definitely stale and should not re-trigger
UPDATE conversations
SET read_by = to_agent
WHERE created_at < NOW() - INTERVAL '7 days'
  AND to_agent IS NOT NULL
  AND read_by = ARRAY[]::varchar[];
```

### 5. Router Registration (Rust)

```rust
// In main.rs, add after line 118:
.route("/conversations/mark-read", post(af64_conversations::mark_read))
```

## GIN Index Recommendation

**Decision (Claude's Discretion):** Add a GIN index on `read_by`.

**Rationale:**
- Table has 9,010 rows currently, growing ~950/day (6,671 in 7 days)
- Perception query runs for EVERY active agent EVERY tick (up to 64 agents x every 60s-10min)
- The `NOT ($1 = ANY(read_by))` clause needs to scan the array for each row
- GIN index on `read_by` enables PostgreSQL to use the index for `ANY()` checks
- Cost: minimal storage overhead (~100KB for 9K rows), no write performance impact at this scale

```sql
CREATE INDEX idx_conversations_read_by ON conversations USING gin (read_by);
```

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual integration testing (no automated test suite for dpn-api) |
| Config file | none |
| Quick run command | `cargo check` (compile check) + `curl` API tests |
| Full suite command | `cargo build --release` + manual curl verification |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| SPAM-01 | Perception excludes read messages | integration | `curl -H "X-API-Key: $KEY" localhost:8080/api/perception/nova` -- verify no read messages | N/A manual |
| SPAM-02 | Mark-read after cognition | integration | Check read_by array in DB after ghost tick completes | N/A manual |
| SPAM-03 | Zero-unread agent gets no cognition | integration | Observe tick log: agent with 0 messages = "idle" | N/A manual |
| FIX-01 | sqlx json feature works | build | `cd /opt/dpn-api && cargo check` | already passes |
| FIX-02 | Mark-read endpoint exists | integration | `curl -X POST -H "Content-Type: application/json" -H "X-API-Key: $KEY" -d '{"agent_id":"test","message_ids":[1]}' localhost:8080/api/conversations/mark-read` | N/A manual |

### Sampling Rate
- **Per task commit:** `cd /opt/dpn-api && cargo check`
- **Per wave merge:** `cargo build --release && pm2 restart dpn-api` + curl verification
- **Phase gate:** All 5 requirements verified via curl + DB queries + tick observation

### Wave 0 Gaps
- None -- no existing test framework to gap-fill. Verification is via manual curl + DB inspection.

## Project Constraints (from CLAUDE.md)

- **Stack**: Rust (dpn-api), Common Lisp/SBCL (ghosts), PostgreSQL -- no new languages
- **DB is the OS**: All state in master_chronicle
- **UTF-8 Rule**: Use `.chars().take(N).collect()`, never byte slicing (not directly relevant to this phase but must be maintained)
- **Lisp JSON quirk**: Parser converts underscores to hyphens (`:message-ids` not `:message_ids`)
- **Ghost LLM**: $0.50/request budget -- this phase reduces wasted requests
- **Single droplet**: All services on same machine -- restart coordination matters

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| PostgreSQL | All DB operations | Yes | Available at 127.0.0.1:5432 | -- |
| Rust/Cargo | dpn-api build | Yes | 2021 Edition | -- |
| SBCL | Ghost runtime | Yes | Via PM2 process | -- |
| dpn-api | Live service | Yes | Port 8080 | -- |
| PM2 | Service management | Yes | Running | -- |

No missing dependencies.

## Open Questions

1. **Perceptions Hash Table Scope in phase-process-cognition**
   - What we know: `phase-process-cognition` receives `(tick-number drives agent-map agent-summaries)` but NOT the `perceptions` hash table directly.
   - What's unclear: Need to either pass `perceptions` into `phase-process-cognition` or store message IDs in the cognition job metadata for later retrieval.
   - Recommendation: Modify `phase-process-cognition` signature to accept `perceptions` (passed from `run-tick`). This is cleaner than threading IDs through the cognition broker. Alternatively, extract all message IDs from the cognition-result's metadata `:source-message` -> `:id` for respond_message actions, and accept that other action types don't mark specific messages.

2. **Mark-Read Timing: Per-Result vs End-of-Phase**
   - What we know: D-01 says "after cognition completes." `phase-process-cognition` processes results one at a time in a `dolist`.
   - What's unclear: Should mark-read fire per-result (immediately after each `execute-cognition-result`) or batch all at end of phase?
   - Recommendation: Per-result is safer (mark immediately after successful execution so even if later results fail, the successful ones are marked). The batch endpoint handles the efficiency concern.

## Sources

### Primary (HIGH confidence)
- `/opt/dpn-api/src/handlers/af64_perception.rs` -- full perception query reviewed
- `/opt/dpn-api/src/handlers/af64_conversations.rs` -- existing conversation CRUD patterns
- `/opt/dpn-api/Cargo.toml` -- confirmed sqlx "json" feature already present
- `/opt/dpn-api/src/main.rs` -- router structure, no conflicting routes
- `/opt/dpn-api/src/error.rs` -- ApiError enum with BadRequest variant
- `/opt/project-noosphere-ghosts/lisp/runtime/perception.lisp` -- has-actionable-items confirmed correct
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` -- full tick flow, phase-process-cognition signature
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` -- execute-cognition-result dispatch, api-post usage
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` -- source-message metadata in cognition jobs
- `/opt/project-noosphere-ghosts/lisp/runtime/cognition-types.lisp` -- cognition-result struct with metadata field
- `/opt/project-noosphere-ghosts/lisp/runtime/api-client.lisp` -- api-post implementation

### Database Verification (HIGH confidence)
- `conversations` table schema: `read_by varchar(50)[]` with default `'{}'`
- Current stats: 9,010 rows total, 67 with non-empty read_by, 6,671 from last 7 days
- Last 24h: ~5,186 messages from 5 agents (nova/kathryn/jmax/lrm/vincent each ~1,296)
- No existing GIN index on read_by
- Existing GIN index on to_agent confirms PostgreSQL GIN support is available

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - all existing libraries, no new dependencies
- Architecture: HIGH - full code review of all integration points completed
- Pitfalls: HIGH - identified from actual codebase patterns and Lisp JSON quirk documentation
- SQL approach: HIGH - PostgreSQL array operations verified against existing usage patterns in codebase

**Research date:** 2026-03-27
**Valid until:** 2026-04-27 (stable -- no external dependency changes expected)
