---
phase: 21
slug: direct-postgresql-foundation
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-29
---

# Phase 21 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | SBCL assert + bash comparison scripts (no formal test framework in AF64) |
| **Config file** | none — Wave 0 creates test harness |
| **Quick run command** | `sbcl --script /opt/project-noosphere-ghosts/tests/test-pg-client.lisp` |
| **Full suite command** | `bash /opt/project-noosphere-ghosts/tests/run-all-tests.sh` |
| **Estimated runtime** | ~5 seconds |

---

## Sampling Rate

- **After every task commit:** Run `sbcl --script /opt/project-noosphere-ghosts/tests/test-pg-client.lisp`
- **After every plan wave:** Run `bash /opt/project-noosphere-ghosts/tests/run-all-tests.sh`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 5 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 21-01-01 | 01 | 0 | DB-01 | integration | `sbcl --script tests/test-pg-client.lisp` | ❌ W0 | ⬜ pending |
| 21-02-01 | 02 | 1 | DB-01 | integration | `sbcl --script tests/test-pg-client.lisp` | ❌ W0 | ⬜ pending |
| 21-03-01 | 03 | 2 | DB-02 | comparison | `bash tests/compare-perception.sh` | ❌ W0 | ⬜ pending |
| 21-04-01 | 04 | 3 | DB-02 | integration | `bash tests/run-all-tests.sh` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `tests/test-pg-client.lisp` — connection, query, result parsing tests
- [ ] `tests/compare-perception.sh` — SQL perception vs HTTP perception JSON diff
- [ ] `tests/run-all-tests.sh` — runner script for full suite

*Wave 0 must create test infrastructure since AF64 has no existing test files.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Tick interval not exceeded | DB-02 | Requires live tick engine timing | Start tick engine, observe 3 consecutive ticks complete within interval |
| Connection recovery after PG restart | DB-01 | Requires service disruption | Restart PostgreSQL, verify next tick reconnects |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 5s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
