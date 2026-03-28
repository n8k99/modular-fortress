# Phase 12: Standing Orders - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-28
**Phase:** 12-standing-orders
**Areas discussed:** Schedule storage, Cron evaluation, Trigger mechanism, Multi-schedule support
**Mode:** --auto (all recommended defaults selected)

---

## Schedule Storage

| Option | Description | Selected |
|--------|-------------|----------|
| Column on projects table | JSONB schedule column, simplest approach | auto |
| Separate schedules table | Normalized, more flexible but more complex | |
| Config file | External YAML/JSON, not in DB | |

**User's choice:** [auto] Column on projects table (recommended default)
**Notes:** DB is the OS — state belongs in master_chronicle. JSONB allows flexible array of cron objects.

---

## Cron Evaluation

| Option | Description | Selected |
|--------|-------------|----------|
| Lisp-side in tick engine | Natural place, already runs on timer | auto |
| API-side schedule checking | Rust cron parser, returns scheduled projects in perception | |
| External cron daemon | System crontab triggers API calls | |

**User's choice:** [auto] Lisp-side in tick engine (recommended default)
**Notes:** Tick engine already evaluates urgency — schedule is just another urgency factor.

---

## Trigger Mechanism

| Option | Description | Selected |
|--------|-------------|----------|
| Inject into acting-set | Owner gets cognition job with project context | auto |
| Create recurring tasks | Auto-generate task records on schedule | |
| Direct tool execution | Skip cognition, run tool directly | |

**User's choice:** [auto] Inject into acting-set (recommended default)
**Notes:** Consistent with existing project review pattern. Executive decides what to do.

---

## Multi-schedule Support

| Option | Description | Selected |
|--------|-------------|----------|
| JSON array of cron objects | Each with expr + label, fires independently | auto |
| Single cron expression | One schedule per project, use subtasks for variants | |
| Comma-separated expressions | Multiple exprs in one string | |

**User's choice:** [auto] JSON array of cron objects (recommended default)
**Notes:** Trading project needs 3 separate schedules (Tokyo/London/NYC). Label is critical for executive context.

---

## Claude's Discretion

- Lisp cron parser implementation details
- Missed schedule handling
- Double-fire prevention (last_fired_at)

## Deferred Ideas

None
