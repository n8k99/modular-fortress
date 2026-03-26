# Phase 4: Tool Execution - Research

**Researched:** 2026-03-26
**Domain:** Common Lisp tool socket dispatch, Python tool implementations, Claude Code CLI integration, PostgreSQL agent scope management
**Confidence:** HIGH

## Summary

The tool execution infrastructure is remarkably mature. The tool-socket.lisp (319 lines) provides complete dispatch: registry loading, scope-based access control, tool_call block parsing from LLM output, and execution via subprocess/psql/special handlers. All 67 registered tools have their backing scripts on disk. The action-executor already integrates tool results into stage_notes and validates tool execution counts for anti-hallucination.

The primary work for this phase is: (1) fix a critical scope bug affecting memory tools, (2) register claude_code as a new tool, (3) audit and fix agent tool_scope assignments in the DB, and (4) verify the 4 priority tool categories actually execute end-to-end. The architecture is sound -- this is wiring and bug-fixing, not new system design.

**Primary recommendation:** Fix the `"*"` wildcard scope bug in tool-socket.lisp first (memory tools are completely broken), then register claude_code tool, audit DB scopes, and smoke-test the 4 priority categories.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01: tool-socket.lisp (319 lines) already implements registry, scope checking, prompt formatting, tool_call parsing
- D-02: 65+ tools registered in tool-registry.json with scope arrays
- D-03: Anti-hallucination via tools-executed count at action-executor line 175
- D-04: Focus on 4 categories: DB tools, task tools, code tools, memory tools
- D-05: Register claude_code as tool in tool-registry.json, consistent with existing dispatch
- D-06: Claude Code CLI invoked as `claude -p "<prompt>" --output-format json` with $0.50/request and 120s timeout
- D-07: Scope restricted to engineering agents (tool_scope includes 'engineering' or 'tools')
- D-08: Tool execution results stored in task stage_notes column for audit trail
- D-09: Existing tools-executed count check remains (sufficient for v1)
- D-10: Audit current tool_scope values in agents table, verify match to roles
- D-11: Key scope mappings defined per department

### Claude's Discretion
- Which specific tool implementations need fixes vs which already work
- Whether to add tool_timeout per-tool config in registry
- Error handling when a tool fails mid-execution (retry vs report)
- Whether external tools (web search, URL fetch) are in scope or deferred

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| TOOL-01 | Staff ghosts execute code tools via Claude Code CLI | D-05/D-06: Register claude_code tool, pattern from existing tool-builder.sh; Claude Code CLI v2.1.84 verified on system |
| TOOL-02 | Staff ghosts execute DB tools (query/mutate master_chronicle) | query_db already works (special case in tool-socket.lisp), write_document has special handler; search_documents uses hybrid_search.py |
| TOOL-03 | Staff ghosts execute API tools (doc creation, task updates, messages) | write_document, list_tasks, assign_task all have working Python scripts on disk |
| TOOL-04 | Staff ghosts execute external tools (web search, URL fetch, embedding) | Discretion: defer to later phase. No web_search or url_fetch tools registered. news_aggregator and article_fetcher exist for RSS/content. |
| TOOL-05 | Tool execution respects agent tool_scope | Scope checking works in get-tools-for-agent BUT wildcard "*" scope is broken for memory tools. Agent scopes need DB audit. |
| TOOL-06 | Tool execution results validated before task marked complete | Already works: validate-stage-output checks tools-executed count, rejects pure-prose output for pipeline stages |
</phase_requirements>

## Architecture Patterns

### Existing Tool Dispatch Flow (Already Implemented)
```
Ghost LLM Output
  -> parse-tool-calls (extract ```tool_call blocks)
  -> for each call: execute-tool-call
     -> normalize-tool-key (snake_case -> :KEYWORD)
     -> lookup in *tool-registry*
     -> check status != "not_built"
     -> check dangerous flag
     -> dispatch by type:
        1. Special case: query_db -> psql subprocess
        2. Special case: build_tool -> tool-builder.sh async
        3. Special case: write_document -> psql INSERT
        4. General: Python script via interpreter
        5. Command-based: /bin/bash -c
  -> results appended to stage_notes
  -> tools-executed count used in validation
```

### Tool Registry JSON Structure
```json
{
  "tools": {
    "tool_name": {
      "script": "/path/to/script.py",
      "description": "What the tool does",
      "parameters": { "param": "description" },
      "scope": ["engineering", "tools"],
      "dangerous": false,
      "interpreter": "/root/gotcha-workspace/.venv/bin/python3",
      "positional_arg": "param_name"
    }
  }
}
```

### Tool Result Flow
```
execute-tool-call returns string
  -> process-tool-calls collects (tool-name result) pairs
  -> execute-work-task appends "--- TOOL: name ---\nresult" to content
  -> content written to stage_notes via api-patch
  -> tools-executed count passed to validate-stage-output
  -> validation gates pipeline advancement
```

### Anti-Patterns to Avoid
- **Registering tools without backing scripts:** All 67 current tools have scripts on disk. Never add a registry entry without a working implementation.
- **String scope instead of array:** The `"*"` wildcard scope is broken. Always use arrays.
- **Inline content in tool_call JSON:** The prompt already warns ghosts not to put long content in JSON args (breaks parsing). The safe-json-extract fallback exists but is fragile.

## Critical Bugs Found

### BUG-1: Wildcard Scope "*" Broken for Memory Tools (HIGH PRIORITY)

**What:** `read_own_memory` and `write_own_memory` use `"scope": "*"` (a string). The Lisp `get-tools-for-agent` code treats non-list/non-vector scope as empty list `'()`, so `intersection` with any agent scope returns nil. These tools are invisible to ALL agents.

**Root cause:** Lines 50-54 of tool-socket.lisp:
```lisp
(tool-scope-raw (cond
    ((listp tool-scope) tool-scope)
    ((vectorp tool-scope) (coerce tool-scope 'list))
    (t '())))  ;; <- "*" string falls here, becomes empty
```

**Fix options (choose one):**
1. **Lisp fix:** Add `((stringp tool-scope)` case -- if `"*"` return t unconditionally, bypassing intersection check
2. **JSON fix:** Replace `"scope": "*"` with an array of all scopes, or a comprehensive array like `["engineering","tools","content","creative","research","trading","strategy","scheduling","tracking","reporting","compliance","legal","music","editorial","writing","social","market","analytics","signals","feeds","worldbuilding","publishing","security","kalshi","forex","portfolio","risk","cross_functional"]`

**Recommendation:** Lisp fix (option 1). Cleaner, supports future `"*"` tools. Add before the `(t '())` clause:
```lisp
((and (stringp tool-scope) (string-equal tool-scope "*")) :wildcard)
```
Then adjust the intersection check to short-circuit when `:wildcard`.

**Confidence:** HIGH -- verified by reading source code directly.

## Standard Stack

### Core (Already In Place)
| Component | Location | Purpose | Status |
|-----------|----------|---------|--------|
| tool-socket.lisp | /opt/project-noosphere-ghosts/lisp/runtime/ | Tool dispatch, registry, scope, parsing | Working (1 bug) |
| tool-registry.json | /opt/project-noosphere-ghosts/config/ | 67 tool definitions with scopes | Working (2 scope bugs) |
| action-executor.lisp | /opt/project-noosphere-ghosts/lisp/runtime/ | Tool integration, validation, pipeline | Working |
| gotcha-workspace tools | /root/gotcha-workspace/tools/ | Python implementations | All 67 scripts exist on disk |
| Claude Code CLI | /root/.local/bin/claude | LLM provider for code tools | v2.1.84 installed |

### Supporting
| Component | Location | Purpose |
|-----------|----------|---------|
| tool-builder.sh | /opt/project-noosphere-ghosts/tools/ | Existing Claude Code integration pattern |
| _config.py | /root/gotcha-workspace/tools/ | DB connection config for Python tools |
| af64.env | /opt/project-noosphere-ghosts/config/ | Runtime env vars (API keys, budget) |

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Tool dispatch | New execution framework | Existing tool-socket.lisp | 319 lines, already handles all dispatch patterns |
| Tool_call parsing | Custom parser | Existing parse-tool-calls | Already handles LLM output quirks with safe-json-extract fallback |
| Stage validation | New validation system | Existing validate-stage-output | Already checks tools-executed count + content length per stage |
| Pipeline advancement | New state machine | Existing advance-pipeline | Already handles stage->stage transitions with goal tracking |
| Claude Code invocation | New CLI wrapper | Existing tool-builder.sh pattern | Proven pattern: clean env, construct prompt, invoke CLI, capture output |

**Key insight:** This phase is about making existing infrastructure work reliably for 4 tool categories, not building new systems. The tool socket is complete -- the gaps are: one scope bug, one missing tool registration, and DB scope assignments.

## Detailed Tool Audit by Category

### Category 1: DB Tools
| Tool | Script | Special Handler | Tested Working |
|------|--------|----------------|----------------|
| query_db | (inline psql) | Yes - direct subprocess | Needs smoke test |
| write_document | (inline psql INSERT) | Yes - special_handler | Needs smoke test |
| search_documents | hybrid_search.py | No - general Python | Needs smoke test |

**query_db notes:** Read-only enforced (checks for INSERT/UPDATE/DELETE/DROP/ALTER/TRUNCATE). Output truncated to 2000 chars. Uses `sudo -u postgres psql`. Safety is adequate for v1.

**write_document notes:** Uses ON CONFLICT (path) DO UPDATE for upsert. Single-quote escaping is manual (char-by-char loop). Works but content with embedded quotes needs testing.

### Category 2: Task Tools
| Tool | Script | Working |
|------|--------|---------|
| list_tasks | list_unassigned_tasks.py | Needs smoke test |
| assign_task | assign_task.py | Needs smoke test |

**Note:** These use the general Python dispatch path (interpreter + script + CLI args). The `build-cli-args` function constructs positional + named flags from tool call args.

### Category 3: Code Tools
| Tool | Script | Working |
|------|--------|---------|
| build_tool | tool-builder.sh | Working (async, spawns Claude Code) |
| codebase_scanner | codebase_scanner/scan.py | Needs smoke test |
| claude_code | NOT YET REGISTERED | Must create |

**claude_code registration plan:**
- Script: new shell script or inline special handler
- Pattern: follows tool-builder.sh env cleanup + `claude -p` invocation
- Key difference: synchronous (returns result), not async like build_tool
- Timeout: 120s (from af64.env discussion, though not explicitly set there)
- Budget: $0.50/request constraint is at the Claude Code CLI level, not tool socket level

### Category 4: Memory Tools
| Tool | Script | Working |
|------|--------|---------|
| read_own_memory | read_own_memory.py | Script works, SCOPE BUG blocks access |
| write_own_memory | write_own_memory.py | Script works, SCOPE BUG blocks access |

**Memory tool scripts analysis:**
- `read_own_memory.py`: Reads `{agent_id}_memories` column from vault_notes daily notes. Takes agent_id positional arg. Uses psycopg2 direct (not via _config.py). Has `--date` and `--days` flags.
- `write_own_memory.py`: Appends timestamped entry to same column. Takes agent_id and memory_text as positional args. Max 500 chars.
- Both tools have `"interpreter"` set in registry (explicit venv path), which the general Python dispatch uses.

## Agent Tool Scope Audit

### Executive Agents (status=active, 8 total)

| Agent | Department | Current Scopes | Issues |
|-------|-----------|---------------|--------|
| eliana | Engineering | exec,github,docker,tasks,wiki,decision,engineering,tools,specs,builder,strategy,compliance,monitoring,maintenance,reporting | Good -- has engineering+tools |
| nova | Systems | memory,temporal,system,all,strategy,trading,market,engineering,content,scheduling,reporting,compliance,creative,research,decision | Good -- has 'all' and all major scopes |
| kathryn | Executive | strategy,kalshi,trading,market,portfolio,risk,compliance,forex,execution,watchlist,alerts,signals,analytics,reporting,decision,scheduling,research,specs,strategy | Good -- covers trading/strategy |
| sylvia | Content & Brand | content,brand,social,worldbuilding,publishing,decision,editorial,writing,creative,feeds,research,specs,strategy | Good -- covers content domain |
| vincent | Creative | creative,content,writing,research,decision,visual,image,video,covers,branding,specs,strategy | Good -- covers creative |
| jmax | Legal | legal,regulation,compliance,risk,policy,ip,decision,audit,trading,kalshi,specs,strategy | Good -- covers legal+compliance |
| sarah | Office of CEO | scheduling,tracking,reporting,content,research,calendar | Missing: no 'tools' scope -- intentional (PA role) |
| lrm | Research | research,audio,notation,archive,wiki,decision,content,creative,music,writing,specs,strategy | Good -- covers music/research |

### Staff Agents (status=dormant, 56 total) - Sample

| Department | Count | Scopes | Issues |
|-----------|-------|--------|--------|
| Engineering | 8 | engineering,tools,specs,builder,research,analytics,monitoring,maintenance | Good -- all have engineering+tools |
| art | 7 | creative,content,writing,research | Missing: no 'tools' scope, cannot use build_tool, codebase tools |
| content_brand | 9 | content,editorial,writing,creative,social,research,feeds[,worldbuilding] | Good for content tools |
| audience_experience | 5 | analytics,research,content,social,feeds | Limited but appropriate |
| legal | 8 | Various compliance/security/research scopes | Appropriate per role |
| strategic_office | 5 | Various strategy/market/trading scopes | Good for trading tools |
| music | 4 | music,research,content,creative,writing | Appropriate |
| support | 3 | research,content,creative,analytics | Limited but appropriate |
| cross_functional | 2 | cross_functional,research,content,social | Very limited |
| digital_partnership | 2 | Varies per agent | carmen_delgado has security+compliance |
| social_impact | 1 | analytics,research,social,content,reporting | Appropriate |
| Operations | 1 (Lara) | content,social,reporting,feeds,research | Appropriate for ops |

### Scope Issues to Fix
1. **Memory tools broken for ALL agents** due to `"*"` scope bug (BUG-1 above)
2. **Staff agents are dormant** -- they won't tick unless activated. But tool_scope is correct for when they do.
3. **D-07 claude_code scope:** Engineering agents (engineering+tools scope) cover: Eliana + 8 engineering staff. This is correct per decision.
4. **No agent has af64_id set except executives** -- staff agents have null af64_id. This means they need to be referenced by `id` column (e.g., "devin", "casey").

## Claude Code Tool Registration Design

### Registration Entry for tool-registry.json
```json
"claude_code": {
  "script": "/opt/project-noosphere-ghosts/tools/claude-code-tool.sh",
  "description": "Execute a coding task using Claude Code CLI. Reads/writes files, runs commands, performs git operations. Returns structured JSON output.",
  "parameters": {
    "prompt": "Task description for Claude Code to execute",
    "allowed_tools": "Optional comma-separated tools (default: Read,Grep,Glob,Write,Edit,Bash)"
  },
  "scope": ["engineering", "tools"],
  "dangerous": false,
  "interpreter": "/bin/bash"
}
```

### Shell Script Pattern (from tool-builder.sh)
Key requirements extracted from the working tool-builder.sh:
1. Clean environment: `unset ANTHROPIC_API_KEY SBCL_HOME ASDF_OUTPUT_TRANSLATIONS`
2. Set HOME/PATH/TERM explicitly
3. Use `claude -p "<prompt>" --output-format json`
4. Add `--allowedTools` flag for sandboxing
5. Add `--model sonnet` (or configurable)
6. Capture stdout as result, return to tool socket
7. Timeout: 120 seconds via Lisp `uiop:run-program` (or wrapper `timeout` command)

### Differences from build_tool
| Aspect | build_tool | claude_code |
|--------|-----------|-------------|
| Invocation | Async (uiop:launch-program) | Synchronous (uiop:run-program) |
| Output | Log file | Captured stdout returned to tool socket |
| Working dir | gotcha-workspace | Configurable (or project root) |
| Purpose | Build Python tools from specs | General code read/write/run |
| Timeout | None (async) | 120s hard limit |

### Special Handler vs Script
**Recommendation:** Use a shell script (not special handler). The build_tool pattern works, and a script is easier to test independently. The tool socket's general script dispatch already handles this via the interpreter field.

## Common Pitfalls

### Pitfall 1: Lisp JSON Underscore-to-Hyphen Conversion
**What goes wrong:** Tool call args with underscored keys (e.g., `tool_name`) get converted to hyphenated keywords (`:TOOL-NAME`) by the Lisp JSON parser.
**Why it happens:** af64's custom JSON parser normalizes all keys.
**How to avoid:** The existing code already handles this (see `gethash :TOOL_NAME` vs `gethash :TOOL-NAME` fallbacks in build_tool handler). New special handlers must do the same.
**Warning signs:** Tool executes but args are nil/empty.

### Pitfall 2: Tool Output Truncation
**What goes wrong:** Tool results silently truncated. query_db caps at 2000 chars, general Python at 3000, stage_notes at 4000.
**Why it happens:** Hardcoded limits in execute-tool-call and execute-work-task.
**How to avoid:** Acceptable for v1. Document the limits. Ghosts producing large outputs (e.g., codebase_scanner) may lose tail content.
**Warning signs:** Incomplete results in stage_notes.

### Pitfall 3: Claude Code CLI Environment Pollution
**What goes wrong:** SBCL environment variables (SBCL_HOME, etc.) confuse the Claude Code CLI OAuth/auth flow.
**Why it happens:** tool-socket.lisp runs as a subprocess of the SBCL process. Environment inherits.
**How to avoid:** tool-builder.sh already demonstrates the fix: explicit unset of SBCL vars + clean PATH/HOME. Any claude_code tool script must replicate this.
**Warning signs:** Claude Code fails with auth errors or hangs.

### Pitfall 4: Dormant Staff Agents
**What goes wrong:** Staff agents have tool_scope set correctly but status=dormant. They won't tick.
**Why it happens:** The tick engine only processes active agents.
**How to avoid:** Tool execution phase must consider that staff ghosts need to be activated (energy > 0, status=active) for tools to actually be exercised. Activation may be a separate concern or prerequisite.
**Warning signs:** Everything looks correct but no staff ever executes tools.

### Pitfall 5: Memory Tool Column Dependency
**What goes wrong:** read_own_memory/write_own_memory fail because the agent doesn't have a `{agent_id}_memories` column in vault_notes.
**Why it happens:** Memory columns are per-agent and must be created ahead of time with ALTER TABLE.
**How to avoid:** Verify which agents have memory columns. The tools check for column existence and fail gracefully.
**Warning signs:** "No memory column found for {agent_id}" in tool output.

## Code Examples

### Fixing the Wildcard Scope Bug (tool-socket.lisp)
```lisp
;; In get-tools-for-agent, replace the tool filtering block:
;; Add wildcard check before intersection
(when (and (not (equal status "not_built"))
           (or (and (stringp (gethash :SCOPE tool-def))
                    (string-equal (gethash :SCOPE tool-def) "*"))
               (intersection scope tool-scope-list :test #'string-equal)))
  (push (cons tool-name tool-def) available))
```
Source: Derived from tool-socket.lisp lines 56-59

### Claude Code Tool Shell Script Pattern
```bash
#!/bin/bash
# claude-code-tool.sh -- Execute coding tasks via Claude Code CLI
# Called by tool-socket.lisp general script dispatch
export HOME=/root
export PATH="/root/.local/bin:/root/.nvm/versions/node/v22.22.0/bin:/usr/local/bin:/usr/bin:/bin"
export TERM=dumb
unset ANTHROPIC_API_KEY CLAUDE_API_KEY XDG_CONFIG_HOME XDG_DATA_HOME
unset SBCL_HOME ASDF_OUTPUT_TRANSLATIONS

PROMPT="$1"
ALLOWED_TOOLS="${2:-Read,Grep,Glob,Write,Edit,Bash}"
TIMEOUT=120

if [ -z "$PROMPT" ]; then
    echo '{"error": "No prompt provided"}'
    exit 1
fi

timeout ${TIMEOUT}s claude -p "$PROMPT" \
  --allowedTools "$ALLOWED_TOOLS" \
  --model sonnet \
  --output-format json \
  2>/tmp/claude-code-tool-err.log

EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ]; then
    echo "ERROR: Claude Code exited with code $EXIT_CODE"
fi
```
Source: Adapted from /opt/project-noosphere-ghosts/tools/tool-builder.sh

### Tool Registry Entry for claude_code
```json
"claude_code": {
  "script": "/opt/project-noosphere-ghosts/tools/claude-code-tool.sh",
  "description": "Execute a coding task using Claude Code CLI. Can read/write files, run commands, search code. Returns structured output.",
  "parameters": {
    "prompt": "The task to execute (describe what to read, write, analyze, or build)",
    "allowed_tools": "Comma-separated Claude Code tools to allow (default: Read,Grep,Glob,Write,Edit,Bash)"
  },
  "scope": ["engineering", "tools"],
  "positional_arg": "prompt",
  "dangerous": false
}
```

### SQL to Audit Agent Scopes
```sql
-- Verify all active agents have appropriate tool_scope
SELECT id, full_name, department, tool_scope,
       'engineering' = ANY(tool_scope) as has_engineering,
       'tools' = ANY(tool_scope) as has_tools,
       'content' = ANY(tool_scope) as has_content
FROM agents
WHERE status IN ('active', 'dormant')
ORDER BY department, id;
```

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual smoke tests via SBCL REPL + Python CLI |
| Config file | None (ad-hoc testing) |
| Quick run command | `python3 /root/gotcha-workspace/tools/engineering/read_own_memory.py eliana --days 1` |
| Full suite command | See per-requirement tests below |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| TOOL-01 | Claude Code CLI tool executes | smoke | `/opt/project-noosphere-ghosts/tools/claude-code-tool.sh "echo hello world"` | Wave 0 (create script) |
| TOOL-02 | DB tools execute (query_db) | smoke | Invoke via Lisp REPL or simulate: `sudo -u postgres psql -d master_chronicle -t -c "SELECT count(*) FROM tasks;"` | Existing |
| TOOL-03 | API tools execute (list_tasks, write_document) | smoke | `/root/gotcha-workspace/.venv/bin/python3 /root/gotcha-workspace/tools/engineering/list_unassigned_tasks.py` | Existing |
| TOOL-04 | External tools (deferred) | manual-only | N/A -- recommend deferring web_search/url_fetch | N/A |
| TOOL-05 | Scope enforcement works | unit | Test get-tools-for-agent returns correct tools for agent with known scope | Wave 0 (manual REPL test) |
| TOOL-06 | Anti-hallucination validation | existing | validate-stage-output already tested by Phase 3 pipeline | Existing |

### Sampling Rate
- **Per task commit:** Smoke test the modified tool (CLI invocation)
- **Per wave merge:** Run all 4 category smoke tests
- **Phase gate:** Start noosphere-ghosts briefly, trigger a test tick with an engineering agent, verify tool execution appears in logs

### Wave 0 Gaps
- [ ] `/opt/project-noosphere-ghosts/tools/claude-code-tool.sh` -- new script for TOOL-01
- [ ] Manual REPL test procedure for scope verification (TOOL-05)

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| SBCL | Lisp runtime | Assumed (noosphere-ghosts runs on it) | -- | -- |
| Claude Code CLI | claude_code tool | Yes | 2.1.84 | -- |
| Python 3 (venv) | All Python tools | Yes | /root/gotcha-workspace/.venv/bin/python3 | -- |
| PostgreSQL | DB tools, memory tools | Yes | Running on 127.0.0.1:5432 | -- |
| psycopg2 | Memory tools | Yes (in venv) | -- | -- |
| dpn-api | Task/agent API calls | Yes | Port 8080 | -- |

**Missing dependencies with no fallback:** None

**Missing dependencies with fallback:** None

## Open Questions

1. **Staff agent activation**
   - What we know: All 56 staff agents are status=dormant. Tool execution requires agents to tick.
   - What's unclear: Is activation of staff agents in scope for this phase, or handled elsewhere?
   - Recommendation: Out of scope for this phase. Phase focuses on making tools work when called. Activation is a tick-engine/energy concern.

2. **Memory column existence for staff agents**
   - What we know: read/write_own_memory requires `{agent_id}_memories` column in vault_notes
   - What's unclear: Which agents have memory columns already created?
   - Recommendation: Quick SQL check during planning. If missing, add ALTER TABLE statements.

3. **TOOL-04 (external tools) scope**
   - What we know: No web_search or url_fetch tools are registered. news_aggregator and article_fetcher exist but are RSS-specific.
   - What's unclear: Whether the decision-maker wants TOOL-04 in this phase
   - Recommendation: Defer. The 3 other categories provide full execution capability. External tools can be Phase 4.5 or v2.

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` -- read in full, 319 lines
- `/opt/project-noosphere-ghosts/config/tool-registry.json` -- read in full, 67 tools audited
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` -- read key sections (validation, work-task execution, pipeline)
- `/opt/project-noosphere-ghosts/tools/tool-builder.sh` -- read in full, Claude Code invocation pattern
- `/opt/project-noosphere-ghosts/config/af64.env` -- read in full, runtime config
- Live DB queries: agents table (all 64 agents + scopes), tasks table schema

### Secondary (MEDIUM confidence)
- `/root/gotcha-workspace/tools/engineering/read_own_memory.py` -- read in full
- `/root/gotcha-workspace/tools/engineering/write_own_memory.py` -- read in full
- Tool script existence audit: all 67 scripts verified on disk via Python os.path.exists

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all code read directly, no external research needed
- Architecture: HIGH -- existing dispatch flow traced through 3 source files
- Pitfalls: HIGH -- bugs found by reading actual Lisp source; env issues from existing tool-builder.sh
- Tool audit: HIGH -- all 67 scripts verified on disk, DB scopes queried live

**Research date:** 2026-03-26
**Valid until:** 2026-04-25 (stable codebase, no external dependencies changing)
