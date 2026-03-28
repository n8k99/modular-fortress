# Phase 16: Foundation Tables & API - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-28
**Phase:** 16-foundation-tables-api
**Areas discussed:** Seed Data, Archives Immutability, Resources Frozen Flag, Templates Version History
**Mode:** --auto (all decisions auto-selected)

---

## Seed Data (Areas)

| Option | Description | Selected |
|--------|-------------|----------|
| 5 domains from ROADMAP success criteria | EM Corp, Orbis, Living Room Music, N8K99/Personal, Infrastructure/Systems with executive owners | ✓ |
| Minimal seed (3 domains) | Start with fewer, expand later | |
| No seed data | Create empty table, populate manually | |

**User's choice:** [auto] 5 domains from ROADMAP success criteria (recommended default)
**Notes:** Owner mapping derived from executive domain routing rules in PROJECT.md

---

## Archives Immutability

| Option | Description | Selected |
|--------|-------------|----------|
| Database trigger | Trigger prevents UPDATE on content fields at DB level | ✓ |
| Application-level | API checks prevent updates, DB allows them | |
| Hybrid | Trigger + API both enforce | |

**User's choice:** [auto] Database trigger (recommended — DB-is-the-OS philosophy)
**Notes:** PATCH allowed on metadata fields (tags, topic) but blocked on content/body

---

## Resources Frozen Flag

| Option | Description | Selected |
|--------|-------------|----------|
| Database trigger | Trigger blocks UPDATE when frozen=true | ✓ |
| API-only enforcement | Only API checks frozen flag | |
| No freeze mechanism | Rely on conventions | |

**User's choice:** [auto] Database trigger (recommended — same rationale as archives)
**Notes:** API returns 409 Conflict for frozen resource update attempts

---

## Templates Version History

| Option | Description | Selected |
|--------|-------------|----------|
| Separate templates_history table | Proper FK, queryable, matches REQUIREMENTS SCHEMA-04 | ✓ |
| JSONB array column | All versions in one row | |
| No version history | Overwrite in place | |

**User's choice:** [auto] Separate templates_history table (recommended — matches spec)
**Notes:** Trigger or application logic copies previous body to history on change

---

## Claude's Discretion

- Column types, index choices, migration script structure
- Handler response format
- Test strategy for triggers

## Deferred Ideas

None
