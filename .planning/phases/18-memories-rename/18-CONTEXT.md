# Phase 18: Memories Rename - Context

**Gathered:** 2026-03-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Rename vault_notes table to memories with backward-compatible view, add compression metadata columns, normalize departments via lookup table, and migrate all Rust code (dpn-api + dpn-core) from vault_notes/VaultNote to memories/Memory. Lisp and Python continue using vault_notes name via view. This is the highest blast-radius change in v1.3.

</domain>

<decisions>
## Implementation Decisions

### Table Rename & View Bridge (MEM-01)
- **D-01:** ALTER TABLE vault_notes RENAME TO memories. Then CREATE VIEW vault_notes AS SELECT * FROM memories.
- **D-02:** View must support INSERT/UPDATE/DELETE for Lisp and Python backward compatibility. Use INSTEAD OF triggers (not RULES) on the view — RULES have known issues with sqlx and can cause unexpected behavior with RETURNING clauses.
- **D-03:** Existing trigger `trg_sync_task_checkbox` must be recreated on the `memories` table (it was attached to vault_notes). The trigger function `sync_task_checkbox` does not reference the table name internally, so only the trigger attachment needs updating.
- **D-04:** All Rust code (dpn-api, dpn-core) migrates to reference `memories` table directly. The vault_notes view is ONLY for Lisp tick engine and Python gotcha-workspace tools.

### Compression Metadata (MEM-02, MEM-03)
- **D-05:** Add `compression_tier` column as VARCHAR with CHECK constraint for values: daily, weekly, monthly, quarterly, yearly. NOT NULL with default 'daily'.
- **D-06:** Backfill compression_tier from existing note_type: daily→daily, weekly→weekly, monthly→monthly, quarterly→quarterly, yearly→yearly, agent_memory→daily, freeform→daily, zenwriter→daily, NULL→daily.
- **D-07:** Add `compressed_from INTEGER[]` column (nullable) — tracks which source memory IDs a compressed record summarizes. NULL for non-compressed records.

### Department Normalization (MEM-04)
- **D-08:** Create `departments` lookup table with: id SERIAL, name VARCHAR(64) UNIQUE, slug VARCHAR(64) UNIQUE, description TEXT, created_at TIMESTAMPTZ.
- **D-09:** Canonical department names (PascalCase, consolidating 19 values):
  - Operations (from: Operations, Systems, support)
  - Engineering (from: Engineering, Systems)
  - Content & Brand (from: Content & Brand, content_brand)
  - Creative (from: Creative, art)
  - Legal (from: Legal, legal)
  - Music (from: music, Research — LRM's research is musicology)
  - Strategy (from: strategic_office, audience_experience, social_impact, digital_partnership)
  - Executive (from: Executive, Office of the CEO, cross_functional)
- **D-10:** Add `department_id INTEGER REFERENCES departments(id)` to agents table. Backfill from existing department text values using the mapping above. Keep original `department` text column for reference (not dropped this phase).

### Rust Migration (MEM-05, MEM-06)
- **D-11:** Rename dpn-core module: `db/vault_notes.rs` → `db/memories.rs`. Rename struct: `VaultNote` → `Memory`. Rename all query functions: `list_vault_notes` → `list_memories`, `get_vault_note_by_id` → `get_memory_by_id`, etc.
- **D-12:** Update dpn-core `mod.rs` and `lib.rs` re-exports from vault_notes to memories.
- **D-13:** Rename dpn-api handler: `handlers/documents.rs` (which handles vault_notes) — rename internal references from vault_notes to memories. Update route registrations if needed.
- **D-14:** Both dpn-core copies (/root and /opt) must be synced after changes.

### Claude's Discretion
- Index migration strategy (indexes on vault_notes auto-rename with table)
- Whether to rename dpn-api handler file from documents.rs to memories.rs (or just rename internal references)
- Order of migration steps (rename first vs add columns first)
- Whether to add updated_at trigger to memories table (currently missing from vault_notes)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Schema & Risk Analysis
- `.planning/REQUIREMENTS.md` — MEM-01 through MEM-06 (acceptance criteria)
- `.planning/research/SUMMARY.md` — v1.3 research summary (vault_notes rename identified as highest risk)
- `.planning/research/PITFALLS.md` — sqlx compile-time checking vs view RULES pitfall

### Existing Code (Modification Targets)
- `/root/dpn-core/src/db/vault_notes.rs` — Primary module to rename (100+ vault_notes references)
- `/opt/dpn-core/src/db/vault_notes.rs` — Operational copy (must stay synced)
- `/opt/dpn-api/src/handlers/documents.rs` — Handlers for vault_notes CRUD (57 references in dpn-api)
- `/opt/project-noosphere-ghosts/lisp/` — 4 references (must continue working via view)
- `/root/gotcha-workspace/tools/` — 218 references (must continue working via view)

### Prior Phase Decisions
- Phase 16 CONTEXT.md D-03: DB trigger enforcement philosophy (apply same to INSTEAD OF triggers)
- Phase 17 CONTEXT.md: dpn-core sync pattern between /root and /opt copies

### STATE.md Blockers
- "sqlx compile-time checking vs. view RULES needs empirical test before Phase 18" — addressed by D-02 (use INSTEAD OF triggers, not RULES)
- "trigger function sync_task_checkbox may reference vault_notes internally" — addressed by D-03 (function body does not reference table name)

</canonical_refs>

<code_context>
## Existing Code Insights

### Current Scale
- 2,719 rows in vault_notes (2,249 daily, 314 weekly, 68 monthly, 26 agent_memory, 25 quarterly, 9 yearly, 2 other)
- 64 per-agent memory columns on vault_notes (one per ghost)
- 19 distinct department values in agents table (mix of PascalCase, snake_case, and inconsistent naming)

### Reference Counts (Blast Radius)
- dpn-core: 100 references to vault_notes
- dpn-api: 57 references to vault_notes
- Lisp tick engine: 4 references (protected by view)
- Python gotcha-workspace: 218 references (protected by view)

### Existing Trigger
- `trg_sync_task_checkbox` on vault_notes — syncs checkbox state to tasks table. Must be migrated to memories table.

### Integration Points
- `memories` table (renamed from vault_notes)
- `departments` lookup table (new)
- `agents.department_id` FK (new column)
- dpn-core: vault_notes.rs → memories.rs (file rename + all internal references)
- dpn-api: documents.rs handler internal references
- lib.rs and mod.rs re-exports

</code_context>

<specifics>
## Specific Ideas

- The 64 per-agent memory columns are NOT being normalized into rows (explicitly out of scope per REQUIREMENTS.md)
- INSTEAD OF triggers on the view (not RULES) are critical — sqlx has known issues with RULES on views
- The vault_notes view must support all operations Lisp/Python currently do: SELECT, INSERT, UPDATE, DELETE
- Department normalization maps 19 values to 8 canonical departments

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 18-memories-rename*
*Context gathered: 2026-03-28*
