---
phase: 2
slug: perception-pipeline
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-26
---

# Phase 2 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test (dpn-api) + curl integration tests |
| **Config file** | /opt/dpn-api/Cargo.toml |
| **Quick run command** | `curl -s -H "X-API-Key: dpn-nova-2026" http://127.0.0.1:8080/api/perception/eliana \| jq '.tasks[0] \| keys'` |
| **Full suite command** | `cd /opt/dpn-api && cargo test perception 2>&1` |
| **Estimated runtime** | ~10 seconds |

---

## Sampling Rate

- **After every task commit:** Run quick curl check
- **After every plan wave:** Run full cargo test suite
- **Before `/gsd:verify-work`:** Full suite must pass
- **Max feedback latency:** 10 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 02-01-01 | 01 | 1 | PERC-01,PERC-03 | integration | `curl perception/eliana \| jq '.tasks[] \| .project_id'` | ✅ endpoint exists | ⬜ pending |
| 02-01-02 | 01 | 1 | PERC-05 | integration | `curl perception/eliana \| jq '.tasks[] \| .scheduled_at'` | ✅ endpoint exists | ⬜ pending |
| 02-02-01 | 02 | 2 | PERC-02 | integration | `curl perception/eliana \| jq '.projects[] \| .goals'` | ✅ endpoint exists | ⬜ pending |
| 02-02-02 | 02 | 2 | PERC-04 | e2e | `dispatch + perception + check tick urgency` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] Dispatch Noosphere Dispatch Pipeline project via dispatch_to_db.py (already done in Phase 1 tests)
- [ ] Verify dpn-api is running on port 8080

*Existing infrastructure covers most phase requirements.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Urgency boost in tick engine | PERC-04 | Requires running tick engine with live perception | Start ghosts, dispatch project, check tick log for +15 boost |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 10s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
