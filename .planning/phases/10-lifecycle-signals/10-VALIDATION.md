---
phase: 10
slug: lifecycle-signals
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-03-27
---

# Phase 10 -- Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | bash + grep (code structure) + curl (API tests) |
| **Config file** | none -- inline commands per task |
| **Quick run command** | Run the automated verify command from the current task |
| **Full suite command** | Run all per-task automated commands sequentially |
| **Estimated runtime** | ~20 seconds |

---

## Sampling Rate

- **After every task commit:** Run that task's automated verify command
- **After every plan wave:** Run all verify commands for the wave's tasks
- **Before `/gsd:verify-work`:** All per-task verify commands must pass
- **Max feedback latency:** 20 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | Status |
|---------|------|------|-------------|-----------|-------------------|--------|
| 10-01-01 | 01 | 1 | LIFE-01, LIFE-03 | structural | `cd /opt/project-noosphere-ghosts && grep -c "lifecycle-state" lisp/runtime/tick-engine.lisp \| grep -q "[1-9]" && grep -c "idle-transition" lisp/runtime/energy.lisp \| grep -q "[1-9]"` | pending |
| 10-02-01 | 02 | 2 | LIFE-02 | structural | `cd /opt/project-noosphere-ghosts && grep -c "lifecycle-state\|IDLE\|task-count" lisp/runtime/action-planner.lisp \| grep -q "[1-9]"` | pending |
| 10-02-02 | 02 | 2 | LIFE-01 | integration | `cd /opt/dpn-api && cargo build 2>&1 \| tail -1 \| grep -q "Finished"` | pending |

*Status: pending / green / red / flaky*

---

## Wave 0 Requirements

None -- all verification uses inline grep, curl, and cargo commands. No external test script needed.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Staff ghost completes last task and becomes IDLE | LIFE-01 | Requires live tick cycle | Start ghosts, assign single task, observe idle transition after completion |
| Executive sees idle agents in review prompt | LIFE-02 | Requires live executive cognition | Trigger executive project review, check prompt includes availability info |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 20s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
