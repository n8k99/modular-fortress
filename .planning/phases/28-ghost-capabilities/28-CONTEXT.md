# Phase 28: Ghost Capabilities - Context

**Gathered:** 2026-03-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Ghosts declare what they can do as InnateScipt expressions in YAML files, replacing the static tool-registry.json for capability discovery and cognition. The tick engine reads capabilities from ghost YAML, injects them into LLM prompts, and ghosts can self-modify their own capabilities via cognition output with parse-round-trip validation.

</domain>

<decisions>
## Implementation Decisions

### YAML file location and format
- **D-01:** Ghost YAML files at `/opt/project-noosphere-ghosts/config/agents/{agent_id}.yaml` — one file per ghost, colocated with existing config directory
- **D-02:** YAML structure:
  ```yaml
  id: ethan_ng
  responsibilities:
    - "![fundamentals:feeds]"
    - "![technicals:oanda_api[pairs: major+minor]]"
    - "{em.content.podcast}"
  ```
- **D-03:** The `responsibilities:` section is a list of valid InnateScipt expression strings — each must pass parse-round-trip validation
- **D-04:** Other YAML sections (persona, department, etc.) are out of scope for Phase 28 — keep minimal, capabilities only. Persona files and DB frontmatter remain as-is for now.

### Responsibility expression syntax
- **D-05:** Each responsibility is a string containing a valid InnateScipt expression — action expressions `![]`, bundle expressions `{}`, reference expressions `@()`, or combinations
- **D-06:** Expressions are NOT evaluated at YAML load time — they're injected into LLM prompts as capability declarations so the ghost knows what it can do
- **D-07:** Example responsibilities for key ghosts:
  - EthanNg: `![fundamentals:feeds]`, `![technicals:oanda_api[pairs: major+minor]]` (surf report)
  - Sylvia: `![write_document]`, `{em.content.blog}`, `{em.content.thought-police}`
  - Nova: `![query_db]`, `![pipeline_status]`, `![claude_code]`
  - Eliana: `![build_tool]`, `![claude_code]`, `![query_db]`

### Tick engine integration
- **D-08:** New function `load-ghost-capabilities` reads YAML file for an agent, returns list of responsibility expression strings
- **D-09:** Action planner replaces `get-tools-for-agent` (scope-based tool-registry.json lookup) with `load-ghost-capabilities` (YAML-based)
- **D-10:** `format-tools-for-prompt` replaced with `format-capabilities-for-prompt` that lists InnateScipt expressions as the ghost's declared capabilities
- **D-11:** tool-registry.json continues to exist in Phase 28 as fallback — full removal is Phase 31. Ghosts WITH YAML files use capabilities; ghosts WITHOUT fall back to tool-registry.json scope matching

### Self-modification mechanism
- **D-12:** Cognition output can include `responsibility_add`, `responsibility_remove`, `responsibility_edit` mutations
- **D-13:** Each mutation validated via InnateScipt parse-round-trip before writing to YAML (reuses Phase 25 pattern from innate-builder.lisp)
- **D-14:** Self-modification = ghost edits its own YAML. Executive modification = executive edits a subordinate's YAML. Both use same validation path.
- **D-15:** YAML writes are atomic — read file, modify responsibilities list, write entire file back

### Claude's Discretion
- YAML parsing library choice (cl-yaml or custom parser for simple structure)
- Exact cognition output format for responsibility mutations
- Which ghosts get initial YAML files (recommend: all 8 executives + key staff like EthanNg)
- Error handling when YAML file doesn't exist (graceful fallback to tool-registry.json per D-11)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Tick engine capability discovery
- `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` — Current `get-tools-for-agent`, `format-tools-for-prompt`, `load-tool-registry` (being replaced)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — Where tools/capabilities get injected into LLM prompts (lines 313-470, 614-681)
- `/opt/project-noosphere-ghosts/config/tool-registry.json` — Current tool definitions with scope arrays (1264 lines)

### InnateScipt validation
- `/opt/project-noosphere-ghosts/lisp/runtime/innate-builder.lisp` — Parse-round-trip validation from Phase 25
- `/opt/innatescript/` — Innate interpreter with parser

### Agent identity
- `/root/gotcha-workspace/context/personas/` — Current persona markdown files
- master_chronicle `agents` table — id, tool_scope, department
- master_chronicle `documents` table — frontmatter with responsibilities/goals

### DB layer
- `/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp` — `db-perceive-responsibilities` function
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` — Package exports

### Requirements
- `.planning/REQUIREMENTS.md` — CAP-01 through CAP-07

### Project context
- Complete Success pipeline spec — EthanNg's surf report responsibilities are the reference implementation for ghost capabilities

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `innate-builder.lisp` — `validate-innate-expression` for parse-round-trip validation of responsibility strings
- `load-tool-registry` pattern in tool-socket.lisp — similar file-loading pattern for YAML
- `db-perceive-responsibilities` — currently reads from DB frontmatter, can be extended to merge YAML capabilities
- `format-tools-for-prompt` — template for new `format-capabilities-for-prompt`

### Established Patterns
- Config files in `/opt/project-noosphere-ghosts/config/` — tool-registry.json, provider-config.json
- Action planner builds system prompts by composing: persona + tools + responsibilities + hard prompts
- Cognition output parsing in action-executor.lisp already extracts structured blocks (tool_call, innate_expression)

### Integration Points
- `action-planner.lisp` lines 458-462 — tool prompt injection, replace with capabilities
- `tool-socket.lisp` — `get-tools-for-agent` becomes fallback-only
- `action-executor.lisp` — add responsibility mutation extraction alongside existing innate expression extraction
- `packages.lisp` — export new symbols

</code_context>

<specifics>
## Specific Ideas

- EthanNg's Complete Success responsibilities (`![fundamentals:feeds]`, `![technicals:oanda_api]`) are the flagship use case — if his YAML works, the pattern works for all ghosts
- The `responsibilities:` section in YAML mirrors the existing `responsibilities` field in DB document frontmatter — eventual migration path is YAML becomes authoritative, DB frontmatter becomes cache
- Executives modifying subordinate capabilities (CAP-06) is how Kathryn adds new trading tools to EthanNg's portfolio without Nathan's intervention

</specifics>

<deferred>
## Deferred Ideas

- Full tool-registry.json retirement — Phase 31 (Tool Migration)
- YAML sections beyond responsibilities (persona, department, drives) — future milestone
- Migrating DB frontmatter responsibilities to YAML — can happen incrementally after Phase 28
- Tool execution through InnateScipt commission — Phase 31 scope
- YAML-defined pipeline handoff chains — Phase 30 (Team Pipelines)

</deferred>

---

*Phase: 28-ghost-capabilities*
*Context gathered: 2026-03-30*
