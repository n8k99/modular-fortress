---
phase: 18-memories-rename
plan: 03
subsystem: api
tags: [rust, axum, sqlx, dpn-api, dpn-core, memories, rename]

requires:
  - phase: 18-02
    provides: "dpn-core memories module (Memory/MemoryLight structs, renamed from vault_notes)"
provides:
  - "Live dpn-api serving all requests against memories table"
  - "Ghost perception reading from memories table"
  - "Synced /opt/dpn-core with memories module"
affects: [noosphere-ghosts, dpn-kb, em-site]

tech-stack:
  added: []
  patterns: ["memories module path replaces vault_notes throughout dpn-api"]

key-files:
  created: []
  modified:
    - /opt/dpn-api/src/handlers/documents.rs
    - /opt/dpn-api/src/handlers/af64_perception.rs
    - /opt/dpn-api/src/handlers/timeline.rs
    - /opt/dpn-core/src/db/memories.rs
    - /opt/dpn-core/src/db/mod.rs
    - /opt/dpn-core/src/lib.rs

key-decisions:
  - "DATABASE_URL required at build time for sqlx compile-time query checking against memories table"
  - "Memory struct query includes compression_tier and compressed_from columns for weekly note lookup"

patterns-established:
  - "dpn_core::db::memories:: is the canonical import path for memory CRUD"
  - "Raw SQL uses FROM memories (not vault_notes) for all new queries"

requirements-completed: [MEM-05]

duration: 6min
completed: 2026-03-28
---

# Phase 18 Plan 03: API Handlers & Deploy Summary

**dpn-api handlers migrated from vault_notes to memories, release binary built, PM2 restarted and serving live traffic**

## Performance

- **Duration:** 6 min
- **Started:** 2026-03-28T22:43:28Z
- **Completed:** 2026-03-28T22:49:28Z
- **Tasks:** 1
- **Files modified:** 6

## Accomplishments
- Replaced all vault_notes/VaultNote references in documents.rs (imports, types, raw SQL, comments, variable names)
- Updated af64_perception.rs raw SQL from vault_notes to memories for ghost memory context
- Updated timeline.rs comment reference
- Deleted stale documents.rs.bak file
- Synced /root/dpn-core to /opt/dpn-core (memories.rs present, vault_notes.rs removed)
- Release build succeeded, PM2 restarted, both /api/documents and /api/perception/nova return valid JSON

## Task Commits

Each task was committed atomically:

1. **Task 1: Update dpn-api handlers and sync dpn-core** - `2d9a512` (feat) in dpn-api, `61afc92` (feat) in /opt/dpn-core

**Plan metadata:** [pending] (docs: complete plan)

## Files Created/Modified
- `/opt/dpn-api/src/handlers/documents.rs` - All vault_notes refs replaced with memories (imports, types, raw SQL, variables)
- `/opt/dpn-api/src/handlers/af64_perception.rs` - Raw SQL FROM vault_notes changed to FROM memories
- `/opt/dpn-api/src/handlers/timeline.rs` - Comment updated to reference memories
- `/opt/dpn-api/src/handlers/documents.rs.bak` - Deleted (stale backup)
- `/opt/dpn-core/src/db/memories.rs` - Synced from /root/dpn-core
- `/opt/dpn-core/src/db/mod.rs` - Synced, exports memories module
- `/opt/dpn-core/src/lib.rs` - Synced, re-exports Memory/MemoryLight

## Decisions Made
- DATABASE_URL must be set at build time for sqlx compile-time checking (the .sqlx offline cache was stale after table rename)
- Memory struct query for weekly notes includes compression_tier and compressed_from columns (new columns from Phase 18-01)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added compression columns to weekly note query**
- **Found during:** Task 1 (Update documents.rs)
- **Issue:** The weekly note raw SQL query used `query_as::<_, VaultNote>` which mapped to the old struct. The new Memory struct has compression_tier and compressed_from fields that must be included in the SELECT
- **Fix:** Added `compression_tier, compressed_from` to the weekly note SELECT query
- **Files modified:** /opt/dpn-api/src/handlers/documents.rs
- **Verification:** cargo build --release succeeded
- **Committed in:** 2d9a512

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Essential for build success with new Memory struct. No scope creep.

## Issues Encountered
- API returned 401 on initial curl test without auth header; added X-API-Key header and both endpoints returned valid JSON

## User Setup Required
None - no external service configuration required.

## Known Stubs
None - all data paths are wired to the live memories table.

## Next Phase Readiness
- Phase 18 (memories-rename) is complete: DB table renamed, view bridge active, dpn-core module renamed, dpn-api live
- Ready for Phase 19+ work (ghosts table, Nexus import, temporal compression)
- The vault_notes view bridge remains active for Lisp/Python compatibility

---
*Phase: 18-memories-rename*
*Completed: 2026-03-28*
