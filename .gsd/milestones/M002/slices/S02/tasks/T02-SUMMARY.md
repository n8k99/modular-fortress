---
id: T02
parent: S02
milestone: M002
key_files:
  - dragonpunk/internal/db/query.go
  - dragonpunk/internal/api/tables.go
key_decisions:
  - Infrastructure tables return 404 for /kinds rather than empty array — clearer semantics
duration: 
verification_result: passed
completed_at: 2026-04-04T23:05:45.613Z
blocker_discovered: false
---

# T02: Kinds endpoint — GET /api/{table}/kinds returns distinct kind values with counts

**Kinds endpoint — GET /api/{table}/kinds returns distinct kind values with counts**

## What Happened

Added ListKinds to db/query.go with GROUP BY kind ORDER BY count DESC. Infrastructure tables (no kind column) return 404 with explanation. Verified: the_work shows 9 kinds (task:2554, extracted_task:1141...), the_realms shows 43 kinds (location:2410, item:1102, dungeon:1043...).

## Verification

curl /api/the_work/kinds returns 9 kinds; curl /api/the_links/kinds returns 404 with explanation

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `curl /api/the_work/kinds` | 0 | ✅ pass — 9 kinds, task:2554 | 50ms |
| 2 | `curl /api/the_links/kinds` | 0 | ✅ pass — 404 no kind column | 50ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `dragonpunk/internal/db/query.go`
- `dragonpunk/internal/api/tables.go`
