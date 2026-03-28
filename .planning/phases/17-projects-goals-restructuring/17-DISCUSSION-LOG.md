# Phase 17: Projects & Goals Restructuring - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-28
**Phase:** 17-projects-goals-restructuring
**Areas discussed:** Lifestage Backfill, Goals FK Migration, Forward-Only Enforcement, Perception Enrichment
**Mode:** --auto (all decisions auto-selected)

---

## Lifestage Backfill Mapping

| Option | Description | Selected |
|--------|-------------|----------|
| Map by status/maturity | completed→Harvest, active→Tree/Sapling, paused→Seed | ✓ |
| All start at Seed | Let executives advance lifestages over time | |
| Manual assignment | Nathan assigns each project's lifestage | |

**User's choice:** [auto] Map by status/maturity (recommended default)
**Notes:** 15 projects total, not 14. Recently created projects (56, 59) get Sapling.

---

## Goals FK Migration

| Option | Description | Selected |
|--------|-------------|----------|
| Best-effort match + nullable FK | Parse wikilinks, match by name, NULL for unresolvable | ✓ |
| Strict match + manual intervention | Require all 44 goals mapped before committing | |
| Drop unmatchable goals | Remove goals referencing deprecated projects | |

**User's choice:** [auto] Best-effort match + nullable FK (recommended)
**Notes:** Only 3 project names in goals text. GOTCHA and Puppet Show have no matching project records.

---

## Forward-Only Enforcement

| Option | Description | Selected |
|--------|-------------|----------|
| Database trigger | BEFORE UPDATE trigger prevents backward transitions | ✓ |
| Application-level | API checks only | |
| CHECK constraint | Cannot reference prior values, so not viable | |

**User's choice:** [auto] Database trigger (recommended — consistent with Phase 16)

---

## Perception Enrichment

| Option | Description | Selected |
|--------|-------------|----------|
| lifestage + area_name | Minimal LEFT JOIN addition | ✓ |
| Full area object | Include area description, owner, etc. | |
| No perception change | Defer to Phase 18+ | |

**User's choice:** [auto] lifestage + area_name (recommended — minimal, per out-of-scope constraint)

---

## Claude's Discretion

- Index choices for FK columns
- Wikilink parsing regex
- ALTER TABLE statement ordering
- Composite index on (area_id, status)

## Deferred Ideas

None
