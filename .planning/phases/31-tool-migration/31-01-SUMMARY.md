---
phase: 31-tool-migration
plan: 01
subsystem: database, lisp-runtime
tags: [area_content, tool-definitions, lisp, jsonb, per-tick-cache]

requires:
  - phase: 30-team-pipelines
    provides: pipeline-definitions.lisp pattern for DB-sourced definitions

provides:
  - 75 tool definitions in area_content with content_type='tool'
  - tool-definitions.lisp module with reload-tool-definitions and lookup-tool-definition
  - Package wiring for action-executor to consume DB-sourced tool definitions

affects: [31-02, 31-03, 31-04]

tech-stack:
  added: []
  patterns: [DB-sourced tool definitions via area_content, per-tick cache reload for tools]

key-files:
  created:
    - /opt/project-noosphere-ghosts/lisp/runtime/tool-definitions.lisp
    - .planning/phases/31-tool-migration/tool-migration.sql
  modified:
    - /opt/project-noosphere-ghosts/lisp/packages.lisp

key-decisions:
  - "All 75 tools inserted as area_content rows with area_id=5 (Infrastructure/Systems), following pipeline-definitions pattern"
  - "Local normalize-tool-key-local in tool-definitions.lisp avoids circular dependency with tool-socket"
  - "OpenClaw scripts all exist so no tools marked deprecated"

patterns-established:
  - "DB-sourced tool definitions: area_content WHERE content_type='tool' with JSONB metadata"
  - "Tool lookup by normalized keyword with raw string fallback"

requirements-completed: [TOOL-01, TOOL-02]

duration: 13min
completed: 2026-03-30
---

# Phase 31 Plan 01: Tool Definition Migration Summary

**75 tool definitions migrated from tool-registry.json to area_content with per-tick DB loader module following pipeline-definitions.lisp pattern**

## What Was Built

### Task 1: Insert 75 tool definitions into area_content
- Generated and executed SQL migration inserting all 75 tools from tool-registry.json
- Each tool stored with complete JSONB metadata: script, interpreter, parameters, positional_arg, cli_args, dangerous, scope
- 16 dangerous tools flagged, 3 special handlers (query_db, write_document, build_tool) identified with special_handler field
- All OpenClaw scripts verified present, none needed deprecation marking
- Commit: `8e283d3`

### Task 2: Create tool-definitions.lisp DB loader module
- Created `/opt/project-noosphere-ghosts/lisp/runtime/tool-definitions.lisp` with 4 functions
- `load-tool-definitions`: SQL query for active tools, parses JSONB metadata, builds hash-table
- `reload-tool-definitions`: per-tick cache refresh with handler-case error resilience
- `lookup-tool-definition`: normalized keyword lookup with raw string fallback
- `normalize-tool-key-local`: local copy of normalize-tool-key to avoid circular dependency
- Updated packages.lisp with new `af64.runtime.tool-definitions` package definition
- Added import-from in action-executor package for reload-tool-definitions, lookup-tool-definition, *tool-definition-cache*
- Commit: `15fe5dd` (in noosphere-ghosts repo)

## Verification Results

- area_content tool count: 75 (target: 70-79)
- Dangerous tools: 16 (exact match)
- Special handlers: 3 (query_db, write_document, build_tool)
- tool-definitions.lisp defun count: 4
- packages.lisp: tool-definitions package defined and wired to action-executor

## Deviations from Plan

None - plan executed exactly as written.

## Known Stubs

None - all tool definitions contain real script paths and complete metadata.

## Self-Check: PASSED
