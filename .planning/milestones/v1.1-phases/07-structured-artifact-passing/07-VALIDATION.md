---
phase: 7
slug: structured-artifact-passing
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-03-26
---

# Phase 7 -- Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | bash + psql (DB tests) + curl (API tests) + grep (code structure) |
| **Config file** | none -- inline commands per task |
| **Quick run command** | Run the automated verify command from the current task |
| **Full suite command** | Run all per-task automated commands sequentially |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run that task's automated verify command
- **After every plan wave:** Run all verify commands for the wave's tasks
- **Before `/gsd:verify-work`:** All per-task verify commands must pass
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | Status |
|---------|------|------|-------------|-----------|-------------------|--------|
| 07-01-01 | 01 | 1 | ART-01 | integration | `psql -U chronicle -d master_chronicle -c "SELECT data_type FROM information_schema.columns WHERE table_name='tasks' AND column_name='stage_notes';" \| grep -q jsonb` | pending |
| 07-01-02 | 01 | 1 | ART-01 | integration | `cd /opt/dpn-api && cargo build 2>&1 \| tail -1 \| grep -q "Finished" && curl -s http://localhost:8080/api/af64/tasks?limit=1 \| jq -e '.[0]' > /dev/null` | pending |
| 07-02-01 | 02 | 2 | ART-01, ART-03 | structural | `cd /opt/project-noosphere-ghosts && grep -c "validate-artifact-base" lisp/runtime/action-executor.lisp \| grep -q "[1-9]" && grep -c "build-stage-artifact" lisp/runtime/action-executor.lisp \| grep -q "[1-9]"` | pending |
| 07-02-02 | 02 | 2 | ART-01 (D-07) | structural | `cd /opt/project-noosphere-ghosts && grep -c "persist-pipeline-deliverable" lisp/runtime/action-executor.lisp \| grep -q "[1-9]" && grep -c "detect-pipeline-type" lisp/runtime/action-executor.lisp \| grep -q "[1-9]"` | pending |
| 07-03-01 | 03 | 2 | ART-02 | structural | `cd /opt/project-noosphere-ghosts && grep -c "load-predecessor-stage-output" lisp/runtime/action-planner.lisp \| grep -q "[1-9]" && grep -c "SCHEMA-VERSION" lisp/runtime/action-planner.lisp \| grep -q "[1-9]"` | pending |

*Status: pending / green / red / flaky*

---

## Wave 0 Requirements

None -- all verification uses inline psql, curl, and grep commands. No external test script needed.

*Existing infrastructure (psql, curl, bash, grep) covers all phase requirements.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Pipeline handoff with structured data | ART-02 | Requires running full pipeline tick with live agents | Start ghost tick, trigger pipeline advancement, verify next agent prompt includes structured output |
| Final deliverable persisted to DB table | ART-01 (D-07) | Requires a pipeline to run to completion | After a full pipeline completes, check documents/vault_notes tables for the deliverable |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 30s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
