---
phase: 18-memories-rename
plan: 02
subsystem: database
tags: [rust, dpn-core, rename, memories, vault_notes, sqlx, compression]

# Dependency graph
requires:
  - phase: 18-01
    provides: "memories table and vault_notes view in PostgreSQL"
provides:
  - "dpn-core db::memories module with Memory/MemoryLight structs"
  - "All dpn-core consumers updated to use memories module"
  - "compression_tier and compressed_from fields on Memory struct"
affects: [18-03, dpn-api handlers referencing dpn-core types]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Memory struct with compression_tier/compressed_from for temporal compression"]

key-files:
  created:
    - "/root/dpn-core/src/db/memories.rs"
  modified:
    - "/root/dpn-core/src/db/mod.rs"
    - "/root/dpn-core/src/lib.rs"
    - "/root/dpn-core/src/timeline/mod.rs"
    - "/root/dpn-core/src/sync/snapshot.rs"
    - "/root/dpn-core/src/sync/incremental.rs"
    - "/root/dpn-core/src/replay/collector.rs"
    - "/root/dpn-core/src/replay/narrative.rs"
    - "/root/dpn-core/src/replay/templates.rs"
    - "/root/dpn-core/src/pipeline/tasks_due.rs"
    - "/root/dpn-core/src/embeddings/batch.rs"
    - "/root/dpn-core/src/context/injection.rs"
    - "/root/dpn-core/src/context/relevance.rs"
    - "/root/dpn-core/src/cache/sqlite.rs"
    - "/root/dpn-core/src/cache/hybrid.rs"
    - "/root/dpn-core/src/cache/sync_queue.rs"
    - "/root/dpn-core/src/db/tests.rs"
    - "/root/dpn-core/src/db/documents.rs"

key-decisions:
  - "Local SQLite cache defaults compression_tier to 'daily' since cache has no compression columns"
  - "EntrySource::VaultNote renamed to EntrySource::Memory with display string changed from 'note' to 'memory'"
  - "ContextSource::VaultNote renamed to ContextSource::Memory with display string changed from 'vault' to 'memory'"
  - "context/relevance.rs also required updates (not listed in plan but blocking)"

patterns-established:
  - "Memory struct includes compression_tier (String) and compressed_from (Option<Vec<i32>>) for temporal compression"
  - "All SELECT queries for Memory include compression_tier and compressed_from columns"
  - "All SELECT queries for MemoryLight include compression_tier column"

requirements-completed: [MEM-06]

# Metrics
duration: 7min
completed: 2026-03-28
---

# Phase 18 Plan 02: dpn-core vault_notes to memories Rename Summary

**Renamed vault_notes module to memories across 18 dpn-core files with Memory/MemoryLight structs including compression_tier and compressed_from fields, cargo check passes clean**

## Performance

- **Duration:** 7 min
- **Started:** 2026-03-28T22:34:50Z
- **Completed:** 2026-03-28T22:41:55Z
- **Tasks:** 2
- **Files modified:** 18

## Accomplishments
- Renamed vault_notes.rs to memories.rs with VaultNote->Memory, VaultNoteLight->MemoryLight, all SQL strings updated
- Added compression_tier and compressed_from fields to Memory struct, compression_tier to MemoryLight
- Updated all 15 consumer files across timeline, sync, replay, pipeline, embeddings, context, cache, and db modules
- cargo check passes with zero vault_notes references in production code

## Task Commits

Each task was committed atomically:

1. **Task 1: Rename vault_notes.rs to memories.rs and update all internal references** - `5fb5186` (feat)
2. **Task 2: Update all dpn-core consumer files and verify compilation** - `df96f80` (feat)

## Files Created/Modified
- `dpn-core/src/db/memories.rs` - Core module: Memory/MemoryLight structs, 13 query functions, all SQL FROM memories
- `dpn-core/src/db/mod.rs` - pub mod memories declaration
- `dpn-core/src/lib.rs` - Re-exports Memory, MemoryLight
- `dpn-core/src/timeline/mod.rs` - TimelineType::Memory, load_memories(), FROM memories SQL
- `dpn-core/src/sync/snapshot.rs` - snapshot_memories()
- `dpn-core/src/sync/incremental.rs` - sync_memories()
- `dpn-core/src/replay/collector.rs` - EntrySource::Memory, memories:: module calls
- `dpn-core/src/replay/narrative.rs` - EntrySource::Memory in match arms
- `dpn-core/src/replay/templates.rs` - EntrySource::Memory reference
- `dpn-core/src/pipeline/tasks_due.rs` - memories:: module, Memory struct with compression fields
- `dpn-core/src/embeddings/batch.rs` - BackfillTarget::Memories
- `dpn-core/src/context/injection.rs` - ContextSource::Memory, FROM memories SQL
- `dpn-core/src/context/relevance.rs` - source_weights.memory
- `dpn-core/src/cache/sqlite.rs` - SQLite schema: CREATE TABLE memories, CacheStats.memories
- `dpn-core/src/cache/hybrid.rs` - Memory/MemoryLight types, all method renames
- `dpn-core/src/cache/sync_queue.rs` - Test string references
- `dpn-core/src/db/tests.rs` - memories:: module calls
- `dpn-core/src/db/documents.rs` - Comment updates

## Decisions Made
- Local SQLite cache structs default compression_tier to "daily" since the local cache schema does not have compression columns
- EntrySource::VaultNote display string changed from "note" to "memory" for consistency
- ContextSource::VaultNote display string changed from "vault" to "memory"

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] context/relevance.rs also needed vault_notes references updated**
- **Found during:** Task 2 (cargo check)
- **Issue:** context/relevance.rs had ContextSource::VaultNote match arm and source_weights.vault_note field, not listed in plan's 14 consumer files
- **Fix:** Updated all 4 references in relevance.rs (enum match, struct field, two defaults)
- **Files modified:** src/context/relevance.rs
- **Verification:** cargo check passes
- **Committed in:** df96f80 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Essential for compilation. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Known Stubs
None - all data paths are fully wired.

## Next Phase Readiness
- dpn-core compiles cleanly with memories module
- Ready for 18-03 (dpn-api handler updates to use new Memory/MemoryLight types)
- The vault_notes view from 18-01 ensures backward compatibility during transition

---
*Phase: 18-memories-rename*
*Completed: 2026-03-28*
