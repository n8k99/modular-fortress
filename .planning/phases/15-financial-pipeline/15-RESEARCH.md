# Phase 15: Financial Pipeline - Research

**Researched:** 2026-03-28
**Domain:** Ghost tool registration + standing order integration for trading briefings and calendar sync
**Confidence:** HIGH

## Summary

Phase 15 migrates Kathryn's financial pipeline from OpenClaw cron to ghost-executed standing orders under Project #10. The pattern is identical to Phases 13 (operations) and 14 (editorial): register tools in tool-registry.json, add label-to-tool entries in action-planner.lisp's `tool-mapping-for-label` function, and ensure the Project #10 schedule entries fire correctly.

The existing infrastructure is fully in place. Project #10 already has 3 schedule entries (Tokyo/London/NYC sessions) seeded in Phase 12. Only the Calendar Sync schedule entry needs adding. Both `trading_briefing.py` and `wave_calendar.py` are production-ready scripts already running successfully under OpenClaw cron.

**Primary recommendation:** Register 2 tools (trading_briefing, wave_calendar_sync) in tool-registry.json, add 4 label mappings to action-planner.lisp, and INSERT the Calendar Sync schedule entry into Project #10.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Kathryn executes the existing `trading_briefing.py` script via tool invocation with `--session {tokyo|london|nyc}` flag. No rewrite.
- **D-02:** Three separate standing order labels map to three tool invocations: "Tokyo Session", "London Session", "NYC Session".
- **D-03:** Tools registered with `trading` scope matching Kathryn's tool_scope.
- **D-04:** Calendar sync runs as a separate tool (`wave_calendar_sync`) executing `wave_calendar.py --action sync`.
- **D-05:** Existing Project #10 schedule has 3 entries. Calendar sync needs adding as a 4th: `{"expr": "0 10 * * *", "label": "Calendar Sync"}` (10:00 UTC = 6 AM ET).
- **D-06:** The dynamic `tool-mapping-for-label` function from Phase 14 already supports per-label mapping. Just add Kathryn's 4 label entries.
- **D-07:** Project #10 schedules are already seeded from Phase 12. Only calendar sync schedule needs adding.
- **D-08:** Kathryn owns Project #10. All trading output attributed to Kathryn via from_agent in conversations.
- **D-09:** The `--discord` flag on trading_briefing.py is omitted for ghost execution. Output goes to conversations table.

### Claude's Discretion
- Whether to register a single `trading_briefing` tool with session as arg or three separate tools (one per session)
- Error handling for OANDA/Kalshi API failures
- Whether calendar sync should be `trading` scope or `scheduling` scope (Kathryn has both)

### Deferred Ideas (OUT OF SCOPE)
- Real-time market alerts (price threshold triggers)
- Kalshi position management via ghost tools
- Multi-source calendar aggregation (beyond ForexFactory)
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| FIN-01 | Trading briefings (Tokyo/London/NYC) execute as ghost pipeline under Project #10, owned by Kathryn | Tool registration pattern from Phase 13/14; Project #10 schedule already has 3 session entries; tool-mapping-for-label needs 3 session entries |
| FIN-02 | Each briefing session produces structured output posted to the appropriate channel | trading_briefing.py already produces structured output; ghost execution posts to conversations table via existing STAND-03 flow |
| OPS-05 | Wave calendar sync executes as ghost work under the financial project | wave_calendar_sync tool registration + Calendar Sync label mapping + schedule INSERT into Project #10 |
</phase_requirements>

## Standard Stack

No new libraries needed. This phase uses existing infrastructure only.

### Core (Existing)
| Component | Location | Purpose | Already Exists |
|-----------|----------|---------|----------------|
| trading_briefing.py | `/root/gotcha-workspace/tools/trading_briefing/` | FOREX + Kalshi + structured briefing output | Yes |
| wave_calendar.py | `/root/gotcha-workspace/tools/wave_calendar/` | ForexFactory calendar sync | Yes |
| tool-registry.json | `/opt/project-noosphere-ghosts/config/` | Ghost tool definitions | Yes (add entries) |
| action-planner.lisp | `/opt/project-noosphere-ghosts/lisp/runtime/` | Label-to-tool dynamic mapping | Yes (add entries) |
| Project #10 schedule | `projects` table, id=10 | Standing order cron schedule | Yes (add 1 entry) |

## Architecture Patterns

### Established Tool Registration Pattern (from Phase 13/14)

Three integration points, each following the exact same pattern used for ops and editorial tools:

**1. tool-registry.json entry:**
```json
"tool_name": {
  "script": "/root/gotcha-workspace/tools/<dir>/<script>.py",
  "description": "What the tool does",
  "parameters": { ... },
  "scope": ["matching", "agent", "scopes"],
  "dangerous": false
}
```

**2. action-planner.lisp mapping entry:**
```lisp
((string-equal label "Label Name") (cons "tool_name" "arg_description"))
```

**3. Project schedule entry (SQL):**
```sql
UPDATE projects SET schedule = schedule || '[{"expr": "cron_expr", "label": "Label Name"}]'::jsonb WHERE id = 10;
```

### Discretion Recommendation: Single Tool with Session Arg

Register ONE `trading_briefing` tool with `session` as a parameter. Three separate tools would be redundant since the script is identical -- only the `--session` flag changes. The label-to-tool mapping already supports args:

```lisp
((string-equal label "Tokyo Session")  (cons "trading_briefing" "session: tokyo"))
((string-equal label "London Session") (cons "trading_briefing" "session: london"))
((string-equal label "NYC Session")    (cons "trading_briefing" "session: nyc"))
```

This matches the existing pattern where `ops_health_check` has `"fix: true"` as args.

### Discretion Recommendation: Calendar Sync Scope

Use `trading` scope, not `scheduling`. Rationale: wave_calendar is a trading support tool (ForexFactory data), Kathryn has `trading` in her scope, and keeping all financial tools under one scope is cleaner. The `scheduling` scope is for generic scheduling capabilities.

### Existing Project #10 Schedule (verified from DB)

```json
[
  {"expr": "0 22 * * 0-4", "label": "Tokyo Session"},
  {"expr": "0 6 * * 1-5", "label": "London Session"},
  {"expr": "0 12 * * 1-5", "label": "NYC Session"}
]
```

**Note on CONTEXT.md D-05:** CONTEXT says use `"0 10 * * *"` (10:00 UTC = 6 AM ET). However, the existing schedule entries use UTC times (0 22, 0 6, 0 12 all appear to be UTC -- Tokyo at 22:00 UTC = 6 PM ET matches the OpenClaw `0 18 * * 1-5` ET schedule). Actually, examining more carefully:

- OpenClaw Tokyo: `0 18 * * 1-5` ET = 22:00 UTC. DB has `0 22 * * 0-4`. The day-of-week shifted (Sun-Thu vs Mon-Fri) because UTC midnight crosses day boundary. This is correct -- Tokyo markets open Sunday night ET which is Monday morning Tokyo time.
- OpenClaw London: `0 2 * * 1-5` ET = 06:00 UTC. DB has `0 6 * * 1-5`. Matches.
- OpenClaw NYC: `0 8 * * 1-5` ET = 12:00 UTC. DB has `0 12 * * 1-5`. Matches.

The tick engine uses UTC cron expressions. So Calendar Sync at 6 AM ET = `0 10 * * *` UTC. CONTEXT D-05 is correct.

### Kathryn's Tool Scope (verified from DB)

```
{strategy,kalshi,trading,market,portfolio,risk,compliance,forex,execution,
 watchlist,alerts,signals,analytics,reporting,decision,scheduling,research,
 specs,strategy}
```

Both `trading` and `scheduling` are present. `trading` is the recommended scope for both tools.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Trading data pipeline | Custom FOREX/Kalshi integration | `trading_briefing.py` | Already handles OANDA prices, Kalshi checks, paper trading, structured output |
| Calendar data | Custom ForexFactory scraper | `wave_calendar.py` | Already handles FF API, rate limits, DB storage |
| Schedule-to-tool routing | Custom dispatcher | `tool-mapping-for-label` in action-planner.lisp | Phase 14 already generalized the dynamic mapping |
| Cron scheduling | New scheduler | Project schedule JSONB field + tick engine | Phase 12 built this infrastructure |

## Common Pitfalls

### Pitfall 1: --discord Flag Leaking Into Ghost Execution
**What goes wrong:** Ghost invokes trading_briefing.py with `--discord` flag, posting to Discord instead of conversations table.
**Why it happens:** OpenClaw version uses `--discord`. Copy-paste from OpenClaw job config.
**How to avoid:** Tool registry parameters must NOT include `--discord`. The script defaults to stdout output (which the ghost captures for conversations) when `--discord` is omitted.
**Warning signs:** Output appears in Discord channel instead of conversations table.

### Pitfall 2: UTC vs ET Schedule Confusion
**What goes wrong:** Calendar sync fires at wrong time because cron expr uses ET hours in a UTC context (or vice versa).
**Why it happens:** The tick engine interprets schedule cron expressions in UTC.
**How to avoid:** Always convert ET to UTC for schedule entries. 6 AM ET = 10:00 UTC = `0 10 * * *`.
**Warning signs:** Compare against existing entries (London 6 UTC = 2 AM ET, verified working).

### Pitfall 3: Label String Mismatch Between DB and Lisp
**What goes wrong:** Standing order fires but no tool mapping found -- Kathryn gets a generic cognition job instead of a tool-directed one.
**Why it happens:** Label in projects.schedule JSON doesn't exactly match the string-equal check in Lisp.
**How to avoid:** Use exact same label strings: "Tokyo Session", "London Session", "NYC Session", "Calendar Sync".
**Warning signs:** Kathryn gets cognition jobs but doesn't execute the tool.

### Pitfall 4: ForexFactory Rate Limiting on Calendar Sync
**What goes wrong:** wave_calendar.py fails silently when ForexFactory rate-limits the request.
**Why it happens:** FF has undocumented rate limits on their free JSON API.
**How to avoid:** wave_calendar.py already handles this gracefully (per CONTEXT specifics). No additional handling needed. Ghost should report the error in conversations output.
**Warning signs:** Calendar sync reports 0 events for extended periods.

## Code Examples

### Tool Registry Entry: trading_briefing
```json
"trading_briefing": {
  "script": "/root/gotcha-workspace/tools/trading_briefing/trading_briefing.py",
  "description": "Execute a trading session briefing. Runs FOREX scan, Kalshi check, calendar events, news sentiment, and paper trade execution. Produces structured market summary.",
  "parameters": {
    "session": "Trading session to run: tokyo, london, nyc, or now"
  },
  "scope": ["trading"],
  "dangerous": false,
  "cli_args": ["--session", "{session}"],
  "interpreter": "/root/gotcha-workspace/.venv/bin/python3"
}
```

### Tool Registry Entry: wave_calendar_sync
```json
"wave_calendar_sync": {
  "script": "/root/gotcha-workspace/tools/wave_calendar/wave_calendar.py",
  "description": "Sync ForexFactory economic calendar events into the database. Classifies events by impact (flat/overhead/nazare) and maps to currency pairs.",
  "parameters": {
    "action": "Calendar action: sync (default), today, week, upcoming, blackout"
  },
  "scope": ["trading"],
  "dangerous": false,
  "cli_args": ["--action", "{action}"],
  "interpreter": "/root/gotcha-workspace/.venv/bin/python3"
}
```

### action-planner.lisp Additions (after editorial section)
```lisp
;; Financial (Kathryn / Project #10)
((string-equal label "Tokyo Session")   (cons "trading_briefing" "session: tokyo"))
((string-equal label "London Session")  (cons "trading_briefing" "session: london"))
((string-equal label "NYC Session")     (cons "trading_briefing" "session: nyc"))
((string-equal label "Calendar Sync")   (cons "wave_calendar_sync" "action: sync"))
```

### Project #10 Schedule Update (SQL)
```sql
UPDATE projects
SET schedule = schedule || '[{"expr": "0 10 * * *", "label": "Calendar Sync"}]'::jsonb
WHERE id = 10;
```

After update, Project #10 schedule should be:
```json
[
  {"expr": "0 22 * * 0-4", "label": "Tokyo Session"},
  {"expr": "0 6 * * 1-5", "label": "London Session"},
  {"expr": "0 12 * * 1-5", "label": "NYC Session"},
  {"expr": "0 10 * * *", "label": "Calendar Sync"}
]
```

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual verification via DB queries + ghost tick observation |
| Config file | N/A (no automated test suite for ghost integration) |
| Quick run command | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -c "SELECT schedule FROM projects WHERE id = 10;"` |
| Full suite command | Verify all 3 integration points: tool-registry.json, action-planner.lisp, projects schedule |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| FIN-01 | Trading briefings execute under Project #10 | smoke | `grep -c "trading_briefing" /opt/project-noosphere-ghosts/config/tool-registry.json` + verify tool-mapping-for-label has 3 session entries | N/A |
| FIN-02 | Each briefing produces structured output | smoke | `/root/gotcha-workspace/.venv/bin/python3 /root/gotcha-workspace/tools/trading_briefing/trading_briefing.py --session now --dry-run 2>&1 | head -20` | N/A |
| OPS-05 | Calendar sync as ghost work | smoke | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -t -c "SELECT schedule FROM projects WHERE id = 10;" | grep -c "Calendar Sync"` | N/A |

### Sampling Rate
- **Per task commit:** Verify grep for new entries in modified files
- **Per wave merge:** Run all smoke commands above
- **Phase gate:** Full schedule query + registry check + Lisp mapping inspection

### Wave 0 Gaps
None -- existing infrastructure covers all phase requirements. No test framework needed; verification is file content checks and DB queries.

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/config/tool-registry.json` -- current tool entries (ops + editorial pattern)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` lines 817-830 -- current tool-mapping-for-label
- DB query: `projects WHERE id = 10` -- verified current schedule entries
- DB query: `agents WHERE full_name ILIKE '%kathryn%'` -- verified tool_scope includes trading, scheduling, forex, kalshi
- `/root/.openclaw/cron/jobs.json` -- OpenClaw source jobs (Tokyo ET 18:00, London ET 02:00, NYC ET 08:00, Calendar 06:00 ET)
- `/root/gotcha-workspace/tools/trading_briefing/trading_briefing.py` -- script interface (--session flag)
- `/root/gotcha-workspace/tools/wave_calendar/wave_calendar.py` -- script interface (--action flag)
- `.planning/phases/14-editorial-pipeline/14-01-PLAN.md` -- Phase 14 plan (identical pattern)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- no new dependencies, all existing tools
- Architecture: HIGH -- identical pattern to Phase 13/14, verified from source
- Pitfalls: HIGH -- known issues from prior phases, verified against OpenClaw source

**Research date:** 2026-03-28
**Valid until:** 2026-04-28 (stable infrastructure, no external dependencies changing)
