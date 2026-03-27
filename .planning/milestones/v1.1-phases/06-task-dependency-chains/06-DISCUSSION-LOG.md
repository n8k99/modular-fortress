# Phase 6: Task Dependency Chains - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 06-task-dependency-chains
**Areas discussed:** Dependency cardinality, Perception filtering, Auto-unblock mechanism, CREATE_TASK syntax

---

## Dependency Cardinality

| Option | Description | Selected |
|--------|-------------|----------|
| Single blocker (keep as-is) | One blocked_by INTEGER FK. Simple, covers most cases. Model multi-predecessor as parent task. | |
| Integer array column | Change blocked_by to INTEGER[] (Postgres array). Task unblocked when ALL refs complete. Native Postgres. | ✓ |
| You decide | Claude picks based on codebase patterns and requirements. | |

**User's choice:** Integer array column
**Notes:** None

### Follow-up: Migration strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Migrate existing values | ALTER COLUMN + UPDATE to convert existing INTEGER values to single-element arrays. No data loss. | ✓ |
| Reset to empty | Drop and re-add as INTEGER[]. Existing values lost. Simpler if no production data. | |
| You decide | Claude checks if any rows currently use blocked_by and picks accordingly. | |

**User's choice:** Migrate existing values (Recommended)
**Notes:** None

---

## Perception Filtering

| Option | Description | Selected |
|--------|-------------|----------|
| SQL query in dpn-api | Add WHERE clause: only return tasks where blocked_by is empty OR all referenced IDs have status='done'. Blocked tasks never leave the API. | ✓ |
| Lisp perception layer | API returns all tasks including blocked. Lisp filters during perceive(). Ghosts see blocked work. | |
| Both layers | SQL filters by default, include ?include_blocked=true param for executive-tier agents. | |

**User's choice:** SQL query in dpn-api (Recommended)
**Notes:** None

### Follow-up: Executive visibility

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, in project context | Executives see blocked tasks as separate section in project review. Informational, not actionable. | ✓ |
| No, same filter for all | Executives see only unblocked tasks too. Dependencies known from completion reports. | |
| You decide | Claude determines based on how project review currently works. | |

**User's choice:** Yes, in project context
**Notes:** None

---

## Auto-Unblock Mechanism

| Option | Description | Selected |
|--------|-------------|----------|
| DB trigger | Extend on_task_completed_after(). Remove completed ID from blocked_by arrays. Same pattern as wave advancement. | ✓ |
| API endpoint logic | Run unblock logic in Rust handler on PATCH status=done. More testable but splits side-effects. | |
| Lisp action-executor | After COMPLETE: parsing, call unblock API endpoint. Ghost-driven but adds latency. | |

**User's choice:** DB trigger (Recommended)
**Notes:** None

### Follow-up: Status change on unblock

| Option | Description | Selected |
|--------|-------------|----------|
| No status change | Just remove from array. Task stays at current status. Perception filtering handles visibility. | |
| Set status to 'open' | Explicitly set status='open' when fully unblocked. Matches wave advancement pattern. | |
| You decide | Claude picks based on how perception filtering and wave trigger interact. | ✓ |

**User's choice:** You decide
**Notes:** None

---

## CREATE_TASK Syntax

| Option | Description | Selected |
|--------|-------------|----------|
| Inline blocked_by param | CREATE_TASK: desc assignee=id blocked_by=#123,#456. Extends existing key=value pattern. | ✓ |
| Separate DEPENDS_ON command | DEPENDS_ON: #789 blocked_by=#123. Separate action line for adding dependencies. | |
| You decide | Claude picks syntax that fits existing parser pattern. | |

**User's choice:** Inline blocked_by param (Recommended)
**Notes:** None

### Follow-up: Invalid task ID handling

| Option | Description | Selected |
|--------|-------------|----------|
| Silently ignore bad IDs | Create task, skip invalid IDs. May be immediately unblocked. Low friction. | |
| Create task but warn | Create with valid IDs only. Post warning to executive about invalid references. | |
| You decide | Claude picks most robust approach based on existing error patterns. | ✓ |

**User's choice:** You decide
**Notes:** None

---

## Claude's Discretion

- Status change behavior when task fully unblocked (no status change vs set to 'open')
- Invalid task ID handling in blocked_by references (ignore vs warn)

## Deferred Ideas

None — discussion stayed within phase scope
