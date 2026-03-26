# Phase 8: Decisions Brain - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 08-decisions-brain
**Areas discussed:** Decision capture flow, Decision context injection, Decisions API design, Decision scope & attribution

---

## Decision Capture Flow

| Option | Description | Selected |
|--------|-------------|----------|
| Enhance existing DECISION: detection | Keep keyword pattern, POST to decisions table via API. Minimal change. | ✓ |
| New structured DECIDE: command | Formal action command with explicit fields. More structured, changes prompt. | |
| You decide | Claude picks based on existing detection. | |

**User's choice:** Enhance existing DECISION: detection (Recommended)

---

## Decision Context Injection

| Option | Description | Selected |
|--------|-------------|----------|
| Last 10 decisions per project | Query with limit=10, order=desc. Bounded context, most relevant first. | ✓ |
| All decisions for the project | Complete history, could grow large. | |
| You decide | Claude picks based on prompt size constraints. | |

**User's choice:** Last 10 decisions per project (Recommended)

---

## Decisions API Design

| Option | Description | Selected |
|--------|-------------|----------|
| GET + POST only | Append-only historical records. No update/delete. | ✓ |
| Full CRUD | Allows correcting/retracting decisions. More flexible. | |
| You decide | Claude designs based on existing patterns. | |

**User's choice:** GET + POST only (Recommended)

---

## Decision Scope & Attribution

### Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Project-scoped only | Decisions always have project_id. Simple. | |
| Project + department scope | Optional department field for department-wide decisions. | ✓ |
| You decide | Claude picks based on schema and executive patterns. | |

**User's choice:** Project + department scope

### Attribution

| Option | Description | Selected |
|--------|-------------|----------|
| Auto-populate from context | Owner=agent_id, stakeholders=project team. Zero friction. | ✓ |
| Executive specifies explicitly | Executive includes stakeholders in DECISION: line. More control. | |
| You decide | Claude picks based on minimal friction. | |

**User's choice:** Auto-populate from context (Recommended)

---

## Claude's Discretion

- Rationale parsing from DECISION: line
- Decision formatting in review prompt
- Whether to add tags/category field

## Deferred Ideas

None — discussion stayed within phase scope
