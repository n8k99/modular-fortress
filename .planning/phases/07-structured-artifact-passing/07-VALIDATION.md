---
phase: 7
slug: structured-artifact-passing
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-26
---

# Phase 7 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | bash + psql (DB tests) + curl (API tests) + sbcl (Lisp validation) |
| **Config file** | none — inline test scripts |
| **Quick run command** | `bash .planning/phases/07-structured-artifact-passing/test_artifacts_e2e.sh quick` |
| **Full suite command** | `bash .planning/phases/07-structured-artifact-passing/test_artifacts_e2e.sh` |
| **Estimated runtime** | ~20 seconds |

---

## Sampling Rate

- **After every task commit:** Run quick test
- **After every plan wave:** Run full suite
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 20 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 07-01-01 | 01 | 1 | ART-01 | integration | `psql -c "SELECT data_type FROM information_schema.columns WHERE column_name='stage_notes'" \| grep jsonb` | ❌ W0 | ⬜ pending |
| 07-01-02 | 01 | 1 | ART-03 | integration | `grep -c "validate-structured-output" action-executor.lisp` | ❌ W0 | ⬜ pending |
| 07-02-01 | 02 | 2 | ART-02 | integration | `grep -c "stage-notes" action-planner.lisp` | ❌ W0 | ⬜ pending |
| 07-02-02 | 02 | 2 | ART-01 | integration | `curl -s /api/af64/tasks/N \| jq '.stage_notes.summary'` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `test_artifacts_e2e.sh` — E2E test script for artifact validation
- [ ] Test data: create task with structured stage_notes JSONB

*Existing infrastructure (psql, curl, bash) covers all phase requirements.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Pipeline handoff with structured data | ART-02 | Requires running full pipeline tick with live agents | Start ghost tick, trigger pipeline advancement, verify next agent prompt includes structured output |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 20s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
