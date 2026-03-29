---
phase: 20
slug: nexus-import-temporal-compression
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-29
---

# Phase 20 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | psql queries + Python scripts (gotcha-workspace/.venv) |
| **Config file** | gotcha-workspace/tools/_config.py |
| **Quick run command** | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -c "SELECT count(*) FROM archives WHERE source_type='chatgpt_import';"` |
| **Full suite command** | `python3 gotcha-workspace/tools/nexus-import/verify.py` (created in Wave 0) |
| **Estimated runtime** | ~10 seconds |

---

## Sampling Rate

- **After every task commit:** Run quick count query
- **After every plan wave:** Run full verification script
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 20-01-01 | 01 | 1 | IMPORT-01 | query | `psql ... "SELECT count(*) FROM archives WHERE source_type='chatgpt_import';"` | N/A | ⬜ pending |
| 20-02-01 | 02 | 2 | IMPORT-02 | query | `psql ... "SELECT count(*), min(period_start), max(period_end) FROM archives WHERE source_type='chatgpt_import';"` | N/A | ⬜ pending |
| 20-03-01 | 03 | 3 | IMPORT-03 | query | `psql ... "SELECT compression_tier, count(*) FROM memories WHERE path LIKE '%nexus%' GROUP BY compression_tier;"` | N/A | ⬜ pending |
| 20-04-01 | 04 | 4 | IMPORT-04 | query | `psql ... "SELECT id, nova_memories IS NOT NULL, lrm_memories IS NOT NULL FROM memories WHERE path LIKE '%nexus%' AND compression_tier='monthly' LIMIT 5;"` | N/A | ⬜ pending |
| 20-05-01 | 05 | 5 | IMPORT-05 | query | `psql ... "SELECT count(*) FROM memories WHERE content LIKE '%## Nexus Imports%';"` | N/A | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `gotcha-workspace/tools/nexus-import/verify.py` — verification script checking all 5 requirements
- [ ] Dedup report output format defined

*Existing DB infrastructure covers all phase requirements.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| LLM summary quality | IMPORT-03/04 | Subjective assessment | Spot-check 3 monthly summaries and 2 ghost memory entries for coherence |
| Note content integrity | IMPORT-05 | Must verify no corruption of existing content | Diff 5 random daily notes before/after linking |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
