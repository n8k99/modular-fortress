---
id: S01
parent: M004
milestone: M004
provides:
  - Working Wails v3 project that builds and launches
  - Foundation for S02 (DB binding) and S03 (system tray)
requires:
  []
affects:
  - S02
  - S03
key_files:
  - dragonpunk-app/main.go
  - dragonpunk-app/frontend/index.html
  - dragonpunk-app/frontend/public/style.css
  - dragonpunk-app/frontend/src/main.ts
  - dragonpunk-app/go.mod
  - dragonpunk-app/bin/dragonpunk-app
key_decisions:
  - dragonpunk-app/ as sibling directory
  - vanilla-ts template (framework deferred)
  - Vault-map color palette for app chrome
  - Hidden inset title bar for macOS
patterns_established:
  - Wails v3 project structure with Go backend + TypeScript frontend + Vite build
  - Vault-map dark aesthetic as the UI foundation
observability_surfaces:
  - wails3 build output confirms compilation success
  - slog available for Go-side logging
drill_down_paths:
  - .gsd/milestones/M004/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M004/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M004/slices/S01/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-05T05:51:09.302Z
blocker_discovered: false
---

# S01: Wails v3 Install + Hello Window

**Wails v3 installed, Dragonpunk desktop app scaffolded and building — native macOS window opens with vault-map aesthetic**

## What Happened

Installed Wails v3.0.0-alpha.74 CLI. All system checks passed (Go 1.25.6, npm 11.5.1, Xcode CLI, Apple Silicon M1 Max). Scaffolded vanilla TypeScript project as dragonpunk-app/ alongside existing dragonpunk/ HTTP server. Customized with Dragonpunk branding, IBM Plex Mono + Cormorant Garamond fonts, vault-map color palette (void/ember/smoke). Build produces 7.5MB single binary in 71 seconds. Binary launches a native macOS window at 1280x800 with translucent backdrop and hidden inset title bar.

## Verification

wails3 doctor passes, wails3 build succeeds, binary launches and window confirmed via mac_list_windows at correct dimensions

## Requirements Advanced

- R005 — First increment — native macOS window shell with Foundry-inspired dark aesthetic

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Created dragonpunk-app/ as sibling rather than restructuring inside dragonpunk/. Keeps HTTP server operational during transition.

## Known Limitations

Screen Recording permission not granted — cannot take automated screenshots. Frontend currently shows placeholder content only (no DB connection yet — that's S02).

## Follow-ups

None.

## Files Created/Modified

- `dragonpunk-app/main.go` — Wails v3 app entry point with Dragonpunk branding
- `dragonpunk-app/frontend/index.html` — Dragonpunk-branded HTML shell
- `dragonpunk-app/frontend/public/style.css` — Vault-map dark palette CSS
- `dragonpunk-app/frontend/src/main.ts` — Frontend entry calling Go bindings
- `dragonpunk-app/go.mod` — Go module with Wails v3 dependency
