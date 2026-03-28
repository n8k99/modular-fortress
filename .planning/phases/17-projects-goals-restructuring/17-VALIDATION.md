---
phase: 17
slug: projects-goals-restructuring
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-28
---

# Phase 17 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | psql assertions + cargo test (dpn-core) + cargo build (dpn-api) |
| **Config file** | dpn-core/Cargo.toml |
| **Quick run command** | `cd /root/dpn-core && cargo test` |
| **Full suite command** | `cd /root/dpn-core && cargo test && cd /opt/dpn-api && cargo build` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cd /root/dpn-core && cargo test`
- **After every plan wave:** Run full suite + psql verification queries
- **Before `/gsd:verify-work`:** Full suite must be green + all 4 success criteria verified
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 17-01-01 | 01 | 1 | SCHEMA-05 | integration | `psql -c "SELECT lifestage FROM projects LIMIT 1"` | ❌ W0 | ⬜ pending |
| 17-01-02 | 01 | 1 | SCHEMA-06 | integration | `psql -c "SELECT project_id FROM goals LIMIT 1"` | ❌ W0 | ⬜ pending |
| 17-01-03 | 01 | 1 | SCHEMA-07 | integration | `psql -c "SELECT area_id FROM projects LIMIT 1"` | ❌ W0 | ⬜ pending |
| 17-02-01 | 02 | 2 | SCHEMA-05 | unit | `cd /root/dpn-core && cargo test` | ❌ W0 | ⬜ pending |
| 17-03-01 | 03 | 3 | API-05 | endpoint | `curl localhost:8080/api/perception/nova` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] SQL migration file with ALTER TABLE + triggers + backfill
- [ ] Trigger test SQL (verify forward-only lifestage enforcement)

*Existing Rust test infrastructure covers dpn-core module testing.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| dpn-api starts cleanly | All | Process startup | `pm2 restart dpn-api && pm2 logs dpn-api --lines 10` |
| Perception shows lifestage | API-05 | Live endpoint | `curl with auth token, verify lifestage in JSON` |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
