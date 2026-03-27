---
phase: 8
slug: decisions-brain
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-03-26
---

# Phase 8 -- Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | bash + psql (DB tests) + curl (API tests) + grep (code structure) |
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
| 08-01-01 | 01 | 1 | DEC-03 | integration | `curl -s http://localhost:8080/api/decisions \| jq -e '.[] \| .id' > /dev/null 2>&1 \|\| curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/api/decisions \| grep -q "200"` | pending |
| 08-01-02 | 01 | 1 | DEC-03 | integration | `cd /opt/dpn-api && cargo build 2>&1 \| tail -1 \| grep -q "Finished"` | pending |
| 08-02-01 | 02 | 2 | DEC-02 | structural | `cd /opt/project-noosphere-ghosts && grep -c "api-post.*decisions" lisp/runtime/action-executor.lisp \| grep -q "[1-9]"` | pending |
| 08-02-02 | 02 | 2 | DEC-01 | structural | `cd /opt/project-noosphere-ghosts && grep -c "decisions" lisp/runtime/action-planner.lisp \| grep -q "[1-9]"` | pending |

*Status: pending / green / red / flaky*

---

## Wave 0 Requirements

None -- all verification uses inline psql, curl, and grep commands. No external test script needed.

*Existing infrastructure (psql, curl, bash, grep) covers all phase requirements.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Executive reviews project and decisions appear in prompt | DEC-01 | Requires live tick with executive cognition | Start ghost tick, trigger executive project review, check LLM prompt includes decisions |
| Executive makes decision and it appears in DB | DEC-02 | Requires live LLM output with DECISION: keyword | Trigger executive cognition, check decisions table for new row |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 20s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
