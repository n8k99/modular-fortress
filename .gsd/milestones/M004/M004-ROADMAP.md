# M004: Wails Desktop Shell — Native Window onto master_chronicle

## Vision
Get a native macOS window running via Wails v3 that connects to master_chronicle PostgreSQL through the existing Dragonpunk Go code and displays live data. This is the first pixel of the Foundry VTT-style interface — proving Wails works as the application shell before building scene canvas, sidebars, or floating windows.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Wails v3 Install + Hello Window | high | — | ✅ | A native macOS window opens with 'Dragonpunk — Modular Fortress' title and placeholder content |
| S02 | Bind Dragonpunk DB to Frontend | medium | S01 | ✅ | Window displays live table names, row counts, and kind breakdowns from master_chronicle |
| S03 | System Tray + App Lifecycle | low | S01 | ✅ | App minimizes to system tray, tray icon click reopens window, tray menu has Quit option |
