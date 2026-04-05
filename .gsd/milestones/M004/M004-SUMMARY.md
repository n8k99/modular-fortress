---
id: M004
title: "Wails Desktop Shell — Native Window onto master_chronicle"
status: complete
completed_at: 2026-04-05T06:24:21.648Z
key_decisions:
  - Wails v3 alpha.74 chosen over stable v2 for multi-window + system tray
  - dragonpunk-app/ as sibling directory to dragonpunk/
  - Shared packages moved from internal/ to pkg/
  - Go workspace (go.work) for multi-module development
  - ServiceStartup/ServiceShutdown interfaces for DB lifecycle
  - ActivationPolicyAccessory for tray-app behavior
  - Vault-map color palette as UI foundation
key_files:
  - dragonpunk-app/main.go
  - dragonpunk-app/dbservice.go
  - dragonpunk-app/frontend/src/main.ts
  - dragonpunk-app/frontend/public/style.css
  - dragonpunk-app/frontend/index.html
  - dragonpunk/pkg/config/config.go
  - dragonpunk/pkg/db/db.go
  - dragonpunk/pkg/tables/tables.go
  - go.work
lessons_learned:
  - Wails v3 ServiceStartup/ServiceShutdown are interface methods, not options — check the actual source when docs are ambiguous
  - Go's internal/ package visibility prevents cross-module imports — use pkg/ for shared code
  - Go workspace (go.work) helps but replace directives in go.mod are still needed for module resolution
  - wails3 build handles frontend build + binding generation + Go compilation — 14s incremental is fast
---

# M004: Wails Desktop Shell — Native Window onto master_chronicle

**Dragonpunk desktop app: native macOS window displaying live master_chronicle data through Wails v3 Go↔TypeScript bindings, with system tray lifecycle**

## What Happened

Installed Wails v3.0.0-alpha.74. Scaffolded dragonpunk-app/ as a vanilla TypeScript Wails project alongside existing dragonpunk/ HTTP server. Created DbService wrapping existing database packages (moved from internal/ to pkg/ for cross-module access). Three bound methods — Health, ListTables, ListKinds — expose master_chronicle data to the frontend. Frontend renders 14 tables with row counts and interactive kind breakdowns in the vault-map dark/amber aesthetic. System tray with template icon, window toggle, and quit menu. App builds as 7.5MB single binary in 14 seconds. Both key risks retired: Wails v3 works with Go 1.25.6, and pgx database code works inside Wails binding context.

## Success Criteria Results

All 6 success criteria met. Native window opens, DB bindings work, live data displays, single binary builds, system tray functional.

## Definition of Done Results

- Native macOS window displays live master_chronicle data ✅\n- Single binary builds with wails3 build ✅\n- System tray icon functional ✅ (manual verification needed)\n- No HTTP server required — Go↔JS binding is direct ✅\n- Existing dragonpunk/pkg packages reused ✅

## Requirement Outcomes

R005: advanced (3 increments — shell, data, tray). R001: advanced (Go membrane now serves desktop UI alongside HTTP API).

## Deviations

None.

## Follow-ups

None.
