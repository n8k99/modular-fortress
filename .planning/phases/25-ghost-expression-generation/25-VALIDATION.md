---
phase: 25
slug: ghost-expression-generation
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-30
---

# Phase 25 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | SBCL REPL + InnateScipt parser (tokenize/parse round-trip) |
| **Config file** | /opt/innatescript/run-tests.sh (175/176 tests) |
| **Quick run command** | `sbcl --load /opt/innatescript/run-tests.sh --quit 2>&1 | tail -5` |
| **Full suite command** | `sbcl --load /opt/innatescript/run-tests.sh --quit` |
| **Estimated runtime** | ~5 seconds |

---

## Sampling Rate

- **After every task commit:** Run `sbcl --load /opt/innatescript/run-tests.sh --quit 2>&1 | tail -5`
- **After every plan wave:** Run full InnateScipt test suite
- **Before `/gsd:verify-work`:** Full suite must be green + manual SBCL compilation test
- **Max feedback latency:** 10 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 25-01-01 | 01 | 1 | INNATE-03 | unit | `sbcl --eval '(load "innate-builder") (assert (string= (build-reference "nova") "@nova"))'` | ❌ W0 | ⬜ pending |
| 25-01-02 | 01 | 1 | INNATE-03 | integration | `psql -c "SELECT id FROM templates WHERE name='test-ghost-gen'" master_chronicle` | ✅ | ⬜ pending |
| 25-02-01 | 02 | 2 | INNATE-03 | compilation | `sbcl --load packages.lisp --eval '(quit)'` (no errors) | ✅ | ⬜ pending |
| 25-02-02 | 02 | 2 | INNATE-03 | e2e | SBCL compile + template round-trip (create → evaluate → verify) | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] Builder function validation tests (build-reference, build-commission, build-search produce valid parseable output)
- [ ] Template CRUD integration test (INSERT + SELECT round-trip)

*Existing InnateScipt parser tests cover syntax validation — new tests focus on generation + persistence.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| LLM produces valid innate_expressions JSON | INNATE-03 | Requires live LLM call | Run ghost tick with template-generation task, inspect cognition output |
| Ghost-created template evaluable by other ghosts | INNATE-03 | Cross-tick verification | Create template in tick N, verify evaluation in tick N+1 |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 10s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
