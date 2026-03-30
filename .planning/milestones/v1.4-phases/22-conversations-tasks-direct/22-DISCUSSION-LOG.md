# Phase 22: Conversations & Tasks Direct - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-29
**Phase:** 22-conversations-tasks-direct
**Areas discussed:** Conversation SQL scope, Task mutation scope, Auxiliary HTTP scope, JSONB array operations
**Mode:** Auto (--auto) — all decisions auto-selected

---

## Conversation SQL scope

| Option | Description | Selected |
|--------|-------------|----------|
| action-executor + tick-engine only | Convert only the two files with most conversation calls | |
| All files with api-post /api/conversations | Convert every file that posts conversations | ✓ |

**User's choice:** [auto] All files — matches success criteria "zero HTTP calls"
**Notes:** ~20 api-post conversation calls in action-executor.lisp, plus mark-read in tick-engine.lisp, plus reads in action-planner.lisp

---

## Task mutation scope

| Option | Description | Selected |
|--------|-------------|----------|
| Status updates only | Just convert api-patch for task status changes | |
| All CRUD | Create, status updates, completion, blocked_by management | ✓ |

**User's choice:** [auto] All CRUD — matches DB-04 requirement
**Notes:** ~15 api-patch task calls + api-post task creation + api-get task reads across action-executor and action-planner

---

## Auxiliary HTTP scope

| Option | Description | Selected |
|--------|-------------|----------|
| Core only (conversations + tasks) | Leave tick-log, drives, etc. as HTTP | |
| Everything | Convert all remaining HTTP to achieve zero-HTTP goal | ✓ |

**User's choice:** [auto] Everything — success criterion 4 requires zero HTTP calls from tick engine
**Notes:** Includes tick-log, tick-reports, drives, decisions, documents, cognition telemetry, agent reads, user-profile reads

---

## JSONB array operations

| Option | Description | Selected |
|--------|-------------|----------|
| array_append() | Use PostgreSQL array functions for read_by | ✓ |
| jsonb_set() | Use JSONB manipulation if column is JSONB type | |

**User's choice:** [auto] array_append() — Claude's discretion on exact implementation based on column type
**Notes:** Must prevent duplicates in read_by array

---

## Claude's Discretion

- Exact SQL for each operation
- File organization (extend db-client.lisp or create new files)
- Transaction boundaries
- Error handling strategy
- Projects reads conversion

## Deferred Ideas

- Removing api-client.lisp entirely
- LISTEN/NOTIFY for real-time events
- Connection pool scaling
