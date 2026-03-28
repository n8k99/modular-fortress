---
phase: 12
slug: standing-orders
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-28
---

# Phase 12 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | curl + jq (API E2E) + SBCL REPL (Lisp verification) |
| **Config file** | none — uses live dpn-api on localhost:8080 |
| **Quick run command** | `curl -s -H "X-API-Key: dpn-nova-2026" "http://localhost:8080/api/perception/nova?tier=prime" \| python3 -c "import sys,json; d=json.load(sys.stdin); print('projects:', len(d.get('projects',[])))"` |
| **Full suite command** | `bash .planning/phases/12-standing-orders/test-standing-orders.sh` |
| **Estimated runtime** | ~5 seconds |

---

## Sampling Rate

- **After every task commit:** Run quick perception check
- **After every plan wave:** Run full suite
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 5 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | Status |
|---------|------|------|-------------|-----------|-------------------|--------|
| 12-01-01 | 01 | 1 | STAND-01 | E2E | DB column + API PATCH verify | pending |
| 12-01-02 | 01 | 1 | STAND-01 | E2E | cron-matcher unit test | pending |
| 12-02-01 | 02 | 2 | STAND-02 | E2E | tick engine schedule boost verify | pending |
| 12-02-02 | 02 | 2 | STAND-03 | E2E | conversation output attribution check | pending |

---

## Wave 0 Requirements

Existing infrastructure covers all phase requirements.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Schedule fires at correct time | STAND-02 | Requires waiting for cron match | Set a 1-min schedule, wait for tick, verify cognition job created |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 5s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
