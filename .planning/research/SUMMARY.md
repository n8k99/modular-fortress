# Project Research Summary

**Project:** PARAT Noosphere Schema (v1.3)
**Domain:** PostgreSQL schema restructuring for an agentic AI substrate
**Researched:** 2026-03-28
**Confidence:** HIGH

## Executive Summary

PARAT v1.3 is a schema restructuring milestone, not a feature build. The goal is to reorganize a live 77-table PostgreSQL database (master_chronicle) around the PARA methodology — Projects, Areas, Resources, Archives, Templates — plus a renamed memory substrate (`vault_notes` -> `memories`). No new languages, no new crate dependencies, no frontend work. Every change extends or reorganizes the existing stack (PostgreSQL, Rust/sqlx, Common Lisp tick engine, Python gotcha tools). The approach is strictly additive-first: create new tables that have zero dependencies, then layer modifications onto existing tables, and save the high-blast-radius rename for last.

The single biggest risk is the `vault_notes` -> `memories` rename. This table is the ghost memory substrate: 2,668 rows with 64 per-agent columns, referenced by dpn-api Rust handlers, dpn-core queries, dpn-kb pages, gotcha-workspace Python tools, and the Lisp tick engine perception endpoint. The safe migration path is view-based: rename the table, create a `vault_notes` view over it, and progressively update code to use the new name. Critically, all Rust code must be updated to reference `memories` directly — sqlx compile-time query macros may reject INSERT/UPDATE against a view with RULES, so the view is only for Lisp/Python backward compat. The second significant risk is the Nexus Chat AI import pipeline (993+ documents from ChatGPT history), which requires deduplication before import and should use deterministic compression rather than per-note LLM calls to avoid a $150+ cost explosion.

The recommended execution order mirrors the dependency graph: (1) additive foundation tables with no blast radius, (2) low-risk column additions to existing tables, (3) the high-risk vault_notes rename with view safety net, (4) organizational structure additions, (5) the Nexus import and temporal compression pipeline. This order ensures ghosts remain operational throughout — perception must never break.

## Key Findings

### Recommended Stack

No new dependencies. All PARAT work uses existing infrastructure: PostgreSQL for schema changes, sqlx 0.8 for Rust query modules, Axum 0.7 for new REST endpoints, Python 3 with psycopg2/pg for migration scripts, and the existing pgvector extension (already installed, HNSW indexes in use on `documents` and `vault_notes`) for embedding columns on archives.

One critical nuance: sqlx's compile-time query checking (`query!()` macro) may reject INSERT/UPDATE operations against views with RULES. The safe approach is to update all Rust code to reference the `memories` table directly, reserving the `vault_notes` view solely for Lisp and Python backward compatibility during the transition window.

**Core technologies:**
- PostgreSQL 14+: Schema changes, new tables, views, rules — the substrate that everything runs on
- Rust (dpn-core/dpn-api, sqlx 0.8): New module files per PARAT pillar, new Axum handlers — no new crates
- Python 3 (gotcha-workspace): Migration scripts, Nexus import pipeline, temporal compression tool
- Common Lisp (SBCL tick engine): Minimal changes — perception query update after rename stabilizes
- pgvector (already installed): VECTOR(768) embedding column on archives for semantic search

### Expected Features

Research identified eight feature pillars with clear complexity ratings and explicit anti-features. The MVP ordering is: foundation tables first (no dependencies, no blast radius), then project/goals modifications, then the vault_notes rename, then org structure, then the Nexus import pipeline.

**Must have (table stakes):**
- `areas` table — new ongoing-domain entity; ghosts and projects need area assignment
- `lifestage` enum on projects (Seed/Sapling/Tree/Harvest) — core PARAT differentiator, low-risk ALTER TABLE
- `goals.project_id` FK migration — 44 rows, currently orphaned as TEXT slugs with no referential integrity
- `vault_notes` -> `memories` rename — semantic alignment with PARAT; highest blast radius in the milestone
- Compression tier formalization — daily/weekly/monthly/quarterly/yearly enum + `compressed_from` INTEGER[] tracking
- `archives` table — immutable historical records; Nexus import terminus and temporal compression terminus
- `resources` table — organizational overlay indexing into existing documents/media (not replacing them)
- `templates` table — stores .dpn expressions as inert text (Innate interpreter is a separate project)

**Should have (differentiators):**
- `ghost_relationships` table formalizing existing `reports_to` text arrays with FK integrity
- `teams` + `team_members` tables for fluid org structure
- Department normalization (19 inconsistent values -> lookup table)
- `temporal_compression_log` table for compression run observability
- Ghost identity documents migrated into templates (seeded from agent descriptions)
- Nova memory injection from Nexus Chat AI import cascade

**Defer (v2+):**
- Innate interpreter evaluation of template bodies (templates store .dpn text only in v1.3)
- Frontend UI for PARAT tables (explicitly out of scope per PROJECT.md)
- Perception endpoint rewrite (incremental additions only, after tables stable)
- Automated lifestage transitions (lifestage is a human judgment, never auto-advance)
- Full migration of 48K documents into resources (resources is a curated overlay, not a warehouse)
- Renaming `agents` table to `ghosts` (8 FK references; use a view instead)

### Architecture Approach

The architecture is conservative by necessity: the live system processes ghost ticks, and any table that breaks perception breaks all 64 ghosts. The governing pattern is "additive before destructive, view-based rename for live migration, organizational overlay rather than data migration." New PARAT tables (areas, archives, resources, templates) are purely additive with no risk. Modifications to existing tables (projects lifestage, goals FK, agents area_id) are low-risk column additions. The vault_notes rename is the only operation requiring careful sequencing — view + rules provides backward compat while code migrates.

**Major components:**
1. `areas` table — ongoing domain entity; anchor for ghost assignment and project scoping
2. `memories` table (renamed from vault_notes) — temporal note substrate with 64 per-ghost memory columns; the ghost cognition backbone
3. `archives` table — immutable historical record; Nexus Chat AI import terminus and temporal compression terminus
4. `resources` table — organizational overlay indexing into existing media/documents/feeds without migrating data
5. `templates` table — stores .dpn expressions as text; future-proofed for Innate interpreter
6. `temporal_compression_log` — observability for daily->weekly->monthly->quarterly->yearly cascade
7. Modified `projects` table — gains `lifestage` VARCHAR(32) with CHECK constraint and `area_id` INTEGER FK
8. Modified `goals` table — gains `project_id` INTEGER FK (backfilled from 44-row TEXT column, then TEXT kept for compat)
9. `ghosts` view over `agents` — PARAT-native name without blast-radius rename of 8 FK dependencies

### Critical Pitfalls

1. **Ghost perception breakage during vault_notes rename** — The perception endpoint in `af64_perception.rs` (line 476) builds dynamic SQL with the literal string `vault_notes`. Renaming without a view drops all ghost memory context immediately. Prevention: `ALTER TABLE vault_notes RENAME TO memories; CREATE VIEW vault_notes AS SELECT * FROM memories;` Then update Rust code to use `memories` directly; leave view only for Lisp/Python compat.

2. **sqlx compile-time checking vs. view RULES** — sqlx `query!()` macros validated at compile time may refuse INSERT/UPDATE against a view with RULES, blocking dpn-core compilation. Prevention: Update all Rust code to reference `memories` directly before or in the same PR as the rename. Do not rely on the view for any Rust code paths.

3. **Goals backfill data loss** — 44 goals have `project` as TEXT slug. If any slug mismatches a `projects.slug`, the new `project_id` FK stays NULL and those goals become silently orphaned. Prevention: Run `SELECT g.project, p.id FROM goals g LEFT JOIN projects p ON g.project = p.slug WHERE p.id IS NULL;` before backfill. Fix mismatches. Verify zero NULLs after.

4. **LLM cost explosion during temporal compression** — Compressing 2,199 daily notes at $0.50/call = ~$157 minimum. Ghost memory column synthesis multiplies this by up to 64. Prevention: Use deterministic merge (concatenate + truncate) as the default. Reserve LLM calls for targeted final-tier synthesis. Compress incrementally (uncompressed days only), never reprocess full history.

5. **Nexus Chat deduplication** — ChatGPT conversation documents exist in two Archive paths (`Archive/Retired Nebulab/` and `Archive/backup-Nebulab/`). Importing both creates duplicate archive entries. Prevention: Dedup by title+date before import. Only import from the canonical path (`Archive/Retired Nebulab/`).

## Implications for Roadmap

Based on the dependency graph in FEATURES.md and the wave structure in ARCHITECTURE.md, a 5-phase roadmap maps cleanly to the blast-radius order.

### Phase 1: Foundation Tables
**Rationale:** Pure additive changes. New tables (areas, archives, resources, templates, temporal_compression_log) have zero dependencies and cannot break anything running. This is the safe starting point that unblocks all later phases.
**Delivers:** Full PARAT table skeleton in master_chronicle. dpn-core gets four new module files (areas.rs, templates.rs, archives.rs, resources.rs). dpn-api gets new handler stubs for all four tables. All new tables verified with seed data (five initial areas seeded: EM Corp, Orbis, Living Room Music, N8K99/Personal, Infrastructure/Systems).
**Addresses:** @Areas, @Resources, @Archives, @Templates pillars from FEATURES.md
**Avoids:** No pitfalls in play — pure addition. Run `cargo build` and verify dpn-api starts after each new module.

### Phase 2: Projects and Goals Restructuring
**Rationale:** Low-risk ALTER TABLE operations on small tables (14 projects, 44 goals). Lifestage column and goals FK migration are independent of the memory rename. Complete these before touching vault_notes so the high-blast-radius rename stands alone in its own phase.
**Delivers:** `projects.lifestage` with CHECK constraint (seed/sapling/tree/harvest), `projects.area_id` FK, `goals.project_id` FK backfilled and verified, updated dpn-core/dpn-api structs and endpoints for lifestage and area fields.
**Uses:** PostgreSQL ALTER TABLE, sqlx struct updates, existing Axum handler patterns
**Implements:** @Projects pillar with growth arc; @Areas linkage to projects
**Avoids:** Pitfall 3 (goals backfill data loss) — run LEFT JOIN verification query before committing the backfill. Count NULLs after and fix before dropping TEXT column.

### Phase 3: Memories Rename (vault_notes -> memories)
**Rationale:** The highest-blast-radius change in the milestone deserves its own isolated phase. Isolation means a clean rollback path: if anything breaks, drop the view and rename back. The sequence is: rename table, rename indexes, create view + rules, inspect and update trigger if needed, update all Rust code to use `memories` directly, verify Lisp/Python still work via view.
**Delivers:** `memories` table live, `vault_notes` view active for Lisp/Python backward compat, all Rust code in dpn-core and dpn-api updated to use `memories`, compression tier enum column and `compressed_from` INTEGER[] column added, `temporal_compression_log` table created.
**Uses:** PostgreSQL RENAME + VIEW + RULES, Rust code migration across dpn-core and dpn-api, trigger function inspection (`sync_task_checkbox`)
**Avoids:** Pitfall 1 (perception breakage) via view; Pitfall 2 (sqlx view issues) by updating Rust to `memories` directly; Pitfall 4 (trigger orphaning) by inspecting trigger body with `SELECT prosrc FROM pg_proc WHERE proname = 'sync_task_checkbox'` before rename.

### Phase 4: Ghost Organizational Structure
**Rationale:** Additive changes to agents table and new junction tables. Does not touch the hot perception path. After memories rename is stable, this is safe to layer on.
**Delivers:** `teams` table + `team_members` junction, `ghost_relationships` table (formalizing `reports_to` text arrays), `ghosts` view over agents, `agents.area_id` FK, `agent_areas` junction for multi-area assignment, department normalization (19 inconsistent values normalized) with optional lookup table, `agents.aliases` column for dual-identity ghosts (Nova = T.A.S.K.S.).
**Implements:** Ghost organizational structure pillar; @Areas ghost assignment
**Avoids:** Pitfall 7 (agents FK cascade) — use VIEW-only approach, never rename agents table. Document as permanent architectural decision.

### Phase 5: Nexus Import and Temporal Compression Pipeline
**Rationale:** Depends on archives table (Phase 1) and memories rename (Phase 3) both being stable. The most complex data pipeline in the milestone. Requires deduplication, deterministic compression, and Nova memory injection.
**Delivers:** 993+ ChatGPT conversations imported to archives (deduplicated by title+date, canonical path only), temporal compression cascade applied to imported content (monthly/quarterly/yearly), Nova memory columns populated with imported context, daily/weekly notes updated with archive reference links, compression pipeline operational for ongoing standing orders.
**Uses:** Python migration scripts in gotcha-workspace, psycopg2/pg for direct DB access, deterministic merge strategy (concatenate + truncate), targeted LLM calls for final synthesis tier only
**Avoids:** Pitfall 5 (dedup) — run title+date dedup query before import; Pitfall 6 (LLM cost explosion) — deterministic first, LLM only for targeted synthesis. Compress incrementally, never reprocess full history.

### Phase Ordering Rationale

- Phases 1 and 2 are fully safe because they are additive-only or operate on small tables (14 projects, 44 goals). Running them first builds confidence and unblocks all downstream work with zero risk to live ghost operations.
- Phase 3 is isolated because vault_notes rename touches every layer of the stack simultaneously (Rust, Lisp, Python, PostgreSQL). Isolation means a clean rollback window — drop view and rename back — if something fails.
- Phase 4 follows Phase 3 because area assignment for agents references the areas table (Phase 1) and should only be wired up after the memory rename has been stable in production for at least one tick cycle.
- Phase 5 is last because it depends on archives (Phase 1) and memories (Phase 3) both being operational, and it is the highest-effort data pipeline with its own deduplication and compression risks.
- The FEATURES.md dependency graph explicitly confirms this order: `archives table -> Nexus Chat AI import pipeline` and `memories rename -> ALL dpn-api endpoints`.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 3 (Memories Rename):** sqlx view + RULES behavior with RETURNING clauses needs empirical testing before finalizing the migration script. Recommend a test branch that renames vault_notes, creates the view, and runs `cargo build` in dpn-core to verify compile-time query behavior before the real migration lands.
- **Phase 5 (Nexus Import Pipeline):** The exact document count needs a pre-import query pass against live data. ARCHITECTURE.md says 2,179 at the Retired Nebulab path; FEATURES.md says 993. Clarify exact canonical count with `SELECT count(*) FROM documents WHERE path LIKE 'Archive/Retired Nebulab/04 Archives/01 Nexus AI Chat Imports%'` before scripting.

Phases with standard patterns (skip research-phase):
- **Phase 1 (Foundation Tables):** Pure CREATE TABLE operations following established dpn-core module patterns. ARCHITECTURE.md provides exact DDL. No research needed.
- **Phase 2 (Projects/Goals):** Standard ALTER TABLE + backfill. Pattern is well-established (add column, backfill, verify NULLs, optionally drop old). No research needed.
- **Phase 4 (Org Structure):** Additive junction tables following existing agent schema patterns. No novel patterns.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Zero new dependencies. All existing libraries confirmed in Cargo.toml and active use. |
| Features | HIGH | Based on direct schema inspection of live 77-table master_chronicle. All row counts and column names verified against live data. |
| Architecture | HIGH | Wave structure derived from live schema. DDL statements in ARCHITECTURE.md are concrete and copy-paste ready. |
| Pitfalls | HIGH | Pitfalls traced to specific file paths and line numbers (af64_perception.rs:476). Not hypothetical risk. |
| Temporal Compression | MEDIUM | Process design is clear but LLM cost/quality tradeoffs for ghost memory column synthesis need phase planning decisions. |
| Nexus Import | MEDIUM | 993 vs. 2,179 document count discrepancy between research files needs live query to resolve before scripting. |

**Overall confidence:** HIGH

### Gaps to Address

- **Exact Nexus document count:** ARCHITECTURE.md says 2,179 at the Retired Nebulab path; FEATURES.md says 993. Run `SELECT count(*) FROM documents WHERE path LIKE 'Archive/Retired Nebulab/04 Archives/01 Nexus AI Chat Imports%'` before Phase 5 scripting to resolve.
- **Trigger function body inspection:** The `sync_task_checkbox` trigger on vault_notes may reference the old table name internally. Run `SELECT prosrc FROM pg_proc WHERE proname = 'sync_task_checkbox'` before Phase 3 execution.
- **sqlx compile behavior with views:** The risk in Pitfall 2 needs empirical validation on a test branch before Phase 3 lands on master.
- **agent_daily_memory merge decision:** This table overlaps conceptually with daily tier memories. Research flagged it as "evaluate whether to merge." This decision must be made before Phase 3 closes.
- **Ghost memory column synthesis budget for compression:** When 7 dailies roll up to a weekly, each ghost's `*_memories` column should synthesize the 7 daily perspectives. At 64 agents per weekly rollup, this is expensive. Decide on batching strategy and per-compression LLM budget before Phase 5.

## Sources

### Primary (HIGH confidence)
- Live database schema inspection (master_chronicle, 77 tables, 2026-03-28) — all row counts, column names, trigger names verified
- `/opt/dpn-api/src/handlers/af64_perception.rs` line 476 — perception SQL string interpolation confirmed
- `/root/dpn-core/src/db/vault_notes.rs` — existing module structure for new module pattern
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Lisp tick engine write paths
- dpn-core `Cargo.toml` + dpn-api `Cargo.toml` — confirmed no new dependencies needed
- `.planning/PROJECT.md` v1.3 milestone definition — scope constraints

### Secondary (MEDIUM confidence)
- [Tiago Forte — The PARA Method](https://fortelabs.com/blog/para/) — PARA pillar definitions and structure
- [PostgreSQL State Machines](https://felixge.de/2017/07/27/implementing-state-machines-in-postgresql/) — lifestage CHECK constraint patterns
- [PKM at Scale](https://www.dsebastien.net/personal-knowledge-management-at-scale-analyzing-8-000-notes-and-64-000-links/) — temporal compression statistics (daily 11.5 links -> yearly 1443.8 links)
- [Redis ChatGPT Memory Project](https://redis.io/blog/chatgpt-memory-project/) — conversation history as vector-embedded memory

### Tertiary (LOW confidence)
- sqlx documentation on compile-time checking behavior with views — needs empirical validation for this specific RULES + RETURNING pattern

---
*Research completed: 2026-03-28*
*Ready for roadmap: yes*
