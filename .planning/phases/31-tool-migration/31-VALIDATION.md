---
phase: 31
slug: tool-migration
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-30
---

# Phase 31 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Manual Lisp REPL + psql verification |
| **Config file** | None -- SBCL REPL for Lisp verification |
| **Quick run command** | `sudo -u postgres psql -d master_chronicle -c "SELECT count(*) FROM area_content WHERE content_type = 'tool'"` |
| **Full suite command** | Start noosphere-ghosts, observe tool execution in tick logs |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** psql count of tool definitions + grep YAML capabilities
- **After every plan wave:** Full tick cycle with tool invocation attempt
- **Before `/gsd:verify-work`:** tool-registry.json deleted, all tools in area_content, all 9 YAMLs updated
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 31-01-01 | 01 | 1 | TOOL-01 | manual | `grep -c "!\[" /opt/project-noosphere-ghosts/config/agents/*.yaml` | N/A | pending |
| 31-01-02 | 01 | 1 | TOOL-02 | smoke | `psql -c "SELECT count(*) FROM area_content WHERE content_type = 'tool'"` | N/A | pending |
| 31-02-01 | 02 | 2 | TOOL-02 | smoke | Start ghosts, send tool-invoking message, check logs | N/A | pending |
| 31-02-02 | 02 | 2 | TOOL-03, TOOL-04 | unit | `test ! -f /opt/project-noosphere-ghosts/config/tool-registry.json` | N/A | pending |

*Status: pending / green / red / flaky*

---

## Wave 0 Requirements

*Existing infrastructure covers all phase requirements. No automated test framework for Lisp runtime; verification is via psql queries and SBCL log inspection (established pattern).*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Resolver invokes Python script | TOOL-02 | Runtime behavior requires live tick | Send tool-invoking message, check PM2 logs |
| Results appear in conversations | TOOL-04 | Requires end-to-end tick cycle | Check conversations table after tool execution |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
