# Phase 13: Operations Pipeline - Research

**Researched:** 2026-03-28
**Domain:** Ghost operational automation -- wiring Nova's cognition to execute Python scripts via standing order triggers
**Confidence:** HIGH

## Summary

Phase 13 migrates Nova's 6 operational cron jobs from OpenClaw into the ghost standing orders framework built in Phase 12. The core challenge is NOT scheduling (Phase 12 solved that) but the execution gap: when a standing order fires, the tick engine creates a **project review** cognition job for Nova, but `execute-project-review` in action-executor.lisp does NOT invoke `process-tool-calls`. Project reviews can only parse task mutations (CREATE_TASK, DELEGATE, COMPLETE, CLASSIFY) and handoffs -- they cannot execute tools. This is the critical architectural gap this phase must close.

The operational scripts themselves (health_check.py, daily_note_populate.py, nightly_daily_to_weekly.py, rollup_weekly.py, rollup_monthly.py, podcast_watcher.py) are proven and tested. They need to be registered in the tool registry and made available to Nova. Additionally, the action planner prompt for standing-order-triggered project reviews needs a schedule-label-to-tool mapping so Nova knows which tool to call for each fired label.

**Primary recommendation:** Add tool execution (process-tool-calls) to `execute-project-review`, register all 6 operational scripts in tool-registry.json with Nova-accessible scopes, and add a label-to-tool mapping section in the action planner's standing order prompt. Add podcast watcher schedule to Project #14.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Nova executes existing gotcha-workspace Python scripts via the `execute_code` tool (Claude CLI). No rewrite of operational scripts -- they are proven and tested.
- **D-02:** Each standing order label maps to a specific tool invocation: "Daily Health Check" runs `health_check.py`, "Daily Note Population" runs `daily_note_populate.py`, "Nightly Synthesis" runs `nightly_daily_to_weekly.py` (or synthesis equivalent), etc.
- **D-03:** The action planner prompt must include enough context for Nova to know WHICH script to run based on the schedule label. This is a prompt engineering task, not a code architecture task.
- **D-04:** Operational output goes to the conversations table attributed to Nova. Health check summaries, synthesis results, and podcast reports all post as Nova's conversation messages.
- **D-05:** Discord delivery is not in scope for this phase. Conversations are the noosphere-native output. If Discord posting is needed, it's a separate concern handled by existing bridges or a future phase.
- **D-06:** Temporal compression uses the existing schedule entries on Project #14. Nightly synthesis fires at 04:05 UTC, weekly finalization fires Saturday 04:30 UTC, monthly fires 1st at 05:00 UTC. These are independent standing order fires, not chained tasks.
- **D-07:** Each temporal level reads the output of the prior level from vault_notes (daily reads today's note, weekly reads the week's dailies, monthly reads the month's weeklies). This is the existing pattern in the Python scripts -- no new infrastructure needed.
- **D-08:** The scripts already handle the temporal cascade logic (wikilinks, frontmatter references). Nova just needs to execute them and report the result.
- **D-09:** Nova executes `podcast_watcher.py` on the "Podcast Watch" schedule. The script checks RSS feeds for Living Room Music and Myths of Orbis, posts new episodes to Discord via webhook.
- **D-10:** Podcast watcher is currently scheduled at 23:10 UTC in OpenClaw. This schedule should be added to Project #14 if not already present.
- **D-11:** The key challenge is prompt engineering: Nova's project review cognition prompt must translate schedule labels into specific tool invocations. The action planner needs a mapping section that says "when you see label X, run tool Y with args Z."
- **D-12:** This mapping lives in the cognition prompt template (action-planner.lisp), not in the noosphere. It's code, not data.

### Claude's Discretion
- Exact prompt wording for the schedule-to-tool mapping
- Whether to batch multiple schedule fires in a single tick (e.g., if daily note population and health check fire close together)
- Error handling if a script fails -- retry logic or just report the failure in conversations

### Deferred Ideas (OUT OF SCOPE)
- Discord output bridge for ghost conversations (ghosts post to noosphere, a bridge publishes to Discord)
- Tasks archive migration (Discord #tasks archive every 6h) -- separate from Nova's operational cadence
- Pipeline wakeup job migration -- this is an OpenClaw internal concern, not an operational pipeline
- Conversations poll migration -- also OpenClaw internal
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| OPS-01 | Daily system health check executes as ghost work under Project #14, owned by Nova | Tool registry registration of health_check.py + tool execution in project reviews + prompt mapping for "Daily Health Check" label |
| OPS-02 | Daily note population and nightly synthesis execute as ghost work attributed to Nova | Tool registry registration of daily_note_populate.py and nightly_daily_to_weekly.py + prompt mapping for "Daily Note Population" and "Nightly Synthesis" labels |
| OPS-03 | Podcast watcher runs on schedule, checks feeds, posts new episodes | Tool registry registration of podcast_watcher.py + schedule addition to Project #14 + prompt mapping for "Podcast Watch" label |
| OPS-04 | Weekly and monthly finalization (temporal compression) execute as ghost work with specific agent attribution | Tool registry registration of rollup_weekly.py and rollup_monthly.py + prompt mapping for "Weekly Finalization" and "Monthly Finalization" labels |
</phase_requirements>

## Architecture Patterns

### Critical Gap: Project Reviews Cannot Execute Tools

The current `execute-project-review` function in action-executor.lisp (line 921) does NOT call `process-tool-calls`. It only:
1. Posts the cognition output as a conversation message
2. Parses task mutations (CREATE_TASK, DELEGATE, COMPLETE, CLASSIFY)
3. Routes handoffs

In contrast, `execute-work-task` (line 390) calls `process-tool-calls` (line 403) which parses `tool_call` blocks and executes tools via the tool socket.

**Fix required:** Add `process-tool-calls` invocation to `execute-project-review` so that standing-order-triggered project reviews can execute tools. This is the minimum viable change to enable operations pipeline work.

### Tool Registry Gap

The 6 operational scripts are NOT registered in `/opt/project-noosphere-ghosts/config/tool-registry.json`. Only `health_check.py` exists there (as `self_improvement`), but it's scoped to `["engineering", "monitoring", "maintenance"]` and marked `dangerous: true`.

**Scripts needing registration:**

| Script | Registry Name | Path | Args |
|--------|--------------|------|------|
| health_check.py | `ops_health_check` | `/root/gotcha-workspace/tools/self_improvement/health_check.py` | `--fix` |
| daily_note_populate.py | `ops_daily_note` | `/root/gotcha-workspace/tools/temporal-sync/daily_note_populate.py` | (none) |
| nightly_daily_to_weekly.py | `ops_nightly_synthesis` | `/root/gotcha-workspace/tools/temporal-sync/nightly_daily_to_weekly.py` | (none) |
| rollup_weekly.py | `ops_weekly_rollup` | `/root/gotcha-workspace/tools/temporal-sync/rollup_weekly.py` | (none) |
| rollup_monthly.py | `ops_monthly_rollup` | `/root/gotcha-workspace/tools/temporal-sync/rollup_monthly.py` | (none) |
| podcast_watcher.py | `ops_podcast_watcher` | `/root/gotcha-workspace/tools/discord/podcast_watcher.py` | (none) |

All should be scoped to include `"operations"` (Nova's tool_scope includes this).

### Nova's Tool Scope

Nova's current tool_scope: `{memory, temporal, system, all, strategy, trading, market, engineering, content, scheduling, reporting, compliance, creative, research, decision, operations}`

Nova has `"operations"` scope, so any tool with `"operations"` in its scope will be available.

### Project #14 Schedules (Current State)

Already seeded in DB:
```json
[
  {"expr": "0 13 * * *",   "label": "Daily Health Check"},
  {"expr": "5 4 * * *",    "label": "Nightly Synthesis"},
  {"expr": "50 3 * * *",   "label": "Daily Note Population"},
  {"expr": "30 4 * * 6",   "label": "Weekly Finalization"},
  {"expr": "0 5 1 * *",    "label": "Monthly Finalization"}
]
```

**Missing:** Podcast Watch schedule (23:10 UTC daily from OpenClaw: `10 23 * * *`).

### Prompt Engineering Pattern

The action planner's `build-project-review-job` (line 817 in action-planner.lisp) already includes a `schedule-context` section when labels fire:

```lisp
(format nil "~%~%## Standing Orders Fired~%This review was triggered by scheduled standing orders:~%~{- ~a~%~}~%Act on the specific standing order(s) that fired. Execute the work associated with each label.~%" fired)
```

This needs to be extended with a mapping table telling Nova exactly which tool to call:

```
## Standing Order Tool Mapping
When a standing order fires, execute the associated tool:
- "Daily Health Check" -> call ops_health_check with --fix flag
- "Daily Note Population" -> call ops_daily_note
- "Nightly Synthesis" -> call ops_nightly_synthesis
- "Weekly Finalization" -> call ops_weekly_rollup
- "Monthly Finalization" -> call ops_monthly_rollup
- "Podcast Watch" -> call ops_podcast_watcher

Call the tool using a tool_call block. After tool execution, summarize the results.
```

### Environment Variables for Scripts

The OpenClaw cron jobs show that health_check.py needs PG connection env vars:
```
PG_HOST=127.0.0.1 PG_PORT=5432 PG_USER=chronicle PG_PASSWORD=chronicle2026 PG_DATABASE=master_chronicle
```

The tool-socket's `execute-tool-call` function runs scripts via subprocess. Need to verify whether these env vars are available in the ghost process environment or need to be injected. Since ghosts run on the same droplet and the tools use `_db.py` from gotcha-workspace which reads env vars, the ghost PM2 process environment must include these vars.

### Recommended Implementation Order

```
Wave 1: Infrastructure
├── Register 6 operational tools in tool-registry.json
├── Add process-tool-calls to execute-project-review
└── Add Podcast Watch schedule to Project #14

Wave 2: Prompt Engineering
├── Extend standing order prompt with label-to-tool mapping
└── Ensure Nova's tool scope overlaps with registered tool scopes

Wave 3: Verification
├── Manual tick test: fire each schedule label, verify tool execution
└── Verify conversation output is attributed to Nova
```

### Anti-Patterns to Avoid
- **Rewriting operational scripts:** D-01 explicitly prohibits this. Scripts work. Use them as-is.
- **Chaining temporal levels:** D-06 says schedules fire independently. Do not build a temporal cascade in the tick engine -- the scripts handle cascading internally via vault_notes reads.
- **Discord output from ghosts:** D-05 explicitly excludes this. Conversations table is the output destination.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Health monitoring | Custom monitoring in Lisp | health_check.py | Proven, checks DB/PM2/logs/git/API |
| Temporal note cascade | Custom Lisp cascading logic | daily_note_populate.py + nightly_daily_to_weekly.py + rollup_weekly.py + rollup_monthly.py | Scripts already read vault_notes and handle frontmatter wikilinks |
| RSS feed checking | Custom RSS parser | podcast_watcher.py | Handles Captivate feeds, state tracking, Discord webhooks |
| Cron scheduling | New scheduler | Phase 12's schedule evaluation in tick-engine.lisp | Already built and working |

## Common Pitfalls

### Pitfall 1: Missing Tool Execution in Project Reviews
**What goes wrong:** Standing orders fire, Nova gets a cognition job, Nova writes tool_call blocks in response, but nothing executes because `execute-project-review` doesn't call `process-tool-calls`.
**Why it happens:** Project reviews were designed for delegation (CREATE_TASK, DELEGATE), not direct execution.
**How to avoid:** Add `process-tool-calls` invocation in `execute-project-review`, similar to how `execute-work-task` does it at line 403.
**Warning signs:** Nova's project review conversations contain `tool_call` blocks as raw text instead of tool execution results.

### Pitfall 2: Missing Environment Variables
**What goes wrong:** Tools fail with DB connection errors when executed by ghosts.
**Why it happens:** PM2 process may not have PG_* env vars set. OpenClaw injected them per-job.
**How to avoid:** Verify PM2 ecosystem file or launch.sh includes necessary env vars. The `_db.py` module in gotcha-workspace reads `PG_HOST`, `PG_PORT`, `PG_USER`, `PG_PASSWORD`, `PG_DATABASE`.
**Warning signs:** Tool execution returns "could not connect to server" errors.

### Pitfall 3: Script Working Directory
**What goes wrong:** Tools fail with import errors (can't find `_db`, `_config`, `week_calc`).
**Why it happens:** Python scripts use relative imports (`from _db import ...`) which depend on the working directory or sys.path manipulation.
**How to avoid:** Tool registry entries must use the full script path, and the tool-socket's execution should set the working directory to the script's parent, OR the scripts must handle sys.path themselves (most already do via `sys.path.insert`).
**Warning signs:** `ModuleNotFoundError: No module named '_db'`.

### Pitfall 4: Nightly Synthesis Requires Ollama
**What goes wrong:** nightly_daily_to_weekly.py fails because it uses local Ollama for AI summarization.
**Why it happens:** The script was designed to run on Mac Studio with local Ollama. On the droplet, Ollama may be available at localhost:11434 but uses `nomic-embed-text` (embeddings), not a chat model.
**How to avoid:** Check if Ollama has a chat model loaded on the droplet. If not, the script may need a `--skip-ai` flag or fallback. Alternatively, Nova's cognition (which is an LLM call itself) could do the summarization directly instead of delegating to Ollama.
**Warning signs:** HTTP connection error to Ollama, or "model not found" errors.

### Pitfall 5: Podcast Watcher Discord Token Dependency
**What goes wrong:** podcast_watcher.py tries to read Discord bot token from `/root/.openclaw/openclaw.json` and fails.
**Why it happens:** Script was built for the OpenClaw agent which had Discord credentials.
**How to avoid:** Verify the token file exists and is readable. If Discord posting is not in scope (D-05), run with `--dry-run` or accept that the script will check feeds but may fail to post. Alternatively, the script could be run in a mode that just reports what it found to conversations without posting to Discord.
**Warning signs:** "Could not find Discord bot token" runtime error.

### Pitfall 6: Tool Scope "dangerous" Flag
**What goes wrong:** The existing `self_improvement` tool is marked `"dangerous": true`, which may prevent execution without approval.
**Why it happens:** Safety check in execute-tool-call (tool-socket.lisp line 189).
**How to avoid:** New operational tool registrations should be `"dangerous": false` since they are read-heavy operations (except health_check with `--fix`). For health_check, determine whether `--fix` auto-remediation warrants the dangerous flag or if it's safe for autonomous execution.
**Warning signs:** Tool returns "dangerous tool blocked" message.

## Code Examples

### Adding process-tool-calls to execute-project-review

Source: action-executor.lisp, modeled on execute-work-task (line 403)

```lisp
(defun execute-project-review (result metadata)
  "Post project review output as a conversation entry. Parse task mutations, handoffs, and delegations."
  (let* ((agent-id (cognition-result-agent-id result))
         (content (cognition-result-content result))
         ;; NEW: Execute tool calls from project review output
         (tool-results (process-tool-calls content agent-id))
         ;; Append tool results to content for conversation posting
         (content (if tool-results
                      (concatenate 'string content
                        (with-output-to-string (s)
                          (dolist (r tool-results)
                            (format s "~%--- TOOL: ~a ---~%~a~%" (first r) (second r)))))
                      content))
         (handoff (parse-handoff content))
         ;; ... rest of existing function
```

### Tool Registry Entry Pattern

Source: tool-registry.json existing entries

```json
"ops_health_check": {
  "script": "/root/gotcha-workspace/tools/self_improvement/health_check.py",
  "description": "Daily system health check. Checks DB, PM2, logs, tools, git status.",
  "parameters": {
    "fix": "Attempt auto-fixes where safe"
  },
  "scope": ["operations", "monitoring", "maintenance"],
  "dangerous": false,
  "interpreter": "python3",
  "env": {
    "PG_HOST": "127.0.0.1",
    "PG_PORT": "5432",
    "PG_USER": "chronicle",
    "PG_PASSWORD": "chronicle2026",
    "PG_DATABASE": "master_chronicle"
  }
}
```

Note: The `env` field may not be supported by the current tool-socket. If not, env vars must be set in the PM2 ecosystem or launch.sh.

### Prompt Extension in action-planner.lisp

Source: build-project-review-job, line 848

```lisp
;; Replace the generic "Act on the specific standing order(s)" with explicit tool mapping
(schedule-context
  (let ((fired (gethash agent-id af64.runtime.tick-engine:*schedule-fired-labels*)))
    (if fired
        (format nil "~%~%## Standing Orders Fired~%This review was triggered by scheduled standing orders:~%~{- ~a~%~}~%~%## Standing Order Tool Mapping~%Execute the corresponding tool for each fired label:~%- \"Daily Health Check\" -> call self_improvement with --fix~%- \"Daily Note Population\" -> call ops_daily_note~%- \"Nightly Synthesis\" -> call ops_nightly_synthesis~%- \"Weekly Finalization\" -> call ops_weekly_rollup~%- \"Monthly Finalization\" -> call ops_monthly_rollup~%- \"Podcast Watch\" -> call ops_podcast_watcher~%~%Call each tool using a ```tool_call block. After execution, summarize the results.~%" fired)
        "")))
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| OpenClaw cron jobs | Ghost standing orders (Phase 12) | 2026-03-28 | Schedule triggers built into tick engine |
| Discord channel output | Conversations table | Phase 13 (this) | Noosphere-native output |
| External agent execution | Ghost cognition + tool socket | Phase 12-13 | No OpenClaw dependency |

**Deprecated/outdated:**
- OpenClaw cron system: Being replaced by standing orders. 14 active jobs, 6 of which are operational (this phase).

## Open Questions

1. **Ollama Availability for Nightly Synthesis**
   - What we know: nightly_daily_to_weekly.py uses Ollama for AI summarization. Droplet has Ollama installed (localhost:11434) with nomic-embed-text.
   - What's unclear: Whether a chat-capable model is loaded on the droplet for summarization.
   - Recommendation: Check `ollama list` during planning. If no chat model, either load one or modify the script to skip AI summarization and use the raw content extraction path.

2. **Tool Socket Environment Variable Injection**
   - What we know: execute-tool-call runs scripts via subprocess. The OpenClaw cron jobs explicitly set PG_* env vars per invocation.
   - What's unclear: Whether the PM2 process environment already includes these vars, or if they need to be injected per-tool.
   - Recommendation: Check PM2 ecosystem config. If vars are missing, either add them to launch.sh or have the tool-socket set them for operations-scoped tools.

3. **Batching Multiple Schedule Fires**
   - What we know: Daily Health Check (13:00 UTC) and Daily Note Population (03:50 UTC) fire at different times. But if the tick engine is stopped and restarted, multiple schedules could fire simultaneously.
   - What's unclear: Whether the project review cognition prompt handles multiple fired labels gracefully, or if Nova tries to do everything in one response.
   - Recommendation: The current implementation passes all fired labels to a single project review. This should work fine -- Nova sees all labels and calls each corresponding tool. No batching logic needed.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Python 3 (venv) | All scripts | Yes | 3.x at /root/gotcha-workspace/.venv | -- |
| PostgreSQL | DB scripts | Yes | Running on localhost:5432 | -- |
| Ollama | nightly_daily_to_weekly.py | Partial | localhost:11434 (nomic-embed-text only) | Skip AI summary, use raw extraction |
| PM2 | noosphere-ghosts process | Yes | Installed | -- |
| Discord bot token | podcast_watcher.py | Yes | /root/.openclaw/openclaw.json | --dry-run mode |

**Missing dependencies with no fallback:**
- None blocking

**Missing dependencies with fallback:**
- Ollama chat model: nightly_daily_to_weekly.py AI summarization may fail. Fallback: script has raw extraction path or Nova summarizes via cognition.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual verification + PM2 log inspection |
| Config file | None -- ghost runtime has no automated test framework |
| Quick run command | `pm2 logs noosphere-ghosts --lines 50` |
| Full suite command | Manual: trigger each schedule, verify conversation output in DB |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| OPS-01 | Health check executes on schedule, produces Nova conversation | smoke | `sudo -u postgres psql -d master_chronicle -c "SELECT id,from_agent,message FROM conversations WHERE from_agent='nova' AND metadata->>'source'='project_review' ORDER BY created_at DESC LIMIT 1"` | N/A (DB query) |
| OPS-02 | Daily note + nightly synthesis execute as ghost work | smoke | Same DB query, check for tool execution output in conversation content | N/A |
| OPS-03 | Podcast watcher runs, checks feeds | smoke | Same DB query + check PM2 logs for `[tools] nova executed` | N/A |
| OPS-04 | Weekly/monthly finalization executes | smoke | Same DB query + check vault_notes for weekly/monthly content updates | N/A |

### Sampling Rate
- **Per task commit:** `pm2 logs noosphere-ghosts --lines 20` to verify compilation
- **Per wave merge:** SBCL load test: restart noosphere-ghosts, check for startup errors
- **Phase gate:** Manual trigger of each standing order label, verify conversation output in DB

### Wave 0 Gaps
- No automated test framework exists for the ghost runtime (Common Lisp)
- Verification is manual: trigger schedule, inspect PM2 logs and conversations table
- None of the Python scripts have pytest test files (they're operational scripts, not libraries)

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` - Confirmed execute-project-review lacks process-tool-calls (line 921-978)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` - Confirmed build-project-review-job schedule-context at line 848
- `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` - Confirmed tool execution via process-tool-calls and execute-tool-call
- `/opt/project-noosphere-ghosts/config/tool-registry.json` - Confirmed operational scripts NOT registered (except health_check as self_improvement)
- DB query: Nova's tool_scope confirmed includes "operations"
- DB query: Project #14 schedules confirmed (5 entries, missing podcast)
- `/root/.openclaw/cron/jobs.json` - OpenClaw schedule reference (env vars, timing, payloads)

### Secondary (MEDIUM confidence)
- Ollama availability: Known installed on droplet for embeddings, chat model availability unverified

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All components verified via direct file reading
- Architecture: HIGH - Critical gap (project review tool execution) confirmed by code inspection
- Pitfalls: HIGH - All pitfalls derived from actual code reading, not speculation

**Research date:** 2026-03-28
**Valid until:** 2026-04-28 (stable -- Common Lisp codebase changes infrequently)
