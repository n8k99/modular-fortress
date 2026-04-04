---
id: T03
parent: S01
milestone: M002
key_files:
  - dragonpunk/internal/api/tables.go
key_decisions:
  - (none)
duration: 
verification_result: passed
completed_at: 2026-04-04T23:02:27.865Z
blocker_discovered: false
---

# T03: Get-by-ID endpoint returning full row as JSON with 404 handling

**Get-by-ID endpoint returning full row as JSON with 404 handling**

## What Happened

GET /api/{table}/{id} returns full row via SELECT * with map[string]any scanning. 404 for missing rows. Bad ID returns 400.

## Verification

the_work/1 returns full task row; the_work/999999 returns 404 JSON

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `curl /api/the_work/1` | 0 | ✅ pass — kind=task, title present | 100ms |
| 2 | `curl /api/the_work/999999` | 0 | ✅ pass — 404 JSON | 50ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `dragonpunk/internal/api/tables.go`
