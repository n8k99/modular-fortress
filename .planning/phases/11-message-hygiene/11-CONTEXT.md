# Phase 11: Message Hygiene - Context

**Gathered:** 2026-03-27
**Status:** Ready for planning

<domain>
## Phase Boundary

Stop ghosts from burning tokens on messages they have already processed. The conversations table has a `read_by varchar(50)[]` column that is never updated and never filtered in perception queries. Fix this end-to-end: API marks messages read, perception filters them out, and agents with nothing unread go idle instead of getting cognition jobs. Also fix the sqlx "json" feature gap blocking lifecycle metadata persistence.

</domain>

<decisions>
## Implementation Decisions

### Read-Marking Trigger
- **D-01:** Messages are marked as read AFTER cognition completes (not after perception loads). This ensures the ghost actually processed the message before marking it. If cognition fails or times out, the message stays unread and will be retried next tick.

### API Design
- **D-02:** Batch mark-read endpoint — one POST call with an array of message IDs, appends agent_id to read_by for all in a single query. Reduces API calls from N-per-message to 1-per-tick.
- **D-03:** Endpoint path: POST /api/conversations/mark-read with body `{ agent_id, message_ids: [int] }`

### Perception Filtering
- **D-04:** Perception query adds `AND NOT ($1 = ANY(read_by))` to the WHERE clause. Messages already read by the requesting agent are excluded from results.
- **D-05:** The `has-actionable-items` function in perception.lisp already works correctly — it checks vector length. Once perception returns empty messages, agents will naturally classify as idle.

### Historical Cleanup
- **D-06:** One-time SQL cleanup on deploy: mark all existing stale messages as read for agents that have already responded to them. This prevents the 336+ duplicate message problem from re-triggering when ghosts restart.

### sqlx Fix
- **D-07:** Add `"json"` to sqlx features in dpn-api Cargo.toml: `features = ["postgres", "runtime-tokio-native-tls", "json"]`. Rebuild and verify metadata JSONB round-trip works for lifecycle state persistence.

### Claude's Discretion
- Exact SQL for the historical cleanup migration
- Whether to add an index on read_by (GIN) for performance — decide based on table size
- Error handling approach for the batch mark-read endpoint

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Perception System
- `/opt/dpn-api/src/handlers/af64_perception.rs` — perception query with message fetching (lines 33-52), needs read_by filter
- `/opt/project-noosphere-ghosts/lisp/runtime/perception.lisp` — `has-actionable-items` function (lines 31-36), already correct
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — ranking/acting-set logic (lines 135-218), filters by actionable items

### Conversations System
- `/opt/dpn-api/src/handlers/af64_conversations.rs` — conversations CRUD, needs mark-read endpoint
- `/opt/dpn-api/src/main.rs` — router, needs new route registration

### Action Executor
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — posts conversations after cognition, needs to call mark-read after processing
- `/opt/project-noosphere-ghosts/lisp/runtime/claude-code-provider.lisp` — LLM provider, may need message ID passthrough

### Infrastructure
- `/opt/dpn-api/Cargo.toml` — sqlx features line (needs "json" addition)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `conversations.read_by varchar(50)[]` column already exists with default `'{}'` — no migration needed for the column itself
- `af64_conversations.rs` follows the pattern of other af64 handlers — batch endpoint should follow same style
- `array_append` and `ANY()` PostgreSQL operators already used elsewhere in the codebase (blocked_by in tasks)

### Established Patterns
- API handlers use `ApiError` enum for errors, `Json<Value>` for responses
- Lisp HTTP calls use `api-post` helper from `util/http.lisp`
- Perception returns max 10 messages per tick (LIMIT 10 in query)

### Integration Points
- Perception query in `af64_perception.rs` line 36 — add read_by filter to WHERE clause
- Action executor `execute-cognition-result` — call mark-read API after successful processing
- Router in `main.rs` — register new mark-read endpoint under `/api/conversations/`

</code_context>

<specifics>
## Specific Ideas

- The current perception query uses a hardcoded `since = "2026-01-01T00:00:00Z"` in perception.lisp — with read_by filtering this becomes less critical but should be noted
- Agents are currently producing ~336+ duplicate messages per 24 hours per agent — 5 active agents = ~1680 wasted cognition jobs per day
- Nova's message says "Two-hundredth consecutive identical dispatch" — she's aware of the problem and tracking it herself

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 11-message-hygiene*
*Context gathered: 2026-03-27*
