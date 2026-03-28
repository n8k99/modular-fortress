# Domain Pitfalls

**Domain:** PARAT Noosphere Schema restructuring
**Researched:** 2026-03-28

## Critical Pitfalls

Mistakes that cause downtime or data loss.

### Pitfall 1: Breaking Ghost Perception During Rename
**What goes wrong:** Renaming `vault_notes` to `memories` without backward compatibility breaks the perception endpoint. The Lisp tick engine calls `/api/perception/:agent_id` which queries `vault_notes` for ghost memory columns. If the table doesn't exist, all ghosts lose their memory context.
**Why it happens:** The perception endpoint in `af64_perception.rs` (line 476) builds dynamic SQL: `SELECT note_date::text, {agent}_memories FROM vault_notes WHERE...`. This is string interpolation, not an ORM query. Renaming the table kills it.
**Consequences:** All ghost cognition degrades. Ghosts act without memory context. Standing orders fail.
**Prevention:** Use view-based rename: `ALTER TABLE vault_notes RENAME TO memories; CREATE VIEW vault_notes AS SELECT * FROM memories;` -- existing queries continue working via the view.
**Detection:** Monitor `pm2 logs noosphere-ghosts` for perception errors immediately after migration.

### Pitfall 2: sqlx Compile-Time Query Checking vs. View Rules
**What goes wrong:** sqlx's `query!()` macro validates queries at compile time against the database schema. If `vault_notes` becomes a view with RULES, sqlx may reject INSERT/UPDATE operations that worked against the real table.
**Why it happens:** PostgreSQL views with RULES have different behavior than tables for RETURNING clauses, triggers, and constraint checking. sqlx may detect the object is a view and refuse certain operations.
**Consequences:** dpn-core fails to compile after the rename, blocking all dpn-api changes.
**Prevention:** (1) Use `sqlx::query()` (runtime, not compile-time checked) for vault_notes queries during transition. (2) Test the view + rules combination with actual sqlx parameterized queries before committing. (3) Best approach: update all Rust code to use `memories` table name directly and skip the view for Rust code (view only needed for Lisp/Python backward compat).
**Detection:** `cargo build` in dpn-core/dpn-api will fail immediately if there's a problem.

### Pitfall 3: Goals Backfill Data Loss
**What goes wrong:** The `goals.project` column is TEXT containing project slugs. When adding `goals.project_id` INTEGER FK, the backfill UPDATE must match slugs to project IDs correctly. If slugs don't match (typos, renamed projects), goals lose their project association.
**Why it happens:** 44 goals all have `project` set. If any slug doesn't match a `projects.slug`, the FK column stays NULL.
**Consequences:** Goals disconnected from projects. Queries using the new FK miss these goals.
**Prevention:** Before backfill: `SELECT g.project, p.id FROM goals g LEFT JOIN projects p ON g.project = p.slug WHERE p.id IS NULL;` -- if any rows returned, fix manually. Keep the old TEXT column until verified.
**Detection:** Count goals with project_id IS NULL after backfill. Should be 0.

## Moderate Pitfalls

### Pitfall 4: Trigger Orphaning on Table Rename
**What goes wrong:** `vault_notes` has a trigger `trg_sync_task_checkbox AFTER UPDATE ON vault_notes`. When the table is renamed to `memories`, the trigger follows the table (PostgreSQL moves triggers with renames). But the trigger function `sync_task_checkbox()` may reference `vault_notes` internally.
**Prevention:** Inspect trigger function body: `SELECT prosrc FROM pg_proc WHERE proname = 'sync_task_checkbox';`. If it references `vault_notes`, update the function after rename.

### Pitfall 5: Nexus Chat Dedup Before Import
**What goes wrong:** Nexus Chat imports exist in both `Archive/Retired Nebulab/04 Archives/01 Nexus AI Chat Imports/` and `Archive/backup-Nebulab/Eckenrode Muziekopname/Engineering/Nexus AI Chat Imports/`. Importing both creates duplicate archive entries.
**Prevention:** Before import, identify duplicates by title+date. Only import from one path prefix (likely `Archive/Retired Nebulab/` as canonical). Query: `SELECT title, count(*) FROM documents WHERE path LIKE 'Archive/%Nexus AI Chat Imports%' GROUP BY title HAVING count(*) > 1;`

### Pitfall 6: Temporal Compression LLM Cost Explosion
**What goes wrong:** Compressing 2199 daily notes into weekly summaries requires ~314 LLM calls (one per week). At $0.50/request (ghost budget), that's ~$157 for one compression pass. Monthly/quarterly/yearly add more.
**Prevention:** (1) Use deterministic merge first (concatenate + truncate), not LLM. (2) If LLM needed, batch multiple days into single call. (3) Set a compression budget and stop when exhausted. (4) Compress incrementally (only new uncompressed days), not full historical reprocess.

### Pitfall 7: Foreign Key Cascade on agents Table
**What goes wrong:** Adding `area_id` to agents is safe. But if someone later tries `ALTER TABLE agents RENAME TO ghosts`, the 8 FK references (agent_state, agent_drives, agent_fitness, agent_daily_memory, tick_log, metamorphosis_log, persona_mutations, agent_document_links) all need CASCADE updates.
**Prevention:** Do NOT rename agents table. Use `CREATE VIEW ghosts AS SELECT * FROM agents`. Document this decision as permanent for v1.3.

## Minor Pitfalls

### Pitfall 8: Index Name Collisions After Rename
**What goes wrong:** `ALTER TABLE vault_notes RENAME TO memories` renames the table but not the indexes (`vault_notes_pkey`, `vault_notes_path_key`, `idx_vault_notes_date`, `idx_vault_notes_type`, `idx_vault_notes_embedding`). Code that references index names breaks.
**Prevention:** Rename indexes too: `ALTER INDEX vault_notes_pkey RENAME TO memories_pkey;` etc. Most code doesn't reference index names directly, but maintenance scripts might.

### Pitfall 9: Resources Overlay Getting Stale
**What goes wrong:** The `resources` table is an overlay pointing to `documents`, `media`, etc. via source_table/source_id. If documents are added/deleted, resources entries become stale (orphaned or missing).
**Prevention:** Either (1) don't populate resources automatically -- treat it as a curated catalog, or (2) add a periodic sync job. For v1.3, option 1 is simpler.

### Pitfall 10: Template .dpn Expressions vs Plain Text
**What goes wrong:** Templates store .dpn expressions in the body field. If code tries to evaluate them before the Innate interpreter exists, it crashes or returns raw expression text to users.
**Prevention:** All template reads in v1.3 return body as-is (plain text). No evaluation. Add a `format` column to distinguish `text` from `dpn` for future use.

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| Foundation tables (Wave 1) | Minimal risk -- pure additive | Test dpn-core compile, dpn-api startup |
| Projects/Goals (Wave 2) | Goals backfill mismatches (Pitfall 3) | LEFT JOIN verification query before backfill |
| Memories rename (Wave 3) | Perception breakage (Pitfall 1), sqlx view issues (Pitfall 2), trigger orphaning (Pitfall 4) | View-based rename, update Rust code to use `memories` directly, test with running ghosts |
| Agents org (Wave 4) | Accidental FK cascade if someone renames (Pitfall 7) | Document as VIEW-only, never rename table |
| Nexus import (Wave 5) | Dedup needed (Pitfall 5), LLM cost (Pitfall 6) | Dedup query first, deterministic compression |

## Sources

- Direct inspection of vault_notes trigger: `trg_sync_task_checkbox`
- dpn-api perception handler: `/opt/dpn-api/src/handlers/af64_perception.rs` line 476
- agents FK analysis: `\d agents` showing 8 referenced-by constraints
- Nexus Chat document paths: `SELECT path FROM documents WHERE path LIKE '%Nexus%'`
- sqlx documentation on compile-time checking behavior with views
