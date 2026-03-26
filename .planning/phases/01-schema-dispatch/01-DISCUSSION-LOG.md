# Phase 1: Schema & Dispatch - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 01-schema-dispatch
**Areas discussed:** Project name extraction, Owner & department routing, Task granularity, End-to-end verification

---

## Project Name Extraction

| Option | Description | Selected |
|--------|-------------|----------|
| Parse # heading | Extract from first H1 heading in PROJECT.md — matches how GSD writes projects | ✓ |
| Add frontmatter to PROJECT.md | GSD template adds name: field in YAML frontmatter — dispatch reads it | |
| CLI flag --name | Explicit name passed on dispatch command — most control, more friction | |

**User's choice:** Parse # heading
**Notes:** PROJECT.md uses markdown headings, not YAML frontmatter. Script should parse H1.

---

## Owner & Department Routing

| Option | Description | Selected |
|--------|-------------|----------|
| Require --owner on dispatch | You always specify: dispatch --owner eliana. Department derived from owner's domain. | ✓ |
| Infer from project domain | Dispatch scans PROJECT.md content for keywords and auto-assigns | |
| Both — flag overrides inference | --owner takes priority, but if omitted, infer from content | |

**User's choice:** Require --owner on dispatch
**Notes:** Explicit control preferred. Department derived from executive roster mapping.

---

## Task Granularity

| Option | Description | Selected |
|--------|-------------|----------|
| One task per plan | Executives decompose further via LLM cognition in Phase 3 | |
| Tasks from plan sections | Parse must_haves/waves from each PLAN.md into individual tasks | |
| Hierarchical — plan + subtasks | One parent task per plan, subtasks for each must_have. Uses parent_id FK. | ✓ |

**User's choice:** Hierarchical — plan + subtasks
**Notes:** Gives executives pre-decomposed work. Subtasks linked via parent_id.

### Follow-up: Wave Ordering

| Option | Description | Selected |
|--------|-------------|----------|
| Waves on parent only | Parent task carries wave number. Subtasks inherit implicitly. | ✓ |
| Waves on both levels | Parent AND subtasks carry wave numbers independently. | |
| Pipeline order on subtasks | Use pipeline_order column for subtask sequencing within parent. | |

**User's choice:** Waves on parent only

---

## End-to-End Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Dispatch this project | Use the Noosphere Dispatch Pipeline project as test case | |
| Test fixture project | Small test .planning/ with known values, assert against DB | |
| Both + automated | Test fixture for CI-style validation, plus real project smoke test | ✓ |

**User's choice:** Both + automated

---

## Claude's Discretion

- Error handling improvements in dispatch_to_db.py
- Exact department mapping table (owner→department)
- Whether to add --dry-run flag

## Deferred Ideas

None
