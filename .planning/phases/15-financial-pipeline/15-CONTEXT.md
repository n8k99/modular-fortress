# Phase 15: Financial Pipeline - Context

**Gathered:** 2026-03-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Kathryn's trading briefings and calendar sync run autonomously as ghost work under Project #10 (Project Complete Success). Three trading sessions (Tokyo, London, NYC) fire on their respective weekday schedules. Wave calendar sync runs daily. All output attributed to Kathryn in the conversations table.

</domain>

<decisions>
## Implementation Decisions

### Trading Briefing Execution
- **D-01:** Kathryn executes the existing `trading_briefing.py` script via tool invocation with `--session {tokyo|london|nyc}` flag. No rewrite — the script handles FOREX data, Kalshi checks, and structured output.
- **D-02:** Three separate standing order labels map to three tool invocations: "Tokyo Session" → `trading_briefing --session tokyo`, "London Session" → `trading_briefing --session london`, "NYC Session" → `trading_briefing --session nyc`.
- **D-03:** Tools registered with `trading` scope matching Kathryn's tool_scope.

### Wave Calendar Sync
- **D-04:** Calendar sync runs as a separate tool (`wave_calendar_sync`) executing `wave_calendar.py --action sync`. This needs a schedule added to Project #10.
- **D-05:** The existing Project #10 schedule has 3 entries (Tokyo/London/NYC). Calendar sync needs adding as a 4th: `{"expr": "0 10 * * *", "label": "Calendar Sync"}` (10:00 UTC = 6 AM ET).

### Standing Order Integration
- **D-06:** The dynamic `tool-mapping-for-label` function from Phase 14 already supports per-label mapping. Just add Kathryn's 4 label-to-tool entries.
- **D-07:** Project #10 schedules are already seeded from Phase 12. Only calendar sync schedule needs adding.

### Ownership
- **D-08:** Kathryn owns Project #10. Standing orders fire for the owning executive. All trading output attributed to Kathryn via from_agent in conversations.
- **D-09:** The `--discord` flag on trading_briefing.py posts to Discord — for ghost execution, we omit this flag. Output goes to conversations table (noosphere-native). Discord delivery is out of scope.

### Claude's Discretion
- Whether to register a single `trading_briefing` tool with session as arg or three separate tools (one per session)
- Error handling for OANDA/Kalshi API failures
- Whether calendar sync should be `trading` scope or `scheduling` scope (Kathryn has both)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Financial Pipeline Scripts
- `/root/gotcha-workspace/tools/trading_briefing/trading_briefing.py` — Trading briefing script with --session flag
- `/root/gotcha-workspace/tools/wave_calendar/wave_calendar.py` — Wave calendar sync with --action sync

### Ghost Infrastructure (from Phase 13+14)
- `/opt/project-noosphere-ghosts/config/tool-registry.json` — Tool registry (add trading tools)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — Dynamic label-to-tool mapping (tool-mapping-for-label function)

### OpenClaw Source
- `/root/.openclaw/cron/jobs.json` — Trading briefing jobs (3) + wave calendar sync job

### Prior Phases
- `.planning/phases/14-editorial-pipeline/14-CONTEXT.md` — Editorial pipeline (dynamic mapping generalization)
- `.planning/phases/13-operations-pipeline/13-CONTEXT.md` — Operations pipeline (tool registration pattern)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `trading_briefing.py` — Complete script, handles FOREX + Kalshi + structured output
- `wave_calendar.py` — Calendar sync with ForexFactory integration
- Phase 14's `tool-mapping-for-label` — Dynamic mapping, just add entries
- Phase 13's tool registration pattern — identical approach

### Established Patterns
- Tool registry: name, command, scope, dangerous flag
- Dynamic mapping: `tool-mapping-for-label` returns (tool-name . args) for any label
- Standing order fires → owner gets cognition job → mapping provides tool → ghost executes

### Integration Points
- tool-registry.json — Add trading_briefing and wave_calendar_sync entries
- action-planner.lisp — Add 4 label entries to tool-mapping-for-label
- Project #10 schedule — Add Calendar Sync entry via psql

</code_context>

<specifics>
## Specific Ideas

- Tokyo fires Sun-Thu 10 PM UTC (markets open Sunday night ET), London Mon-Fri 6 AM UTC, NYC Mon-Fri 12 PM UTC
- OpenClaw trading jobs use `--discord` flag — ghost version omits it (output to conversations)
- Wave calendar sync checks ForexFactory and may be rate-limited — script handles this gracefully
- Kathryn's tool_scope has 19 capabilities including `trading`, `scheduling`, `forex`, `kalshi`

</specifics>

<deferred>
## Deferred Ideas

- Real-time market alerts (price threshold triggers) — would need a market data watcher, not a standing order
- Kalshi position management via ghost tools — separate capability
- Multi-source calendar aggregation (beyond ForexFactory) — future enhancement

</deferred>

---

*Phase: 15-financial-pipeline*
*Context gathered: 2026-03-28*
