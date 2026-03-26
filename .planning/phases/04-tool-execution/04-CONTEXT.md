# Phase 4: Tool Execution - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Make tool execution work end-to-end for staff ghosts. The tool socket system already exists (319 lines, 65+ registered tools, scope-based access control). This phase ensures the highest-value tool categories actually function, adds Claude Code CLI as a registered tool, persists tool execution results for audit, and fixes agent tool_scope assignments.

</domain>

<decisions>
## Implementation Decisions

### Critical Finding: Tool Socket Already Exists
- **D-01:** `tool-socket.lisp` (319 lines) implements: `*tool-registry*` loaded from `config/tool-registry.json`, `get-tools-for-agent` with scope overlap, `format-tools-for-prompt` for LLM injection, `parse-tool-calls` for extracting `tool_call` blocks from LLM output.
- **D-02:** 65+ tools registered in `tool-registry.json` with scope arrays. Tools include query_db, write_document, build_tool, assign_task, search_documents, memory tools, trading tools, content tools.
- **D-03:** Anti-hallucination already exists: `tools-executed` count checked at line 175 of action-executor. Ghosts that claim work without calling tools fail validation.

### Tool Categories to Make Work
- **D-04:** Focus on 4 categories this phase:
  1. **DB tools**: query_db, write_document, search_documents — CRUD against master_chronicle
  2. **Task tools**: list_tasks, assign_task — staff manage their own work items
  3. **Code tools**: build_tool, codebase_scanner + new claude_code tool — engineering ghosts read/write code
  4. **Memory tools**: read_own_memory, write_own_memory — cognitive continuity across ticks

### Claude Code CLI Integration
- **D-05:** Register `claude_code` as a tool in `tool-registry.json`. Tool socket handles invocation, scope checks, and timeout. Consistent with existing tool dispatch pattern.
- **D-06:** Claude Code CLI invoked as: `claude -p "<prompt>" --output-format json` with budget $0.50/request and 120s timeout (from af64.env config).
- **D-07:** Scope restricted to engineering agents (tool_scope includes 'engineering' or 'tools').

### Anti-Hallucination: Result Persistence
- **D-08:** Tool execution results stored in task `stage_notes` column. Executives can review what staff actually did — full audit trail.
- **D-09:** Existing `tools-executed` count check remains (sufficient for v1). Result persistence is additional, not replacement.

### Tool Scope Assignment
- **D-10:** Audit current `tool_scope` values in agents table. Verify they match agent roles per executive roster. Update where needed. One-time data fix via SQL.
- **D-11:** Key mappings: Engineering agents → ['engineering', 'tools', 'specs', 'builder']. Content agents → ['content', 'writing', 'editorial']. Strategy agents → ['strategy', 'market', 'trading', 'research']. etc.

### Carried From Prior Phases
- Phase 1: Hierarchical tasks with parent_id
- Phase 2: Perception returns tool_scope for agents
- Phase 3: CREATE_TASK creates tasks via API, executives delegate with assignee

### Claude's Discretion
- Which specific tool implementations need fixes vs which already work (research will determine)
- Whether to add a `tool_timeout` per-tool config in registry
- Error handling when a tool fails mid-execution (retry vs report)
- Whether external tools (web search, URL fetch) are in scope for this phase or deferred

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Tool Socket (Primary System)
- `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` — 319-line tool dispatch system. Key: `load-tool-registry`, `get-tools-for-agent`, `format-tools-for-prompt`, `parse-tool-calls`, `safe-json-extract`
- `/opt/project-noosphere-ghosts/config/tool-registry.json` — 65+ tool definitions with scope arrays

### Action Executor (Tool Validation)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Lines 175-182: tools-executed validation. Lines 327+: pipeline advancement with tool dispatch

### Tool Implementations (GOTCHA Workspace)
- `/root/gotcha-workspace/tools/` — Python tool implementations. Check `manifest.md` for index.
- `/root/gotcha-workspace/tools/_config.py` — PG_CONFIG and workspace paths
- `/root/gotcha-workspace/tools/_db.py` — Database utilities

### Agent Configuration
- Live `agents` table in master_chronicle — `tool_scope` column (text[])
- `/opt/project-noosphere-ghosts/config/af64.env` — Claude Code CLI budget and timeout

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- Tool socket dispatch loop already works: parse tool_call → lookup in registry → check scope → execute
- Tool result format expected: JSON with status, data, error fields
- `format-tools-for-prompt` automatically injects available tools into LLM context
- Python tool implementations in gotcha-workspace follow callable function pattern

### Established Patterns
- Tool registry JSON: `{ "tools": { "tool_name": { "scope": [...], "description": "...", "parameters": {...}, "status": "..." } } }`
- LLM outputs: ` ```tool_call\n{"tool": "name", "args": {...}}\n``` `
- Tool scope matching: agent scope ∩ tool scope must be non-empty
- `handler-case` wrapping for all external calls

### Integration Points
- `execute-task-work` in action-executor calls tool socket for task execution
- Tool results feed back into conversation output (ghost reports what it did)
- `stage_notes` column on tasks table stores execution notes — tool results go here
- Claude Code CLI at `/root/.local/bin/claude` with `--output-format json`

</code_context>

<specifics>
## Specific Ideas

- Start by auditing which of the 65+ tools actually have working implementations behind them in gotcha-workspace
- The claude_code tool should be a thin wrapper: construct prompt from task context, invoke CLI, parse JSON response
- Memory tools (read_own_memory, write_own_memory) are critical for ghost cognitive continuity — prioritize these

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 04-tool-execution*
*Context gathered: 2026-03-26*
