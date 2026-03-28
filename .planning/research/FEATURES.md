# Feature Landscape: PARAT Noosphere Schema

**Domain:** Organizational knowledge architecture for an agentic AI substrate (PostgreSQL)
**Researched:** 2026-03-28
**Confidence:** HIGH (existing schema inspected, PARA methodology well-documented, domain-specific requirements from PROJECT.md)

## Table Stakes

Features users (ghosts + Nathan) expect. Missing = PARAT schema feels incomplete.

### Pillar 1: @Projects with Lifestage Arc

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `lifestage` enum column on `projects` (Seed/Sapling/Tree/Harvest) | Core PARAT differentiator from vanilla PARA. Projects have a growth arc, not just active/done. | Low | ADD column to existing `projects` table. Coexists with `status` (active/paused/completed/archived). Lifestage is semantic (growth phase), status is operational (is work happening). |
| Lifestage transition rules | Prevents nonsensical jumps (Harvest -> Seed). Enforces forward-only arc with revert requiring explicit action. | Low | CHECK constraint or trigger. Valid transitions: Seed->Sapling->Tree->Harvest. Revert = explicit decision logged in `decisions` table. |
| `goals` table FK to `projects` | Currently `goals.project` is TEXT with no FK. PARAT requires goals as proper sub-entities of projects. | Med | Migration: add `project_id INTEGER REFERENCES projects(id)`, populate from text match, then drop text `project` column. 44 goals to migrate. |
| `area_id` FK on `projects` | Projects belong to an Area (ongoing domain). A project about "EM website redesign" belongs to the "EM Corp" area. | Low | New nullable FK column. Not all projects have an area (standalone projects are valid). |
| Project-to-document linkage preservation | Existing `document_id` FK on projects links to a canonical document. Must survive PARAT migration. | Low | Already exists. No change needed, but PARAT should not break this. |

**Edge cases:**
- Projects at Harvest that get revived: Allow Harvest->Seed transition but require a `decisions` entry explaining why (prevents accidental revert).
- Lifestage vs. status overlap: A project can be `paused` at any lifestage. Status=archived implies lifestage=Harvest but not vice versa (Harvest means "bearing fruit", not "dead").
- 14 existing projects need lifestage backfill. Default to Sapling for active, Tree for completed, Seed for new.

### Pillar 2: @Areas as Ongoing Domains

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `areas` table | New table. Areas are ongoing responsibility domains with no deadline. Examples: "EM Corp", "Orbis", "Living Room Music", "Operations", "Legal". | Low | CREATE TABLE with id, name, slug, description, owner (agent FK), status, created_at, updated_at. |
| Ghost assignment to areas | Ghosts perceive work scoped to their area. Executives own areas; staff are assigned. | Med | `area_id` FK on `agents` table (primary area), plus `agent_areas` junction table for multi-area assignment. Perception endpoint must filter by area. |
| Area-scoped perception | When a ghost ticks, it sees tasks/projects within its assigned areas. | Med | Modify `af64_perception.rs` to include area context. Area membership determines what projects/tasks appear in perception. |
| Projects linked to areas | `area_id` FK on `projects` table connects active work to ongoing domains. | Low | Same as the project FK noted above. |
| Area health metrics | Nathan needs to see which areas are active, neglected, or overloaded. | Low | Derived from project count, task completion rate, last activity timestamp. View or materialized view. |

**Edge cases:**
- Ghost assigned to multiple areas: Use junction table, not just single FK. Nova touches Operations AND cross-functional. Kathryn touches Strategic AND Financial.
- Department vs. Area: Departments are organizational (HR structure). Areas are domain-scoped (what you work on). An agent in "Engineering" department can be assigned to "Orbis" area. These are orthogonal.
- Orphaned areas: Areas with no assigned ghosts should surface in health checks, not be deleted.
- Existing department data: 19 distinct departments exist in `agents.department`. Areas are NOT a replacement for departments -- they supplement them. Map: department = org structure, area = work domain.

### Pillar 3: @Resources as Read-Optimized Reference

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `resources` table | Consolidates images, media, reference docs. Read-optimized: no lifecycle, no status changes, just content retrieval. | Med | New table. Must decide relationship to existing `documents` (48K), `media`, `lrm_corpus`, `music_files` tables. |
| Resource categorization | Type field: image, document, media, reference, tool, template_asset. | Low | ENUM or varchar with CHECK. |
| Resource linking to areas/projects | A resource belongs to an area or is used by a project. Many-to-many. | Low | `resource_links` junction table with polymorphic `linkable_type` + `linkable_id`, or separate FKs. |
| Embedding support | Resources should be searchable via semantic similarity (existing pattern with vector(768)). | Low | Already established pattern in `documents` and `vault_notes`. Copy it. |
| Immutable content flag | Resources should not be casually edited. Mark as `frozen` when confirmed. | Low | Boolean column. Triggers prevent UPDATE on content when frozen=true. |

**Edge cases:**
- Relationship to existing `documents` table: Do NOT migrate 48K documents into resources. Resources is a curated subset. Use `resource_source_type` + `resource_source_id` to link back to `documents`, `media`, etc. Resources are a view/index layer, not a data copy.
- Relationship to existing `media` table: Media (constrained to map/brand/burg/portrait types) is a subset of what resources covers. Keep `media` as-is, let resources reference it.
- Large binary content: Resources table stores metadata + path, not binary blobs. Actual files stay on disk or in existing tables.

### Pillar 4: @Archives as Immutable Historical Content

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `archives` table | Immutable historical records. Temporal compression terminus -- once content reaches yearly compression, it moves to archive. | Med | New table with strict immutability (no UPDATE triggers except metadata). |
| Source tracking | Every archive entry knows where it came from: vault_notes, conversations, documents, chat imports. | Low | `source_type` + `source_id` columns. |
| Temporal metadata | `period_start`, `period_end` dates. An archived yearly summary covers Jan 1 - Dec 31. | Low | Two date columns. Enables range queries. |
| Archive search | Full-text search + embedding search on archived content. | Low | `tsvector` column + vector(768) embedding. Established pattern. |
| Nexus Chat AI Import as archive content | 993 ChatGPT conversation imports already exist in `documents` table under Archive paths. These become the first archive entries. | Med | Migration script: SELECT from documents WHERE path LIKE 'Archive/Retired Nebulab/04 Archives/01 Nexus AI Chat Imports%', INSERT into archives with source_type='chatgpt_import'. |

**Edge cases:**
- Immutability enforcement: Use a trigger that raises an exception on UPDATE to content/title columns. Allow metadata updates (tags, embeddings).
- Archive vs. project status "archived": These are different concepts. Project status=archived means "work stopped." Archive table means "historical record preserved." A project can be archived (status) AND have archive entries (historical summaries).
- Deduplication: The Nexus Chat AI imports exist in both `Archive/Retired Nebulab/` and `Archive/backup-Nebulab/` paths (duplicate data). Dedup during migration.
- 39,509 documents under `Archive/` paths: NOT all of these become archive table entries. Most are Orbis world lore. Only temporally-compressed summaries and chat imports go into archives table. The rest stay in documents.

### Pillar 5: @Templates with Live Expression Bodies

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `templates` table | Stores .dpn Innate expressions in body field. Templates are generative -- they evaluate against context. | Med | New table: id, name, slug, body (TEXT -- the .dpn expression), description, category, parameters JSONB, created_at, updated_at. |
| Template categorization | Types: ghost_howto, project_template, report_template, note_template, perception_template. | Low | VARCHAR category column. |
| Parameter schema | Templates declare what inputs they expect. `parameters` JSONB field defines the schema. | Low | Store as JSONB: `{"agent_id": "string", "date": "date", "area": "string"}`. Evaluated at runtime by Innate interpreter (future). |
| Template versioning | Templates evolve. Keep history of body changes. | Low | `version` integer column + `templates_history` table for previous versions. |
| Ghost identity documents as templates | "How to be Nova" documents live in templates, not files. Each ghost has a template defining their persona/behavior. | Med | Seed templates from existing agent descriptions + document_path content. |

**Edge cases:**
- Innate interpreter does not exist yet: Templates store .dpn expressions but evaluation is deferred to the Innate project. For now, templates are inert text that humans and ghosts read. The body field is future-proofed, not functional.
- Template rendering without Innate: Until Innate exists, ghosts consume templates as plain text. The .dpn expressions are documentation of intent, not executable code.
- Template parameters validation: No runtime validation until Innate exists. Parameters JSONB is a schema declaration, not enforced.

### Pillar 6: Temporal Compression (vault_notes -> memories)

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Rename `vault_notes` to `memories` | Semantic alignment with PARAT. These are the noosphere's memory substrate, not "vault notes." | High | ALTER TABLE RENAME. All 2,668 rows preserved. Update ALL references in dpn-api, dpn-core, dpn-kb, ghost tick engine, gotcha-workspace tools. Highest blast radius of any change. |
| Formalized compression tiers | daily -> weekly -> monthly -> quarterly -> yearly. Each tier compresses the tier below. | Med | Already partially exists: note_type has daily(2199), weekly(314), monthly(68), quarterly(25), yearly(9). Formalize with `compression_tier` enum and `compressed_from` FK array linking to source entries. |
| Compression source tracking | A weekly memory knows which 7 daily memories it summarizes. | Med | `compressed_from` INTEGER[] column. Links weekly to its constituent dailies. Enables drill-down. |
| Ghost memory columns preservation | 64 agent-specific `*_memories` TEXT columns on vault_notes. These are per-ghost perspectives on each note. | Low | Carry forward on rename. These columns are the ghost memory substrate -- each ghost has its own interpretation of each note. |
| Compression automation rules | Nova/T.A.S.K.S. already runs temporal compression as a standing order. Formalize the rules. | Med | Daily notes older than 7 days compress into weekly. Weekly older than 4 weeks into monthly. Monthly older than 3 months into quarterly. Quarterly older than 12 months into yearly. |
| Archive terminus | Yearly memories older than 2 years move to `archives` table. | Low | Standing order or cron. Move content to archives, mark memory as `archived=true`. |

**Edge cases:**
- Ghost memory columns during compression: When 7 dailies compress into 1 weekly, each ghost's `*_memories` column on the weekly should synthesize (not concatenate) the 7 daily perspectives. This requires LLM cognition per ghost per compression event.
- Compression is lossy by design: The whole point is to reduce volume while preserving signal. Daily details are lost; only salient facts survive into weekly. This is a feature, not a bug.
- note_date gaps: Not every day has a daily note. Compression must handle sparse data (e.g., only 3 of 7 days have entries for a given week).
- Monthly/quarterly/yearly notes currently have NULL note_date: Need to backfill with period start dates during migration.
- Compression order matters: Must compress daily->weekly BEFORE weekly->monthly. Cannot skip tiers.
- agent_memory note_type (26 entries): These are a different concept from ghost memory columns. They are standalone memory notes, not temporal compression products. Need a separate handling path.
- Blast radius of rename: `vault_notes` is referenced in dpn-api Rust handlers, dpn-core queries, dpn-kb frontend, gotcha-workspace Python tools (daily notes, synthesis, temporal compression, podcasts), and the tick engine perception endpoint. Every reference must be updated atomically.
- `agent_daily_memory` table overlap: This table (agent_id + log_date + actions/decisions/knowledge/blockers/handoffs) overlaps with daily memories. Evaluate merging into the memories table or keeping as a separate agent-specific daily log.

### Pillar 7: Ghost Organizational Structure

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Team structure | Ghosts belong to teams. Teams have a lead. Teams map to departments but are more fluid. | Med | `teams` table: id, name, department, lead_id (FK agents), area_id (FK areas), created_at. `team_members` junction: team_id, agent_id, role_in_team. |
| Org chart relationships formalized | `reports_to` already exists as text[]. Formalize with proper FK relationships for integrity. | Med | Already have `reports_to`, `mentor`, `mentee`, `collaborators`, `liaises_with` on agents. Create `ghost_relationships` table with from_agent, to_agent, relationship_type for canonical data. Keep text[] for backward compat. |
| Executive blog capability | Executives publish reflections as their voice in the noosphere. | Low | Use conversations table with channel='blog' or message_type='blog_post'. No new table needed. |
| Department consolidation | 19 distinct department values with inconsistent casing (Legal vs legal, Engineering vs engineering). | Low | Normalize department values. Create `departments` lookup table, FK from agents. |
| Ghost identity preservation | Some ghosts have dual identities (Nova IS T.A.S.K.S.). Schema must support aliases. | Low | `aliases` text[] column on agents. Do NOT rename agents table (excessive blast radius). |

**Edge cases:**
- Department vs. Area (again): Departments are organizational (HR). Areas are work domains. They are orthogonal. Do not conflate.
- reports_to is text[], not FK: 8 executives report to {nathan}. Staff report to various combinations. Converting to FK-based relationships is a data migration. Keep text[] for backward compat, add relationships table as the canonical source going forward.
- Team membership is dynamic: Ghosts can move between teams for projects. Junction table handles this; do not hardcode team assignment on agents.
- Inconsistent department naming: "Engineering" vs "engineering", "Legal" vs "legal". Normalize before creating lookup table.

### Pillar 8: Conversation History Import into Ghost Memory

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Nexus Chat AI Import pipeline | 993 ChatGPT conversation documents -> structured archive entries -> temporally compressed -> injected into Nova's ghost memory columns. | High | Multi-step pipeline: (1) Parse existing documents, (2) Extract date + topic + key facts, (3) Write to archives, (4) Generate temporal summaries (daily->weekly->monthly), (5) Inject into Nova's memory columns on relevant memories. |
| Memory injection validation | Imported memories must be distinguishable from organic memories. | Low | `source` column on memories: 'organic', 'imported', 'compressed'. Imported memories tagged so ghosts know their provenance. |
| Temporal cascading | Chat imports span Dec 2023 - May 2025. Must be compressed into the existing temporal hierarchy. | High | Group by month, generate monthly summaries, then quarterly, then yearly. These become archive entries and also populate Nova's `nova_memories` columns on corresponding monthly/quarterly/yearly memories. |
| Cross-reference links | Imported memories should link to the source archive entries. | Low | `source_archive_id` FK or JSONB metadata with archive references. |
| Daily/weekly note link injection | After import, relevant daily/weekly notes get links to the imported content added to their content field. | Med | Append markdown links to existing note content. Must not corrupt existing data. |

**Edge cases:**
- Chat imports are Nathan's conversations, not Nova's: The import gives Nova context about Nathan's history, but the memories are attributed as "imported from Nathan's ChatGPT history", not as Nova's organic experience.
- Date extraction from chat titles: Titles follow pattern "YYYY-MM-DD - Topic". Parse with regex. Some titles have "Title: " prefix instead.
- Duplicate content: Same conversations exist in both `Archive/Retired Nebulab/` and `Archive/backup-Nebulab/` paths. Dedup by title + date before import.
- Volume: 993 documents averaging ~5KB each = ~5MB of text. Temporal compression will reduce this by ~10x at each tier.
- Memory column width: The `*_memories` TEXT columns have no size limit, but extremely long memory text degrades LLM context quality. Compression must produce concise summaries (target: 500 words per monthly, 2000 words per yearly).

## Differentiators

Features that set PARAT apart from vanilla PARA or generic knowledge management.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Ghost-aware memory columns | Every memory has 64 ghost perspective columns. Each ghost sees the same events through their own lens. | Already exists | Unique to this system. Carry forward, do not lose. |
| Lifestage arc (not just status) | Projects grow organically (Seed->Sapling->Tree->Harvest) instead of binary active/done. Enables portfolio management by maturity. | Low | No other system does this. Nathan's innovation. |
| Templates as generative layer | Templates contain live .dpn expressions, not static text. Future Innate interpreter makes templates executable. | Med | Future-proofing for Innate language project. No other knowledge system has evaluable template bodies. |
| Temporal compression with ghost synthesis | Compression is not just summarization -- each ghost writes their own compressed perspective. 64 parallel compressions per tier transition. | High | Differentiator but expensive (64 LLM calls per compression event per note). Batch during off-peak. |
| Area-scoped ghost perception | Ghosts only see work in their assigned areas. Reduces noise, increases relevance. | Med | Standard in multi-agent systems but novel combined with PARA structure. |
| Conversation import as memory substrate | Historical AI conversations become ghost memory, enabling cognitive continuity across AI systems. | High | Novel approach: using one AI's history to bootstrap another AI's memory. |

## Anti-Features

Features to explicitly NOT build.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Migrating all 48K documents into resources table | Massive, unnecessary data duplication. Resources is a curated index, not a warehouse. | Resources reference documents via source_type/source_id. Documents table stays as-is. |
| Renaming `agents` table to `ghosts` | Blast radius across dpn-api, dpn-core, tick engine, all tools. Every FK, every query, every API endpoint breaks. | Add ghost-oriented columns (aliases, ghost identity). Keep table name `agents`. |
| Automated lifestage transitions | Projects should not auto-advance lifestage based on task completion percentages. Lifestage is a human judgment. | Nathan or executives explicitly set lifestage. Provide nudges but never auto-transition. |
| Real-time temporal compression | Compressing notes as they are written defeats the purpose. You need the raw daily data before you can compress. | Compression runs on schedule (weekly on Sundays, monthly on 1st, etc.) via Nova standing orders. |
| Storing binary files in PostgreSQL | BLOBs in the DB destroy performance and backup size. | Resources table stores metadata + file paths. Actual binaries on disk. |
| Deep Innate interpreter integration | Innate is a separate project. Building an interpreter into PARAT is scope creep. | Templates store .dpn text. Evaluation is the Innate project's job. |
| Replacing departments with areas | Departments are org structure (HR). Areas are work domains. They serve different purposes. | Keep both. An agent has a department (where they sit) AND area assignments (what they work on). |
| Importing all ChatGPT conversations into all ghosts | Only Nova needs the historical context as the operations/memory ghost. Other ghosts do not need Nathan's ChatGPT history. | Import into Nova's memory columns only. Other ghosts can query archives if needed. |
| Migrating 39K Archive/ documents to archives table | Most Archive/ documents are Orbis world lore, not temporal records. | Only chat imports (993) and temporal compression products go into archives. Lore stays in documents. |
| Normalizing ghost memory columns into rows | Wide-table with 64 columns works at current scale. EAV normalization adds complexity without benefit at 64 agents. | Keep per-agent columns. Revisit only if agent count exceeds ~200. |
| Perception endpoint rewrite | Too risky during schema migration. Perception is load-bearing for every ghost tick. | Incremental additions (area context, lifestage) after tables stable. |
| Frontend UI for PARAT | Backend-only milestone per PROJECT.md constraints. | Deferred to v1.4. |

## Feature Dependencies

```
areas table -> area_id FK on projects (projects belong to areas)
areas table -> area_id FK on agents/teams (ghosts assigned to areas)
areas table -> area-scoped perception (perception filters by area)

goals FK migration -> projects table (goals.project_id references projects.id)

memories rename -> ALL dpn-api endpoints (vault_notes -> memories)
memories rename -> ALL gotcha-workspace tools referencing vault_notes
memories rename -> dpn-kb frontend queries
memories rename -> tick engine perception

compression tiers -> memories table (compression_tier column)
compression tiers -> archives table (yearly terminus)
compression source tracking -> compressed_from column on memories

archives table -> Nexus Chat AI import pipeline
archives table -> temporal compression terminus
archives table -> immutability triggers

templates table -> ghost identity documents (seed from agents.description)
templates table -> future Innate interpreter (stores .dpn but does not evaluate)

teams table -> agents table (team membership)
teams table -> areas table (teams assigned to areas)
departments lookup -> agents.department normalization

Nexus Chat AI import -> archives table (must exist first)
Nexus Chat AI import -> memories table (must be renamed first)
Nexus Chat AI import -> Nova memory columns (injection target)
Nexus Chat AI import -> temporal compression (cascade after import)
```

## MVP Recommendation

**Phase 1 priority -- Schema foundation (do first, no blast radius):**
1. `areas` table (new, no dependencies)
2. `archives` table (new, no dependencies)
3. `resources` table (new, no dependencies)
4. `templates` table (new, no dependencies)
5. `lifestage` column on `projects` (ALTER TABLE, low risk)
6. `goals.project_id` FK migration (44 rows, manageable)
7. `area_id` FK on `projects` (depends on areas table)

**Phase 2 priority -- Memory migration (high blast radius, do carefully):**
1. `vault_notes` -> `memories` rename (touches everything)
2. Compression tier formalization
3. `compressed_from` source tracking
4. Department normalization + lookup table

**Phase 3 priority -- Organizational structure:**
1. `teams` table + `team_members` junction
2. `ghost_relationships` table (formalized from text arrays)
3. Area assignments for ghosts (agent_areas junction)

**Phase 4 priority -- Import and API surface:**
1. Nexus Chat AI import pipeline (archives + temporal cascade)
2. Nova memory injection
3. dpn-api Rust endpoints for all PARAT tables

**Defer:** Innate interpreter integration, frontend UI, perception rewrite.

## Existing Table Dependencies (77-table schema)

| PARAT Feature | Existing Tables Affected | Nature of Change |
|---------------|--------------------------|------------------|
| Projects lifestage | `projects` (14 rows) | ADD column, backfill 14 rows |
| Goals FK | `goals` (44 rows) | ADD column, migrate data, DROP old text column |
| Areas | `projects`, `agents` | ADD FK columns to both |
| Resources | `documents`, `media` | Referenced by resources, not modified |
| Archives | `documents` | Source data for migration (993 chat imports). Documents unchanged. |
| Memories rename | `vault_notes` (2,668 rows, 64 ghost columns) | RENAME table. Blast radius: dpn-api handlers, dpn-core queries, dpn-kb pages, gotcha-workspace tools, tick engine perception |
| Teams | `agents` (64 rows) | New junction table, agents get team FK |
| Departments | `agents` | Normalize 19 inconsistent values, optional lookup table |
| Perception changes | `af64_perception.rs` | Add area/lifestage context to perception response |
| Conversations | `conversations` (9,534 rows) | Source for future archive entries, blog channel. No schema change. |
| Memory entries | `memory_entries` (889 rows) | Coexists with memories table. Different purpose: memory_entries = semantic vector facts, memories = temporal notes with ghost perspectives |
| Agent daily memory | `agent_daily_memory` | Potential overlap with daily tier in memories. Evaluate whether to merge or keep separate. |

## Sources

- [Tiago Forte - The PARA Method](https://fortelabs.com/blog/para/) -- canonical PARA definition
- [Todoist PARA Guide](https://www.todoist.com/productivity-methods/para-method) -- practical PARA implementation
- [PKM at Scale - 8000 Notes Analysis](https://www.dsebastien.net/personal-knowledge-management-at-scale-analyzing-8-000-notes-and-64-000-links/) -- temporal compression statistics (daily 11.5 links -> yearly 1443.8 links)
- [PostgreSQL State Machines](https://felixge.de/2017/07/27/implementing-state-machines-in-postgresql/) -- lifestage transition patterns
- [Redis ChatGPT Memory Project](https://redis.io/blog/chatgpt-memory-project/) -- conversation history as vector-embedded memory
- Direct schema inspection of master_chronicle (77 tables, 2026-03-28)
