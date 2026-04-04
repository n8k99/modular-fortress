---
id: T01
parent: S01
milestone: M003
key_files:
  - dragonpunk/internal/db/query.go
  - dragonpunk/internal/db/columns.go
key_decisions:
  - Column validation via information_schema cached at startup
  - Move uses pgx transaction with SELECT FOR UPDATE locking
  - id and created_at always rejected from write operations
duration: 
verification_result: passed
completed_at: 2026-04-04T23:11:40.778Z
blocker_discovered: false
---

# T01: Generic Create, Update, Delete, Move query functions with column validation and transactional move

**Generic Create, Update, Delete, Move query functions with column validation and transactional move**

## What Happened

Added Create (INSERT RETURNING *), Update (SET + updated_at=now() RETURNING *), Delete (DELETE with affected check), Move (BEGIN → SELECT FOR UPDATE → INSERT target → DELETE source → COMMIT). Column validation via ColumnMap loaded at startup from information_schema. Rejects writes to id/created_at and unknown columns.

## Verification

go build compiles

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `go build ./cmd/dragonpunk/` | 0 | ✅ pass | 2000ms |

## Deviations

Move function included in T01 rather than separate slice — it was straightforward to implement alongside CUD.

## Known Issues

None.

## Files Created/Modified

- `dragonpunk/internal/db/query.go`
- `dragonpunk/internal/db/columns.go`
