---
phase: 28
slug: ghost-capabilities
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-30
---

# Phase 28 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Manual SBCL REPL validation (no automated test framework in noosphere-ghosts) |
| **Config file** | None — project uses manual testing |
| **Quick run command** | `cd /opt/project-noosphere-ghosts && sbcl --non-interactive --eval '(asdf:load-system :af64)' --quit` |
| **Full suite command** | `pm2 restart noosphere-ghosts && sleep 60 && pm2 logs noosphere-ghosts --lines 100 --nostream` |
| **Estimated runtime** | ~45 seconds (load) + ~90 seconds (tick cycle) |

---

## Sampling Rate

- **After every task commit:** Run SBCL load test
- **After every plan wave:** Restart PM2 and observe 2-3 tick cycles
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 90 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 28-01-01 | 01 | 1 | CAP-01, CAP-02, CAP-07 | load | `sbcl --non-interactive --eval '(asdf:load-system :af64)' --quit` | N/A | ⬜ pending |
| 28-01-02 | 01 | 1 | CAP-01 | file check | `ls /opt/project-noosphere-ghosts/config/agents/*.yaml \| wc -l` (expect 9) | N/A | ⬜ pending |
| 28-02-01 | 02 | 2 | CAP-02, CAP-03 | load + grep | `sbcl ... \| grep format-capabilities-for-prompt` | N/A | ⬜ pending |
| 28-03-01 | 03 | 3 | CAP-04, CAP-05, CAP-06, CAP-07 | load | `sbcl --non-interactive --eval '(asdf:load-system :af64)' --quit` | N/A | ⬜ pending |
| 28-03-02 | 03 | 3 | CAP-04, CAP-05 | integration | `pm2 restart noosphere-ghosts && sleep 60 && pm2 logs --nostream --lines 100` | N/A | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] No automated test suite for ghost-capabilities.lisp — verify via REPL
- [ ] yaml.lisp parser needs REPL-level verification against sample YAML files
- [ ] Verify YAML file permissions allow SBCL process to read/write config/agents/

*Existing infrastructure (SBCL ASDF load + PM2 logs) covers compile-time and runtime verification.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Ghost adds responsibility via cognition output | CAP-04 | Requires LLM to generate mutation block | Send message to ghost requesting new capability, verify YAML updated |
| Executive modifies subordinate capability | CAP-06 | Requires executive cognition cycle | Send message to executive targeting subordinate, verify subordinate YAML |
| Capabilities appear in LLM prompt | CAP-03 | Requires inspecting cognition job prompt | Check tick logs for InnateScipt expressions in system prompt |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 90s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
