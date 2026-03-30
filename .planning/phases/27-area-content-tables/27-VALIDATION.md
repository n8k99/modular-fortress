---
phase: 27
slug: area-content-tables
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-30
---

# Phase 27 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | SBCL ASDF load + PostgreSQL SQL verification |
| **Config file** | /opt/project-noosphere-ghosts/af64.asd |
| **Quick run command** | `cd /opt/project-noosphere-ghosts && sbcl --non-interactive --eval '(asdf:load-system :af64)' --quit` |
| **Full suite command** | SBCL load + SQL count verification + resolver test |
| **Estimated runtime** | ~60 seconds |

---

## Sampling Rate

- **After every task commit:** Run SBCL load test
- **After every plan wave:** Run SQL verification queries
- **Before `/gsd:verify-work`:** Full suite (load + SQL + resolver) must be green
- **Max feedback latency:** 60 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 27-01-01 | 01 | 1 | AREA-01 | sql | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -c "\d area_content"` | N/A | ⬜ pending |
| 27-01-02 | 01 | 1 | AREA-02 | sql | `PGPASSWORD=chronicle2026 psql ... -c "SELECT count(*) FROM area_content WHERE area_id = 1"` | N/A | ⬜ pending |
| 27-02-01 | 02 | 2 | AREA-03 | load+manual | `sbcl --non-interactive --eval '(asdf:load-system :af64)' --quit` | N/A | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Existing infrastructure covers all phase requirements. No new test framework needed.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Resolver returns area content for {em.content} | AREA-03 | Requires running Innate expression in SBCL REPL | Load af64, call (resolve-reference :bundle "em.content" ...), verify returns plist with area_content data |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
