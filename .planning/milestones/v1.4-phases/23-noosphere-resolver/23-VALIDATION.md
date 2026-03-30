---
phase: 23
slug: noosphere-resolver
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-29
---

# Phase 23 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | SBCL assert + Innate test runner (innatescript/run-tests.sh) |
| **Config file** | none — Wave 0 creates resolver-specific tests |
| **Quick run command** | `sbcl --script /opt/project-noosphere-ghosts/tests/test-noosphere-resolver.lisp` |
| **Full suite command** | `bash /opt/project-noosphere-ghosts/tests/run-all-tests.sh` |
| **Estimated runtime** | ~6 seconds |

---

## Sampling Rate

- **After every task commit:** Run quick resolver test
- **After every plan wave:** Run full suite
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 6 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 23-01-01 | 01 | 1 | INNATE-01 | integration | `sbcl --script tests/test-noosphere-resolver.lisp` | ❌ W0 | ⬜ pending |
| 23-02-01 | 02 | 2 | INNATE-01 | integration | `sbcl --script tests/test-noosphere-resolver.lisp` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `tests/test-noosphere-resolver.lisp` — resolve-reference, resolve-search, deliver-commission, resolve-wikilink tests against live DB

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Resolver in live tick with Innate expressions | INNATE-01 | Requires running tick engine + Innate evaluator | Start tick engine, send .dpn template with @references, verify resolution |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 6s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
