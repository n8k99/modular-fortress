---
phase: 18-memories-rename
plan: 01
subsystem: database
tags: [postgresql, migration, rename, view, triggers, departments]

# Dependency graph
requires:
  - phase: 17-projects-goals
    provides: "projects table with lifestage, migration patterns (postgres user, PM2 rebuild)"
provides:
  - "memories table (renamed from vault_notes) with compression_tier and compressed_from columns"
  - "vault_notes backward-compatible view with INSTEAD OF triggers for INSERT/UPDATE/DELETE"
  - "departments lookup table with 8 canonical entries"
  - "agents.department_id FK backfilled for all 64 agents"
affects: [18-memories-rename, 19-nexus-import, 20-temporal-compression]

# Tech tracking
tech-stack:
  added: []
  patterns: [INSTEAD OF triggers for backward-compatible view bridge, RETURNING INTO for id propagation through view triggers]

key-files:
  created:
    - ".planning/phases/18-memories-rename/18-01-migration.sql"
  modified:
    - "master_chronicle: vault_notes table renamed to memories"
    - "master_chronicle: departments table created"
    - "master_chronicle: agents.department_id column added"

key-decisions:
  - "Fixed INSERT trigger to use RETURNING INTO for id propagation through view"
  - "Terminated stale dpn-api connections holding locks on agents table during migration"
  - "Trigger trg_sync_task_checkbox survived rename without drop/recreate (Pitfall 4 confirmed)"

patterns-established:
  - "INSTEAD OF trigger pattern: use RETURNING INTO + NEW.id assignment for id propagation through views"
  - "Migration lock management: terminate idle-in-transaction sessions blocking DDL"

requirements-completed: [MEM-01, MEM-02, MEM-03, MEM-04]

# Metrics
duration: 5min
completed: 2026-03-28
---

# Phase 18 Plan 01: Memories Rename Summary

**vault_notes table renamed to memories with INSTEAD OF view bridge, compression metadata columns, and departments lookup table with FK backfill**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-28T22:27:16Z
- **Completed:** 2026-03-28T22:32:58Z
- **Tasks:** 2
- **Files modified:** 1 (SQL migration script) + 4 DB objects (memories table, vault_notes view, departments table, agents.department_id)

## Accomplishments
- Renamed vault_notes (2830 rows, 75 columns) to memories table with all indexes and sequence renamed
- Created vault_notes view with full INSTEAD OF triggers supporting INSERT (with RETURNING id), UPDATE (all 75 columns), and DELETE
- Added compression_tier VARCHAR with CHECK constraint, backfilled from note_type (2414 daily, 314 weekly, 68 monthly, 25 quarterly, 9 yearly)
- Added compressed_from INTEGER[] column for temporal compression tracking
- Created departments lookup table with 8 canonical entries, backfilled all 64 agents with department_id FK
- Verified trg_sync_task_checkbox trigger survived table rename (no drop/recreate needed)

## Task Commits

Each task was committed atomically:

1. **Task 1+2: Create and execute SQL migration + verify view bridge** - `4b0ff79` (feat)

**Plan metadata:** (pending)

## Files Created/Modified
- `.planning/phases/18-memories-rename/18-01-migration.sql` - Complete transactional migration script (261 lines)

## Decisions Made
- Fixed INSERT trigger to use `RETURNING id INTO new_id; NEW.id := new_id;` pattern so that `INSERT INTO vault_notes ... RETURNING id` works correctly through the view (original plan's trigger returned NULL for id)
- Had to terminate 6 stale dpn-api connections holding locks on agents table to allow ALTER TABLE to proceed
- Confirmed trigger trg_sync_task_checkbox stays attached through table rename (PostgreSQL preserves trigger by OID, no drop/recreate needed per Pitfall 4)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] INSERT trigger returned NULL id via RETURNING**
- **Found during:** Task 2 (view bridge verification)
- **Issue:** The INSERT trigger excluded id from the column list (correct for nextval), but RETURNING id through the view returned NULL because NEW.id was never set in the trigger function
- **Fix:** Added `RETURNING id INTO new_id` to the INSERT statement and `NEW.id := new_id` before RETURN NEW
- **Files modified:** 18-01-migration.sql (updated), vault_notes_insert_fn() function in DB
- **Verification:** INSERT INTO vault_notes ... RETURNING id now returns valid auto-generated id (tested: 61322)
- **Committed in:** 4b0ff79

**2. [Rule 3 - Blocking] Stale connections blocking ALTER TABLE on agents**
- **Found during:** Task 1 (migration execution)
- **Issue:** ALTER TABLE agents ADD COLUMN was blocked by idle-in-transaction dpn-api connections holding locks on the agents table
- **Fix:** Terminated stale blocking sessions via pg_terminate_backend()
- **Files modified:** None (runtime operation)
- **Verification:** Migration completed successfully after lock clearance
- **Committed in:** 4b0ff79

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** Both fixes necessary for correctness and completion. No scope creep.

## Issues Encountered
- dpn-api connections held locks on agents table during migration. Resolved by terminating stale sessions. dpn-api connection pool automatically reconnects.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- memories table ready for Rust code migration (Plan 18-02: dpn-core module rename)
- vault_notes view confirmed working for Lisp/Python backward compatibility
- departments table ready for API exposure if needed
- dpn-api may need restart after connections were terminated (pool auto-reconnects)

## Known Stubs
None - all data is live, no placeholders.

## Self-Check: PASSED
- 18-01-migration.sql: FOUND
- 18-01-SUMMARY.md: FOUND
- Commit 4b0ff79: FOUND

---
*Phase: 18-memories-rename*
*Completed: 2026-03-28*
