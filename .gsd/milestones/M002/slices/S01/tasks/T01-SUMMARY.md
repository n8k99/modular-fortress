---
id: T01
parent: S01
milestone: M002
key_files:
  - dragonpunk/internal/tables/tables.go
  - dragonpunk/internal/db/query.go
key_decisions:
  - Per-table list column selection rather than universal core columns
  - map[string]any for generic row scanning — no struct-per-table
duration: 
verification_result: passed
completed_at: 2026-04-04T23:02:07.096Z
blocker_discovered: false
---

# T01: Table whitelist + generic row scanning with per-table column selection

**Table whitelist + generic row scanning with per-table column selection**

## What Happened

Created tables package with hardcoded whitelist of 14 Nine Table names. Created generic List and GetByID functions using pgx dynamic queries with map[string]any scanning. Per-table list columns handle schema differences (temporal has no status, infrastructure tables have different shapes). NULL values omitted from JSON.

## Verification

go build compiles; all 14 tables return data via list endpoint

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `go build ./cmd/dragonpunk/` | 0 | ✅ pass | 2000ms |

## Deviations

Added per-table column selection (temporal, identity, infrastructure tables) — not in original plan but necessary because tables have different schemas.

## Known Issues

None.

## Files Created/Modified

- `dragonpunk/internal/tables/tables.go`
- `dragonpunk/internal/db/query.go`
