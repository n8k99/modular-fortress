---
id: T02
parent: S01
milestone: M004
key_files:
  - dragonpunk-app/main.go
  - dragonpunk-app/greetservice.go
  - dragonpunk-app/frontend/index.html
  - dragonpunk-app/frontend/public/style.css
  - dragonpunk-app/frontend/src/main.ts
  - dragonpunk-app/go.mod
key_decisions:
  - dragonpunk-app/ as sibling directory to dragonpunk/ — keeps HTTP server operational during transition
  - vanilla-ts template — defers framework choice per D015
  - Vault-map color palette (void/ember/smoke) for app chrome
duration: 
verification_result: passed
completed_at: 2026-04-05T05:49:58.335Z
blocker_discovered: false
---

# T02: Scaffolded Wails v3 project as dragonpunk-app/ with vanilla TypeScript, Dragonpunk branding, and vault-map color palette

**Scaffolded Wails v3 project as dragonpunk-app/ with vanilla TypeScript, Dragonpunk branding, and vault-map color palette**

## What Happened

Created Wails v3 project using `wails3 init -t vanilla-ts`. Placed at dragonpunk-app/ alongside existing dragonpunk/ directory. Customized main.go with Dragonpunk app name, 1280x800 window, translucent macOS backdrop, void background color (#08080D from vault-map palette). Replaced template frontend with Dragonpunk-branded HTML/CSS using IBM Plex Mono + Cormorant Garamond fonts and the ember/void/smoke color scheme. Removed nested .git directory. Updated go.mod to Go 1.25.6.

## Verification

wails3 build completed in 71 seconds with no errors. Binary at bin/dragonpunk-app is 7.5MB.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `wails3 build` | 0 | ✅ pass | 70800ms |
| 2 | `ls -lh bin/dragonpunk-app` | 0 | ✅ pass — 7.5MB binary | 100ms |

## Deviations

Created dragonpunk-app/ as a sibling directory rather than restructuring inside dragonpunk/. This keeps the existing HTTP server working while the Wails shell develops. They'll merge later.

## Known Issues

None.

## Files Created/Modified

- `dragonpunk-app/main.go`
- `dragonpunk-app/greetservice.go`
- `dragonpunk-app/frontend/index.html`
- `dragonpunk-app/frontend/public/style.css`
- `dragonpunk-app/frontend/src/main.ts`
- `dragonpunk-app/go.mod`
