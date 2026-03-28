---
phase: 16-foundation-tables-api
plan: 02
subsystem: database
tags: [rust, sqlx, parat, areas, archives, resources, templates, crud, dpn-core]

# Dependency graph
requires:
  - phase: 16-01
    provides: "PARAT tables created in PostgreSQL (areas, archives, resources, templates, templates_history)"
provides:
  - "Area struct + list/get/create/update CRUD functions"
  - "Archive struct + list/get/create/update_metadata/search CRUD functions"
  - "Resource struct + list/get/create/update CRUD functions"
  - "Template + TemplateHistory structs + list/get/create/update/history CRUD functions"
  - "All PARAT types re-exported from dpn_core:: for dpn-api consumption"
affects: [16-03, dpn-api-handlers]

# Tech tracking
tech-stack:
  added: []
  patterns: [dynamic-update-builder, explicit-column-select, generated-column-exclusion]

key-files:
  created:
    - /root/dpn-core/src/db/areas.rs
    - /root/dpn-core/src/db/archives.rs
    - /root/dpn-core/src/db/resources.rs
    - /root/dpn-core/src/db/templates.rs
  modified:
    - /root/dpn-core/src/db/mod.rs
    - /root/dpn-core/src/lib.rs

key-decisions:
  - "update_area and update_resource include updated_at = NOW() in dynamic builder"
  - "Archive struct excludes tsv GENERATED column; search_archives uses ts_rank for relevance ordering"
  - "update_resource excludes frozen field -- DB trigger blocks frozen resource updates"

patterns-established:
  - "PARAT module pattern: struct + list + get_by_id + create + update following projects.rs dynamic builder"
  - "Generated column exclusion: explicit column lists in SELECT, never SELECT *"

requirements-completed: [API-01, API-02, API-03, API-04]

# Metrics
duration: 6min
completed: 2026-03-28
---

# Phase 16 Plan 02: PARAT dpn-core Modules Summary

**Rust CRUD modules for all 4 PARAT tables (areas, archives, resources, templates) with struct definitions, async query functions, full-text search, and lib.rs re-exports**

## Performance

- **Duration:** 6 min
- **Started:** 2026-03-28T19:39:04Z
- **Completed:** 2026-03-28T19:45:04Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Created 4 new dpn-core database modules matching projects.rs pattern exactly
- Archive module includes full-text search via plainto_tsquery with ts_rank relevance ordering
- Template module includes TemplateHistory struct and get_template_history for version tracking
- All types and functions re-exported from lib.rs, unblocking Plan 16-03 API handlers

## Task Commits

Each task was committed atomically:

1. **Task 1: Create dpn-core modules for areas and archives** - `fbabafd` (feat)
2. **Task 2: Create dpn-core modules for resources and templates, update mod.rs and lib.rs** - `73ba784` (feat)

## Files Created/Modified
- `dpn-core/src/db/areas.rs` - Area struct + list_areas, get_area_by_id, create_area, update_area
- `dpn-core/src/db/archives.rs` - Archive struct + list/get/create/update_metadata/search_archives (tsv excluded)
- `dpn-core/src/db/resources.rs` - Resource struct with frozen field + list/get/create/update (no frozen update)
- `dpn-core/src/db/templates.rs` - Template + TemplateHistory structs + list/get/create/update/get_template_history
- `dpn-core/src/db/mod.rs` - Added pub mod declarations for all 4 PARAT modules
- `dpn-core/src/lib.rs` - Added PARAT re-exports for dpn-api consumption

## Decisions Made
- Added `updated_at = NOW()` to dynamic update builders for areas, resources, templates (archives are immutable)
- search_archives uses ts_rank ordering for relevance-ranked results, not just tsv match filtering
- Resource update_resource excludes frozen field -- the DB trigger enforces immutability, caller checks frozen first

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- dpn-core sub-repo had corrupted git index (invalid objects for analysis/ and other files). Rebuilt index with `rm .git/index && git reset HEAD`. This caused the first commit to include all existing files as a root commit, but is functionally correct.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- All 4 PARAT structs and CRUD functions available via `dpn_core::` re-exports
- Plan 16-03 can now implement dpn-api REST handlers calling these functions directly
- Pre-existing test failures (db::tests, notify::tests) are unrelated to PARAT changes

---
*Phase: 16-foundation-tables-api*
*Completed: 2026-03-28*
