---
phase: 13
slug: operations-pipeline
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-28
---

# Phase 13 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Shell scripts + curl + psql (infrastructure phase — no unit test framework) |
| **Config file** | none — validation via API calls, DB queries, and process checks |
| **Quick run command** | `curl -s -H "X-API-Key: dpn-nova-2026" http://localhost:8080/api/perception/nova?tier=prime \| python3 -c "import sys,json; print(json.dumps(json.load(sys.stdin).get('projects',[]),indent=2))"` |
| **Full suite command** | `bash .planning/phases/13-operations-pipeline/validate-ops.sh` |
| **Estimated runtime** | ~10 seconds |

---

## Sampling Rate

- **After every task commit:** Verify affected API endpoints respond correctly
- **After every plan wave:** Full perception + tool registry + schedule validation
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 10 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 13-01-01 | 01 | 1 | OPS-01 | integration | `curl + psql tool registry check` | N/A | pending |
| 13-01-02 | 01 | 1 | OPS-02,OPS-03 | integration | `psql tool registry + schedule check` | N/A | pending |
| 13-02-01 | 02 | 2 | OPS-01,OPS-02,OPS-03,OPS-04 | integration | `perception API + process test` | N/A | pending |

*Status: pending / green / red / flaky*

---

## Wave 0 Requirements

*Existing infrastructure covers all phase requirements — validation is via API calls, DB queries, and Lisp compilation checks.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Nova runs health check on schedule fire | OPS-01 | Requires live ghost tick with matching cron | Set 1-min schedule, start ghosts, observe tick log |
| Nightly synthesis produces vault note | OPS-02 | Requires daily note content in DB | Verify after first scheduled run |
| Podcast watcher detects new episode | OPS-03 | Requires RSS feed to have new content | Check last run output in conversations |
| Weekly finalization cascades to monthly | OPS-04 | Requires weekly boundary passage | Verify after Saturday schedule fire |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 10s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
