# Phase 14: Editorial Pipeline - Research

**Researched:** 2026-03-28
**Domain:** Ghost tool registration + action planner prompt engineering (Common Lisp / Python)
**Confidence:** HIGH

## Summary

Phase 14 migrates Sylvia's nightly editorial pipeline from an OpenClaw cron job to ghost-executed standing order work under Project #12 (Cognitive Submission). The existing `nightly_editorial.py` script handles the full pipeline -- query comments, fetch articles, synthesize via Claude API, save to documents, export. The ghost infrastructure just needs to trigger it correctly.

The critical technical challenge is **generalizing the label-to-tool mapping** in action-planner.lisp. Phase 13 hardcoded a mapping table that only covers Nova's ops_* tools for Project #14. This mapping must be extended to include Sylvia's editorial tool for Project #12 without breaking Nova's existing mapping. The mapping is a format string embedded in the `schedule-context` let-binding of `build-project-review-job`.

**There is one blocking issue:** The `nightly_editorial.py` script's `call_claude()` function reads an OAuth token from `/root/.openclaw/agents/main/agent/auth-profiles.json` (OpenClaw auth profiles), NOT from the `ANTHROPIC_API_KEY` environment variable. The ghost process has `ANTHROPIC_API_KEY` in its environment (via `af64.env`), but the script never uses it for the synthesis API call. The script must be patched to use the env var as primary auth method, falling back to auth-profiles if needed.

**Primary recommendation:** Register `editorial_nightly` tool in tool-registry.json with `editorial` scope, patch `nightly_editorial.py` to use ANTHROPIC_API_KEY env var, and extend the action-planner schedule-context to include project-aware tool mappings.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Sylvia executes the existing `nightly_editorial.py` script via tool invocation. No rewrite -- the script already handles the full pipeline (query comments, fetch articles, synthesize via Claude API, save to documents, export).
- **D-02:** The tool must be registered in the ghost tool registry with `editorial` scope matching Sylvia's tool_scope. Same pattern as Phase 13 ops tools.
- **D-03:** If no reader comments exist for the day, the script returns HEARTBEAT_OK -- Sylvia reports "no editorial today" in conversations.
- **D-04:** Project #12 already has the schedule: `{"expr": "0 1 * * *", "label": "Nightly Editorial"}` (01:00 UTC = 9 PM ET). No schedule changes needed.
- **D-05:** The action planner's label-to-tool mapping (Phase 13 pattern) needs extending to include Sylvia's editorial mapping: "Nightly Editorial" -> `editorial_nightly` tool.
- **D-06:** Since the Phase 13 mapping was built for Nova/Project #14, the mapping system must be generalized to work per-project/per-executive, not hardcoded to Nova.
- **D-07:** Sylvia owns Project #12. The standing order fires for the owning executive. Sylvia gets the cognition job, executes the tool, and posts results as herself in conversations.
- **D-08:** The editorial output is attributed to Sylvia (via conversation from_agent), not Nova or a system account. This satisfies EDIT-01.
- **D-09:** Nathan's reader comments are SACRED -- the Thought Police post preserves >80% of his original words. This is enforced by the existing script's system prompt, not by ghost infrastructure.
- **D-10:** The script already handles synthesis quality. Ghost infrastructure just needs to trigger it correctly.

### Claude's Discretion
- How to generalize the label-to-tool mapping for multiple executives (refactor vs per-project sections)
- Whether to register additional editorial tools (delegate_editorial.py, trigger_editorial.py) or just the nightly script
- Error handling if the Anthropic API call inside nightly_editorial.py fails

### Deferred Ideas (OUT OF SCOPE)
- Real-time editorial trigger (when Nathan finishes commenting, not just 9 PM) -- would require conversation watcher, not a standing order
- Multi-author editorial support (other team members commenting) -- out of scope, Thought Police is Nathan-only
- Editorial quality scoring/feedback loop -- future enhancement
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| EDIT-01 | Nightly editorial pipeline executes as ghost pipeline under Project #12, owned by Sylvia | Tool registration with `editorial` scope + Sylvia's tool_scope includes `editorial` + Project #12 schedule already seeded + action planner mapping extension |
| EDIT-02 | Editorial output follows the existing Thought Police format and posts to the correct destination | Existing `nightly_editorial.py` handles format and destination (documents table + dpn-publish export). Ghost just triggers script. |
</phase_requirements>

## Standard Stack

No new libraries or packages needed. This phase modifies existing files only.

### Core Files Modified

| File | Language | Purpose | Modification |
|------|----------|---------|--------------|
| `tool-registry.json` | JSON | Ghost tool registry | Add `editorial_nightly` entry |
| `action-planner.lisp` | Common Lisp | Cognition prompt builder | Generalize label-to-tool mapping |
| `nightly_editorial.py` | Python | Editorial synthesis script | Patch auth to use ANTHROPIC_API_KEY env var |

### Files NOT Modified (verified working)

| File | Reason |
|------|--------|
| `action-executor.lisp` | Already calls `process-tool-calls` in `execute-project-review` (Phase 13 fix) |
| `tool-socket.lisp` | Already handles scope-based tool filtering. Sylvia's `editorial` scope matches |
| `tick-engine.lisp` | Already handles schedule firing via `*schedule-fired-labels*` |
| Project #12 schedule | Already seeded: `[{"expr": "0 1 * * *", "label": "Nightly Editorial"}]` |

## Architecture Patterns

### Pattern 1: Tool Registration (from Phase 13)

**What:** Register a Python script as a ghost-callable tool in tool-registry.json.
**When to use:** Any time a script needs to be invoked by a ghost via tool_call blocks.

```json
"editorial_nightly": {
  "script": "/root/gotcha-workspace/tools/editorial/nightly_editorial.py",
  "description": "Run the nightly Thought Police editorial pipeline. Collects reader comments, fetches articles, synthesizes post, saves to documents.",
  "parameters": {
    "date": "Target date (YYYY-MM-DD, defaults to today)"
  },
  "scope": ["editorial"],
  "dangerous": false
}
```

**Key details:**
- The `scope` array must overlap with the agent's `tool_scope` from the agents table
- Sylvia's tool_scope: `{content,brand,social,worldbuilding,publishing,decision,editorial,writing,creative,feeds,research,specs,strategy}` -- `editorial` is present
- The tool-socket runs scripts via `/root/gotcha-workspace/.venv/bin/python3` by default (line 280 of tool-socket.lisp)
- No `--dry-run` or `--discord` flags needed for ghost invocation

### Pattern 2: Label-to-Tool Mapping in Action Planner

**What:** The `schedule-context` variable in `build-project-review-job` (action-planner.lisp line 847-851) builds a prompt section when standing orders fire. It includes a markdown table mapping labels to tools.

**Current implementation (Phase 13, hardcoded for Nova/ops):**
```lisp
(format nil "...
## Standing Order Tool Mapping
| Label | Tool | Args |
|-------|------|------|
| Daily Health Check | ops_health_check | fix: true |
| Daily Note Population | ops_daily_note | (none) |
| Nightly Synthesis | ops_nightly_synthesis | (none) |
| Weekly Finalization | ops_weekly_rollup | (none) |
| Monthly Finalization | ops_monthly_rollup | (none) |
| Podcast Watch | ops_podcast_watcher | (none) |
..." fired)
```

**Problem:** This is a single hardcoded table inside one format string. When Sylvia's standing order fires for Project #12, she sees the same ops_* mapping table -- but she should see an editorial mapping instead.

**Recommended generalization approach:**
Build the tool mapping dynamically based on which labels actually fired, rather than dumping a static table. Use an alist or cond to map each known label to its tool:

```lisp
;; Per-label tool mappings (all projects)
(defun tool-mapping-for-label (label)
  "Return (tool-name . args-string) for a standing order label."
  (cond
    ((string-equal label "Daily Health Check")     '("ops_health_check" "fix: true"))
    ((string-equal label "Daily Note Population")  '("ops_daily_note" "(none)"))
    ((string-equal label "Nightly Synthesis")      '("ops_nightly_synthesis" "(none)"))
    ((string-equal label "Weekly Finalization")    '("ops_weekly_rollup" "(none)"))
    ((string-equal label "Monthly Finalization")   '("ops_monthly_rollup" "(none)"))
    ((string-equal label "Podcast Watch")          '("ops_podcast_watcher" "(none)"))
    ((string-equal label "Nightly Editorial")      '("editorial_nightly" "(none)"))
    (t nil)))
```

Then build the table dynamically from only the fired labels:
```lisp
(schedule-context
  (let ((fired (gethash agent-id af64.runtime.tick-engine:*schedule-fired-labels*)))
    (if fired
        (let ((rows (loop for label in fired
                          for mapping = (tool-mapping-for-label label)
                          when mapping
                          collect (format nil "| ~a | ~a | ~a |" label (car mapping) (cdr mapping)))))
          (format nil "~%~%## Standing Orders Fired~%...~%~{~a~%~}~%..." fired rows))
        "")))
```

This approach:
- Only shows tools relevant to the fired labels (Sylvia sees editorial tools, Nova sees ops tools)
- Easily extensible for Phase 15 (Kathryn/financial)
- Keeps mapping as code per D-12

### Pattern 3: HEARTBEAT_OK for No-Op Runs

**What:** When no reader comments exist for the day, the script should return a recognizable signal.
**Current behavior:** Script prints "No reader comments from {date} -- nothing to do." and returns (exit 0).
**Ghost expectation (D-03):** Script should return HEARTBEAT_OK so Sylvia can report accordingly.
**Implementation:** Add a print/return of "HEARTBEAT_OK" before the early return when no comments found.

### Anti-Patterns to Avoid
- **Hardcoding project-specific mappings in a single block:** The Phase 13 approach of one big format string with all ops tools becomes unmaintainable as more projects are added. Generalize now.
- **Registering trigger_editorial.py or delegate_editorial.py:** These are OpenClaw-era tools that send messages to Nova or create pipeline tasks. The ghost nightly flow is simpler -- Sylvia calls `nightly_editorial.py` directly. Only register what the standing order needs.
- **Modifying the schedule on Project #12:** It already works. `0 1 * * *` = 01:00 UTC = 9 PM ET. Verified in database.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Tool scope filtering | Custom scope check | tool-socket.lisp `get-tools-for-agent` | Already compares agent tool_scope with tool scope arrays |
| Schedule firing | Custom cron logic | tick-engine.lisp `*schedule-fired-labels*` | Already fires labels and populates this dynamic variable per tick |
| Tool execution | Custom subprocess call | tool-socket.lisp `execute-tool-call` | Already handles Python script invocation, output capture, error handling |
| Conversation attribution | Custom API call | execute-project-review in action-executor.lisp | Already posts review output as from-agent conversation |

## Common Pitfalls

### Pitfall 1: Auth-profiles vs ANTHROPIC_API_KEY
**What goes wrong:** `nightly_editorial.py` line 129-141 reads an OAuth token from `/root/.openclaw/agents/main/agent/auth-profiles.json`. If this file is missing or stale, the Claude synthesis call fails silently with `[ERROR: auth-profiles.json not found]`.
**Why it happens:** The script was written for OpenClaw-era auth. The ghost process has `ANTHROPIC_API_KEY` in its environment but the script ignores it.
**How to avoid:** Patch `call_claude()` to use `ANTHROPIC_API_KEY` env var as the primary auth method (with `x-api-key` header instead of Bearer OAuth), falling back to auth-profiles only if env var is empty.
**Warning signs:** Script output contains `[ERROR:` prefix strings.

### Pitfall 2: Script Exit Code on No Comments
**What goes wrong:** When no comments exist, the script returns cleanly (exit 0) but prints a human-readable message. The ghost sees this as success output and may try to parse it.
**Why it happens:** The `run()` function does an early `return` with no structured output.
**How to avoid:** Print "HEARTBEAT_OK" as a recognizable token when no comments exist, per D-03.
**Warning signs:** Sylvia posts the raw "No reader comments" message as editorial output.

### Pitfall 3: Mapping Table Shows Wrong Tools for Wrong Executive
**What goes wrong:** If the mapping table remains hardcoded, Sylvia sees ops_* tools in her standing order prompt and tries to call ops_health_check instead of editorial_nightly.
**Why it happens:** Phase 13 built the table for Nova only. The schedule-context code path is shared by all executives.
**How to avoid:** Build the mapping table dynamically from fired labels, not as a static block.
**Warning signs:** Sylvia's cognition output contains tool_call blocks for ops_* tools.

### Pitfall 4: Python Working Directory
**What goes wrong:** The `nightly_editorial.py` script uses `sys.path.insert(0, ...)` relative to `__file__` to find `_config.py`. If tool-socket runs it from a different cwd, the relative import might fail.
**Why it happens:** tool-socket.lisp invokes scripts as `(list interpreter script cli-args)` without setting cwd.
**How to avoid:** The script uses `Path(__file__).resolve().parent.parent` which is absolute -- this works regardless of cwd. Verified: the sys.path manipulation is file-relative, not cwd-relative. No issue expected.
**Warning signs:** `ModuleNotFoundError: No module named '_config'` in tool output.

### Pitfall 5: ANTHROPIC_API_KEY Header Format
**What goes wrong:** The current script uses `Authorization: Bearer {token}` (OAuth format). If switching to the raw API key from env, the header must be `x-api-key: {key}` instead. Using `Authorization: Bearer sk-ant-...` against the Anthropic API will return 401.
**Why it happens:** OAuth tokens and API keys use different header formats.
**How to avoid:** When using env var API key, use `x-api-key` header. When using auth-profiles OAuth token, use `Authorization: Bearer` header.
**Warning signs:** HTTP 401 from Anthropic API.

## Code Examples

### Tool Registry Entry for editorial_nightly
```json
"editorial_nightly": {
  "script": "/root/gotcha-workspace/tools/editorial/nightly_editorial.py",
  "description": "Run the nightly Thought Police editorial pipeline. Collects Nathan's reader comments for today (or specified date), fetches source articles, synthesizes editorial via Claude API, saves to documents table, triggers dpn-publish export.",
  "parameters": {
    "date": "Target date YYYY-MM-DD (default: today)"
  },
  "scope": ["editorial"],
  "dangerous": false
}
```

### Patched call_claude() Auth (nightly_editorial.py)
```python
def call_claude(prompt: str) -> str:
    """Synthesize editorial via Anthropic API. Prefers ANTHROPIC_API_KEY env var."""
    api_key = os.environ.get("ANTHROPIC_API_KEY", "")

    if api_key:
        # Direct API key auth (ghost environment)
        headers = {
            "x-api-key": api_key,
            "anthropic-version": "2023-06-01",
            "content-type": "application/json",
        }
    else:
        # Fallback: OAuth token from auth-profiles (legacy OpenClaw)
        profiles_path = Path("/root/.openclaw/agents/main/agent/auth-profiles.json")
        if not profiles_path.exists():
            return "[ERROR: No ANTHROPIC_API_KEY and auth-profiles.json not found]"
        try:
            data = json.loads(profiles_path.read_text())
            profiles = data.get("profiles", {})
            profile = profiles.get("anthropic:claude-tasks") or profiles.get("anthropic:default") or {}
            access_token = profile.get("token", "")
            if not access_token:
                return "[ERROR: No Anthropic token in auth-profiles.json]"
        except Exception as e:
            return f"[ERROR: Could not read auth-profiles: {e}]"
        headers = {
            "Authorization": f"Bearer {access_token}",
            "anthropic-version": "2023-06-01",
            "anthropic-beta": "oauth-2025-04-20",
            "content-type": "application/json",
        }
    # ... rest of function unchanged
```

### Dynamic Label-to-Tool Mapping (action-planner.lisp)
```lisp
(defun tool-mapping-for-label (label)
  "Return (tool-name . args-description) for a standing order label, or NIL if unknown."
  (cond
    ;; Operations (Nova / Project #14)
    ((string-equal label "Daily Health Check")     (cons "ops_health_check" "fix: true"))
    ((string-equal label "Daily Note Population")  (cons "ops_daily_note" "(none)"))
    ((string-equal label "Nightly Synthesis")      (cons "ops_nightly_synthesis" "(none)"))
    ((string-equal label "Weekly Finalization")    (cons "ops_weekly_rollup" "(none)"))
    ((string-equal label "Monthly Finalization")   (cons "ops_monthly_rollup" "(none)"))
    ((string-equal label "Podcast Watch")          (cons "ops_podcast_watcher" "(none)"))
    ;; Editorial (Sylvia / Project #12)
    ((string-equal label "Nightly Editorial")      (cons "editorial_nightly" "(none)"))
    ;; Unknown label -- no mapping
    (t nil)))

(defun build-tool-mapping-table (fired-labels)
  "Build a markdown table of label-to-tool mappings for the given fired labels."
  (let ((rows (loop for label in fired-labels
                    for mapping = (tool-mapping-for-label label)
                    when mapping
                    collect (format nil "| ~a | ~a | ~a |" label (car mapping) (cdr mapping)))))
    (if rows
        (format nil "| Label | Tool | Args |~%|-------|------|------|~%~{~a~%~}" rows)
        "")))
```

### HEARTBEAT_OK Output (nightly_editorial.py)
```python
if not rows:
    print(f"   No reader comments from {target_date} -- nothing to do.")
    print("HEARTBEAT_OK")
    return
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| OpenClaw cron fires `trigger_editorial.py` which messages Nova | Standing order on Project #12 fires, Sylvia gets cognition job, calls tool directly | Phase 12-14 (this migration) | Removes OpenClaw dependency, editorial stays with Sylvia |
| OAuth auth-profiles for Anthropic API | ANTHROPIC_API_KEY env var in ghost process | This phase | More reliable, no stale token issues |
| Hardcoded ops-only tool mapping | Dynamic per-label mapping function | This phase | Supports multiple executives/projects |

## Open Questions

1. **delegate_editorial.py and trigger_editorial.py registration**
   - What we know: These are OpenClaw-era tools. `delegate_editorial.py` creates a 7-stage pipeline task chain. `trigger_editorial.py` sends a message to Nova (not Sylvia).
   - What's unclear: Whether they have any value in the ghost world.
   - Recommendation: Do NOT register them. The nightly flow is simpler: one tool call, one script. The delegate pipeline is for manual batches which are out of scope per the deferred ideas. `trigger_editorial.py` targets Nova which contradicts D-07/D-08.

2. **Error handling for Anthropic API failure**
   - What we know: The script returns `[Anthropic API error ...]` as the editorial content if the API call fails. This would get saved to documents as an error message.
   - What's unclear: Whether to add ghost-level retry or just let the error propagate.
   - Recommendation: Let the error propagate. The script output will contain `[Anthropic API error` which Sylvia will post in conversations. Nathan can see it and manually re-trigger. Adding retry logic is scope creep.

3. **`--discord` flag deprecation**
   - What we know: The script has a `--discord` flag that calls `openclaw message send`. Ghost output goes to conversations table instead.
   - What's unclear: Whether the discord flag should be removed.
   - Recommendation: Leave it. It's harmless (non-fatal on failure) and doesn't affect ghost invocation. Don't modify what you don't need to.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual validation (SBCL compilation + DB state verification) |
| Config file | None -- Lisp compilation is the test |
| Quick run command | `cd /opt/project-noosphere-ghosts/lisp && sbcl --noinform --non-interactive --eval '(require :asdf)' --eval '(load "packages.lisp")' --eval '(load "runtime/action-planner.lisp")' --eval '(format t "OK~%")'` |
| Full suite command | `cd /opt/project-noosphere-ghosts && bash launch.sh` (starts tick loop, verify first tick completes) |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| EDIT-01 | editorial_nightly registered in tool-registry.json with editorial scope | smoke | `python3 -c "import json; d=json.load(open('/opt/project-noosphere-ghosts/config/tool-registry.json')); assert 'editorial_nightly' in d['tools']; assert 'editorial' in d['tools']['editorial_nightly']['scope']"` | N/A |
| EDIT-01 | Sylvia's tool_scope includes editorial | smoke | `sudo -u postgres psql -d master_chronicle -t -c "SELECT tool_scope FROM agents WHERE id='sylvia'" \| grep editorial` | N/A |
| EDIT-01 | Action planner mapping includes Nightly Editorial -> editorial_nightly | smoke | `grep -q 'editorial.nightly' /opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` | N/A |
| EDIT-01 | SBCL compiles action-planner.lisp without errors | unit | Quick run command above | N/A |
| EDIT-02 | nightly_editorial.py uses ANTHROPIC_API_KEY from env | smoke | `grep -q 'ANTHROPIC_API_KEY' /root/gotcha-workspace/tools/editorial/nightly_editorial.py && grep -q 'x-api-key' /root/gotcha-workspace/tools/editorial/nightly_editorial.py` | N/A |
| EDIT-02 | nightly_editorial.py outputs HEARTBEAT_OK when no comments | smoke | `ANTHROPIC_API_KEY=test /root/gotcha-workspace/.venv/bin/python3 /root/gotcha-workspace/tools/editorial/nightly_editorial.py --date 2020-01-01 2>&1 \| grep HEARTBEAT_OK` | N/A |

### Sampling Rate
- **Per task commit:** SBCL compilation check + tool-registry JSON parse
- **Per wave merge:** Full compilation + smoke tests above
- **Phase gate:** All smoke tests pass before `/gsd:verify-work`

### Wave 0 Gaps
None -- no test framework needed. Validation is SBCL compilation + grep-based smoke tests + DB state checks.

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/config/tool-registry.json` -- Existing tool registrations (6 ops_* tools pattern)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` line 847-851 -- Hardcoded tool mapping table
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` line 921-926 -- execute-project-review with process-tool-calls
- `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` line 173-303 -- Tool execution with scope filtering
- `/root/gotcha-workspace/tools/editorial/nightly_editorial.py` -- Full editorial script with auth-profiles auth
- `/opt/project-noosphere-ghosts/config/af64.env` -- ANTHROPIC_API_KEY available in ghost process
- Database query: Sylvia tool_scope = `{content,brand,social,worldbuilding,publishing,decision,editorial,writing,creative,feeds,research,specs,strategy}`
- Database query: Project #12 schedule = `[{"expr": "0 1 * * *", "label": "Nightly Editorial"}]`

### Secondary (MEDIUM confidence)
- `.planning/phases/13-operations-pipeline/13-01-PLAN.md` -- Phase 13 plan structure for reference
- `.planning/phases/13-operations-pipeline/13-02-PLAN.md` -- Phase 13 tool mapping plan

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- All files inspected, patterns confirmed from Phase 13 execution
- Architecture: HIGH -- Dynamic mapping approach verified against existing Lisp codebase patterns
- Pitfalls: HIGH -- Auth issue discovered by reading actual source code, not assumed

**Research date:** 2026-03-28
**Valid until:** 2026-04-28 (stable infrastructure, no external dependency changes expected)
