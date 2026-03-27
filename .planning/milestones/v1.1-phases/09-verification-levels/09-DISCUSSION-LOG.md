# Phase 9: Verification Levels - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-26
**Phase:** 09-verification-levels
**Areas discussed:** Quality assessment format, Severity classification, Executive urgency escalation

---

## Quality Assessment Format

| Option | Description | Selected |
|--------|-------------|----------|
| Leverage Phase 7 issues array | Extract issues from structured artifact, include in completion report. No new command. | ✓ |
| New VERIFY: command block | Separate command alongside COMPLETE: with explicit assessment. | |
| You decide | | |

**User's choice:** Leverage Phase 7 issues array (Recommended)

---

## Severity Classification

| Option | Description | Selected |
|--------|-------------|----------|
| Self-assessed by completing ghost | LLM includes severity in issues array based on context. Simple. | ✓ |
| Rule-based classification | CRITICAL if must_haves unmet, WARNING if partial. Deterministic but rigid. | |
| You decide | | |

**User's choice:** Self-assessed by completing ghost (Recommended)

---

## Executive Urgency Escalation

| Option | Description | Selected |
|--------|-------------|----------|
| Tag completion message metadata | Add severity to conversation metadata. Use existing message boost. | |
| New urgency modifier in tick engine | quality_issue_boost in urgency formula. Separate from message boost. | ✓ |
| You decide | | |

**User's choice:** New urgency modifier in tick engine

---

## Claude's Discretion

- Exact urgency boost value for CRITICAL issues
- Whether to store quality assessment in task field
- How to surface quality issues in perception

## Deferred Ideas

None
