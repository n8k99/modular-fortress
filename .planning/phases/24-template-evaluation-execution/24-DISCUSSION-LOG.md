# Phase 24: Template Evaluation & Execution - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-29
**Phase:** 24-template-evaluation-execution
**Areas discussed:** Template loading, Innate evaluation integration, Commission-to-tool mapping, Error handling
**Mode:** Auto (--auto)

---

## Template Loading Trigger

| Option | Description | Selected |
|--------|-------------|----------|
| All cognition jobs | Evaluate templates for every job type | |
| Standing order/operations only | Only for scheduled pipeline work | ✓ |

**User's choice:** [auto] Standing order/operations — templates enrich operational pipeline work
**Notes:** No template = no enrichment, additive only

---

## Innate Evaluation Integration

| Option | Description | Selected |
|--------|-------------|----------|
| In action-planner before job creation | Evaluate before input-context is built | ✓ |
| In cognition-broker before LLM dispatch | Evaluate at dispatch time | |

**User's choice:** [auto] In action-planner — natural fit, all context available
**Notes:** Uses :scope :query for read-only evaluation

---

## Commission-to-Tool Mapping

| Option | Description | Selected |
|--------|-------------|----------|
| Direct tool invocation during eval | Execute tools immediately during template evaluation | |
| Fire-and-forget conversation | Commission inserts conversation, target perceives next tick | ✓ |

**User's choice:** [auto] Fire-and-forget — maintains async architecture
**Notes:** Reuses existing conversation→perception→execution pipeline

---

## Error Handling

| Option | Description | Selected |
|--------|-------------|----------|
| handler-case with fallback string | Catch errors, inject error context | ✓ |
| Let errors propagate | Fail the cognition job | |

**User's choice:** [auto] handler-case with fallback — tick must never crash

## Claude's Discretion

- Exact planner integration point, template-to-project association, eval-env construction

## Deferred Ideas

- Ghost expression generation — Phase 25
- Template creation by ghosts — Phase 25
