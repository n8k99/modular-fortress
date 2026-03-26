---
phase: 4
slug: tool-execution
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-26
---

# Phase 4 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | curl + jq + psql assertions against live API and DB |
| **Config file** | /opt/project-noosphere-ghosts/config/tool-registry.json |
| **Quick run command** | `curl -s -H "X-API-Key: dpn-nova-2026" http://127.0.0.1:8080/api/agents/eliana \| jq '.agent.tool_scope'` |
| **Full suite command** | `bash .planning/phases/04-tool-execution/test_tools_e2e.sh` |
| **Estimated runtime** | ~20 seconds |

---

## Sampling Rate

- **After every task commit:** Quick tool invocation check
- **After every plan wave:** Full E2E suite
- **Before `/gsd:verify-work`:** Full suite must pass
- **Max feedback latency:** 20 seconds

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Ghost invokes Claude Code CLI for code task | TOOL-01 | Requires live tick with LLM cognition | Start ghosts, assign code task, check stage_notes for claude output |
| Tool results visible to executive in stage_notes | TOOL-06 | Multi-agent observation | Assign task, staff executes, exec reviews stage_notes |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity
- [ ] Feedback latency < 20s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
