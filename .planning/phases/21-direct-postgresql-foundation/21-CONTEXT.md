# Phase 21: Direct PostgreSQL Foundation - Context

**Gathered:** 2026-03-29
**Status:** Ready for planning

<domain>
## Phase Boundary

Replace the Lisp tick engine's HTTP curl calls to dpn-api with direct PostgreSQL queries for perception and agent state updates. The tick engine currently shells out to curl for every API call (6 HTTP calls in tick-engine.lisp, 1 in perception.lisp). After this phase, perception and state updates run as SQL queries over a connection pool, eliminating the HTTP round-trip. dpn-api continues serving frontends and MCP tools — only the ghost-to-noosphere path changes.

</domain>

<decisions>
## Implementation Decisions

### PostgreSQL Client Approach (DB-01)
- **D-01:** Claude's discretion on which approach to use — vendor cl-postgres, write minimal wire protocol client, or use libpq FFI. Must follow AF64 zero-deps convention (no Quicklisp). The chosen approach needs to handle: authentication (md5 or scram-sha-256), simple queries, parameterized queries, and result parsing into Lisp data structures.

### Migration Strategy
- **D-02:** Big bang replacement — replace all HTTP calls in one shot, no dual-path fallback. This means once the SQL path is wired in, the old HTTP calls are removed entirely. Simpler code, no conditional branching based on config flags.
- **D-03:** dpn-api is NOT removed or modified. It continues serving Next.js frontends (dpn-kb, em-site, n8k99-site) and MCP tools (dpn-mcp). Only the ghost tick engine stops calling it.

### Connection Management
- **D-04:** Connection pool with 2-3 connections opened at startup. Connections persist across ticks and reconnect on failure. This enables parallel perception queries for multiple agents within a single tick cycle.
- **D-05:** Connection parameters: host=127.0.0.1, port=5432, db=master_chronicle, user=chronicle, password=chronicle2026 (same as dpn-api uses).

### Query Architecture
- **D-06:** Restructure queries for Lisp rather than mirroring dpn-api's Rust SQL exactly. The perception endpoint in af64_perception.rs does complex multi-JOIN queries that return JSON — the Lisp version should query each data type separately (messages, tasks, projects, documents, team activity) and build the perception hash-table directly from row results.
- **D-07:** The perception data shape (hash-table with :messages, :tasks, :projects, :documents, :team-activity keys) MUST remain identical so downstream tick engine code (ranking, classification, execution) works unchanged.
- **D-08:** State updates (energy, tier, last_tick_at) are simple UPDATE statements — these replace the current `api-patch` calls.

### Claude's Discretion
- PG client implementation choice (D-01)
- Exact connection pool size (2 or 3)
- Whether to use prepared statements or simple queries
- Error handling strategy for connection failures mid-tick
- Whether to add a health-check query at startup

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Ghost Tick Engine (current HTTP path)
- `/opt/project-noosphere-ghosts/lisp/util/http.lisp` — Current curl-based HTTP client (56 lines), all API calls go through here
- `/opt/project-noosphere-ghosts/lisp/runtime/perception.lisp` — Current perception via `api-get` (36 lines)
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — Tick cycle with 6 HTTP calls for state updates (562 lines)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — May have HTTP calls for task/conversation mutations (Phase 22 scope, but read for awareness)

### dpn-api Perception Endpoint (SQL reference)
- `/opt/dpn-api/src/handlers/af64_perception.rs` — The Rust perception handler with all JOINs and filters — reference for query logic

### AF64 System Definition
- `/opt/project-noosphere-ghosts/af64.asd` — ASDF system definition, new PG module needs to be registered here
- `/opt/project-noosphere-ghosts/config/provider-config.json` — Runtime config, may need DB connection params

### Requirements
- `.planning/REQUIREMENTS.md` — DB-01 (perception SQL), DB-02 (state updates SQL)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `af64.utils.http` package — will be partially replaced but the pattern (wrapper functions around a transport) should be replicated for the SQL layer
- `perception.lisp` — `perceive` function returns a hash-table with known keys — this interface must be preserved
- `empty-perception` — fallback structure when perception fails — SQL path needs the same

### Established Patterns
- AF64 uses `defpackage` per module with explicit exports
- Error handling via `handler-case` wrapping tick execution
- JSON parsing via `af64.utils.json` (custom, converts underscores to hyphens)
- Config loaded from `provider-config.json` at startup

### Integration Points
- `perceive` function in perception.lisp — called by tick-engine, returns hash-table
- `api-get`, `api-post`, `api-patch` — called throughout tick-engine.lisp for state updates
- Tick report persistence — currently via HTTP POST to /api/af64/tick-reports
- Mark-as-read — currently via HTTP POST to /api/conversations/mark-read

</code_context>

<specifics>
## Specific Ideas

- The connection pool enables future parallel perception (query multiple agents simultaneously within one tick)
- Restructured queries for Lisp means we can avoid the JSON serialization/deserialization overhead entirely — rows go straight to Lisp data structures
- The `api-get`/`api-post`/`api-patch` functions in http.lisp could be replaced with `db-query`/`db-execute` equivalents following the same wrapper pattern

</specifics>

<deferred>
## Deferred Ideas

- Conversations and task mutations via SQL — Phase 22
- Removing dpn-api entirely — out of scope (serves frontends)
- LISTEN/NOTIFY for real-time perception — future enhancement

</deferred>

---

*Phase: 21-direct-postgresql-foundation*
*Context gathered: 2026-03-29*
