# Phase 18: Memories Rename - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-28
**Phase:** 18-memories-rename
**Areas discussed:** View Strategy, Compression Backfill, Department Normalization, Rust Migration Scope
**Mode:** --auto

---

## View Compatibility Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| INSTEAD OF triggers on view | View supports INSERT/UPDATE/DELETE via triggers | ✓ |
| RULES on view | Simpler but sqlx issues known | |
| No view, update all code | Maximum blast radius | |

**User's choice:** [auto] INSTEAD OF triggers (recommended — avoids sqlx RULES issues)

---

## Compression Tier Backfill

| Option | Description | Selected |
|--------|-------------|----------|
| Direct note_type mapping | daily→daily, weekly→weekly, etc. Unmapped→daily | ✓ |
| Leave NULL, backfill later | Defers work | |
| LLM-based classification | Expensive, unnecessary | |

**User's choice:** [auto] Direct mapping (recommended)

---

## Department Normalization

| Option | Description | Selected |
|--------|-------------|----------|
| Lookup table with 8 canonical depts | PascalCase, FK from agents | ✓ |
| Just clean up text values | No FK integrity | |
| Keep as-is | Doesn't meet MEM-04 | |

**User's choice:** [auto] Lookup table (recommended)

---

## Rust Migration Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Full rename (vault_notes→memories, VaultNote→Memory) | Clean break in Rust | ✓ |
| Alias only | Less disruption but messy | |
| Partial (dpn-core only) | Leaves dpn-api inconsistent | |

**User's choice:** [auto] Full rename (recommended)

---

## Claude's Discretion

- Index migration, handler file naming, migration step ordering, updated_at trigger

## Deferred Ideas

None
