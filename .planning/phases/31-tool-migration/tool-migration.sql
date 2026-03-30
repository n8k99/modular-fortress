-- Tool Migration: Insert all tool definitions into area_content
-- Generated from tool-registry.json
-- Phase 31, Plan 01, Task 1

BEGIN;

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'query_db', 'Run a read-only SQL query against master_chronicle. Returns text results.', '{"script": null, "parameters": {"sql": "SQL query string (SELECT only)"}, "dangerous": false, "command": "sudo -u postgres psql -d master_chronicle -t -c", "special_handler": "query_db", "scope": ["engineering", "tools", "specs", "market", "analytics", "reporting", "research", "scheduling", "tracking", "builder", "content", "trading", "kalshi", "forex", "compliance", "creative"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'write_document', 'Write a document (spec, design, review) to the documents table. Content is persisted as a real DB record.', '{"script": null, "parameters": {"path": "Document path (e.g. Projects/Noosphere Ghosts/Tooling/tool_name/spec.md)", "title": "Document title", "content": "Full markdown content"}, "dangerous": false, "special_handler": "write_document", "scope": ["engineering", "tools", "specs", "builder", "content", "writing", "creative", "editorial"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'build_tool', 'Build a Python tool from a specification. Spawns Claude Code to write actual files to ~/gotcha-workspace/tools/<tool_name>/', '{"script": "/opt/project-noosphere-ghosts/tools/tool-builder.sh", "interpreter": "/bin/bash", "parameters": {"tool_name": "Name of the tool (snake_case)", "spec": "Full specification text"}, "dangerous": false, "special_handler": "build_tool", "scope": ["engineering", "tools", "builder"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'claude_code', 'Execute a coding task using Claude Code CLI. Can read/write files, run commands, search code, perform git operations. Returns structured output.', '{"script": "/opt/project-noosphere-ghosts/tools/claude-code-tool.sh", "interpreter": "/bin/bash", "parameters": {"prompt": "The task to execute (describe what to read, write, analyze, or build)", "allowed_tools": "Comma-separated Claude Code tools to allow (default: Read,Grep,Glob,Write,Edit,Bash)"}, "positional_arg": "prompt", "dangerous": false, "scope": ["engineering", "tools"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'list_tasks', 'List open tasks, optionally filtered by assignee or department.', '{"script": "/root/gotcha-workspace/tools/engineering/list_unassigned_tasks.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"assignee": "Optional agent_id filter", "department": "Optional department filter"}, "dangerous": false, "scope": ["engineering", "tools", "scheduling", "tracking", "reporting", "strategy"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'assign_task', 'Assign a task to an agent. Cannot assign to nathan.', '{"script": "/root/gotcha-workspace/tools/engineering/assign_task.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"task_id": "Task ID (integer)", "assignee": "Agent ID to assign to"}, "dangerous": false, "scope": ["engineering", "scheduling", "tracking", "strategy"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'search_documents', 'Search documents using keyword + vector similarity. Returns matching document excerpts.', '{"script": "/root/gotcha-workspace/tools/memory/hybrid_search.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"query": "Search query text (positional, passed directly)"}, "positional_arg": "query", "dangerous": false, "scope": ["engineering", "tools", "content", "research", "market", "analytics", "writing", "music", "specs", "creative", "strategy", "compliance"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'write_tool_spec', 'Write a tool specification document to disk and DB.', '{"script": "/root/gotcha-workspace/tools/engineering/write_tool_spec.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"tool_name": "Tool name (snake_case)", "spec_content": "Full spec markdown"}, "dangerous": false, "scope": ["engineering", "tools", "specs"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'security_review', 'Run automated security checks on a tool''s source code.', '{"script": "/root/gotcha-workspace/tools/engineering/security_review.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"tool_name": "Tool to review"}, "dangerous": false, "scope": ["engineering", "tools", "specs", "compliance"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'run_tool_tests', 'Run test suite for a built tool.', '{"script": "/root/gotcha-workspace/tools/engineering/run_tool_tests.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"tool_name": "Tool to test"}, "dangerous": false, "scope": ["engineering", "tools"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'deploy_tool', 'Deploy a tool: add to manifest, verify imports, register.', '{"script": "/root/gotcha-workspace/tools/engineering/deploy_tool.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"tool_name": "Tool to deploy"}, "dangerous": false, "scope": ["engineering", "tools"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'lookup_agents', 'Look up agent details: role, department, energy, tool_scope.', '{"script": "/root/gotcha-workspace/tools/engineering/lookup_agents.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"agent_id": "Optional specific agent to look up"}, "dangerous": false, "scope": ["engineering", "scheduling", "tracking", "reporting", "strategy"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'read_schedule', 'Read upcoming events from the events table.', '{"script": "/root/gotcha-workspace/tools/engineering/read_schedule.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"days": "Number of days ahead to check (default 7)"}, "dangerous": false, "scope": ["scheduling", "tracking", "reporting", "strategy"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'read_own_memory', 'Read your own memory entries from daily notes. Returns your memories from today and yesterday.', '{"script": "/root/gotcha-workspace/tools/engineering/read_own_memory.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"agent_id": "Your agent ID"}, "dangerous": false, "scope": "*"}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'write_own_memory', 'Save a memory/insight/decision to your daily note column. Your future self will see this tomorrow.', '{"script": "/root/gotcha-workspace/tools/engineering/write_own_memory.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"agent_id": "Your agent ID", "memory_text": "What to remember (max 500 chars)"}, "dangerous": false, "scope": "*"}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'delegate_investment', 'Create an investment pipeline goal with 6 stage tasks (thesis→research→analysis→compliance→documentation→approval). Assigns to the investment team.', '{"script": "/root/gotcha-workspace/tools/investment/delegate_investment.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"description": "What to investigate/trade", "tool_name": "Optional slug override"}, "dangerous": false, "scope": ["strategy", "kalshi", "trading", "market", "portfolio", "forex"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'delegate_editorial', 'Create an editorial pipeline goal with 7 stage tasks (collection→research→curation→composition→editing→polish→publish). Assigns to Sylvia''s editorial team.', '{"script": "/root/gotcha-workspace/tools/editorial/delegate_editorial.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"description": "Editorial task description", "slug": "Optional slug override"}, "dangerous": false, "scope": ["content", "brand", "social", "publishing", "editorial"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'delegate_modular_fortress', 'Create a Modular Fortress diamond pipeline (discovery→security track + modularity track). Carmen Delgado''s cross-department delegation.', '{"script": "/root/gotcha-workspace/tools/modular_fortress/delegate_modular_fortress.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"description": "What to analyze/refactor", "slug": "Optional slug override"}, "dangerous": false, "scope": ["cross_functional", "engineering", "legal", "security", "compliance"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'trade_executor', 'Execute trades on Kalshi. Supports paper and live mode. Actions: scan, trade, close, status, journal.', '{"script": "/root/gotcha-workspace/tools/trade_executor/trade_executor.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "scan|trade|close|status|journal"}, "positional_arg": "action", "dangerous": true, "scope": ["trading", "kalshi", "market", "portfolio"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'thesis_writer', 'Generate investment thesis for a market or contract. Researches and produces structured thesis document.', '{"script": "/root/gotcha-workspace/tools/thesis_writer/thesis_writer.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"topic": "Market topic or contract to analyze"}, "positional_arg": "topic", "dangerous": false, "scope": ["strategy", "research", "market", "trading"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'market_scanner', 'Scan markets for momentum signals. Covers stocks, crypto, commodities via free APIs.', '{"script": "/root/gotcha-workspace/tools/market_scanner/market_pulse.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"mode": "scan|report"}, "positional_arg": "mode", "dangerous": false, "scope": ["market", "trading", "scanning", "signals"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'market_data', 'Fetch market data from Yahoo Finance, CoinGecko, and other free APIs. Returns price, volume, momentum indicators.', '{"script": "/root/gotcha-workspace/tools/market_data/market_data.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"symbol": "Ticker or asset symbol", "source": "yahoo|coingecko"}, "dangerous": false, "scope": ["market", "trading", "research", "scanning"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'news_aggregator', 'Aggregate news from RSS feeds and APIs. Classify by topic and sentiment using Ollama.', '{"script": "/root/gotcha-workspace/tools/news_aggregator/news_aggregator.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "fetch|classify|report"}, "positional_arg": "action", "dangerous": false, "scope": ["research", "market", "signals", "feeds", "content"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'forex_trader', 'FOREX paper trader using live OANDA prices. Scans signals, executes paper trades, reports P&L.', '{"script": "/root/gotcha-workspace/tools/forex_trader/forex_trader.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "scan-and-trade|report|close-all|history", "dry_run": "Preview only (default false)", "limit": "Max trades per run"}, "dangerous": true, "scope": ["trading", "forex", "execution"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'forex_sentiment', 'Fetch financial/geopolitical news, score for risk signals, produce trading bias gate (risk-on/risk-off/blackout).', '{"script": "/root/gotcha-workspace/tools/forex_sentiment/forex_sentiment.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "fetch|check|report|status", "hours": "Lookback window in hours"}, "dangerous": false, "scope": ["trading", "forex", "sentiment", "research"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'forex_fitness', 'Track FOREX trading performance: win rate, pips, drawdown, Sharpe ratio, fitness score (0-100).', '{"script": "/root/gotcha-workspace/tools/forex_fitness/forex_fitness.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "score|report|level|history", "days": "Lookback days (default 30)"}, "dangerous": false, "scope": ["trading", "forex", "analytics", "reporting"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'kalshi_integration', 'Centralized Kalshi API client. Markets, orderbook, balance, positions, orders, fills. RSA auth, rate limiting, DB caching.', '{"script": "/root/gotcha-workspace/tools/kalshi_integration/kalshi_integration.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "markets|market|orderbook|balance|positions|orders|fills|sync|cached|history", "ticker": "Specific market ticker", "keyword": "Search keyword", "series": "Series ticker", "status": "Market status filter", "limit": "Max results"}, "positional_arg": "action", "dangerous": true, "scope": ["kalshi", "trading", "market", "portfolio"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'kalshi_market_integration', 'Kalshi watchlist management, price movement detection, market event tracking, report export.', '{"script": "/root/gotcha-workspace/tools/kalshi_market_integration/kalshi_market_integration.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "watchlist-create|watchlist-add|watchlist-remove|watchlist-list|watchlist-show|watchlist-sync|movements|events|event-add|report", "name": "Watchlist name", "ticker": "Market ticker", "threshold": "Price movement threshold", "hours": "Lookback hours"}, "positional_arg": "action", "dangerous": true, "scope": ["kalshi", "trading", "market", "watchlist"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'kalshi_thesis_filter', 'Filter Kalshi markets by thesis, tag trades/positions, thesis-grouped P&L reporting.', '{"script": "/root/gotcha-workspace/tools/kalshi_thesis_filter/kalshi_thesis_filter.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "filter|markets|trades|summary|positions|tag-trade|tag-position", "keywords": "Comma-separated keywords", "category": "Market category", "thesis": "Thesis name to tag", "status": "Market status filter"}, "positional_arg": "command", "dangerous": true, "scope": ["kalshi", "trading", "portfolio", "research"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'position_tracker', 'Track trading positions with P&L across stocks, crypto, prediction markets. Open/close/add/update with audit trail.', '{"script": "/root/gotcha-workspace/tools/position_tracker/position_tracker.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "open|add|close|update|list|get|history|summary|init", "symbol": "Ticker symbol", "side": "long|short", "qty": "Quantity", "price": "Price per unit", "type": "stock|crypto|prediction|forex", "thesis": "Investment thesis reference"}, "positional_arg": "command", "dangerous": true, "scope": ["trading", "portfolio", "market", "kalshi", "forex"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'risk_calculator', 'Position sizing (Kelly criterion), expected value, portfolio heat, VaR, performance stats, full risk dashboard.', '{"script": "/root/gotcha-workspace/tools/risk_calculator/risk_calculator.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "position-size|ev|portfolio-heat|var|performance|report", "prob": "Win probability (0-1)", "price": "Entry price", "bankroll": "Total capital", "kelly_fraction": "Kelly fraction (default 0.25)", "confidence": "VaR confidence level", "lookback_days": "Days for historical VaR"}, "dangerous": false, "scope": ["trading", "risk", "portfolio", "compliance", "analytics"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'exposure_audit', 'Portfolio risk exposure analysis: total notional, concentration risk, paper-vs-live separation, risk flags.', '{"script": "/root/gotcha-workspace/tools/exposure_audit/exposure_audit.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "report|flags|positions|paper-check|summary", "concentration_threshold": "Max % per position (default 0.25)", "json": "Output as JSON"}, "dangerous": false, "scope": ["compliance", "risk", "portfolio", "trading", "audit"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'regulatory_check', 'Compliance checks: paper-first enforcement, thesis coverage, concentration limits, price staleness, pending orders, audit trail.', '{"script": "/root/gotcha-workspace/tools/regulatory_check/regulatory_check.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"check": "paper_first|thesis_coverage|concentration_risk|price_staleness|pending_orders|audit_trail", "format": "text|json", "concentration_threshold": "Max % per position", "stale_hours": "Max hours before price is stale", "fail_on_critical": "Exit 1 on critical failures"}, "dangerous": false, "scope": ["compliance", "risk", "trading", "audit", "legal"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'probability_model', 'Estimate win probability for trades using Bayesian scoring (historical win rate, momentum, sentiment, P&L trend).', '{"script": "/root/gotcha-workspace/tools/probability_model/probability_model.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "score|report|history", "market_id": "Market ID to score", "symbol": "Symbol to score", "side": "long|short", "no_save": "Don''t persist score"}, "positional_arg": "command", "dangerous": false, "scope": ["trading", "analytics", "market", "research"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'trade_journal', 'Trade review, annotation, daily reflection. Track lessons, mood, and trading decisions.', '{"script": "/root/gotcha-workspace/tools/trade_journal/trade_journal.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "add-entry|list-entries|get-entry|daily|stats|report", "title": "Entry title", "body": "Entry body text", "mood": "Trading mood (1-5)", "date": "Date (YYYY-MM-DD)", "days": "Lookback days", "tag": "Filter by tag", "limit": "Max results"}, "dangerous": false, "scope": ["trading", "journaling", "analytics", "reporting"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'trading_briefing', 'Execute a trading session briefing. Runs FOREX scan, Kalshi check, calendar events, news sentiment, and paper trade execution. Produces structured market summary.', '{"script": "/root/gotcha-workspace/tools/trading_briefing/trading_briefing.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"session": "Trading session to run: tokyo, london, nyc, or now"}, "cli_args": ["--session", "{session}"], "dangerous": false, "scope": ["trading"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'market_data_ingest', 'Fetch OHLCV data from CoinGecko (crypto), Yahoo Finance (stocks), Kalshi (prediction markets) into DB.', '{"script": "/root/gotcha-workspace/tools/market_data_ingest/market_data_ingest.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"type": "crypto|stocks|kalshi|all", "symbols": "Comma-separated symbols", "tickers": "Kalshi tickers", "days": "Lookback days", "query": "Kalshi search query", "limit": "Max results"}, "dangerous": false, "scope": ["market", "trading", "data", "research"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'market_data_processor', 'Aggregate, query, and cross-reference market signals with positions. Time-bucket analysis, correlation.', '{"script": "/root/gotcha-workspace/tools/market_data_processor/market_data_processor.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "latest|history|overview|aggregate|correlate|summary", "symbol": "Ticker symbol", "asset_type": "stock|crypto|prediction", "hours": "Lookback hours", "bucket": "Time bucket size"}, "dangerous": false, "scope": ["market", "analytics", "trading", "signals"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'market_monitoring', 'Watchlist with price targets and stop-losses. Check thresholds, generate and manage alerts.', '{"script": "/root/gotcha-workspace/tools/market_monitoring/market_monitoring.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "watchlist|add|remove|check|alerts|clear-alerts", "symbol": "Ticker symbol", "asset_type": "stock|crypto|prediction|forex", "target": "Target price", "stop": "Stop-loss price", "notes": "Position notes"}, "dangerous": true, "scope": ["market", "trading", "watchlist", "alerts", "forex"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'market_dashboard', 'Unified read-only view: positions, signals, sentiment, trade performance. Aggregates from multiple tables.', '{"script": "/root/gotcha-workspace/tools/market_dashboard/market_dashboard.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"section": "portfolio|positions|signals|trades|sentiment|probs", "mode": "paper|live", "limit": "Max results", "format": "text|json"}, "dangerous": false, "scope": ["market", "trading", "reporting", "analytics", "portfolio"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'market_pulse_monitor', 'Read market signals, watchlist, alerts, Kalshi snapshots from DB. Pulse overview of market state.', '{"script": "/root/gotcha-workspace/tools/market_pulse_monitor/market_pulse_monitor.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "pulse|signals|watchlist|alerts|kalshi|ohlcv|json", "asset_type": "Filter by asset type", "signal": "Filter by signal type", "status": "Filter by status", "limit": "Max results"}, "positional_arg": "command", "dangerous": false, "scope": ["market", "trading", "monitoring", "signals", "kalshi"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'wave_calendar_sync', 'Sync ForexFactory economic calendar events into the database. Classifies events by impact (flat/overhead/nazare) and maps to currency pairs.', '{"script": "/root/gotcha-workspace/tools/wave_calendar/wave_calendar.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "Calendar action: sync (default), today, week, upcoming, blackout"}, "cli_args": ["--action", "{action}"], "dangerous": false, "scope": ["trading"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'article_fetcher', 'Fetch and store article content from URLs with metadata extraction.', '{"script": "/root/gotcha-workspace/tools/article_fetcher/article_fetcher.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "fetch|list|get|search|delete", "url": "Article URL to fetch", "tags": "Tags for the article", "id": "Article ID", "query": "Search query", "limit": "Max results"}, "positional_arg": "command", "dangerous": false, "scope": ["content", "research", "feeds"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'article_parser', 'Extract structured metadata from articles: keywords, entities, reading time, language detection.', '{"script": "/root/gotcha-workspace/tools/article_parser/article_parser.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "parse|get|list|search|delete", "id": "Article ID to parse", "all": "Parse all unparsed articles", "force": "Re-parse existing"}, "positional_arg": "command", "dangerous": false, "scope": ["content", "research", "analytics"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'article_summarizer', 'Generate summaries of articles in multiple styles (brief, detailed, bullets, tldr).', '{"script": "/root/gotcha-workspace/tools/article_summarizer/article_summarizer.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "summarize|get|list|search|delete", "id": "Article ID to summarize", "style": "brief|detailed|bullets|tldr", "all": "Summarize all unsummarized", "force": "Re-summarize existing"}, "positional_arg": "command", "dangerous": false, "scope": ["content", "research", "writing"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'comment_collector', 'Store, retrieve, and search comments from any source (YouTube, Reddit, Twitter/X, blogs).', '{"script": "/root/gotcha-workspace/tools/comment_collector/comment_collector.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "add|list|get|search|delete|stats", "source_type": "youtube|reddit|twitter|blog", "source_id": "Source identifier", "content": "Comment text", "author": "Author name", "query": "Search query"}, "positional_arg": "command", "dangerous": false, "scope": ["content", "research", "social"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'rss_feed_ingester', 'Fetch and persist RSS/Atom feed entries, manage feed registry, deduplicate by GUID.', '{"script": "/root/gotcha-workspace/tools/rss_feed_ingester/rss_feed_ingester.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "add|fetch|list-feeds|list-entries|remove|mark-read|init-schema", "url": "Feed URL", "name": "Feed name", "feed_id": "Feed ID", "tags": "Feed tags", "all": "Fetch all feeds", "unread": "Show only unread"}, "positional_arg": "command", "dangerous": false, "scope": ["content", "research", "feeds", "market"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'sentiment_analyzer', 'VADER-based sentiment scoring for text. Analyze from DB sources or direct text input.', '{"script": "/root/gotcha-workspace/tools/sentiment_analyzer/sentiment_analyzer.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "analyze-text|analyze-source|stats|list|get", "text": "Text to analyze", "source": "DB table to analyze (articles, discord_messages, etc.)", "label": "Filter by pos|neg|neutral", "limit": "Max results"}, "positional_arg": "command", "dangerous": false, "scope": ["analytics", "sentiment", "research", "content"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'thought_police_publisher', 'Stage 7 (publish) of editorial pipeline. Manages publish tasks, marks documents as published.', '{"script": "/root/gotcha-workspace/tools/thought_police_publisher/publish.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "list|context|preview|publish|unpublish|note|status", "task_id": "Pipeline task ID", "slug": "URL slug", "author": "Author name", "message": "Note or unpublish reason"}, "dangerous": true, "scope": ["editorial", "content", "publishing"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'morning_pages_parser', 'Parse morning pages from daily notes, classify chunks, extract entities, route to destinations.', '{"script": "/root/gotcha-workspace/tools/morning_pages_parser/main.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"date": "Date to parse (YYYY-MM-DD)", "dry_run": "Preview only", "json": "JSON output"}, "dangerous": false, "scope": ["content", "memory", "scheduling"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'editorial_composer', 'Stage 4 of editorial pipeline. Surface composition tasks, retrieve research context, accept composed content.', '{"script": "/root/gotcha-workspace/tools/editorial_composer/compose.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "list|context|submit|note|status", "task_id": "Pipeline task ID", "content": "Composed content text", "file": "Content from file path", "stage_notes": "Notes for this stage"}, "dangerous": true, "scope": ["editorial", "content", "writing"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'codebase_scanner', 'Walk directory trees, collect file/language/LOC stats, detect TODO/FIXME markers.', '{"script": "/root/gotcha-workspace/tools/codebase_scanner/scan.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"path": "Directory to scan", "label": "Scan label", "list_scans": "List previous scans", "scan_id": "View specific scan"}, "dangerous": false, "scope": ["engineering", "analytics", "research"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'security_linter', 'Static analysis for security vulnerabilities: hardcoded secrets, SQL injection, command injection, etc.', '{"script": "/root/gotcha-workspace/tools/security_linter/security_linter.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"path": "File or directory to lint", "format": "text|json", "persist": "Save results to DB"}, "dangerous": false, "scope": ["engineering", "security", "compliance", "audit"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'dependency_mapper', 'Build and visualize dependency graphs for tasks, documents, and projects from master_chronicle.', '{"script": "/root/gotcha-workspace/tools/dependency_mapper/dependency_mapper.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"source": "tasks|documents|projects", "project_id": "Filter by project", "detect_cycles": "Check for circular dependencies"}, "dangerous": false, "scope": ["engineering", "analytics", "scheduling"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'pattern_detector', 'Detect code patterns in Python projects: duplicates, coupling, anti-patterns.', '{"script": "/root/gotcha-workspace/tools/pattern_detector/detect.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"path": "Directory to scan", "label": "Scan label", "min_similarity": "Min similarity threshold (0-1)", "format": "text|json"}, "dangerous": false, "scope": ["engineering", "analytics", "research"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'cascade_reporter', 'Trace task/project dependency chains. Report blocked tasks and cascade impacts.', '{"script": "/root/gotcha-workspace/tools/cascade_reporter/cascade_reporter.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "get-blocked|cascade", "task_id": "Task ID for cascade analysis", "project_id": "Filter by project", "goal_id": "Filter by goal", "max_depth": "Max dependency depth"}, "dangerous": false, "scope": ["engineering", "scheduling", "reporting", "analytics"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'event_tracker', 'CRUD calendar events in master_chronicle. Create, list, update, delete, search events.', '{"script": "/root/gotcha-workspace/tools/event_tracker/event_tracker.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "add|list|get|update|delete|search", "title": "Event title", "date": "Event date (YYYY-MM-DD)", "time": "Event time (HH:MM)", "description": "Event description", "duration": "Event duration", "recurrence": "Recurrence rule", "tags": "Comma-separated tags", "id": "Event ID"}, "positional_arg": "command", "dangerous": true, "scope": ["scheduling", "calendar", "tracking"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'recurrence_engine', 'Spawn task instances from recurring templates based on recurrence patterns.', '{"script": "/root/gotcha-workspace/tools/recurrence_engine/recurrence_engine.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"action": "list|check|next", "execute": "Actually create child tasks (default: dry-run preview)", "pattern": "Recurrence pattern to check", "template_id": "Template task ID"}, "dangerous": true, "scope": ["scheduling", "tracking", "automation"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'burg_pipeline', 'Seed→Sapling promotion for fictional world-building burgs. Generates template sections via YAML frontmatter.', '{"script": "/root/gotcha-workspace/tools/burg_pipeline/burg_pipeline.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"burg": "Specific burg name to promote", "state": "Process all burgs in a state", "all": "Process all Seed-level burgs", "limit": "Max burgs to process", "dry_run": "Preview only"}, "dangerous": true, "scope": ["creative", "content", "writing"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'memory_compressor', 'Deduplicate, prune, and rollup memory tables. Compress old logs into summaries.', '{"script": "/root/gotcha-workspace/tools/memory_compressor/memory_compressor.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "stats|dedupe|prune|rollup", "min_importance": "Min importance to keep (for prune)", "older_than": "Days threshold", "dry_run": "Preview only"}, "positional_arg": "command", "dangerous": true, "scope": ["memory", "maintenance", "analytics"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'self_improvement', 'Automated system health monitor. Checks DB, PM2, logs, tools, git status, API gateways.', '{"script": "/root/gotcha-workspace/tools/self_improvement/health_check.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"dry_run": "Report only, no fixes", "json": "JSON output", "fix": "Attempt auto-fixes for issues found"}, "dangerous": true, "scope": ["engineering", "monitoring", "maintenance"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'ops_health_check', 'Operational health monitor for Nova. Checks DB, PM2, logs, tools, git status. Can auto-fix safe issues.', '{"script": "/root/gotcha-workspace/tools/self_improvement/health_check.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"dry_run": "Report only, no fixes", "json": "JSON output", "fix": "Attempt auto-fixes for issues found"}, "dangerous": false, "scope": ["operations", "monitoring", "maintenance"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'ops_daily_note', 'Populate empty sections of today''s daily note with actual data from the database.', '{"script": "/root/gotcha-workspace/tools/temporal-sync/daily_note_populate.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"date": "Target date (YYYY-MM-DD, default today)", "dry_run": "Preview without writing"}, "dangerous": false, "scope": ["operations", "temporal"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'ops_nightly_synthesis', 'Summarize yesterday''s daily log into the weekly note using Ollama for AI summarization.', '{"script": "/root/gotcha-workspace/tools/temporal-sync/nightly_daily_to_weekly.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"date": "Specific date to summarize (YYYY-MM-DD, default yesterday)", "dry_run": "Preview only", "ollama_host": "Ollama endpoint (default http://localhost:11434)", "ollama_model": "Model to use (default llama3.2)"}, "dangerous": false, "scope": ["operations", "temporal"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'ops_weekly_rollup', 'Roll up 7 daily logs into a weekly summary in the database.', '{"script": "/root/gotcha-workspace/tools/temporal-sync/rollup_weekly.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"week": "Specific week (e.g. 2026-W06, default current week)", "dry_run": "Preview only"}, "dangerous": false, "scope": ["operations", "temporal"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'ops_monthly_rollup', 'Roll up weekly summaries into a monthly summary in the database.', '{"script": "/root/gotcha-workspace/tools/temporal-sync/rollup_monthly.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"month": "Specific month (e.g. 2026-02, default current month)", "dry_run": "Preview only"}, "dangerous": false, "scope": ["operations", "temporal"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'ops_podcast_watcher', 'Check podcast RSS feeds for new episodes and post to Discord #announcements.', '{"script": "/root/gotcha-workspace/tools/discord/podcast_watcher.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"podcast": "Specific podcast slug (e.g. myths-of-orbis, living-room-music)", "dry_run": "Print without posting to Discord", "force": "Post latest episode regardless of state"}, "dangerous": false, "scope": ["operations", "content"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'save_spec', 'Save a project or goal spec document to the DB at its canonical path. Use after spec is discussed and approved.', '{"script": "/root/.openclaw/skills/spec-writer/scripts/save_spec.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"project": "Project name", "name": "Spec/goal name", "type": "project|goal", "content": "Full spec markdown content (or pipe via stdin)"}, "dangerous": true, "scope": ["strategy", "engineering", "specs", "decision", "content", "creative"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'create_pipeline', 'Generate pipeline tasks (spec→design→build→test→deploy) from a goal spec. Creates a goal task and stage tasks with dependencies.', '{"script": "/root/.openclaw/skills/spec-writer/scripts/create_pipeline.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"goal": "Goal name (snake_case)", "doc_id": "Goal document ID (from save_spec)", "project_id": "Project ID"}, "dangerous": true, "scope": ["strategy", "engineering", "specs", "decision", "scheduling"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'research_topic', 'Query DB for related documents, existing projects, tasks, and goal docs before making decisions. Use BEFORE asking Nathan questions.', '{"script": "/root/.openclaw/skills/spec-writer/scripts/research.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"topic": "Topic to research (positional)", "limit": "Max results per category (default 10)"}, "positional_arg": "topic", "dangerous": false, "scope": ["strategy", "engineering", "research", "specs", "decision", "content", "market", "trading", "compliance", "creative"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'thematic_matcher', 'Find semantically related content via pgvector similarity or full-text search across documents, notes, tasks.', '{"script": "/root/gotcha-workspace/tools/thematic_matcher/thematic_matcher.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "match|compare|search", "query": "Search text", "id": "Document ID to match from", "source": "Source table", "tables": "Tables to search across"}, "positional_arg": "command", "dangerous": false, "scope": ["research", "analytics", "content", "memory"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'worldbuilding_pipeline', 'Orbis worldbuilding pipeline — promote burgs and provinces from Seed to Sapling. Commands: status, prepare, tasks, next-burg, complete-burg, next-province, complete-province, report. First arg is command, second is state name.', '{"script": "/root/gotcha-workspace/tools/orbis/worldbuilding_pipeline.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"command": "One of: status, prepare, tasks, next-burg, complete-burg, next-province, complete-province, report", "state": "State name (e.g. Sunshield Bastion, Luminarian Empire)", "extra": "Burg or province title (for complete commands)"}, "positional_args": ["command", "state", "extra"], "dangerous": false, "scope": ["content", "writing", "creative", "worldbuilding"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'burg_template', 'Read the Burg Generation Template (doc #8060) — the master template for writing Sapling-level burg entries. Contains constraints, voice guidelines, and section requirements.', '{"script": null, "parameters": {"sql": "SQL query (use: SELECT content FROM documents WHERE id = 8060)"}, "dangerous": false, "command": "sudo -u postgres psql -d master_chronicle -t -c", "scope": ["content", "writing", "creative", "worldbuilding"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'delegate_worldbuilding', 'Delegation tool for Sylvia to manage the worldbuilding pipeline. Actions: prepare (send context compile to Nova), assign-burgs (create agent_requests for burg writers), assign-provinces (same for provinces), status (check progress), kickoff (full start).', '{"script": "/root/gotcha-workspace/tools/orbis/delegate_worldbuilding.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"state": "State name (e.g. Luminarian Empire)", "action": "One of: prepare, assign-burgs, assign-provinces, status, kickoff"}, "cli_args": ["--state", "{state}", "--action", "{action}"], "dangerous": false, "scope": ["content", "writing", "creative", "worldbuilding", "management"]}'::jsonb, 'active');

INSERT INTO area_content (area_id, content_type, title, body, metadata, status)
VALUES (5, 'tool', 'editorial_nightly', 'Run the nightly Thought Police editorial pipeline. Collects Nathan''s reader comments for today (or specified date), fetches source articles, synthesizes editorial via Claude API, saves to documents table, triggers dpn-publish export.', '{"script": "/root/gotcha-workspace/tools/editorial/nightly_editorial.py", "interpreter": "/root/gotcha-workspace/.venv/bin/python3", "parameters": {"date": "Target date YYYY-MM-DD (default: today)"}, "dangerous": false, "scope": ["editorial"]}'::jsonb, 'active');

COMMIT;

-- Verify counts
SELECT 'Total tools: ' || count(*) FROM area_content WHERE content_type = 'tool';
SELECT 'Dangerous: ' || count(*) FROM area_content WHERE content_type = 'tool' AND (metadata->>'dangerous')::boolean = true;
SELECT 'Special handlers: ' || count(*) FROM area_content WHERE content_type = 'tool' AND metadata->>'special_handler' IS NOT NULL;
