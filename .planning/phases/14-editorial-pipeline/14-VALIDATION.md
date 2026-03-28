---
phase: 14
slug: editorial-pipeline
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-28
---

# Phase 14 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Shell scripts + curl + psql (infrastructure phase) |
| **Config file** | none — validation via API calls, DB queries, and Lisp compilation |
| **Quick run command** | `grep -c "editorial_nightly" /opt/project-noosphere-ghosts/config/tool-registry.json` |
| **Full suite command** | `bash -c "cd /opt/project-noosphere-ghosts && sbcl --noinform --non-interactive --load lisp/packages.lisp --eval '(format t \"LOAD OK~%\")' --quit 2>&1 \| tail -1"` |
| **Estimated runtime** | ~5 seconds |

---

## Sampling Rate

- **After every task commit:** Verify affected files compile/parse correctly
- **After every plan wave:** Full SBCL load test + tool registry validation
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 5 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 14-01-01 | 01 | 1 | EDIT-01 | integration | `grep + psql tool check` | N/A | pending |
| 14-01-02 | 01 | 1 | EDIT-01,EDIT-02 | integration | `SBCL load + grep mapping` | N/A | pending |

*Status: pending / green / red / flaky*

---

## Wave 0 Requirements

*Existing infrastructure covers all phase requirements.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Sylvia runs editorial on schedule fire | EDIT-01 | Requires live ghost tick with matching cron | Set 1-min schedule, start ghosts, observe |
| Editorial output follows Thought Police format | EDIT-02 | Requires Nathan's reader comments to exist | Verify after first scheduled run |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 5s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
