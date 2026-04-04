---
id: T01
parent: S02
milestone: M001
key_files:
  - (none)
key_decisions:
  - Drop all 83 legacy tables — Nine Tables are sole schema going forward (D008)
  - nebulab_user granted ALL on all 14 Nine Tables + sequences
duration: 
verification_result: passed
completed_at: 2026-04-04T22:53:32.746Z
blocker_discovered: false
---

# T01: Granted nebulab_user access to all 14 Nine Tables and dropped 83 legacy tables

**Granted nebulab_user access to all 14 Nine Tables and dropped 83 legacy tables**

## What Happened

The predawn migration (commit 838ef9b) had successfully created all 14 Nine Tables with 742K rows. But they were owned by nathaneckenrode, so nebulab_user (Dragonpunk's DB user) couldn't see them. Granted full access to nebulab_user, then dropped all 83 legacy tables and the firehose view. Nathan confirmed visibility in VS Code DB explorer.

## Verification

psql shows 14 tables only; curl localhost:8888/api/health returns table_count:14; VS Code DB explorer shows Nine Tables with data

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `psql: SELECT count(*) FROM information_schema.tables WHERE table_schema='public'` | 0 | ✅ pass — 14 tables | 200ms |
| 2 | `curl -s localhost:8888/api/health` | 0 | ✅ pass — table_count:14 | 100ms |

## Deviations

Original goal was 'schema validation' — actual work was permissions fix + legacy cleanup. Migration itself was already done predawn.

## Known Issues

Some migrated entries may be in wrong tables — captured as R009 (CRUD+Edit+Move requirement).

## Files Created/Modified

None.
