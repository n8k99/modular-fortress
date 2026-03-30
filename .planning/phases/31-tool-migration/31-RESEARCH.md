# Phase 31: Tool Migration - Research

**Researched:** 2026-03-30
**Domain:** Lisp runtime tool invocation, Python subprocess dispatch, DB-sourced tool definitions
**Confidence:** HIGH

## Summary

Phase 31 migrates the ghost tool system from the static `tool-registry.json` file to a DB-sourced model (area_content with content_type='tool') where the noosphere resolver dispatches Python scripts when evaluating InnateScipt `![tool_name]` expressions. The registry contains 44 tools spanning trading, content, engineering, ops, and creative domains. Nine ghost YAML files already declare some `![]` expressions, but most tools are missing from YAML responsibilities. The D-11 fallback pattern exists in exactly 2 code locations in action-planner.lisp (lines 480-488 in `build-pipeline-task-job`), plus the build-message-job and build-task-job and build-proactive-job functions load YAML capabilities but do NOT have D-11 fallbacks (they just use YAML-only). The core execution path in `execute-tool-call` (tool-socket.lisp) resolves tool names against `*tool-registry*` hash-table and must be refactored to use DB lookups.

**Primary recommendation:** Insert all 44 tool definitions as area_content rows (content_type='tool', area_id=5 for infrastructure-systems), refactor `execute-tool-call` to load from DB instead of `*tool-registry*`, extend `resolve-search` in noosphere-resolver.lisp to detect tool expressions and dispatch to execution, add missing tool `![]` expressions to all 9 ghost YAMLs, then remove the D-11 fallback blocks and delete tool-registry.json.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01: All tools in tool-registry.json need InnateScipt expression wrappers in ghost YAML responsibilities
- D-02: Ghost YAML files already declare some tools as `![]` search expressions; missing tools must be added
- D-03: Python invocation via `uiop:run-program` subprocess (existing pattern)
- D-04: The `resolve-search` method extended to detect tool expressions and dispatch to Python script execution
- D-05: Tool invocation uses existing `![tool_name]` search expression syntax
- D-06: Tool arguments passed as InnateScipt key-value pairs: `![tool_name]{key=value, key2=value2}`
- D-07: Tool metadata migrates to master_chronicle (DB is the OS) via area_content pattern
- D-08: execute-tool-call refactored to use DB-sourced tool definitions
- D-09: Remove D-11 fallback pattern in action-planner blocks
- D-10: Delete tool-registry.json and remove load-tool-registry / *tool-registry*
- D-11: All ghosts must have YAML files with complete capability declarations before registry deletion
- D-12: Results flow via existing cognition pipeline (conversation output)

### Claude's Discretion
- Exact tool_definitions table schema or area_content content_type for tool metadata
- Which Python scripts need path adjustments for the new invocation mechanism
- Whether to batch-migrate all tools at once or incrementally
- Exact resolve-search dispatch logic for tool vs table-lookup expressions

### Deferred Ideas (OUT OF SCOPE)
- Dynamic tool registration by ghosts (creating new tools at runtime)
- Tool execution metrics/logging in a dedicated table
- Tool permission system beyond "dangerous" flag
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| TOOL-01 | Existing Python tools (Kalshi, trading, ops) wrapped as InnateScipt expressions | Complete tool inventory (44 tools), YAML gap analysis showing which need adding per ghost |
| TOOL-02 | Noosphere resolver can invoke Python scripts when evaluating InnateScipt tool expressions | resolve-search extension pattern documented, execute-tool-call invocation flow mapped |
| TOOL-03 | tool-registry.json retired -- all tool access flows through InnateScipt capabilities | D-11 fallback locations identified (lines 479-488), registry removal steps documented |
| TOOL-04 | Tool execution results flow back through the same cognition pipeline | Existing process-tool-calls pattern in action-executor.lisp confirmed as correct flow |
</phase_requirements>

## Complete Tool Inventory (from tool-registry.json)

### All 44 Tools

| # | Tool Name | Script Path | Dangerous | Special Handler | Scope Count |
|---|-----------|-------------|-----------|-----------------|-------------|
| 1 | query_db | (psql command) | No | Yes (inline SQL) | 16 |
| 2 | write_document | (inline INSERT) | No | Yes (write_document) | 8 |
| 3 | build_tool | /opt/project-noosphere-ghosts/tools/tool-builder.sh | No | Yes (async) | 3 |
| 4 | claude_code | /opt/project-noosphere-ghosts/tools/claude-code-tool.sh | No | No (interpreter=/bin/bash) | 2 |
| 5 | list_tasks | /root/gotcha-workspace/tools/engineering/list_unassigned_tasks.py | No | No | 6 |
| 6 | assign_task | /root/gotcha-workspace/tools/engineering/assign_task.py | No | No | 4 |
| 7 | search_documents | /root/gotcha-workspace/tools/memory/hybrid_search.py | No | No (positional_arg=query) | 12 |
| 8 | write_tool_spec | /root/gotcha-workspace/tools/engineering/write_tool_spec.py | No | No | 3 |
| 9 | security_review | /root/gotcha-workspace/tools/engineering/security_review.py | No | No | 4 |
| 10 | run_tool_tests | /root/gotcha-workspace/tools/engineering/run_tool_tests.py | No | No | 2 |
| 11 | deploy_tool | /root/gotcha-workspace/tools/engineering/deploy_tool.py | No | No | 2 |
| 12 | lookup_agents | /root/gotcha-workspace/tools/engineering/lookup_agents.py | No | No | 5 |
| 13 | read_schedule | /root/gotcha-workspace/tools/engineering/read_schedule.py | No | No | 4 |
| 14 | read_own_memory | /root/gotcha-workspace/tools/engineering/read_own_memory.py | No | No (scope=*) | * |
| 15 | write_own_memory | /root/gotcha-workspace/tools/engineering/write_own_memory.py | No | No (scope=*) | * |
| 16 | delegate_investment | /root/gotcha-workspace/tools/investment/delegate_investment.py | No | No | 6 |
| 17 | delegate_editorial | /root/gotcha-workspace/tools/editorial/delegate_editorial.py | No | No | 5 |
| 18 | delegate_modular_fortress | /root/gotcha-workspace/tools/modular_fortress/delegate_modular_fortress.py | No | No | 5 |
| 19 | trade_executor | /root/gotcha-workspace/tools/trade_executor/trade_executor.py | **Yes** | No (positional_arg=action) | 4 |
| 20 | thesis_writer | /root/gotcha-workspace/tools/thesis_writer/thesis_writer.py | No | No (positional_arg=topic) | 4 |
| 21 | market_scanner | /root/gotcha-workspace/tools/market_scanner/market_pulse.py | No | No (positional_arg=mode) | 4 |
| 22 | market_data | /root/gotcha-workspace/tools/market_data/market_data.py | No | No | 4 |
| 23 | news_aggregator | /root/gotcha-workspace/tools/news_aggregator/news_aggregator.py | No | No (positional_arg=action) | 5 |
| 24 | forex_trader | /root/gotcha-workspace/tools/forex_trader/forex_trader.py | **Yes** | No | 3 |
| 25 | forex_sentiment | /root/gotcha-workspace/tools/forex_sentiment/forex_sentiment.py | No | No | 4 |
| 26 | forex_fitness | /root/gotcha-workspace/tools/forex_fitness/forex_fitness.py | No | No | 4 |
| 27 | kalshi_integration | /root/gotcha-workspace/tools/kalshi_integration/kalshi_integration.py | **Yes** | No (positional_arg=action) | 4 |
| 28 | kalshi_market_integration | /root/gotcha-workspace/tools/kalshi_market_integration/kalshi_market_integration.py | **Yes** | No (positional_arg=action) | 4 |
| 29 | kalshi_thesis_filter | /root/gotcha-workspace/tools/kalshi_thesis_filter/kalshi_thesis_filter.py | **Yes** | No (positional_arg=command) | 4 |
| 30 | position_tracker | /root/gotcha-workspace/tools/position_tracker/position_tracker.py | **Yes** | No (positional_arg=command) | 5 |
| 31 | risk_calculator | /root/gotcha-workspace/tools/risk_calculator/risk_calculator.py | No | No | 5 |
| 32 | exposure_audit | /root/gotcha-workspace/tools/exposure_audit/exposure_audit.py | No | No | 5 |
| 33 | regulatory_check | /root/gotcha-workspace/tools/regulatory_check/regulatory_check.py | No | No | 5 |
| 34 | probability_model | /root/gotcha-workspace/tools/probability_model/probability_model.py | No | No (positional_arg=command) | 4 |
| 35 | trade_journal | /root/gotcha-workspace/tools/trade_journal/trade_journal.py | No | No | 4 |
| 36 | trading_briefing | /root/gotcha-workspace/tools/trading_briefing/trading_briefing.py | No | No (cli_args pattern) | 1 |
| 37 | market_data_ingest | /root/gotcha-workspace/tools/market_data_ingest/market_data_ingest.py | No | No | 4 |
| 38 | market_data_processor | /root/gotcha-workspace/tools/market_data_processor/market_data_processor.py | No | No | 4 |
| 39 | market_monitoring | /root/gotcha-workspace/tools/market_monitoring/market_monitoring.py | **Yes** | No | 5 |
| 40 | market_dashboard | /root/gotcha-workspace/tools/market_dashboard/market_dashboard.py | No | No | 5 |
| 41 | market_pulse_monitor | /root/gotcha-workspace/tools/market_pulse_monitor/market_pulse_monitor.py | No | No (positional_arg=command) | 5 |
| 42 | wave_calendar_sync | /root/gotcha-workspace/tools/wave_calendar/wave_calendar.py | No | No (cli_args pattern) | 1 |
| 43 | article_fetcher | /root/gotcha-workspace/tools/article_fetcher/article_fetcher.py | No | No (positional_arg=command) | 3 |
| 44 | article_parser | /root/gotcha-workspace/tools/article_parser/article_parser.py | No | No (positional_arg=command) | 3 |
| 45 | article_summarizer | /root/gotcha-workspace/tools/article_summarizer/article_summarizer.py | No | No (positional_arg=command) | 3 |
| 46 | comment_collector | /root/gotcha-workspace/tools/comment_collector/comment_collector.py | No | No (positional_arg=command) | 3 |
| 47 | rss_feed_ingester | /root/gotcha-workspace/tools/rss_feed_ingester/rss_feed_ingester.py | No | No (positional_arg=command) | 4 |
| 48 | sentiment_analyzer | /root/gotcha-workspace/tools/sentiment_analyzer/sentiment_analyzer.py | No | No (positional_arg=command) | 4 |
| 49 | thought_police_publisher | /root/gotcha-workspace/tools/thought_police_publisher/publish.py | **Yes** | No | 3 |
| 50 | morning_pages_parser | /root/gotcha-workspace/tools/morning_pages_parser/main.py | No | No | 3 |
| 51 | editorial_composer | /root/gotcha-workspace/tools/editorial_composer/compose.py | **Yes** | No | 3 |
| 52 | codebase_scanner | /root/gotcha-workspace/tools/codebase_scanner/scan.py | No | No | 3 |
| 53 | security_linter | /root/gotcha-workspace/tools/security_linter/security_linter.py | No | No | 4 |
| 54 | dependency_mapper | /root/gotcha-workspace/tools/dependency_mapper/dependency_mapper.py | No | No | 3 |
| 55 | pattern_detector | /root/gotcha-workspace/tools/pattern_detector/detect.py | No | No | 3 |
| 56 | cascade_reporter | /root/gotcha-workspace/tools/cascade_reporter/cascade_reporter.py | No | No | 4 |
| 57 | event_tracker | /root/gotcha-workspace/tools/event_tracker/event_tracker.py | **Yes** | No (positional_arg=command) | 3 |
| 58 | recurrence_engine | /root/gotcha-workspace/tools/recurrence_engine/recurrence_engine.py | **Yes** | No | 3 |
| 59 | burg_pipeline | /root/gotcha-workspace/tools/burg_pipeline/burg_pipeline.py | **Yes** | No | 3 |
| 60 | memory_compressor | /root/gotcha-workspace/tools/memory_compressor/memory_compressor.py | **Yes** | No (positional_arg=command) | 3 |
| 61 | self_improvement | /root/gotcha-workspace/tools/self_improvement/health_check.py | **Yes** | No | 3 |
| 62 | ops_health_check | /root/gotcha-workspace/tools/self_improvement/health_check.py | No | No | 3 |
| 63 | ops_daily_note | /root/gotcha-workspace/tools/temporal-sync/daily_note_populate.py | No | No | 2 |
| 64 | ops_nightly_synthesis | /root/gotcha-workspace/tools/temporal-sync/nightly_daily_to_weekly.py | No | No | 2 |
| 65 | ops_weekly_rollup | /root/gotcha-workspace/tools/temporal-sync/rollup_weekly.py | No | No | 2 |
| 66 | ops_monthly_rollup | /root/gotcha-workspace/tools/temporal-sync/rollup_monthly.py | No | No | 2 |
| 67 | ops_podcast_watcher | /root/gotcha-workspace/tools/discord/podcast_watcher.py | No | No | 2 |
| 68 | save_spec | /root/.openclaw/skills/spec-writer/scripts/save_spec.py | **Yes** | No | 6 |
| 69 | create_pipeline | /root/.openclaw/skills/spec-writer/scripts/create_pipeline.py | **Yes** | No | 5 |
| 70 | research_topic | /root/.openclaw/skills/spec-writer/scripts/research.py | No | No (positional_arg=topic) | 11 |
| 71 | thematic_matcher | /root/gotcha-workspace/tools/thematic_matcher/thematic_matcher.py | No | No (positional_arg=command) | 4 |
| 72 | worldbuilding_pipeline | /root/gotcha-workspace/tools/orbis/worldbuilding_pipeline.py | No | No (positional_args) | 4 |
| 73 | burg_template | (psql command) | No | No | 4 |
| 74 | delegate_worldbuilding | /root/gotcha-workspace/tools/orbis/delegate_worldbuilding.py | No | No (cli_args pattern) | 5 |
| 75 | editorial_nightly | /root/gotcha-workspace/tools/editorial/nightly_editorial.py | No | No | 1 |

**Note:** Actual count is higher than 44 -- there are 75 tool entries in the registry. The initial description underestimated.

### Tool Categories

| Category | Tools | Count |
|----------|-------|-------|
| **Engineering** | query_db, write_document, build_tool, claude_code, list_tasks, assign_task, write_tool_spec, security_review, run_tool_tests, deploy_tool, lookup_agents, codebase_scanner, security_linter, dependency_mapper, pattern_detector, cascade_reporter | 16 |
| **Trading/Market** | trade_executor, thesis_writer, market_scanner, market_data, forex_trader, forex_sentiment, forex_fitness, kalshi_integration, kalshi_market_integration, kalshi_thesis_filter, position_tracker, risk_calculator, exposure_audit, probability_model, trade_journal, trading_briefing, market_data_ingest, market_data_processor, market_monitoring, market_dashboard, market_pulse_monitor, delegate_investment | 22 |
| **Content/Editorial** | search_documents, article_fetcher, article_parser, article_summarizer, comment_collector, rss_feed_ingester, sentiment_analyzer, thought_police_publisher, editorial_composer, editorial_nightly, delegate_editorial, morning_pages_parser, news_aggregator | 13 |
| **Ops/Temporal** | read_schedule, ops_health_check, ops_daily_note, ops_nightly_synthesis, ops_weekly_rollup, ops_monthly_rollup, ops_podcast_watcher, self_improvement, wave_calendar_sync, event_tracker, recurrence_engine, memory_compressor | 12 |
| **Universal** | read_own_memory, write_own_memory | 2 |
| **Strategy** | save_spec, create_pipeline, research_topic, delegate_modular_fortress, regulatory_check | 5 |
| **Worldbuilding/Creative** | burg_pipeline, burg_template, worldbuilding_pipeline, delegate_worldbuilding | 4 |
| **Compliance** | regulatory_check, exposure_audit | (shared with trading) |

### Dangerous Tools (16 total)

trade_executor, forex_trader, kalshi_integration, kalshi_market_integration, kalshi_thesis_filter, position_tracker, market_monitoring, thought_police_publisher, editorial_composer, event_tracker, recurrence_engine, burg_pipeline, memory_compressor, self_improvement, save_spec, create_pipeline

### Special Handler Tools (3 inline implementations)

1. **query_db** -- runs psql directly via `uiop:run-program`, NOT via Python script. Read-only SQL guard built into `execute-tool-call`.
2. **write_document** -- INSERT/UPSERT into documents table directly via psql.
3. **build_tool** -- async subprocess via `uiop:launch-program` (not run-program), reads spec from file/DB/args.

These 3 have inline Lisp implementations in `execute-tool-call` that bypass the general Python script path. They need special handling in the DB migration.

## Ghost YAML Current State vs Required

### Current YAML Capabilities

| Ghost | Current Responsibilities | Missing Tools (by old scope match) |
|-------|------------------------|------------------------------------|
| **nova** | `![query_db]`, `![pipeline_status]`, `![claude_code]` | ops_health_check, ops_daily_note, ops_nightly_synthesis, ops_weekly_rollup, ops_monthly_rollup, ops_podcast_watcher, self_improvement, read_schedule, read_own_memory, write_own_memory |
| **eliana** | `![build_tool]`, `![claude_code]`, `![query_db]` | list_tasks, assign_task, write_tool_spec, security_review, run_tool_tests, deploy_tool, lookup_agents, codebase_scanner, security_linter, dependency_mapper, pattern_detector, cascade_reporter, read_own_memory, write_own_memory |
| **kathryn** | `![market_scanner]`, `{em.strategy.portfolio}` | trade_executor, thesis_writer, market_data, forex_trader, forex_sentiment, forex_fitness, kalshi_integration, kalshi_market_integration, kalshi_thesis_filter, position_tracker, risk_calculator, exposure_audit, probability_model, trade_journal, trading_briefing, market_data_ingest, market_data_processor, market_monitoring, market_dashboard, market_pulse_monitor, wave_calendar_sync, delegate_investment, news_aggregator, regulatory_check, read_own_memory, write_own_memory |
| **sylvia** | `![write_document]`, `{em.content.blog}`, `{em.content.thought.police}` | search_documents, article_fetcher, article_parser, article_summarizer, comment_collector, rss_feed_ingester, sentiment_analyzer, thought_police_publisher, editorial_composer, editorial_nightly, delegate_editorial, morning_pages_parser, delegate_worldbuilding, read_own_memory, write_own_memory |
| **vincent** | `![write_document]`, `{em.creative.covers}` | search_documents, read_own_memory, write_own_memory |
| **jmax** | `![query_db]`, `{em.legal.compliance}` | regulatory_check, exposure_audit, search_documents, read_own_memory, write_own_memory |
| **lrm** | `![query_db]`, `{em.music.archive}` | search_documents, read_own_memory, write_own_memory |
| **sarah** | `![query_db]` | list_tasks, assign_task, lookup_agents, read_schedule, read_own_memory, write_own_memory |
| **ethan_ng** | `![fundamentals.feeds]`, `![technicals.oanda_api]`, `{em.content.podcast}` | forex_trader, forex_sentiment, forex_fitness, market_data, news_aggregator, read_own_memory, write_own_memory |

**Key insight:** `read_own_memory` and `write_own_memory` have scope `"*"` (all agents). Every ghost needs these. `query_db` has 16 scopes covering nearly all ghosts.

### YAML Capability Assignment Strategy

Ghost capabilities should map to their departmental role (per CLAUDE.md executive roster), NOT just recreate old scope matching:

- **Nova (COO):** ops tools, health checks, temporal tools, pipeline_status, query_db, claude_code
- **Eliana (CTO):** engineering tools, build_tool, claude_code, code analysis tools
- **Kathryn (CSO):** all trading/market tools, wave_calendar, risk tools
- **Sylvia (Content):** all editorial/content tools, article tools, search_documents
- **Vincent (Creative):** write_document, search_documents (limited -- visual domain)
- **JMax (Legal):** compliance tools, regulatory_check, exposure_audit, query_db
- **LRM (Music):** query_db, search_documents (music domain)
- **Sarah (PA):** list_tasks, assign_task, lookup_agents, read_schedule, query_db
- **Ethan Ng (Staff):** FOREX-specific tools, market_data, news feeds

## Architecture Patterns

### Tool Definition in area_content (Recommended Schema)

Use existing `area_content` table with `content_type = 'tool'` and metadata JSONB:

```sql
INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (
  5,  -- infrastructure-systems area
  'tool',
  'market_scanner',  -- tool name = title
  'Scan markets for momentum signals. Covers stocks, crypto, commodities via free APIs.',  -- description = body
  '{
    "script": "/root/gotcha-workspace/tools/market_scanner/market_pulse.py",
    "interpreter": "/root/gotcha-workspace/.venv/bin/python3",
    "parameters": {"mode": "scan|report"},
    "positional_arg": "mode",
    "dangerous": false,
    "cli_args": null
  }'::jsonb,
  'active'
);
```

**Why area_content not a new table:** Follows the exact pattern from Phase 30 pipeline-definitions.lisp. The planner wrote `load-pipeline-definitions` to query `area_content WHERE content_type = 'pipeline'`. Tool definitions follow identically with `content_type = 'tool'`.

### Tool Definition Loading Pattern (from pipeline-definitions.lisp)

```lisp
;; Pattern from pipeline-definitions.lisp line 36-51
(defun load-tool-definitions ()
  "Load all active tool definitions from area_content.
   Returns hash-table mapping tool-name-keyword to tool-def hash-table."
  (handler-case
      (let* ((sql "SELECT title, body, metadata FROM area_content WHERE content_type = 'tool' AND status = 'active'")
             (rows (db-query sql))
             (registry (make-hash-table :test #'equal)))
        (loop for i from 0 below (length rows)
              for row = (aref rows i)
              for name = (gethash :TITLE row)
              for raw-meta = (gethash :METADATA row)
              for meta = (if (stringp raw-meta) (parse-json raw-meta) raw-meta)
              when (and name (hash-table-p meta))
              do (setf (gethash :DESCRIPTION meta) (or (gethash :BODY row) ""))
                 (setf (gethash (normalize-tool-key name) registry) meta))
        registry)
    (error (e)
      (format t "[tool-defs] Error loading: ~a~%" e)
      (make-hash-table :test #'equal))))
```

### resolve-search Extension Pattern

The current `resolve-search` in noosphere-resolver.lisp (line 220-244) dispatches based on `search-type-to-table`. For tool invocation, add a tool dispatch path:

```lisp
(defmethod resolve-search ((r noosphere-resolver) search-type terms)
  "Resolve ![type]{key=value} search directives.
   If search-type matches a known tool name, invoke the tool.
   Otherwise, map to table and run SQL query."
  ;; Check if this is a tool invocation
  (let ((tool-def (lookup-tool-definition search-type)))
    (if tool-def
        ;; Tool dispatch: invoke Python script, return result
        (invoke-tool search-type tool-def terms)
        ;; Existing behavior: table lookup
        (let ((table (search-type-to-table search-type)))
          ...existing code...))))
```

### execute-tool-call Refactoring

Current flow (tool-socket.lisp lines 173-303):
1. `(unless *tool-registry-loaded* (load-tool-registry))` -- loads from JSON file
2. `(gethash (normalize-tool-key tool-name) *tool-registry*)` -- hash lookup
3. Special handlers for query_db, write_document, build_tool
4. General Python script path via `uiop:run-program`

New flow:
1. Load tool definitions from area_content (cached per tick, like pipeline-definitions)
2. Lookup by normalized key from DB-sourced hash-table
3. Special handlers preserved for query_db, write_document, build_tool (inline implementations)
4. General Python script path unchanged

### D-11 Fallback Removal Targets

Exact locations in action-planner.lisp:

**Location 1: `build-pipeline-task-job`** (lines 469-496)
```lisp
;; Lines 469-488: YAML capabilities first, tool-registry.json fallback (D-09, D-10, D-11)
(yaml-capabilities (handler-case ...))
(capabilities-prompt (when yaml-capabilities ...))
;; Lines 479-488: Fallback to tool-registry.json if no YAML capabilities (per D-11)
(agent-tools (unless yaml-capabilities
               (handler-case
                   (let ((fn (find-symbol "GET-TOOLS-FOR-AGENT" :af64.runtime.action-executor)))
                     (if fn (funcall fn agent-id) '()))
                 (error () '()))))
(tools-prompt (unless yaml-capabilities
                (handler-case
                    (let ((fn (find-symbol "FORMAT-TOOLS-FOR-PROMPT" :af64.runtime.action-executor)))
                      (if fn (funcall fn agent-tools) nil))
                  (error () nil))))
(effective-prompt (or capabilities-prompt tools-prompt))
```

**After removal:** Remove `agent-tools`, `tools-prompt`, and `effective-prompt` bindings. Replace with just `capabilities-prompt` used directly. The `yaml-capabilities` loading stays.

**Locations 2-4 (build-message-job, build-task-job, build-proactive-job):** These already use YAML-only pattern (no D-11 fallback). They load `yaml-capabilities` and `cap-prompt` but do NOT have `unless yaml-capabilities` fallbacks. No changes needed for D-09 removal in these. Confirmed by grep -- only `build-pipeline-task-job` has the `unless yaml-capabilities` pattern.

**Correction:** The CONTEXT.md mentions "4 D-11 fallback blocks" but grep confirms only 1 location (lines 480, 485 in build-pipeline-task-job). The other 3 prompt builders (build-message-job at ~326, build-task-job at ~558, build-proactive-job at ~740) load YAML capabilities but have NO tool-registry fallback -- they only inject capabilities when YAML is available.

### Result Flow (Already Correct)

Tool results already flow correctly through the cognition pipeline:

1. `execute-work-task` (action-executor.lisp ~443): calls `process-tool-calls` -> appends results to content -> posts via `db-update-task` with stage_notes
2. `execute-project-review` (action-executor.lisp ~1002): calls `process-tool-calls` -> concatenates results -> posts via `db-insert-conversation`

No changes needed for TOOL-04.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Tool definition storage | New table or file format | area_content with content_type='tool' | Pattern proven in Phase 30, area_content already has JSONB metadata column |
| Per-tick cache refresh | Custom cache invalidation | Reload pattern from pipeline-definitions.lisp | 30s-10min tick = ~1ms query cost is negligible |
| Python subprocess invocation | New execution layer | Existing uiop:run-program in execute-tool-call | Already handles output capture, error handling, CLI arg building |
| Tool result formatting | New output pipeline | Existing process-tool-calls in action-executor | Already handles result concatenation, conversation posting |

## Common Pitfalls

### Pitfall 1: Special Handler Tools Need Preservation
**What goes wrong:** Deleting tool-registry.json without preserving the 3 inline tool handlers (query_db, write_document, build_tool) in execute-tool-call.
**Why it happens:** These tools have NO script path -- they are implemented directly in Lisp with special cond branches.
**How to avoid:** The refactored execute-tool-call must keep these 3 special handlers as cond branches that fire BEFORE the general DB-lookup path. The DB metadata for these tools should set `script: null` and `special_handler: "query_db"` etc. so the resolver knows they exist but dispatches to inline code.
**Warning signs:** query_db, write_document, or build_tool returning "tool not found" errors.

### Pitfall 2: normalize-tool-key Conversion
**What goes wrong:** Tool names with underscores don't match after keyword conversion.
**Why it happens:** The JSON parser converts underscores to hyphens (`:MARKET-SCANNER` not `:MARKET_SCANNER`). The `normalize-tool-key` function also converts underscores to hyphens.
**How to avoid:** Store tool names in area_content.title as snake_case (e.g., "market_scanner"). The `normalize-tool-key` function handles conversion when looking up. Use the same normalization in the new loader.
**Warning signs:** Tools found in DB but not matching at lookup time.

### Pitfall 3: Dangerous Tool Guard Must Survive Migration
**What goes wrong:** Dangerous tools become executable without the guard.
**Why it happens:** The `dangerous` flag lives in tool-registry.json metadata. Must migrate to area_content.metadata.
**How to avoid:** Include `"dangerous": true/false` in every tool's JSONB metadata. The execute-tool-call dangerous check (line 189-191) reads from `(gethash :DANGEROUS tool-def)` -- this path must work identically with DB-sourced defs.
**Warning signs:** trade_executor, forex_trader, or other dangerous tools executing without approval blocks.

### Pitfall 4: cli_args and positional_arg Diversity
**What goes wrong:** Tools that use non-standard argument passing fail.
**Why it happens:** Some tools use `positional_arg`, some use `cli_args` arrays, some use both, some use custom `interpreter` paths.
**How to avoid:** Migrate ALL metadata fields faithfully: `script`, `interpreter`, `parameters`, `positional_arg`, `cli_args`, `dangerous`, `command`, `special_handler`. The `build-cli-args` function (tool-socket.lisp line 153-171) reads `positional_arg` from tool-def -- this must be preserved.
**Warning signs:** CLI arguments passed in wrong order, tools getting wrong interpreter.

### Pitfall 5: OpenClaw Scripts May Not Exist
**What goes wrong:** 3 tools reference `/root/.openclaw/skills/spec-writer/scripts/` which is being deprecated per CLAUDE.md.
**Why it happens:** save_spec, create_pipeline, research_topic point to OpenClaw paths.
**How to avoid:** Verify these scripts exist before migration. If not, mark as `status: 'deprecated'` in area_content rather than `active`. The resolver skips non-active tools.
**Warning signs:** Tool invocation fails with "script not found" for OpenClaw tools.

### Pitfall 6: scope=* Universal Tools
**What goes wrong:** read_own_memory and write_own_memory are supposed to be available to ALL agents but get missed in YAML assignments.
**Why it happens:** These tools have `scope: "*"` (wildcard) in the registry.
**How to avoid:** Add `![read_own_memory]` and `![write_own_memory]` to ALL 9 ghost YAMLs.
**Warning signs:** Ghosts unable to read/write their own memories.

## Code Examples

### area_content Tool Definition (SQL Insert)

```sql
-- Example: market_scanner tool definition
INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (
  5,  -- infrastructure-systems
  'tool',
  'market_scanner',
  'Scan markets for momentum signals. Covers stocks, crypto, commodities via free APIs.',
  '{
    "script": "/root/gotcha-workspace/tools/market_scanner/market_pulse.py",
    "interpreter": "/root/gotcha-workspace/.venv/bin/python3",
    "parameters": {"mode": "scan|report"},
    "positional_arg": "mode",
    "dangerous": false
  }'::jsonb,
  'active'
);
```

### Tool Definition Loader (Lisp)

```lisp
;; Cache pattern from pipeline-definitions.lisp
(defvar *tool-definition-cache* (make-hash-table :test #'equal)
  "Cached tool definitions from area_content. Refreshed per tick.")

(defun reload-tool-definitions ()
  "Load all active tool definitions from area_content, rebuild cache."
  (handler-case
      (let* ((sql "SELECT title, body, metadata FROM area_content WHERE content_type = 'tool' AND status = 'active'")
             (rows (db-query sql)))
        (clrhash *tool-definition-cache*)
        (loop for i from 0 below (length rows)
              for row = (aref rows i)
              for name = (gethash :TITLE row)
              for raw-meta = (gethash :METADATA row)
              for meta = (if (stringp raw-meta) (parse-json raw-meta) raw-meta)
              when (and name (hash-table-p meta))
              do (setf (gethash :DESCRIPTION meta) (or (gethash :BODY row) ""))
                 (setf (gethash (normalize-tool-key name) *tool-definition-cache*) meta))
        (format t "[tool-defs] Loaded ~a tool definitions from DB~%"
                (hash-table-count *tool-definition-cache*)))
    (error (e)
      (format t "[tool-defs] Error loading: ~a~%" e))))
```

### Ghost YAML with Full Tool Capabilities (Nova Example)

```yaml
id: nova
ship_assignment: "ISS Pinnacle of Society"
starting_point:
  x: 0
  y: 0
rpg_persona:
  deity_codename: "Hermes"
  ship_role: "Fleet AI - First Among Equals"
  personality_traits:
    - "resourceful"
    - "direct"
    - "bridging"
orbis_access:
  energy_min: 20
  trust_min: 30
responsibilities:
  - "![query_db]"
  - "![pipeline_status]"
  - "![claude_code]"
  - "![ops_health_check]"
  - "![ops_daily_note]"
  - "![ops_nightly_synthesis]"
  - "![ops_weekly_rollup]"
  - "![ops_monthly_rollup]"
  - "![ops_podcast_watcher]"
  - "![read_schedule]"
  - "![read_own_memory]"
  - "![write_own_memory]"
```

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual Lisp REPL + psql verification |
| Config file | None -- SBCL REPL for Lisp verification |
| Quick run command | `sudo -u postgres psql -d master_chronicle -c "SELECT count(*) FROM area_content WHERE content_type = 'tool'"` |
| Full suite command | Start noosphere-ghosts, observe tool execution in tick logs |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| TOOL-01 | All tools have InnateScipt wrappers in YAML | manual | `grep -c "!\[" /opt/project-noosphere-ghosts/config/agents/*.yaml` | N/A |
| TOOL-02 | Resolver invokes Python scripts | smoke | Start ghosts, send tool-invoking message, check logs | N/A |
| TOOL-03 | tool-registry.json deleted | unit | `test ! -f /opt/project-noosphere-ghosts/config/tool-registry.json` | N/A |
| TOOL-04 | Results flow through cognition pipeline | smoke | Check conversations table after tool execution | N/A |

### Sampling Rate
- **Per task commit:** psql count of tool definitions + grep YAML capabilities
- **Per wave merge:** Full tick cycle with tool invocation
- **Phase gate:** tool-registry.json deleted, all 75 tools in area_content, all 9 YAMLs updated

### Wave 0 Gaps
None -- no automated test framework for Lisp runtime. Verification is via psql queries and SBCL log inspection.

## Open Questions

1. **pipeline_status tool in Nova's YAML**
   - What we know: Nova's YAML declares `![pipeline_status]` but this tool does NOT exist in tool-registry.json
   - What's unclear: Is this a ghost-generated expression or a planned future tool?
   - Recommendation: Keep it in YAML since it's not harmful, but don't add to DB as a tool definition

2. **ethan_ng's custom expressions**
   - What we know: Ethan has `![fundamentals.feeds]` and `![technicals.oanda_api]` which are NOT in tool-registry.json
   - What's unclear: These may be InnateScipt-style declarations that don't map to actual tools
   - Recommendation: Keep existing expressions, add actual tool `![]` entries alongside

3. **OpenClaw tool paths**
   - What we know: 3 tools (save_spec, create_pipeline, research_topic) point to `/root/.openclaw/` which is being deprecated
   - What's unclear: Whether these scripts still function
   - Recommendation: Mark as `status: 'deprecated'` in area_content if scripts don't exist

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/config/tool-registry.json` -- complete tool inventory (75 entries)
- `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` -- execute-tool-call, load-tool-registry, process-tool-calls (323 lines)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` -- D-11 fallback blocks at lines 479-488 (1104 lines)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` -- process-tool-calls usage at lines 455 and 1007 (1329 lines)
- `/opt/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` -- resolve-search at line 220 (408 lines)
- `/opt/project-noosphere-ghosts/lisp/runtime/pipeline-definitions.lisp` -- area_content loading pattern (282 lines)
- `/opt/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp` -- YAML loading, write-ghost-yaml (258 lines)
- All 9 ghost YAML files in `/opt/project-noosphere-ghosts/config/agents/`
- `master_chronicle` area_content table schema (verified via psql)

### Secondary (MEDIUM confidence)
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` -- export declarations for tool-socket functions

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all code read directly, no external dependencies
- Architecture: HIGH -- follows proven Phase 30 area_content pattern exactly
- Pitfalls: HIGH -- identified from direct code reading of all execution paths
- Tool inventory: HIGH -- complete JSON parsed, all 75 tools documented

**Research date:** 2026-03-30
**Valid until:** 2026-04-30 (stable internal codebase, no external API changes)
