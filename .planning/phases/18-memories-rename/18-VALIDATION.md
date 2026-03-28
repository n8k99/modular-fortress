---
phase: 18
slug: memories-rename
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-28
---

# Phase 18 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | psql assertions + cargo test/build |
| **Config file** | dpn-core/Cargo.toml |
| **Quick run command** | `cd /root/dpn-core && cargo test` |
| **Full suite command** | `cd /root/dpn-core && cargo test && cd /opt/dpn-api && cargo build` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run quick run command
- **After every plan wave:** Run full suite + psql view verification
- **Before `/gsd:verify-work`:** Full suite green + all 4 success criteria verified
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 18-01-01 | 01 | 1 | MEM-01 | integration | `psql -c "SELECT * FROM memories LIMIT 1"` | ❌ W0 | ⬜ pending |
| 18-01-02 | 01 | 1 | MEM-02 | integration | `psql -c "SELECT compression_tier FROM memories LIMIT 1"` | ❌ W0 | ⬜ pending |
| 18-01-03 | 01 | 1 | MEM-04 | integration | `psql -c "SELECT * FROM departments"` | ❌ W0 | ⬜ pending |
| 18-02-01 | 02 | 2 | MEM-05,06 | unit | `cargo build + cargo test` | ❌ W0 | ⬜ pending |
| 18-03-01 | 03 | 3 | MEM-05 | endpoint | `pm2 restart dpn-api && curl /api/memories` | ❌ W0 | ⬜ pending |

---

## Wave 0 Requirements

- [ ] SQL migration with rename + view + INSTEAD OF triggers + compression columns + departments
- [ ] View bridge verification (INSERT/UPDATE/DELETE through vault_notes view)

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| dpn-api starts cleanly | All | Process startup | `pm2 restart dpn-api && pm2 logs dpn-api --lines 10` |
| Lisp perception works | MEM-01 | Live ghost tick | Check ghost tick after rename |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
