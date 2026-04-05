---
id: T01
parent: S03
milestone: M004
key_files:
  - dragonpunk-app/main.go
key_decisions:
  - ActivationPolicyAccessory — app runs as tray app, no dock icon when hidden
  - Template icon for macOS tray (adapts to menu bar style)
duration: 
verification_result: passed
completed_at: 2026-04-05T06:22:51.185Z
blocker_discovered: false
---

# T01: System tray added — icon with tooltip, click toggles window, context menu with Show and Quit

**System tray added — icon with tooltip, click toggles window, context menu with Show and Quit**

## What Happened

Updated main.go with Wails v3 system tray: template icon for macOS, tooltip 'Dragonpunk — Modular Fortress', OnClick toggles window show/hide, context menu with 'Show Dragonpunk' and 'Quit'. Changed ActivationPolicy to Accessory so app can run as tray-only without dock icon when window is hidden. Build succeeds in 14s. App launches with DB connection and tray icon (tray icon needs manual visual verification).

## Verification

wails3 build succeeds. App runs, DB connects, window opens. Tray functionality requires manual verification (tray icon, click toggle, quit menu).

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `wails3 build` | 0 | ✅ pass — 13.8s | 13800ms |
| 2 | `pgrep + mac_list_windows` | 0 | ✅ pass — process running, window open | 200ms |

## Deviations

None.

## Known Issues

Tray icon and click behavior cannot be verified programmatically — needs Nathan to check manually.

## Files Created/Modified

- `dragonpunk-app/main.go`
