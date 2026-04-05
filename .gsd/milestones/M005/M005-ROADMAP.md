# M005: Scene Canvas + Sidebar Shell — The Foundry Layout

## Vision
Replace the placeholder data display with the actual Foundry VTT-inspired layout: a full-viewport scene canvas with two collapsible icon-rail sidebars (LHSB for domains, RHSB for composed views). This is the skeleton that all future UI work builds on — every panel, every scene, every floating window plugs into this shell.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Layout Shell + Scene Canvas | high | — | ✅ | App opens with three-column layout: icon rails on both sides, Orbis map filling the center, pannable and zoomable |
| S02 | Sidebar Panel Toggle + Domain Browser | medium | S01 | ⬜ | Click a domain icon in LHSB — slide-out panel shows folder-hierarchy of entries from that table. Click again to close. |
