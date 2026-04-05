# S01: Layout Shell + Scene Canvas

**Goal:** Build the three-column Foundry VTT layout: LHSB icon rail | scene canvas (Orbis map JPEG, pannable/zoomable) | RHSB icon rail
**Demo:** After this: App opens with three-column layout: icon rails on both sides, Orbis map filling the center, pannable and zoomable

## Tasks
- [x] **T01: Three-column Foundry VTT layout built — LHSB (9 domain icons + gear) | scene canvas | RHSB (6 composed view icons)** — Replace the current placeholder HTML/CSS with a three-column flexbox layout. LHSB: narrow icon rail (48px wide) with 9 domain icons + gear at bottom. RHSB: narrow icon rail with 6 composed view icons. Center: scene canvas div filling all remaining space. Use the vault-map color palette. Icons can be emoji initially — custom SVGs later.
  - Estimate: 25min
  - Files: dragonpunk-app/frontend/index.html, dragonpunk-app/frontend/public/style.css, dragonpunk-app/frontend/src/main.ts
  - Verify: wails3 build succeeds, app shows three-column layout with icon rails on both sides
- [x] **T02: Orbis map renders as pannable/zoomable scene canvas background with cursor-centered zoom** — Add the Orbis map JPEG as the scene canvas background. The screenshot file is at the project root: Screenshot 2026-03-30 at 23.00.12.png. Implement pan (mouse drag) and zoom (scroll wheel) using CSS transforms on a container div. The map should start centered and be freely pannable in all directions. Zoom should scale around the mouse cursor position.
  - Estimate: 25min
  - Files: dragonpunk-app/frontend/src/main.ts, dragonpunk-app/frontend/public/style.css
  - Verify: Orbis map visible in canvas area, mouse drag pans, scroll wheel zooms smoothly
