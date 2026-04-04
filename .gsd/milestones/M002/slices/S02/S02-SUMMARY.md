---
id: S02
parent: M002
milestone: M002
provides:
  - Slug lookup for wikilink resolution
  - Kind enumeration for UI filters
requires:
  - slice: S01
    provides: Generic query infrastructure and table whitelist
affects:
  []
key_files:
  - dragonpunk/internal/db/query.go
  - dragonpunk/internal/api/tables.go
  - dragonpunk/internal/api/router.go
key_decisions:
  - Explicit /slug/ route prefix
  - Infrastructure tables return 404 for /kinds
patterns_established:
  - Slug-based lookup as wikilink resolver foundation
observability_surfaces:
  - none
drill_down_paths:
  - .gsd/milestones/M002/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M002/slices/S02/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-04T23:06:04.020Z
blocker_discovered: false
---

# S02: Slug lookup + kind listing

**Slug-based row lookup and kind enumeration for all Nine Tables**

## What Happened

Added two endpoints: slug lookup for wikilink-style navigation (GET /api/{table}/slug/{slug}) and kind listing for filter dropdowns (GET /api/{table}/kinds). Both verified against live data — Orbis lookups work, kind taxonomy visible.

## Verification

Slug lookup returns Orbis from the_realms; kinds endpoint shows 9 kinds for the_work, 43 for the_realms; infrastructure tables correctly report no kind column

## Requirements Advanced

- R001 — Four more endpoints added to Dragonpunk Read API

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

None.

## Follow-ups

None.

## Files Created/Modified

- `dragonpunk/internal/db/query.go` — Added GetBySlug and ListKinds functions
- `dragonpunk/internal/api/tables.go` — Added GetBySlug and ListKinds handlers
- `dragonpunk/internal/api/router.go` — Wired slug and kinds routes
