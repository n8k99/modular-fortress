---
phase: 1
slug: schema-dispatch
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-26
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | pytest (gotcha-workspace/.venv) + psql assertions |
| **Config file** | gotcha-workspace/tools/gsd/dispatch_to_db.py |
| **Quick run command** | `source gotcha-workspace/.venv/bin/activate && python gotcha-workspace/tools/gsd/dispatch_to_db.py --status` |
| **Full suite command** | `source gotcha-workspace/.venv/bin/activate && python -m pytest gotcha-workspace/tools/gsd/test_dispatch.py -v` |
| **Estimated runtime** | ~5 seconds |

---

## Sampling Rate

- **After every task commit:** Run quick status check
- **After every plan wave:** Run full test suite
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 5 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 01-01-01 | 01 | 1 | SCHM-01 | integration | `python -m pytest test_dispatch.py::test_project_name_from_heading -v` | ❌ W0 | ⬜ pending |
| 01-01-02 | 01 | 1 | SCHM-02 | integration | `python -m pytest test_dispatch.py::test_project_dispatch -v` | ❌ W0 | ⬜ pending |
| 01-01-03 | 01 | 1 | SCHM-04 | integration | `python -m pytest test_dispatch.py::test_department_routing -v` | ❌ W0 | ⬜ pending |
| 01-02-01 | 02 | 1 | SCHM-03 | integration | `python -m pytest test_dispatch.py::test_hierarchical_tasks -v` | ❌ W0 | ⬜ pending |
| 01-02-02 | 02 | 2 | SCHM-05 | integration | `python -m pytest test_dispatch.py::test_status_report -v` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `gotcha-workspace/tools/gsd/test_dispatch.py` — integration tests for all SCHM requirements
- [ ] Test fixtures: minimal .planning/ directory with PROJECT.md + PLAN.md files

*Test infrastructure uses existing gotcha-workspace/.venv with psycopg2-binary already installed.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Live project dispatch | SCHM-02 | Dispatches real Noosphere project to production DB | Run `dispatch_to_db.py --project --owner eliana` and verify in psql |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 5s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
