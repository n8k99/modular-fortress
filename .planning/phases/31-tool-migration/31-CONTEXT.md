# Phase 31: Tool Migration - Context

**Gathered:** 2026-03-30 (assumptions mode)
**Status:** Ready for planning

<domain>
## Phase Boundary

All existing Python tools are accessible as InnateScipt expressions, and tool-registry.json is retired. Tool discovery flows entirely through ghost YAML capabilities and InnateScipt resolution. Tool execution results flow back through the cognition pipeline as conversation output attributed to the executing ghost.

</domain>

<decisions>
## Implementation Decisions

### Tool inventory
- **D-01:** All tools in tool-registry.json need InnateScipt expression wrappers in ghost YAML responsibilities. This includes trading tools (Kalshi, FOREX, position tracking, risk), content tools (articles, RSS, editorial), engineering tools (build_tool, claude_code, query_db), and ops tools (wave_calendar, trading_briefing).
- **D-02:** Ghost YAML files already declare some tools as `![]` search expressions (e.g., `![market_scanner]`, `![build_tool]`). Missing tools must be added to the appropriate ghost's responsibilities section.

### Python invocation mechanism
- **D-03:** Noosphere resolver invokes Python scripts via `uiop:run-program` subprocess calls — the same pattern already used in `execute-tool-call` in tool-socket.lisp. No new execution layer (HTTP, socket) needed on single-droplet.
- **D-04:** The `resolve-search` method in noosphere-resolver.lisp is extended to detect tool expressions and dispatch to Python script execution.

### InnateScipt expression format
- **D-05:** Tool invocation uses the existing `![tool_name]` search expression syntax. Ghost YAML already uses this format for capability declarations. The resolver's search dispatch is extended to handle tool-type expressions.
- **D-06:** Tool arguments passed as InnateScipt key-value pairs: `![tool_name]{key=value, key2=value2}`.

### Tool metadata storage (after registry retirement)
- **D-07:** Tool metadata (script paths, parameters, interpreter, dangerous flags) migrates to master_chronicle — consistent with "DB is the OS" principle and Phase 30's area_content pattern for pipeline definitions. A `tool_definitions` table or area_content entries store the metadata the resolver needs to invoke scripts.
- **D-08:** The `execute-tool-call` function in tool-socket.lisp is refactored to look up tool definitions from the DB instead of `*tool-registry*` hash-table.

### Tool-registry.json retirement
- **D-09:** Remove the D-11 fallback pattern in 4 action-planner prompt builders (`unless yaml-capabilities` checks at lines ~326, ~469, ~558, ~740 of action-planner.lisp).
- **D-10:** Delete `tool-registry.json` and remove `load-tool-registry` / `*tool-registry*` from tool-socket.lisp.
- **D-11:** All ghosts must have YAML files with complete capability declarations before the registry is deleted.

### Result flow
- **D-12:** Tool execution results flow back through the existing cognition pipeline pattern — results appended to content, posted as conversation entries via `db-insert-conversation`, attributed to the executing ghost. Same pattern as `execute-work-task` and `execute-project-review` in action-executor.lisp.

### Claude's Discretion
- Exact tool_definitions table schema or area_content content_type for tool metadata
- Which Python scripts need path adjustments for the new invocation mechanism
- Whether to batch-migrate all tools at once or incrementally (recommend: all at once since registry deletion is atomic)
- Exact `resolve-search` dispatch logic for tool vs table-lookup expressions

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Tool execution (being replaced)
- `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` — `execute-tool-call`, `load-tool-registry`, `*tool-registry*`, `get-tools-for-agent`
- `/opt/project-noosphere-ghosts/config/tool-registry.json` — Current tool definitions (being retired)

### Action planner fallback (D-11 removal targets)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — 4 `unless yaml-capabilities` fallback blocks

### Ghost capabilities (Phase 28)
- `/opt/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp` — YAML loading, capability injection
- `/opt/project-noosphere-ghosts/config/agents/*.yaml` — 9 ghost YAML files with existing `![]` expressions

### Noosphere resolver (extension target)
- `/opt/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` — `resolve-search`, search dispatch
- `/opt/project-noosphere-ghosts/lisp/runtime/innate-builder.lisp` — `build-search` expression constructor

### Result flow pattern
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — `process-tool-calls`, conversation output pattern

### Requirements
- `.planning/REQUIREMENTS.md` — TOOL-01 through TOOL-04

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `execute-tool-call` in tool-socket.lisp — Python subprocess invocation pattern via `uiop:run-program`
- `resolve-search` in noosphere-resolver.lisp — search dispatch to extend with tool invocation
- `ghost-capabilities.lisp` — YAML loading pattern for capability declarations
- `process-tool-calls` in action-executor.lisp — result concatenation and conversation posting pattern
- `pipeline-definitions.lisp` (Phase 30) — pattern for loading structured data from area_content

### Established Patterns
- DB is the OS — tool metadata belongs in master_chronicle, consistent with pipeline definitions in area_content
- `uiop:run-program` for subprocess execution — proven pattern for Python script invocation
- `db-insert-conversation` for attributed output — all ghost output flows through conversations table
- `![]` search expressions in ghost YAML — already the declared format for tool capabilities

### Integration Points
- `noosphere-resolver.lisp` — `resolve-search` extended to dispatch tool expressions to Python execution
- `tool-socket.lisp` — `execute-tool-call` refactored to use DB-sourced tool definitions
- `action-planner.lisp` — 4 fallback blocks removed (D-11 cleanup)
- `action-executor.lisp` — tool result flow unchanged (already correct pattern)
- `packages.lisp` — new exports for tool definition loading

</code_context>

<specifics>
## Specific Ideas

- Phase 30 established the pattern: area_content with JSONB metadata for structured definitions loaded by Lisp module. Tool definitions can follow the same pattern.
- Ghost YAML already declares `![market_scanner]`, `![build_tool]` etc. — the gap is that these are just strings in prompts, not actually resolvable by the noosphere resolver yet.
- The retirement of tool-registry.json is atomic — all tools must be wrapped and the new lookup working before the file is deleted.

</specifics>

<deferred>
## Deferred Ideas

- Dynamic tool registration by ghosts (creating new tools at runtime) — future milestone
- Tool execution metrics/logging in a dedicated table — future observability concern
- Tool permission system beyond "dangerous" flag — future security concern

</deferred>

---

*Phase: 31-tool-migration*
*Context gathered: 2026-03-30*
