---
phase: 31-tool-migration
plan: 02
subsystem: ghost-config
tags: [yaml, capabilities, innate, tools]
dependency_graph:
  requires: []
  provides: [ghost-yaml-capabilities]
  affects: [tool-registry-retirement, ghost-cognition]
tech_stack:
  added: []
  patterns: ["![tool_name] InnateScipt capability expressions in YAML"]
key_files:
  created: []
  modified:
    - /opt/project-noosphere-ghosts/config/agents/nova.yaml
    - /opt/project-noosphere-ghosts/config/agents/eliana.yaml
    - /opt/project-noosphere-ghosts/config/agents/kathryn.yaml
    - /opt/project-noosphere-ghosts/config/agents/sylvia.yaml
    - /opt/project-noosphere-ghosts/config/agents/vincent.yaml
    - /opt/project-noosphere-ghosts/config/agents/jmax.yaml
    - /opt/project-noosphere-ghosts/config/agents/lrm.yaml
    - /opt/project-noosphere-ghosts/config/agents/sarah.yaml
    - /opt/project-noosphere-ghosts/config/agents/ethan_ng.yaml
decisions:
  - "Worldbuilding tools assigned to JMax (canon arbiter) per domain routing"
  - "delegate_modular_fortress assigned to Eliana (engineering coordination)"
  - "OpenClaw tools (save_spec, create_pipeline, research_topic) skipped — deprecated"
metrics:
  duration: "1min"
  completed: "2026-03-30"
  tasks_completed: 1
  tasks_total: 1
  files_modified: 9
---

# Phase 31 Plan 02: Ghost YAML Tool Capabilities Summary

Complete InnateScipt tool capability declarations added to all 9 ghost YAML files, mapping departmental tools to executive roles per D-01/D-02/D-11.

## One-liner

All 9 ghost YAMLs now declare full tool capabilities as ![tool_name] expressions, with 97 new tool entries across executives and staff, universal memory tools on every ghost.

## What Was Done

### Task 1: Update all 9 ghost YAML files with complete tool capabilities

Added missing ![tool_name] expressions to each ghost's responsibilities section based on departmental role assignments:

| Ghost | Role | Total Tools | Total Entries | Key Additions |
|-------|------|-------------|---------------|---------------|
| Nova | COO | 16 | 16 | ops_health_check, ops_daily_note, memory_compressor, recurrence_engine |
| Eliana | CTO | 18 | 18 | codebase_scanner, security_linter, dependency_mapper, delegate_modular_fortress |
| Kathryn | CSO | 27 | 28 | trade_executor, kalshi_integration, forex_trader, market_pulse_monitor |
| Sylvia | Content | 16 | 18 | editorial_composer, thought_police_publisher, rss_feed_ingester, sentiment_analyzer |
| Vincent | Creative | 4 | 5 | search_documents (limited domain — visual only) |
| JMax | Legal | 10 | 11 | worldbuilding_pipeline, burg_pipeline, regulatory_check, exposure_audit |
| LRM | Music | 4 | 5 | search_documents (limited domain — music only) |
| Sarah | PA | 7 | 7 | list_tasks, assign_task, lookup_agents, read_schedule |
| Ethan Ng | FOREX | 9 | 10 | forex_trader, forex_sentiment, forex_fitness, news_aggregator |

- **Universal tools:** read_own_memory + write_own_memory added to all 9 ghosts
- **Non-tool expressions preserved:** All {em.*} expressions (7 total across 6 ghosts) retained
- **Non-responsibilities sections preserved:** id, ship_assignment, starting_point, rpg_persona, orbis_access all untouched

**Commit:** 690181a

## Deviations from Plan

None - plan executed exactly as written.

## Verification Results

```
eliana.yaml: 18 tools
ethan_ng.yaml: 9 tools
jmax.yaml: 10 tools
kathryn.yaml: 27 tools
lrm.yaml: 4 tools
nova.yaml: 16 tools
sarah.yaml: 7 tools
sylvia.yaml: 16 tools
vincent.yaml: 4 tools
```

- All 9 YAMLs have read_own_memory and write_own_memory (confirmed: `grep -l 'read_own_memory' | wc -l` = 9)
- All {em.*} non-tool expressions preserved (7 entries across ethan_ng, jmax, kathryn, lrm, sylvia, vincent)
- All non-responsibilities sections intact (ship_assignment, starting_point, rpg_persona, orbis_access in all 9 files)

## Known Stubs

None - all tool declarations reference real or planned InnateScipt tool names from the area_content table.

## Self-Check: PASSED
