---
id: T01
parent: S02
milestone: M002
key_files:
  - dragonpunk/internal/db/query.go
  - dragonpunk/internal/api/tables.go
  - dragonpunk/internal/api/router.go
key_decisions:
  - Explicit /slug/ prefix in route to avoid ambiguity with numeric ID
duration: 
verification_result: passed
completed_at: 2026-04-04T23:05:35.879Z
blocker_discovered: false
---

# T01: Slug lookup endpoint — GET /api/{table}/slug/{slug} returns full row

**Slug lookup endpoint — GET /api/{table}/slug/{slug} returns full row**

## What Happened

Added GetBySlug to db/query.go and slug handler to tables.go. Route uses /slug/ prefix to avoid conflict with numeric ID route. Verified with real slugs: etask-1773 from the_work, Orbis from the_realms. 404 for missing slugs.

## Verification

curl /api/the_realms/slug/Orbis returns kind=world, title=Orbis; missing slug returns 404

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `curl /api/the_realms/slug/Orbis` | 0 | ✅ pass — kind=world, title=Orbis | 50ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `dragonpunk/internal/db/query.go`
- `dragonpunk/internal/api/tables.go`
- `dragonpunk/internal/api/router.go`
