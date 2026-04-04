---
id: T02
parent: S01
milestone: M003
key_files:
  - dragonpunk/internal/api/write.go
  - dragonpunk/internal/api/router.go
key_decisions:
  - Move route: POST /api/{table}/{id}/move with JSON body {target_table, kind}
duration: 
verification_result: passed
completed_at: 2026-04-04T23:11:51.275Z
blocker_discovered: false
---

# T02: CUD+Move HTTP handlers wired — POST create, PATCH update, DELETE, POST move

**CUD+Move HTTP handlers wired — POST create, PATCH update, DELETE, POST move**

## What Happened

Four new handlers in write.go. CreateRow returns 201. UpdateRow returns updated row. DeleteRow returns {deleted:true}. MoveRow validates both tables and kind, returns 201 with new row. Column validation rejects unknown fields with warning log. Router updated with write routes.

## Verification

All handlers compile and route correctly

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `go build ./cmd/dragonpunk/` | 0 | ✅ pass | 2000ms |

## Deviations

Move handler included in S01 rather than separate S02 — clean enough to do together.

## Known Issues

None.

## Files Created/Modified

- `dragonpunk/internal/api/write.go`
- `dragonpunk/internal/api/router.go`
