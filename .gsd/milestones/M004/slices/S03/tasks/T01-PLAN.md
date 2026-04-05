---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T01: Add system tray with window toggle and quit menu

Update main.go to create a system tray icon using Wails v3 app.SystemTray.New(). Set icon, tooltip, click handler to toggle window visibility, and context menu with Show Window and Quit items. Change ActivationPolicy to Accessory so the app can run as a tray-only app. The window should start visible but be hideable to tray.

## Inputs

- `Wails v3 system tray API docs from research`

## Expected Output

- `Updated main.go with system tray code`
- `Working tray icon in macOS menu bar`

## Verification

wails3 build succeeds. App launches with tray icon visible. Click tray toggles window. Quit exits.
