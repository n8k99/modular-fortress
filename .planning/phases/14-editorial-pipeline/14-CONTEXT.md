# Phase 14: Editorial Pipeline - Context

**Gathered:** 2026-03-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Sylvia's nightly editorial pipeline runs autonomously as ghost work under Project #12 (Cognitive Submission). At 9 PM ET, the standing order fires, Sylvia gets a cognition job, and she executes the Thought Police editorial pipeline — collecting Nathan's reader comments, fetching source articles, synthesizing a post, and saving it to the documents table.

</domain>

<decisions>
## Implementation Decisions

### Pipeline Execution
- **D-01:** Sylvia executes the existing `nightly_editorial.py` script via tool invocation. No rewrite — the script already handles the full pipeline (query comments, fetch articles, synthesize via Claude API, save to documents, export).
- **D-02:** The tool must be registered in the ghost tool registry with `editorial` scope matching Sylvia's tool_scope. Same pattern as Phase 13 ops tools.
- **D-03:** If no reader comments exist for the day, the script returns HEARTBEAT_OK — Sylvia reports "no editorial today" in conversations.

### Standing Order Integration
- **D-04:** Project #12 already has the schedule: `{"expr": "0 1 * * *", "label": "Nightly Editorial"}` (01:00 UTC = 9 PM ET). No schedule changes needed.
- **D-05:** The action planner's label-to-tool mapping (Phase 13 pattern) needs extending to include Sylvia's editorial mapping: "Nightly Editorial" → `editorial_nightly` tool.
- **D-06:** Since the Phase 13 mapping was built for Nova/Project #14, the mapping system must be generalized to work per-project/per-executive, not hardcoded to Nova.

### Ownership
- **D-07:** Sylvia owns Project #12. The standing order fires for the owning executive. Sylvia gets the cognition job, executes the tool, and posts results as herself in conversations.
- **D-08:** The editorial output is attributed to Sylvia (via conversation from_agent), not Nova or a system account. This satisfies EDIT-01.

### Voice Rules
- **D-09:** Nathan's reader comments are SACRED — the Thought Police post preserves >80% of his original words. This is enforced by the existing script's system prompt, not by ghost infrastructure.
- **D-10:** The script already handles synthesis quality. Ghost infrastructure just needs to trigger it correctly.

### Claude's Discretion
- How to generalize the label-to-tool mapping for multiple executives (refactor vs per-project sections)
- Whether to register additional editorial tools (delegate_editorial.py, trigger_editorial.py) or just the nightly script
- Error handling if the Anthropic API call inside nightly_editorial.py fails

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Editorial Pipeline Scripts
- `/root/gotcha-workspace/tools/editorial/nightly_editorial.py` — Main editorial script (query, fetch, synthesize, save)
- `/root/gotcha-workspace/tools/editorial/trigger_editorial.py` — Manual trigger utility
- `/root/gotcha-workspace/tools/editorial/delegate_editorial.py` — Delegation helper
- `/root/gotcha-workspace/goals/cognitive-submission-editorial.md` — Full pipeline spec with voice rules and data flow

### Ghost Infrastructure (from Phase 13)
- `/opt/project-noosphere-ghosts/config/tool-registry.json` — Tool registry (add editorial tool here)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — Label-to-tool mapping (extend for Sylvia/Project #12)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Tool execution in project reviews

### OpenClaw Source
- `/root/.openclaw/cron/jobs.json` — "Cognitive Submission: Nightly Editorial" job with full payload description

### Prior Phase
- `.planning/phases/13-operations-pipeline/13-CONTEXT.md` — Operations pipeline decisions (same tool registration pattern)
- `.planning/phases/12-standing-orders/12-CONTEXT.md` — Standing orders framework

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `nightly_editorial.py` — Complete pipeline script, handles all steps including Claude API synthesis
- Phase 13 tool registration pattern — 6 ops_* tools registered with same approach
- Phase 13 label-to-tool mapping in action-planner.lisp — needs generalization for Sylvia
- `execute-project-review` already calls `process-tool-calls` (Phase 13 fix)
- Project #12 already has schedule seeded from Phase 12

### Established Patterns
- Tool registry entry: name, command, scope array, dangerous flag
- Standing order fires → owner gets cognition job → action planner includes tool mapping → ghost executes tool
- Conversation output attributed to the acting executive (STAND-03 from Phase 12)

### Integration Points
- tool-registry.json — Add editorial_nightly entry with `editorial` scope
- action-planner.lisp — Generalize mapping to support multiple projects/executives
- Sylvia's tool_scope already includes `editorial` — perfect match

</code_context>

<specifics>
## Specific Ideas

- The OpenClaw editorial job has 3 consecutive errors (Discord delivery failure) — ghost version won't have this issue since output goes to conversations table
- Sylvia's tool_scope includes 13 capabilities — `editorial` is already there
- The script uses ANTHROPIC_API_KEY from environment — need to verify this is available in the ghost PM2 process (same concern as Phase 13 PG_* vars)
- Export step (`dpn-publish`) is part of the existing script — ghost just needs to trigger the script, export happens automatically

</specifics>

<deferred>
## Deferred Ideas

- Real-time editorial trigger (when Nathan finishes commenting, not just 9 PM) — would require conversation watcher, not a standing order
- Multi-author editorial support (other team members commenting) — out of scope, Thought Police is Nathan-only
- Editorial quality scoring/feedback loop — future enhancement

</deferred>

---

*Phase: 14-editorial-pipeline*
*Context gathered: 2026-03-28*
