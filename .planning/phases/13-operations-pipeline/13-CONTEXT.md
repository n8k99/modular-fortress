# Phase 13: Operations Pipeline - Context

**Gathered:** 2026-03-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Nova's daily operational cadence runs autonomously as ghost work under Project #14 (Operation Normality). This phase migrates 6 OpenClaw cron jobs into ghost-executed standing orders: daily health check, daily note population, nightly synthesis, podcast watcher, weekly finalization, and monthly finalization. The standing orders framework (Phase 12) provides the schedule trigger mechanism — this phase wires Nova's cognition to execute the actual operational work when those schedules fire.

</domain>

<decisions>
## Implementation Decisions

### Ghost Tool Execution
- **D-01:** Nova executes existing gotcha-workspace Python scripts via the `execute_code` tool (Claude CLI). No rewrite of operational scripts — they are proven and tested.
- **D-02:** Each standing order label maps to a specific tool invocation: "Daily Health Check" runs `health_check.py`, "Daily Note Population" runs `daily_note_populate.py`, "Nightly Synthesis" runs `nightly_daily_to_weekly.py` (or synthesis equivalent), etc.
- **D-03:** The action planner prompt must include enough context for Nova to know WHICH script to run based on the schedule label. This is a prompt engineering task, not a code architecture task.

### Output Destinations
- **D-04:** Operational output goes to the conversations table attributed to Nova. Health check summaries, synthesis results, and podcast reports all post as Nova's conversation messages.
- **D-05:** Discord delivery is not in scope for this phase. Conversations are the noosphere-native output. If Discord posting is needed, it's a separate concern handled by existing bridges or a future phase.

### Temporal Cascade (Daily/Weekly/Monthly)
- **D-06:** Temporal compression uses the existing schedule entries on Project #14. Nightly synthesis fires at 04:05 UTC, weekly finalization fires Saturday 04:30 UTC, monthly fires 1st at 05:00 UTC. These are independent standing order fires, not chained tasks.
- **D-07:** Each temporal level reads the output of the prior level from vault_notes (daily reads today's note, weekly reads the week's dailies, monthly reads the month's weeklies). This is the existing pattern in the Python scripts — no new infrastructure needed.
- **D-08:** The scripts already handle the temporal cascade logic (wikilinks, frontmatter references). Nova just needs to execute them and report the result.

### Podcast Watcher
- **D-09:** Nova executes `podcast_watcher.py` on the "Podcast Watch" schedule. The script checks RSS feeds for Living Room Music and Myths of Orbis, posts new episodes to Discord via webhook.
- **D-10:** Podcast watcher is currently scheduled at 23:10 UTC in OpenClaw. This schedule should be added to Project #14 if not already present.

### Cognition Prompt Design
- **D-11:** The key challenge is prompt engineering: Nova's project review cognition prompt must translate schedule labels into specific tool invocations. The action planner needs a mapping section that says "when you see label X, run tool Y with args Z."
- **D-12:** This mapping lives in the cognition prompt template (action-planner.lisp), not in the noosphere. It's code, not data.

### Claude's Discretion
- Exact prompt wording for the schedule-to-tool mapping
- Whether to batch multiple schedule fires in a single tick (e.g., if daily note population and health check fire close together)
- Error handling if a script fails — retry logic or just report the failure in conversations

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Operational Scripts (migration source)
- `/root/gotcha-workspace/tools/self_improvement/health_check.py` — Daily health check with --fix flag
- `/root/gotcha-workspace/tools/self_improvement/manifest_audit.py` — Manifest audit (runs with health check)
- `/root/gotcha-workspace/tools/temporal-sync/daily_note_populate.py` — Daily note population
- `/root/gotcha-workspace/tools/temporal-sync/nightly_daily_to_weekly.py` — Nightly synthesis (daily to weekly rollup)
- `/root/gotcha-workspace/tools/temporal-sync/rollup_weekly.py` — Weekly finalization
- `/root/gotcha-workspace/tools/temporal-sync/rollup_monthly.py` — Monthly finalization
- `/root/gotcha-workspace/tools/discord/podcast_watcher.py` — Podcast RSS checker

### OpenClaw Cron Jobs (schedule source)
- `/root/.openclaw/cron/jobs.json` — 14 active jobs with cron expressions, payloads, and delivery targets

### Ghost Infrastructure
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — Cognition prompt construction, especially `build-project-review-job` (line ~799)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Tool execution and conversation posting
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — Schedule evaluation in ranking phase

### Prior Phase
- `.planning/phases/12-standing-orders/12-CONTEXT.md` — Standing orders framework decisions
- `.planning/phases/12-standing-orders/12-01-SUMMARY.md` — Schedule infrastructure implementation
- `.planning/phases/12-standing-orders/12-02-SUMMARY.md` — Tick engine integration

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- All 6 operational Python scripts exist and work — no rewrites needed
- Standing order schedule already seeded on Project #14 (5 entries: daily health, nightly synthesis, daily note, weekly, monthly)
- `build-project-review-job` already includes schedule labels in cognition prompt
- Nova's tool_scope includes code execution via Claude CLI

### Established Patterns
- Standing order fires inject project owner into acting-set with +50 boost
- Action planner receives schedule labels via `*schedule-fired-labels*` hash table
- Cognition jobs include project context and fired label names
- Tool execution via `execute_code` action type in action-executor.lisp

### Integration Points
- `action-planner.lisp` — Needs mapping from schedule labels to tool invocations in the project review prompt
- `action-executor.lisp` — May need to handle operational script output formatting
- Project #14 schedule — May need podcast watcher schedule added (23:10 UTC)

</code_context>

<specifics>
## Specific Ideas

- OpenClaw's operational jobs all target Discord channel 1421565389745295441 for output — ghost equivalent is conversations table, not Discord
- Health check runs with `--fix` flag (auto-remediation) and PG connection env vars
- Nightly synthesis reads daily note content + nova_memories column from vault_notes
- Weekly/monthly finalization follows frontmatter wikilinks to cascade temporal notes
- Podcast watcher posts to Discord via webhook — this is the one job that has an external output target
- All times in OpenClaw are America/New_York timezone — standing order schedules on Project #14 are already UTC-converted

</specifics>

<deferred>
## Deferred Ideas

- Discord output bridge for ghost conversations (ghosts post to noosphere, a bridge publishes to Discord)
- Tasks archive migration (Discord #tasks archive every 6h) — separate from Nova's operational cadence
- Pipeline wakeup job migration — this is an OpenClaw internal concern, not an operational pipeline
- Conversations poll migration — also OpenClaw internal

</deferred>

---

*Phase: 13-operations-pipeline*
*Context gathered: 2026-03-28*
