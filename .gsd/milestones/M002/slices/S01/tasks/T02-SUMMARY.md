---
id: T02
parent: S01
milestone: M002
key_files:
  - dragonpunk/internal/api/tables.go
  - dragonpunk/internal/api/router.go
key_decisions:
  - 10s query timeout for list operations
  - Separate COUNT query for total — simple and reliable
duration: 
verification_result: passed
completed_at: 2026-04-04T23:02:19.579Z
blocker_discovered: false
---

# T02: List endpoint with pagination, kind filter, and text search across all 14 tables

**List endpoint with pagination, kind filter, and text search across all 14 tables**

## What Happened

GET /api/{table} returns paginated JSON with total count. Query params: limit (default 50, max 200), offset, kind (exact match), q (ILIKE on title+body). All 14 tables return correct data. the_links (370K rows) paginates in 25ms.

## Verification

All 14 tables return paginated JSON; kind filter narrows the_work tasks to 2554; text search finds 670 orbis-related realms

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `curl /api/the_work?kind=task&limit=3` | 0 | ✅ pass — total=2554, kinds={task} | 100ms |
| 2 | `curl /api/the_links?limit=10&offset=100` | 0 | ✅ pass — 370K rows, 25ms | 25ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `dragonpunk/internal/api/tables.go`
- `dragonpunk/internal/api/router.go`
