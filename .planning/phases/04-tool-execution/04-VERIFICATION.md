---
phase: 04-tool-execution
verified: 2026-03-26T07:30:00Z
status: gaps_found
score: 4/5 success criteria verified
re_verification: false
gaps:
  - truth: "External tools (web search, URL fetch) are available to authorized ghosts for research-type tasks"
    status: failed
    reason: "TOOL-04 has no backing implementation. No web_search or url_fetch tool exists in tool-registry.json. The phase plan deferred this with rationale, and the E2E test marks it SKIP, but REQUIREMENTS.md marks TOOL-04 as 'Complete' which is a tracking inconsistency."
    artifacts:
      - path: "/opt/project-noosphere-ghosts/config/tool-registry.json"
        issue: "No web_search or url_fetch entry. Only RSS-based article_fetcher and news_aggregator exist, which are domain-specific pipeline tools, not general external tools per TOOL-04 spec."
    missing:
      - "Either implement web_search/url_fetch tools and register them, or explicitly mark TOOL-04 as DEFERRED in REQUIREMENTS.md (not 'Complete') so the tracking record is accurate"
human_verification:
  - test: "Live Claude Code CLI invocation end-to-end"
    expected: "A ghost with engineering scope calls claude-code-tool.sh with a real prompt, receives structured JSON output, and the result appears in task stage_notes"
    why_human: "Test script --quick mode skips live LLM call (TOOL-01). Full live invocation requires an active claude CLI session with quota available. Cannot verify without running the ghost engine."
  - test: "Ghost scope enforcement at runtime (not just registry)"
    expected: "A ghost with content-only scope (e.g., sylvia) cannot call claude_code when executing a task — tool is absent from its available tools list at cognition time"
    why_human: "Scope enforcement is verified statically (jq + grep checks). Runtime behavior — whether get-tools-for-agent is called correctly for each ghost before cognition — requires a live ghost tick."
---

# Phase 4: Tool Execution Verification Report

**Phase Goal:** Staff ghosts execute real work using authorized tools and validate results before marking tasks complete
**Verified:** 2026-03-26T07:30:00Z
**Status:** gaps_found (1 gap — TOOL-04 implementation absent, tracking record inconsistent)
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths (from ROADMAP.md Success Criteria)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A staff ghost assigned a code task invokes Claude Code CLI to read/write files or run commands, and the result is persisted | ✓ VERIFIED | claude-code-tool.sh exists, executable, contains `claude -p` + 120s timeout + JSON output; tool-registry.json has claude_code entry wired to the script; action-executor.lisp persists tool results to stage_notes (lines 367-376) |
| 2 | A staff ghost can query or mutate master_chronicle via dpn-api DB tools and API tools | ✓ VERIFIED | E2E test passed: SELECT count(*) FROM tasks returned 2535; write_document INSERT/UPSERT returned id=61191; list_unassigned_tasks.py exited 0; Task API returned valid JSON array |
| 3 | Tool execution respects agent tool_scope — a ghost without code tool authorization cannot invoke Claude Code CLI | ✓ VERIFIED | Wildcard scope fix present in tool-socket.lisp (stringp check + :wildcard keyword, lines 53-61); claude_code scope is ["engineering","tools"]; eliana has engineering in scope; sylvia has content but NOT engineering (verified via API) |
| 4 | Tool execution results are validated (output checked, not just "I did it") before the task status moves to done | ✓ VERIFIED | validate-stage-output in action-executor.lisp (line 78) checks tools-executed count; pipeline stages return REJECTED with "0 tools executed" message (line 192); tools-executed is a deterministic count from process-tool-calls, not parseable from ghost content |
| 5 | External tools (web search, URL fetch) are available to authorized ghosts for research-type tasks | ✗ FAILED | No web_search or url_fetch tool exists in tool-registry.json. TOOL-04 explicitly deferred by Plan 02 with documented rationale. E2E test marks it SKIP. However, REQUIREMENTS.md shows TOOL-04 as [x] Complete — tracking record contradicts the deferral. |

**Score:** 4/5 success criteria verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` | Wildcard scope handling for scope='*' | ✓ VERIFIED | Contains `(stringp tool-scope)` check (1 match) and `:wildcard` keyword (3 matches). get-tools-for-agent short-circuits intersection when tool-scope is :wildcard. |
| `/opt/project-noosphere-ghosts/tools/claude-code-tool.sh` | Shell script for Claude Code CLI invocation | ✓ VERIFIED | 34-line substantive script. Contains `claude -p`, 120s timeout, SBCL env cleanup (`unset SBCL_HOME`), `--output-format json`. Executable bit set. |
| `/opt/project-noosphere-ghosts/config/tool-registry.json` | claude_code tool registration entry | ✓ VERIFIED | claude_code entry present with script pointing to claude-code-tool.sh, scope: ["engineering","tools"], interpreter: "/bin/bash". |
| `.planning/phases/04-tool-execution/test_tools_e2e.sh` | E2E smoke test covering all tool categories | ✓ VERIFIED | 241-line substantive script. Contains TOOL-01 through TOOL-06 sections, D-08 stage_notes verification, --quick flag. Executable. |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `config/tool-registry.json` | `tools/claude-code-tool.sh` | script path in claude_code registry entry | ✓ WIRED | `"script": "/opt/project-noosphere-ghosts/tools/claude-code-tool.sh"` confirmed in registry |
| `lisp/runtime/tool-socket.lisp` | `config/tool-registry.json` | load-tool-registry reads JSON | ✓ WIRED | Lines 19-20: hardcoded path `config/tool-registry.json` in load-tool-registry; sets *tool-registry* from :TOOLS key |
| `test_tools_e2e.sh` | `tools/claude-code-tool.sh` | invokes script directly | ✓ WIRED | Line 13: `CLAUDE_TOOL="/opt/project-noosphere-ghosts/tools/claude-code-tool.sh"` and line 57 invokes it |
| `test_tools_e2e.sh` | `config/tool-registry.json` | validates tool registration with jq | ✓ WIRED | Line 12: `REGISTRY=...tool-registry.json`; lines 49, 133, 141 use jq against it |
| `test_tools_e2e.sh` | `master_chronicle.agents` | SQL audit of tool_scope column | ✓ WIRED | E2E test uses psql + API calls to verify agent scope; DB confirms 0 active agents with NULL/empty tool_scope |
| `action-executor.lisp` | `tasks.stage_notes` | api-patch with tool results | ✓ WIRED | Lines 373-376: api-patch to `/api/af64/tasks/:id` with :stage-notes key; tool results appended to content before write |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| `action-executor.lisp` execute-work-task | `tools-executed` count | `process-tool-calls` return length (line 364) | Yes — deterministic count from actual socket execution | ✓ FLOWING |
| `action-executor.lisp` execute-work-task | stage_notes content | ghost output + tool results concatenated (line 372) | Yes — actual tool output appended to actual LLM content | ✓ FLOWING |
| `tool-socket.lisp` get-tools-for-agent | `scope` list | API GET `/api/agents/:id` response (line 35) | Yes — live DB-backed agent record via dpn-api (confirmed: eliana has engineering, sylvia has content) | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| E2E test --quick completes all non-live checks | `bash test_tools_e2e.sh --quick` | PASS: 16, FAIL: 0, SKIP: 2, exit 0 | ✓ PASS |
| DB query tool path works | `psql ... SELECT count(*) FROM tasks` | 2535 (numeric) | ✓ PASS |
| write_document path works | `psql ... INSERT INTO documents ... RETURNING id` | 61191 (numeric ID) | ✓ PASS |
| claude_code scope enforced | `jq '.tools.claude_code.scope'` | ["engineering","tools"] | ✓ PASS |
| read_own_memory scope is wildcard | `jq '.tools.read_own_memory.scope'` | "*" | ✓ PASS |
| Wildcard fix in tool-socket.lisp | `grep "stringp tool-scope"` | 1 match | ✓ PASS |
| Anti-hallucination present | `grep "REJECTED.*0 tools executed"` in action-executor.lisp | 1 match (line 192) | ✓ PASS |
| Agent scope audit (0 nulls) | `SELECT count(*) FROM agents WHERE ... tool_scope IS NULL` | 0 | ✓ PASS |
| Live Claude Code CLI invocation | Skipped — requires live LLM quota | n/a | ? SKIP |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| TOOL-01 | 04-01-PLAN.md | Staff ghosts execute code tools via Claude Code CLI | ✓ SATISFIED | claude-code-tool.sh created, executable; claude_code registered in tool-registry.json with engineering+tools scope; action-executor persists results to stage_notes |
| TOOL-02 | 04-02-PLAN.md | Staff ghosts execute DB tools (query/mutate master_chronicle via dpn-api) | ✓ SATISFIED | E2E test confirms: SELECT count returns 2535, INSERT UPSERT returns numeric ID; psql path confirmed working |
| TOOL-03 | 04-02-PLAN.md | Staff ghosts execute API tools (dpn-api endpoints) | ✓ SATISFIED | E2E test: list_unassigned_tasks.py exits 0; Task API returns valid JSON array |
| TOOL-04 | 04-02-PLAN.md | Staff ghosts execute external tools (web search, URL fetch) | ✗ DEFERRED (tracking gap) | No web_search/url_fetch implementation exists. Plan 02 correctly deferred with rationale. E2E test marks SKIP. But REQUIREMENTS.md marks [x] Complete — mismatch. |
| TOOL-05 | 04-01-PLAN.md, 04-02-PLAN.md | Tool execution respects agent tool_scope | ✓ SATISFIED | Wildcard scope fix verified (3 :wildcard references in tool-socket.lisp); scope enforcement verified via API for eliana (engineering) and sylvia (content, not engineering); 0 agents with null scope |
| TOOL-06 | 04-02-PLAN.md | Tool results validated before task marked complete | ✓ SATISFIED | validate-stage-output in action-executor.lisp checks tools-executed > 0 for pipeline stages; REJECTED message confirmed at line 192; deterministic count from process-tool-calls cannot be faked by ghost |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `REQUIREMENTS.md` | tracking table | TOOL-04 marked [x] Complete despite no implementation | ⚠️ Warning | Tracking record contradicts deferral; could mislead Phase 5 planning or stakeholder review. Not a code bug — no runtime impact. |

No code stubs, placeholder returns, or TODO/FIXME anti-patterns found in the four key implementation files.

### Human Verification Required

#### 1. Live Claude Code CLI End-to-End Invocation

**Test:** Start noosphere-ghosts (`pm2 start noosphere-ghosts`), assign a simple code task to an engineering ghost (e.g., eliana or a staff engineer), observe the tick logs for `[tools] <agent> executed 1 tool(s)`, then check `SELECT stage_notes FROM tasks WHERE id = '<task_id>'` to confirm tool output is persisted.
**Expected:** stage_notes contains "--- TOOL: claude_code ---" followed by structured JSON output from Claude Code CLI. Task status advances through pipeline stage.
**Why human:** Live LLM call requires active Claude Code CLI quota. The --quick E2E test skips the live invocation (TOOL-01 smoke test row 4 is SKIP). Runtime ghost behavior with actual cognition cannot be verified by grep/file inspection alone.

#### 2. Ghost Scope Enforcement at Runtime

**Test:** Ensure a content ghost (sylvia or content staff) is assigned a task with a pipeline stage that would trigger tool use. Observe that `get-tools-for-agent` returns no claude_code entry in the available tools formatted in the prompt.
**Expected:** Claude Code is absent from the AVAILABLE TOOLS block in sylvia's cognition prompt. Sylvia cannot accidentally invoke it.
**Why human:** Scope enforcement is statically verified (registry scopes + DB scopes match). But whether `get-tools-for-agent` is called at all during cognition dispatch, and whether the formatted tools block is correctly injected into the prompt, requires runtime observation.

### Gaps Summary

**1 gap — TOOL-04 implementation absent with tracking inconsistency**

The phase plan correctly identified that no `web_search` or `url_fetch` implementations exist and deferred TOOL-04 with explicit rationale ("No web_search or url_fetch implementations exist. RSS-based news_aggregator and article_fetcher exist but are not general external tools. Defer to Phase 4.5 or v2."). The E2E test marks TOOL-04 as SKIP, not FAIL. This was an appropriate judgment call.

However, REQUIREMENTS.md was not updated to reflect the deferral — it still shows `[x] TOOL-04: Complete` and the tracking table shows `TOOL-04 | Phase 4 | Complete`. This creates a false impression that external tools are implemented when they are not.

**Recommended resolution:** Update REQUIREMENTS.md to mark TOOL-04 as DEFERRED (not Complete), with a note pointing to Phase 4.5 or v2 for the actual implementation. This does not require re-opening Phase 4 — it is a documentation correction. The underlying phase goal ("Staff ghosts execute real work using authorized tools and validate results") is substantially achieved for the 5 tools that do exist.

**Overall assessment:** Phase 4 goal is substantially achieved. The four implemented tool categories (code via Claude Code CLI, DB via psql, API via dpn-api, memory via read/write_own_memory) are wired end-to-end, scope enforcement is functional, anti-hallucination validation is in place and non-fakeable, and tool results reach stage_notes. The sole gap is a documentation tracking error for a feature that was explicitly and correctly deferred.

---

_Verified: 2026-03-26T07:30:00Z_
_Verifier: Claude (gsd-verifier)_
