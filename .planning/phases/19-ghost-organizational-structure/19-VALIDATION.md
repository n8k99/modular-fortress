---
phase: 19
slug: ghost-organizational-structure
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-28
---

# Phase 19 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | psql assertions + cargo build |
| **Config file** | dpn-core/Cargo.toml |
| **Quick run command** | `cd /root/dpn-core && cargo test` |
| **Full suite command** | `cd /root/dpn-core && cargo test && cd /opt/dpn-api && cargo build` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** psql verification of new tables/data
- **After every plan wave:** Full suite
- **Before `/gsd:verify-work`:** All success criteria verified
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 19-01-01 | 01 | 1 | ORG-01,02,03,04 | integration | `psql -c "SELECT * FROM teams"` | ❌ W0 | ⬜ pending |
| 19-02-01 | 02 | 2 | EXPANDED | integration | `psql -c "SELECT * FROM routines"` | ❌ W0 | ⬜ pending |
| 19-02-02 | 02 | 2 | EXPANDED | integration | `psql document YAML check` | ❌ W0 | ⬜ pending |

---

## Wave 0 Requirements

- [ ] SQL migration with new tables + backfill
- [ ] Document update verification queries

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| EM Staff YAML correct | EXPANDED | Content verification | Spot-check 3 documents |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify
- [ ] Sampling continuity
- [ ] Wave 0 covers all MISSING references
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
