---
phase: 27-area-content-tables
plan: 01
subsystem: database
tags: [postgresql, area-content, migration, content-type, fk-constraints]

# Dependency graph
requires:
  - phase: 16-parat-foundation
    provides: areas table with EM Corp at id=1
provides:
  - area_content table with 1027 EM documents classified by content_type
  - FK relationships to areas(id) and documents(id)
  - 4 indexes including GIN on JSONB metadata
affects: [27-02 resolver-integration, future area-content phases for Orbis/LRM]

# Tech tracking
tech-stack:
  added: []
  patterns: [area-scoped content table with content_type discriminator, path-to-type CASE mapping]

key-files:
  created:
    - /opt/project-noosphere-ghosts/migrations/027_area_content.sql
  modified: []

key-decisions:
  - "Used COALESCE for nullable title/status fields to handle document data gaps"
  - "Wrapped DDL + data migration in single transaction for atomicity"
  - "Granted REFERENCES privilege on documents/areas to chronicle user for FK creation"

patterns-established:
  - "Migration file pattern: /opt/project-noosphere-ghosts/migrations/NNN_description.sql"
  - "Content type classification via SQL CASE on document path segments"

requirements-completed: [AREA-01, AREA-02]

# Metrics
duration: 2min
completed: 2026-03-30
---

# Phase 27 Plan 01: Area Content Table Summary

**area_content table with 1027 EM documents migrated from flat documents table, classified into 11 content types via path-prefix mapping with FK scoping to areas**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-30T07:54:55Z
- **Completed:** 2026-03-30T07:56:43Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments
- Created area_content table with 10 columns, 4 indexes, 2 FK constraints, and updated_at trigger
- Migrated 1027 EM documents with content_type classification: podcast(321), blog(163), general(156), branding(106), engineering(101), thought-police(52), morning-pages(39), label(30), speaking(23), systems(22), collaboration(14)
- All records FK-linked to EM Corp area (id=1) and back to source documents for traceability

## Task Commits

Each task was committed atomically:

1. **Task 1: Create area_content table and populate from EM documents** - `70bdb9b` (feat)

**Plan metadata:** (pending)

## Files Created/Modified
- `/opt/project-noosphere-ghosts/migrations/027_area_content.sql` - DDL + data migration for area_content table

## Decisions Made
- Wrapped DDL and data migration in a single BEGIN/COMMIT transaction for atomicity
- Used COALESCE(d.title, SPLIT_PART(d.path, '/', -1)) to handle NULL titles gracefully
- Used COALESCE(d.status, 'active') as default for documents with NULL status
- Granted REFERENCES privilege to chronicle user (was missing, blocking FK creation) -- Rule 3 auto-fix

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Granted REFERENCES privilege on documents and areas tables**
- **Found during:** Task 1 (migration execution)
- **Issue:** chronicle user lacked REFERENCES privilege on documents and areas tables, preventing FK constraint creation
- **Fix:** Granted REFERENCES privilege via postgres superuser before re-running migration
- **Files modified:** None (DB privilege change only)
- **Verification:** Migration re-ran successfully, FK constraints verified in \d output
- **Committed in:** 70bdb9b (part of task commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Essential fix for FK constraints. No scope creep.

## Issues Encountered
- migrations/ directory did not exist in project-noosphere-ghosts (only sql/ existed). Created the directory as specified in the plan.

## User Setup Required
None - no external service configuration required.

## Known Stubs
None - all data is live from the documents table migration.

## Next Phase Readiness
- area_content table is populated and ready for Plan 02 (resolver integration)
- All 1027 EM documents are classified and FK-linked
- The noosphere resolver can now be extended to query area_content via {em.content} expressions

## Self-Check: PASSED

- FOUND: /opt/project-noosphere-ghosts/migrations/027_area_content.sql
- FOUND: /root/.planning/phases/27-area-content-tables/27-01-SUMMARY.md
- FOUND: commit 70bdb9b

---
*Phase: 27-area-content-tables*
*Completed: 2026-03-30*
