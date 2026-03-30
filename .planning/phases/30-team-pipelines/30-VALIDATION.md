---
phase: 30
slug: team-pipelines
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-30
---

# Phase 30 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Manual SBCL REPL verification (no automated test framework in ghost codebase) |
| **Config file** | None -- SBCL load-and-run |
| **Quick run command** | `pm2 restart noosphere-ghosts && sleep 5 && pm2 logs noosphere-ghosts --lines 50` |
| **Full suite command** | Trigger a pipeline task, observe advance-pipeline log output |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** `pm2 logs noosphere-ghosts --lines 20` (check for load errors)
- **After every plan wave:** Restart ghosts, verify pipeline loading log messages
- **Before `/gsd:verify-work`:** At least one pipeline task advances through DB-loaded definitions
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 30-01-01 | 01 | 1 | PIPE-01 | smoke | `psql -c "SELECT count(*) FROM area_content WHERE content_type = 'pipeline'"` | N/A (SQL) | pending |
| 30-01-02 | 01 | 1 | PIPE-02 | smoke | `psql -c "SELECT metadata->'stages' FROM area_content WHERE content_type = 'pipeline' LIMIT 1"` | N/A (SQL) | pending |
| 30-02-01 | 02 | 2 | PIPE-03 | manual | Restart ghosts, observe `[pipeline]` log messages showing DB-loaded data | N/A | pending |
| 30-02-02 | 02 | 2 | PIPE-04 | manual | Query tasks with active pipeline stages, verify stage/goal_id populated | N/A | pending |

*Status: pending / green / red / flaky*

---

## Wave 0 Requirements

*Existing infrastructure covers all phase requirements. No automated test framework exists in the ghost codebase; verification is via REPL and log inspection (established pattern for all prior phases).*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Tick engine loads pipeline from DB | PIPE-03 | Runtime behavior requires live system | Restart ghosts, check log for DB-loaded pipeline data |
| Pipeline advance uses DB definitions | PIPE-04 | Requires active pipeline task | Trigger pipeline task, observe stage progression in logs |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
