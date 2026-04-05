---
id: S03
parent: M004
milestone: M004
provides:
  - Tray-based app lifecycle — minimize to tray, reopen from tray
requires:
  - slice: S01
    provides: Working Wails v3 project scaffold
affects:
  []
key_files:
  - dragonpunk-app/main.go
key_decisions:
  - ActivationPolicyAccessory for tray-app behavior
  - Template icon for macOS dark/light adaptation
patterns_established:
  - System tray pattern for Wails v3 macOS apps
observability_surfaces:
  - slog on window show/hide/quit from tray
drill_down_paths:
  - .gsd/milestones/M004/slices/S03/tasks/T01-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-05T06:23:20.435Z
blocker_discovered: false
---

# S03: System Tray + App Lifecycle

**System tray icon with window toggle and quit menu — app runs as macOS tray application**

## What Happened

Added system tray using Wails v3 SystemTray API. Template icon adapts to macOS menu bar style. Click handler toggles main window visibility with slog output. Context menu provides 'Show Dragonpunk' and 'Quit'. ActivationPolicy set to Accessory so the app behaves as a tray app — no dock icon when window is hidden. This completes the two-switch architecture's UI side: the app can minimize to tray while the noosphere keeps running.

## Verification

Build succeeds, app runs with DB connected. Tray functionality requires manual visual verification.

## Requirements Advanced

- R005 — Third increment — system tray enables the two-switch architecture (app can hide to tray while noosphere runs)

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

Cannot programmatically verify tray icon or click behavior. Manual verification needed.

## Follow-ups

Nathan should verify: tray icon visible in menu bar, click toggles window, right-click shows menu, Quit exits cleanly.

## Files Created/Modified

- `dragonpunk-app/main.go` — Added system tray icon, click handler, context menu, ActivationPolicyAccessory
