---
phase: 24
slug: template-evaluation-execution
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-29
---

# Phase 24 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | SBCL assert + bash scripts |
| **Config file** | none — Wave 0 extends existing test harness |
| **Quick run command** | `sbcl --script /opt/project-noosphere-ghosts/tests/test-template-eval.lisp` |
| **Full suite command** | `bash /opt/project-noosphere-ghosts/tests/run-all-tests.sh` |
| **Estimated runtime** | ~8 seconds |

---

## Sampling Rate

- **After every task commit:** Run quick test
- **After every plan wave:** Run full suite
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 8 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 24-01-01 | 01 | 1 | INNATE-02 | integration | `sbcl --script tests/test-template-eval.lisp` | ❌ W0 | ⬜ pending |
| 24-02-01 | 02 | 2 | INNATE-02, INNATE-04 | integration | `bash tests/run-all-tests.sh` | ❌ W0 | ⬜ pending |

---

## Wave 0 Requirements

- [ ] `tests/test-template-eval.lisp` — Template loading, Innate evaluation, error handling tests

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Commission triggers tool invocation | INNATE-04 | Requires live tick cycle with multiple agents | Insert template with (sarah_lin){sync_calendar}, run tick, verify conversation appears |
| Template content in LLM prompt | INNATE-02 | Requires LLM call observation | Check cognition job input-context contains evaluated content |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 8s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
