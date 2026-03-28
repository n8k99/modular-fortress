---
phase: 17-projects-goals-restructuring
plan: 01
subsystem: database
tags: [postgres, migration, lifestage, fk, trigger, dpn-core, rust]

# Dependency graph
requires:
  - phase: 16-foundation-tables-api
    provides: areas table with 5 seeded domains for FK references
provides:
  - lifestage column on projects (Seed/Sapling/Tree/Harvest) with forward-only trigger
  - area_id FK on projects linking to areas table with backfill
  - project_id FK on goals linking to projects table with wikilink migration
  - updated dpn-core Project struct with lifestage and area_id fields
affects: [17-02, perception-endpoint, dpn-api-handlers]

# Tech tracking
tech-stack:
  added: []
  patterns: [forward-only lifecycle trigger, wikilink-to-FK text migration]

key-files:
  created:
    - /root/migrations/17-projects-goals-restructuring.sql
  modified:
    - /root/dpn-core/src/db/projects.rs
    - /opt/dpn-core/src/db/projects.rs

key-decisions:
  - "Used sudo -u postgres for migration execution (projects table owned by postgres, not chronicle user)"
  - "Forward-only trigger allows stage skipping (e.g., Seed->Tree) per D-03 wording"

patterns-established:
  - "Lifecycle state machine via DB trigger with ordinal rank comparison"
  - "Wikilink text-to-FK migration using exact string matching (no regex needed)"

requirements-completed: [SCHEMA-05, SCHEMA-06, SCHEMA-07]

# Metrics
duration: 4min
completed: 2026-03-28
---

# Phase 17 Plan 01: Projects & Goals Schema Migration Summary

**Lifestage lifecycle column on projects with forward-only trigger, area_id FK linking projects to areas, and project_id FK migrating goals from wikilink text to integer references**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-28T21:07:12Z
- **Completed:** 2026-03-28T21:11:11Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- All 15 projects have lifestage NOT NULL with correct values (1 Harvest, 1 Seed, 11 Tree, 2 Sapling)
- Forward-only trigger prevents backward lifestage transitions (verified: Harvest->Seed raises exception)
- All 32 DragonPunk goals migrated from wikilink text to project_id = 1 via integer FK
- All 15 projects assigned to correct area_id (Infrastructure=5, EM Corp=1, Orbis=2, Living Room Music=3, N8K99/Personal=4)
- dpn-core Project struct updated with lifestage (String) and area_id (Option<i32>), all queries updated, tests pass

## Task Commits

Each task was committed atomically:

1. **Task 1: SQL migration -- lifestage, area_id, project_id with backfill and trigger** - `cb0266a` (feat)
2. **Task 2: Update dpn-core Project struct and CRUD functions** - `e17d224` in dpn-core (feat)

## Files Created/Modified
- `migrations/17-projects-goals-restructuring.sql` - Complete schema migration with DDL, backfill, trigger, verification
- `dpn-core/src/db/projects.rs` - Project struct + all CRUD functions updated with lifestage and area_id
- `/opt/dpn-core/src/db/projects.rs` - Synced copy for dpn-api builds

## Decisions Made
- Migration executed as postgres user since projects table is owned by postgres (not chronicle)
- Forward-only trigger allows non-sequential forward transitions (Seed->Tree valid) per D-03 wording
- DATABASE_URL env var required for dpn-core build due to sqlx compile-time query checking

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Used postgres user for migration execution**
- **Found during:** Task 1
- **Issue:** chronicle user lacks ALTER TABLE permission on projects (owned by postgres)
- **Fix:** Executed migration via `sudo -u postgres psql` instead of chronicle user
- **Files modified:** None (runtime fix only)
- **Verification:** Migration completed successfully, all verification queries pass
- **Committed in:** cb0266a

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary for migration execution. No scope creep.

## Issues Encountered
- Pre-existing dpn-core build requires DATABASE_URL for sqlx compile-time checking (not a new issue)
- 12 pre-existing test failures in dpn-core unrelated to project changes (notify, db connectivity tests)

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Schema foundation complete for Plan 02 (dpn-api handler updates + perception endpoint enrichment)
- dpn-api build will fail until Plan 02 updates handler call sites for new create_project/update_project signatures
- Both dpn-core copies synchronized and ready

## Known Stubs
None - all data fully wired, no placeholders.

---
*Phase: 17-projects-goals-restructuring*
*Completed: 2026-03-28*
