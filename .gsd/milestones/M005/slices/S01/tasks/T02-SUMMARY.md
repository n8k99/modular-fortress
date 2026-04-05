---
id: T02
parent: S01
milestone: M005
key_files:
  - dragonpunk-app/frontend/public/orbis-map.png
  - dragonpunk-app/frontend/src/main.ts
key_decisions:
  - CSS transform-based pan/zoom — simpler than Canvas API, sufficient for static background images
  - Start at 0.5x zoom to show more of the map initially
duration: 
verification_result: passed
completed_at: 2026-04-05T06:52:14.700Z
blocker_discovered: false
---

# T02: Orbis map renders as pannable/zoomable scene canvas background with cursor-centered zoom

**Orbis map renders as pannable/zoomable scene canvas background with cursor-centered zoom**

## What Happened

Orbis map PNG (3.1MB, from Foundry VTT screenshot) copied to frontend/public/orbis-map.png. Rendered as an img inside a CSS-transformed container. Pan via mouse drag (left button), zoom via scroll wheel with cursor-position-centered scaling (zoom factor 0.9/1.1, clamped to 0.1-3x range). Initial view at 0.5x scale with slight offset to show the map centered. Cursor changes to grab/grabbing during pan. Canvas area fills all space between sidebars.

## Verification

Built and ran as part of T01 combined implementation. App launches with map visible in canvas area.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `wails3 build (combined with T01)` | 0 | ✅ pass | 17700ms |

## Deviations

Combined with T01 implementation. The Orbis map is a screenshot with Foundry chrome around it \u2014 a clean version of the map image should replace it eventually.

## Known Issues

Map image includes Foundry VTT chrome (toolbars, sidebar). A clean export of just the Orbis map JPEG would be better. Pan/zoom needs visual verification by Nathan.

## Files Created/Modified

- `dragonpunk-app/frontend/public/orbis-map.png`
- `dragonpunk-app/frontend/src/main.ts`
