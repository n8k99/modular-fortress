# S03: System Tray + App Lifecycle

**Goal:** Add system tray icon with show/hide window toggle and quit menu. App minimizes to tray and reopens from tray click.
**Demo:** After this: App minimizes to system tray, tray icon click reopens window, tray menu has Quit option

## Tasks
- [x] **T01: System tray added — icon with tooltip, click toggles window, context menu with Show and Quit** — Update main.go to create a system tray icon using Wails v3 app.SystemTray.New(). Set icon, tooltip, click handler to toggle window visibility, and context menu with Show Window and Quit items. Change ActivationPolicy to Accessory so the app can run as a tray-only app. The window should start visible but be hideable to tray.
  - Estimate: 15min
  - Files: dragonpunk-app/main.go
  - Verify: wails3 build succeeds. App launches with tray icon visible. Click tray toggles window. Quit exits.
