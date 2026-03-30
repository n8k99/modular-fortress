---
phase: 26
slug: runtime-stability
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-30
---

# Phase 26 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | SBCL ASDF load + live PM2 tick cycle |
| **Config file** | /opt/project-noosphere-ghosts/af64.asd |
| **Quick run command** | `cd /opt/project-noosphere-ghosts && sbcl --non-interactive --eval '(asdf:load-system :af64)' --quit` |
| **Full suite command** | `pm2 restart noosphere-ghosts && sleep 30 && pm2 logs noosphere-ghosts --lines 50 --nostream` |
| **Estimated runtime** | ~45 seconds (load) + ~60 seconds (tick) |

---

## Sampling Rate

- **After every task commit:** Run SBCL load test
- **After every plan wave:** Run full PM2 tick cycle verification
- **Before `/gsd:verify-work`:** Full suite must be green (clean load + clean tick)
- **Max feedback latency:** 60 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 26-01-01 | 01 | 1 | STAB-01 | load | `sbcl --non-interactive --eval '(asdf:load-system :af64)' --quit` | N/A | ⬜ pending |
| 26-02-01 | 02 | 1 | STAB-02 | load | `sbcl --non-interactive --eval '(asdf:load-system :af64)' --quit` | N/A | ⬜ pending |
| 26-02-02 | 02 | 2 | STAB-02 | integration | `pm2 restart noosphere-ghosts && sleep 30 && pm2 logs noosphere-ghosts --lines 50 --nostream` | N/A | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Existing infrastructure covers all phase requirements. No new test framework needed — verification is SBCL compile + live tick cycle.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Tick cycle completes without runtime errors | STAB-02 | Requires live DB + ghost perception | Restart PM2, watch logs for complete tick without errors |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
