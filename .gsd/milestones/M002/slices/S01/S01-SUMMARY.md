---
id: S01
parent: M002
milestone: M002
provides:
  - Paginated list endpoint for all 14 tables
  - Get-by-ID for full row retrieval
  - Kind filter and text search
requires:
  []
affects:
  - S02
key_files:
  - dragonpunk/internal/tables/tables.go
  - dragonpunk/internal/db/query.go
  - dragonpunk/internal/api/tables.go
  - dragonpunk/internal/api/errors.go
  - dragonpunk/internal/api/router.go
key_decisions:
  - Generic map[string]any scanning — no struct-per-table
  - Per-table list column selection
  - Separate COUNT query for pagination totals
patterns_established:
  - Table whitelist for safe dynamic SQL
  - Generic polymorphic row scanning
  - JSON error response pattern
observability_surfaces:
  - Request logging with table name, query params, row count
drill_down_paths:
  - .gsd/milestones/M002/slices/S01/tasks/T01-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-04T23:06:34.797Z
blocker_discovered: false
---

# S01: Generic List + Get endpoints for all Nine Tables

**All 14 Nine Tables browsable via paginated list, get-by-id, kind filter, and text search**

## What Happened

Built generic read handlers using dynamic SQL with table whitelist. Per-table column selection handles schema differences across domain and infrastructure tables. Pagination with 50/200 default/max limits. Kind filter for exact match, ILIKE search on title+body. 370K-row table paginates in 25ms.

## Verification

All 14 tables return data via curl loop; kind filter narrows correctly; text search finds matches; 404 for invalid tables and missing rows; large table pagination under 25ms

## Requirements Advanced

- R001 — Dragonpunk now serves real data endpoints for all Nine Tables
- R009 — Read portion of CRUD delivered

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Per-table list columns added for temporal, identity, and infrastructure tables — not in original plan but required by schema differences.

## Known Limitations

None.

## Follow-ups

None.

## Files Created/Modified

- `dragonpunk/internal/tables/tables.go` — Nine Tables whitelist
- `dragonpunk/internal/db/query.go` — Generic List, GetByID with dynamic SQL
- `dragonpunk/internal/api/tables.go` — List and GetRow handlers
- `dragonpunk/internal/api/errors.go` — JSON error response helper
- `dragonpunk/internal/api/router.go` — Table routes wired
