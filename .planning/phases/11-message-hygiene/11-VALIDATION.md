---
phase: 11
slug: message-hygiene
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-27
---

# Phase 11 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | curl + jq (API E2E) + SBCL REPL (Lisp verification) |
| **Config file** | none — uses live dpn-api on localhost:8080 |
| **Quick run command** | `curl -s -H "X-API-Key: dpn-nova-2026" http://localhost:8080/api/perception/vincent?tier=working \| python3 -c "import sys,json; d=json.load(sys.stdin); print('msgs:', len(d.get('messages',[])))"` |
| **Full suite command** | `bash .planning/phases/11-message-hygiene/test-message-hygiene.sh` |
| **Estimated runtime** | ~5 seconds |

---

## Sampling Rate

- **After every task commit:** Run quick perception check
- **After every plan wave:** Run full suite
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 5 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 11-01-01 | 01 | 1 | FIX-01 | build | `cd /opt/dpn-api && cargo build --release 2>&1 \| tail -1` | N/A | pending |
| 11-01-02 | 01 | 1 | FIX-02 | E2E | `curl -s -X POST ... /api/conversations/mark-read` | N/A | pending |
| 11-01-03 | 01 | 1 | SPAM-01 | E2E | perception query returns 0 for read messages | N/A | pending |
| 11-02-01 | 02 | 2 | SPAM-02 | E2E | read_by array populated after ghost tick | N/A | pending |
| 11-02-02 | 02 | 2 | SPAM-03 | E2E | idle agent gets no cognition job | N/A | pending |

*Status: pending / green / red / flaky*

---

## Wave 0 Requirements

- Existing infrastructure covers all phase requirements. No new test framework needed.
- curl/jq scripts against live API are the established verification pattern from v1.0/v1.1.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Ghost stops spamming after restart | SPAM-03 | Requires running ghosts for multiple ticks | Start ghosts, observe 3+ ticks, verify no duplicate messages |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 5s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
