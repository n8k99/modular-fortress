---
phase: 5
slug: feedback-reporting
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-26
---

# Phase 5 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | psql assertions + curl + Lisp grep |
| **Config file** | DB trigger functions in master_chronicle |
| **Quick run command** | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -c "SELECT prosrc FROM pg_proc WHERE proname='on_task_completed_after'" \| grep -c wave` |
| **Full suite command** | `bash .planning/phases/05-feedback-reporting/test_feedback_e2e.sh` |
| **Estimated runtime** | ~15 seconds |

---

## Sampling Rate

- **After every task commit:** Quick psql check
- **After every plan wave:** Full E2E suite
- **Before `/gsd:verify-work`:** Full suite must pass
- **Max feedback latency:** 15 seconds

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Ghost reports completion to executive via conversation | REPT-01 | Requires live tick | Start ghosts, complete a task, check conversations |
| Nathan only notified for escalations | REPT-06 | Multi-agent observation | Run ticks, verify Nathan's conversation queue is empty unless escalated |

---

## Validation Sign-Off

- [ ] All tasks have automated verify
- [ ] Sampling continuity
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
