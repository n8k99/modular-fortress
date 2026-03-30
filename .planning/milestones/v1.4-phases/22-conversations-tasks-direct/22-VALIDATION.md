---
phase: 22
slug: conversations-tasks-direct
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-29
---

# Phase 22 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | SBCL assert + bash comparison scripts (reuse Phase 21 pattern) |
| **Config file** | none — Wave 0 extends existing test harness |
| **Quick run command** | `sbcl --script /opt/project-noosphere-ghosts/tests/test-db-conversations.lisp` |
| **Full suite command** | `bash /opt/project-noosphere-ghosts/tests/run-all-tests.sh` |
| **Estimated runtime** | ~8 seconds |

---

## Sampling Rate

- **After every task commit:** Run quick test for the area modified
- **After every plan wave:** Run full suite
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 8 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 22-01-01 | 01 | 1 | DB-03 | integration | `sbcl --script tests/test-db-conversations.lisp` | ❌ W0 | ⬜ pending |
| 22-01-02 | 01 | 1 | DB-04 | integration | `sbcl --script tests/test-db-tasks.lisp` | ❌ W0 | ⬜ pending |
| 22-02-01 | 02 | 2 | DB-03, DB-04 | integration | `bash tests/run-all-tests.sh` | ❌ W0 | ⬜ pending |
| 22-03-01 | 03 | 3 | DB-03, DB-04 | comparison | `bash tests/verify-zero-http.sh` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `tests/test-db-conversations.lisp` — INSERT, mark-read, SELECT conversation tests
- [ ] `tests/test-db-tasks.lisp` — Task CRUD, blocked_by management tests
- [ ] `tests/verify-zero-http.sh` — grep for api-get/api-post/api-patch in runtime/*.lisp (must be zero)

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Live tick cycle with SQL conversations | DB-03 | Requires running tick engine | Start tick engine, verify ghost posts conversation via SQL |
| Task unblock cascade in live tick | DB-04 | Requires multi-agent tick | Complete a blocking task, verify dependent task unblocks |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 8s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
