---
id: S01
parent: M005
milestone: M005
provides:
  - Layout shell that all future UI components plug into
  - Scene canvas with pan/zoom for maps and token placement
requires:
  - slice: M004/S02
    provides: DbService with Health and ListKinds bindings
affects:
  - S02
key_files:
  - dragonpunk-app/frontend/index.html
  - dragonpunk-app/frontend/public/style.css
  - dragonpunk-app/frontend/src/main.ts
  - dragonpunk-app/frontend/public/orbis-map.png
key_decisions:
  - Flexbox three-column layout
  - CSS transform pan/zoom for scene canvas
  - 48px icon rails with 280px slide-out panels
  - Emoji icons initially
patterns_established:
  - Foundry VTT layout pattern: icon rail + slide panel + scene canvas
  - Sidebar panel toggle: click icon to open, click again or another to close/swap
observability_surfaces:
  - Status bar showing DB connection state
drill_down_paths:
  - .gsd/milestones/M005/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M005/slices/S01/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-05T06:52:52.537Z
blocker_discovered: false
---

# S01: Layout Shell + Scene Canvas

**Foundry VTT three-column layout with icon-rail sidebars, Orbis map canvas with pan/zoom, and sidebar panel toggles showing live DB data**

## What Happened

Built the complete Foundry-inspired layout shell. Three-column flexbox: 48px LHSB icon rail (9 domain icons per D018 + settings gear) | scene canvas with Orbis map (pan/zoom via CSS transforms) | 48px RHSB icon rail (6 composed view icons). Sidebar icons toggle 280px slide-out panels. LHSB domain panels call DbService.ListKinds for live kind breakdowns. Status bar shows DB connection state. Title bar drag region enables window movement. Vault-map aesthetic maintained throughout.

## Verification

Build succeeds, app launches, DB connects, window title correct. Visual layout verification needed by Nathan.

## Requirements Advanced

- R005 — Fourth increment — actual Foundry VTT layout shell with sidebars, scene canvas, pan/zoom, and panel toggles

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

T01 and T02 combined into single implementation since layout and canvas are interdependent.

## Known Limitations

Orbis map image includes Foundry VTT chrome. RHSB panels show placeholder 'Coming soon'. No ghost tokens or floating windows yet.

## Follow-ups

Nathan visual verification. Replace Orbis screenshot with clean map export. S02 will populate domain browser panels with entry listings.

## Files Created/Modified

- `dragonpunk-app/frontend/index.html` — Complete rewrite: three-column layout with LHSB, scene canvas, RHSB
- `dragonpunk-app/frontend/public/style.css` — Complete rewrite: sidebar, canvas, panel, status bar styles
- `dragonpunk-app/frontend/src/main.ts` — Complete rewrite: canvas pan/zoom, sidebar toggle, domain panel loading
- `dragonpunk-app/frontend/public/orbis-map.png` — New: Orbis map image for scene background
