---
phase: 3
slug: executive-cognition
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-26
---

# Phase 3 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Manual Lisp REPL + curl API tests + DB queries |
| **Config file** | /opt/project-noosphere-ghosts/config/af64.env |
| **Quick run command** | `curl -s -H "X-API-Key: dpn-nova-2026" http://127.0.0.1:8080/api/af64/tasks?source=ghost \| jq length` |
| **Full suite command** | `bash .planning/phases/03-executive-cognition/test_exec_cognition.sh` |
| **Estimated runtime** | ~15 seconds |

---

## Sampling Rate

- **After every task commit:** Run quick API check
- **After every plan wave:** Run full test suite
- **Before `/gsd:verify-work`:** Full suite must pass
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 03-01-01 | 01 | 1 | EXEC-04 | integration | `curl POST /api/af64/tasks + verify task_id auto-generated` | ✅ endpoint exists | ⬜ pending |
| 03-01-02 | 01 | 1 | EXEC-01 | integration | `grep parse-create-task-lines action-executor.lisp` | ❌ W0 | ⬜ pending |
| 03-02-01 | 02 | 2 | EXEC-01,EXEC-02 | integration | `grep "wave\|must.haves" action-planner.lisp` | ✅ file exists | ⬜ pending |
| 03-03-01 | 03 | 3 | EXEC-03,EXEC-05 | e2e | `pm2 start noosphere-ghosts + check tick logs` | ✅ process exists | ⬜ pending |

---

## Wave 0 Requirements

- [ ] Verify dpn-api running on port 8080
- [ ] Verify noosphere-ghosts process exists in PM2

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Executive produces structured decomposition via LLM | EXEC-01 | Requires live tick with LLM cognition | Start ghosts, wait for exec tick, check conversation output |
| Executive monitors and adjusts across ticks | EXEC-05 | Multi-tick observation | Run 3+ ticks, verify exec re-reviews when staff complete tasks |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity
- [ ] Wave 0 covers all MISSING references
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
