---
phase: 20-nexus-import-temporal-compression
plan: 01
subsystem: database
tags: [postgresql, python, deduplication, archives, chatgpt-import, psycopg2]

# Dependency graph
requires:
  - phase: 16-foundation-tables-api
    provides: archives table with immutability trigger
  - phase: 18-memories-rename
    provides: memories table with compression_tier column
provides:
  - nexus-import Python pipeline package in gotcha-workspace/tools/nexus-import/
  - 990 canonical ChatGPT conversations in archives table with chatgpt_import source_type
  - dedup audit report in archives table with dedup_audit source_type
  - verification script for all 5 IMPORT requirements
affects: [20-02, 20-03, temporal-compression, ghost-memory-injection]

# Tech tracking
tech-stack:
  added: []
  patterns: [parameterized SQL regex patterns to avoid Python escape warnings, idempotent pipeline stages with already-imported checks]

key-files:
  created:
    - gotcha-workspace/tools/nexus-import/__init__.py
    - gotcha-workspace/tools/nexus-import/dedup.py
    - gotcha-workspace/tools/nexus-import/import_archives.py
    - gotcha-workspace/tools/nexus-import/_prompts.py
    - gotcha-workspace/tools/nexus-import/verify.py
  modified:
    - gotcha-workspace/tools/manifest.md

key-decisions:
  - "Used parameterized SQL for all regex patterns to avoid Python 3.12 invalid escape sequence warnings"
  - "990 canonical conversations identified (988 backup-Nebulab + 2 retired-only) -- slightly different from research estimate of 992"
  - "Date range extends to 2025-10-04 (not 2025-06-10 as estimated) due to retired-only entries"

patterns-established:
  - "Pipeline stage pattern: each stage is idempotent, checks already-processed records before inserting"
  - "get_canonical_set() handles both dict and tuple psycopg2 cursors for cross-module reuse"

requirements-completed: [IMPORT-01, IMPORT-02]

# Metrics
duration: 4min
completed: 2026-03-29
---

# Phase 20 Plan 01: Nexus Import Pipeline Summary

**Deduplicated 1984 ChatGPT conversations across two archive paths into 990 canonical entries, imported all into archives table with extracted dates, topics, and trivial flags**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-29T00:47:32Z
- **Completed:** 2026-03-29T00:51:45Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Built nexus-import Python pipeline package with 5 modules (dedup, import, verify, prompts, init)
- Deduplicated 991 backup-Nebulab + 993 retired-Nebulab docs into 990 canonical conversations (990 overlap, 2 retired-only, 0 backup-only after date filter)
- Imported all 990 conversations into archives table with chatgpt_import source_type, period dates (2023-12-21 to 2025-10-04), and metadata
- 168 conversations flagged as trivial (< 2000 chars), 822 non-trivial for future LLM summarization
- Audit report archived as dedup_audit record documenting full mapping

## Task Commits

Each task was committed atomically:

1. **Task 1: Create nexus-import package** - `834d847` (feat)
2. **Task 2: Execute dedup and import pipeline** - `041b8ef` (feat)

## Files Created/Modified
- `gotcha-workspace/tools/nexus-import/__init__.py` - Package init with stage documentation
- `gotcha-workspace/tools/nexus-import/dedup.py` - Dedup analysis with canonical set query and audit report generation
- `gotcha-workspace/tools/nexus-import/import_archives.py` - Archive import with date/title extraction and trivial flagging
- `gotcha-workspace/tools/nexus-import/_prompts.py` - LLM prompt templates for future summarization stages
- `gotcha-workspace/tools/nexus-import/verify.py` - SQL verification for all 5 IMPORT requirements
- `gotcha-workspace/tools/manifest.md` - Added nexus-import pipeline tool entries

## Decisions Made
- Used parameterized SQL queries with regex patterns passed as parameters instead of inline regex in triple-quoted strings, avoiding Python 3.12 escape sequence warnings
- 990 canonical conversations (not ~992 as estimated) -- the backup set had 988 date-matching docs after filtering, plus 2 retired-only entries
- Date range extends to 2025-10-04 from retired-only entries (research estimated 2025-06-10 based on backup-only data)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed get_canonical_set for dict cursor compatibility**
- **Found during:** Task 2 (pipeline execution)
- **Issue:** `get_canonical_set()` iterated RealDictRow keys instead of values when called from import_archives.py with dict_cursor=True, causing `WHERE id = 'id'` errors
- **Fix:** Added isinstance check to handle both dict and tuple cursor row types
- **Files modified:** gotcha-workspace/tools/nexus-import/dedup.py
- **Verification:** Import completed successfully for all 990 conversations
- **Committed in:** 041b8ef (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Essential fix for cross-module cursor compatibility. No scope creep.

## Issues Encountered
None beyond the auto-fixed deviation above.

## User Setup Required
None - no external service configuration required.

## Known Stubs
None - all data is live in the database with real content.

## Next Phase Readiness
- 990 conversations in archives table ready for LLM summarization (Plan 02)
- 822 non-trivial conversations identified for per-conversation summary
- 168 trivial conversations will be skipped for LLM but still archived
- verify.py confirms IMPORT-01 and IMPORT-02 pass; IMPORT-03/04/05 awaiting Plans 02 and 03

---
*Phase: 20-nexus-import-temporal-compression*
*Completed: 2026-03-29*
