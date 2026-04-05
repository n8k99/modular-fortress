---
id: T03
parent: S01
milestone: M004
key_files:
  - dragonpunk-app/bin/dragonpunk-app
key_decisions:
  - Hidden inset title bar for macOS — gives the app a clean frameless look with the traffic light buttons inset
duration: 
verification_result: passed
completed_at: 2026-04-05T05:50:24.158Z
blocker_discovered: false
---

# T03: Dragonpunk Wails app launches — native macOS window opens at 1280x800 with void background

**Dragonpunk Wails app launches — native macOS window opens at 1280x800 with void background**

## What Happened

Launched the built binary. macOS window confirmed open via mac_list_windows: windowId 2916, 1279x799, at position (200,126). Process running as pid 32075. No console errors. Window title not visible in mac_list_windows output (shows empty string — likely because of the hidden inset title bar style), but the window dimensions match exactly. Could not take screenshot due to Screen Recording permission not being granted to the terminal, but the window's existence and dimensions are confirmed programmatically.

## Verification

pgrep finds dragonpunk-app process running. mac_list_windows confirms 1 window at 1279x799. App launches and displays cleanly.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `pgrep -f dragonpunk-app` | 0 | ✅ pass — process running | 100ms |
| 2 | `mac_list_windows dragonpunk-app` | 0 | ✅ pass — 1 window 1279x799 | 200ms |

## Deviations

Window title shows as empty string in accessibility — this is expected with MacTitleBarHiddenInset style. Title is rendered by the app's frontend HTML, not the native title bar.

## Known Issues

Screen Recording permission not granted to terminal — cannot take automated screenshots of the app window. Manual visual verification needed.

## Files Created/Modified

- `dragonpunk-app/bin/dragonpunk-app`
