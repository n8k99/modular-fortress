---
phase: 15
slug: financial-pipeline
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-28
---

# Phase 15 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Shell scripts + curl + psql (infrastructure phase) |
| **Config file** | none — validation via DB queries, tool registry, and Lisp compilation |
| **Quick run command** | `grep -c "trading_briefing" /opt/project-noosphere-ghosts/config/tool-registry.json` |
| **Full suite command** | `bash -c "cd /opt/project-noosphere-ghosts && sbcl --noinform --non-interactive --load lisp/packages.lisp --eval '(format t \"LOAD OK~%\")' --quit 2>&1 \| tail -1"` |
| **Estimated runtime** | ~5 seconds |

---

## Sampling Rate

- **After every task commit:** Verify affected files compile/parse correctly
- **After every plan wave:** Full SBCL load + tool registry + schedule validation
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 5 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 15-01-01 | 01 | 1 | FIN-01,OPS-05 | integration | `grep + psql check` | N/A | pending |
| 15-01-02 | 01 | 1 | FIN-01,FIN-02 | integration | `SBCL load + grep mapping` | N/A | pending |

---

## Wave 0 Requirements

*Existing infrastructure covers all phase requirements.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Kathryn runs Tokyo briefing on schedule | FIN-01 | Requires live tick at matching time | Set 1-min schedule, start ghosts, observe |
| Each session produces structured output | FIN-02 | Requires market data APIs to be live | Verify after first scheduled run |
| Calendar sync runs on schedule | OPS-05 | Requires ForexFactory availability | Verify after 10:00 UTC fire |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 5s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
