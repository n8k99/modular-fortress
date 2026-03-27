# Phase 10: Lifecycle Signals - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-27
**Phase:** 10-lifecycle-signals
**Areas discussed:** Idle signal mechanism, Executive availability view, Energy-availability alignment

---

## Idle Signal Mechanism

| Option | Description | Selected |
|--------|-------------|----------|
| Automatic detection | Tick engine detects empty task queue, sets idle flag. No LLM command. | ✓ |
| Explicit IDLE: command | Staff outputs IDLE: in LLM response. More intentional but adds prompt overhead. | |
| You decide | | |

**User's choice:** Automatic detection (Recommended)

---

## Executive Availability View

| Option | Description | Selected |
|--------|-------------|----------|
| Enhance team roster | Add status, energy, task count to format-team-roster. Uses existing API data. | ✓ |
| Separate idle_agents field | New array in perception response. Clean separation but new query. | |
| You decide | | |

**User's choice:** Enhance team roster (Recommended)

---

## Energy-Availability Alignment

| Option | Description | Selected |
|--------|-------------|----------|
| Boost on idle signal | One-time +10-15 energy on idle transition. Quick recovery to working/prime tier. | ✓ |
| Keep existing as-is | Regular +5/tick rest sufficient. No changes. | |
| You decide | | |

**User's choice:** Boost on idle signal (Recommended)

---

## Claude's Discretion

- Exact idle transition energy boost value
- Whether to add lifecycle_state column or use existing fields
- Task count query strategy in format-team-roster
- Idle agent sorting in roster

## Deferred Ideas

None
