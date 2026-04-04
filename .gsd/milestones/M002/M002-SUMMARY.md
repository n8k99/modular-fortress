---
id: M002
title: "Dragonpunk Read API — Browse the Nine Tables"
status: complete
completed_at: 2026-04-04T23:07:04.312Z
key_decisions:
  - Generic map[string]any scanning — no struct-per-table
  - Per-table list column selection for schema differences
  - Table whitelist for safe dynamic SQL
key_files:
  - dragonpunk/internal/tables/tables.go
  - dragonpunk/internal/db/query.go
  - dragonpunk/internal/api/tables.go
  - dragonpunk/internal/api/errors.go
  - dragonpunk/internal/api/router.go
lessons_learned:
  - Polymorphic tables need per-table column selection even for list views — not all share the same core columns
  - Infrastructure tables (links, index, aliases, ledger) have fundamentally different shapes from domain tables
---

# M002: Dragonpunk Read API — Browse the Nine Tables

**All 14 Nine Tables browsable via Dragonpunk — list, get, filter, search, slug lookup, kind enumeration**

## What Happened

Built a generic read API serving all 14 Nine Tables through one handler pattern. Dynamic SQL with hardcoded table whitelist. Per-table column selection handles schema differences. Pagination, kind filter, text search, slug lookup, and kind enumeration all working against 742K rows of live data. 370K-row table paginates in 25ms.

## Success Criteria Results

All criteria met — 7 endpoint patterns serving 14 tables with pagination, filtering, search, slug lookup, and kind enumeration.

## Definition of Done Results

- [x] Generic list endpoint for all 14 tables with pagination\n- [x] Generic get-by-id returning full row as JSON\n- [x] Kind filter and text search\n- [x] Table name whitelist — 404 for invalid tables\n- [x] JSON error responses\n- [x] Slug lookup\n- [x] Kind enumeration with counts

## Requirement Outcomes

R001: advanced — 7 endpoint patterns live\nR009: Read portion delivered, CUD+Move remains for M003

## Deviations

None.

## Follow-ups

None.
