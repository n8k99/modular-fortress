---
phase: 6
slug: task-dependency-chains
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-26
---

# Phase 6 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | bash + psql (DB trigger tests) + curl (API tests) |
| **Config file** | none — inline test scripts |
| **Quick run command** | `bash .planning/phases/06-task-dependency-chains/test_deps_e2e.sh quick` |
| **Full suite command** | `bash .planning/phases/06-task-dependency-chains/test_deps_e2e.sh` |
| **Estimated runtime** | ~15 seconds |

---

## Sampling Rate

- **After every task commit:** Run quick test
- **After every plan wave:** Run full suite
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 06-01-01 | 01 | 1 | DEP-01 | integration | `psql -c "SELECT ... WHERE blocked_by @> ARRAY[N]"` | ❌ W0 | ⬜ pending |
| 06-01-02 | 01 | 1 | DEP-02 | integration | `psql -c "UPDATE tasks SET status='done' WHERE id=N; SELECT blocked_by FROM tasks WHERE id=M"` | ❌ W0 | ⬜ pending |
| 06-02-01 | 02 | 2 | DEP-03 | integration | `curl POST /api/af64/tasks with blocked_by field` | ❌ W0 | ⬜ pending |
| 06-02-02 | 02 | 2 | DEP-04 | integration | `python dispatch_to_db.py --dry-run; check blocked_by values` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `test_deps_e2e.sh` — E2E test script for dependency chain verification
- [ ] Test data setup: create parent/child tasks with blocked_by references

*Existing infrastructure (psql, curl, bash) covers all phase requirements.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Executive sees blocked tasks in project review | DEP-01 (extended) | Requires running tick engine with live agent | Start ghost tick, verify executive perception includes blocked task section |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
