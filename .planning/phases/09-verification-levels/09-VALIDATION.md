---
phase: 9
slug: verification-levels
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-03-26
---

# Phase 9 -- Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | bash + grep (code structure) + psql (DB queries) |
| **Config file** | none -- inline commands per task |
| **Quick run command** | Run the automated verify command from the current task |
| **Full suite command** | Run all per-task automated commands sequentially |
| **Estimated runtime** | ~15 seconds |

---

## Sampling Rate

- **After every task commit:** Run that task's automated verify command
- **After every plan wave:** Run all verify commands for the wave's tasks
- **Before `/gsd:verify-work`:** All per-task verify commands must pass
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | Status |
|---------|------|------|-------------|-----------|-------------------|--------|
| 09-01-01 | 01 | 1 | VER-01, VER-03 | structural | `cd /opt/project-noosphere-ghosts && grep -c "extract-quality-assessment" lisp/runtime/action-executor.lisp \| grep -q "[1-9]" && grep -c "severity-level" lisp/runtime/action-executor.lisp \| grep -q "[1-9]"` | pending |
| 09-02-01 | 02 | 1 | VER-02 | structural | `cd /opt/project-noosphere-ghosts && grep -c "quality-issue-boost" lisp/runtime/tick-engine.lisp \| grep -q "[1-9]"` | pending |
| 09-02-02 | 02 | 1 | VER-02 | structural | `cd /opt/dpn-api && grep -c "critical_issues\|severity_level" src/handlers/af64_perception.rs \| grep -q "[1-9]"` | pending |

*Status: pending / green / red / flaky*

---

## Wave 0 Requirements

None -- all verification uses inline grep commands against source files. No external test script needed.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Staff ghost completes task with quality issues in output | VER-01 | Requires live tick with staff cognition | Start ghost tick, observe completion report includes severity classification |
| Executive perceives CRITICAL issue at elevated urgency | VER-02 | Requires live tick with executive perception | Check tick log for elevated urgency on executive with CRITICAL task |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 15s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
