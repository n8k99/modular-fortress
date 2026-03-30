# Phase 29: Orbis Foundation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-30
**Phase:** 29-orbis-foundation
**Areas discussed:** Coordinate system, Ship assignments, RPG persona fields, Trust/energy thresholds
**Mode:** Auto (--auto flag, all defaults selected)

---

## Coordinate System

| Option | Description | Selected |
|--------|-------------|----------|
| Integer x/y grid | Simple, works with Drunkard's Walk | ✓ |
| Float lat/lon | Over-engineered for current needs | |
| Named zones only | Too coarse for movement | |

**User's choice:** [auto] Integer x/y (recommended default)

---

## Ship Assignments

| Option | Description | Selected |
|--------|-------------|----------|
| String referencing Pantheon Formation ship/role | Matches existing lore | ✓ |
| Enum of ship types | Too rigid | |

**User's choice:** [auto] String field (recommended default)

---

## RPG Persona Fields

| Option | Description | Selected |
|--------|-------------|----------|
| deity_codename + ship_role + personality_traits | Minimal but complete identity | ✓ |
| Full D&D-style stat block | Over-scoped for Phase 29 | |
| Just deity_codename | Too minimal | |

**User's choice:** [auto] Three-field rpg_persona section (recommended default)

---

## Trust/Energy Thresholds

| Option | Description | Selected |
|--------|-------------|----------|
| Numeric min thresholds per ghost | Flexible, per-ghost configuration | ✓ |
| Global thresholds only | Less flexible | |
| No thresholds yet | Defers too much | |

**User's choice:** [auto] Per-ghost numeric thresholds (recommended default)

---

## Claude's Discretion

- Exact coordinate values, deity codename assignments, default thresholds
- Whether to add fields to all 9 YAML files or executives only

## Deferred Ideas

- Drunkard's Walk, world object encounters, map visualization, threshold enforcement
