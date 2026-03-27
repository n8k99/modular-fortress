# Phase 11: Message Hygiene - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-27
**Phase:** 11-message-hygiene
**Areas discussed:** Read-marking trigger, Bulk vs individual, Historical cleanup, sqlx fix scope
**Mode:** --auto (all recommended defaults selected)

---

## Read-marking Trigger

| Option | Description | Selected |
|--------|-------------|----------|
| After cognition completes | Ghost processes message first, then marks read | auto |
| After perception loads | Mark read immediately when fetched — risks losing unprocessed messages | |

**User's choice:** [auto] After cognition completes (recommended default)
**Notes:** Ensures ghost actually processed the message. Failed cognition = message retried next tick.

---

## Bulk vs Individual

| Option | Description | Selected |
|--------|-------------|----------|
| Batch endpoint | One POST with array of IDs — 1 call per tick | auto |
| Per-message endpoint | Individual POST per message — N calls per tick | |

**User's choice:** [auto] Batch endpoint (recommended default)
**Notes:** Reduces API overhead. Single query with array_append for all IDs.

---

## Historical Cleanup

| Option | Description | Selected |
|--------|-------------|----------|
| One-time SQL cleanup | Mark all existing stale messages as read | auto |
| Let them expire naturally | Wait for read_by to accumulate organically | |
| Delete duplicates | Remove spam messages from DB | |

**User's choice:** [auto] One-time SQL cleanup (recommended default)
**Notes:** Prevents 336+ duplicate messages from re-triggering on ghost restart.

---

## sqlx Fix Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Add json feature + verify round-trip | Fix root cause and confirm E2E | auto |
| Just add the feature flag | Minimal change, trust it works | |

**User's choice:** [auto] Add json feature + verify round-trip (recommended default)
**Notes:** Phase 10 lifecycle signals depend on this. Verify metadata persistence works.

---

## Claude's Discretion

- Exact SQL for historical cleanup migration
- Whether to add GIN index on read_by
- Error handling for batch mark-read endpoint

## Deferred Ideas

None
