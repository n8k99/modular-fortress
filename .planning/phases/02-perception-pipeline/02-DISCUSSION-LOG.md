# Phase 2: Perception Pipeline - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-26
**Phase:** 02-perception-pipeline
**Areas discussed:** Task field enrichment, Assignment column migration, Scheduling & filtering, Urgency boost verification

---

## Task Field Enrichment

| Option | Description | Selected |
|--------|-------------|----------|
| All GSD fields | Add project_id, source, context, parent_id, priority, assigned_to, scheduled_at | ✓ |
| Selective — key fields only | project_id, source, priority, parent_id only | |
| Separate GSD endpoint | Keep perception as-is, add separate enriched endpoint | |

**User's choice:** All GSD fields

---

## Assignment Column Migration

| Option | Description | Selected |
|--------|-------------|----------|
| Migrate to assigned_to | Update all queries to use assigned_to (text[] with ANY) | ✓ |
| Support both columns | Query WHERE assignee = $1 OR $1 = ANY(assigned_to) | |
| Backfill assignee from assigned_to | Trigger syncs assigned_to[0] to assignee | |

**User's choice:** Migrate to assigned_to

---

## Scheduling & Filtering

| Option | Description | Selected |
|--------|-------------|----------|
| Filter by scheduled_at | Dispatch sets scheduled_at, perception filters | |
| Wave-aware perception | Check context JSON for wave, return lowest incomplete | |
| No filtering — executive manages | Show all tasks, executives decide ordering | ✓ |

**User's choice:** No filtering — executive manages

---

## Urgency Boost Verification

| Option | Description | Selected |
|--------|-------------|----------|
| End-to-end test | Dispatch project, run perception, check urgency in tick logs | ✓ |
| Unit test in Lisp | Test urgency function with mock perception | |
| Just verify code path | Read tick-engine.lisp, confirm branch fires | |

**User's choice:** End-to-end test

---

## Claude's Discretion

- Whether to add project goals/current_context to perception response
- Error handling for malformed assigned_to arrays
- Whether to truncate context JSON in response

## Deferred Ideas

None
