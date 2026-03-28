# Phase 17: Projects & Goals Restructuring - Context

**Gathered:** 2026-03-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Add lifestage lifecycle column to projects, migrate goals.project from text wikilinks to integer FK, add area_id FK from projects to areas table, and enrich ghost perception responses with lifestage and area context. Modifications to existing tables only — no new tables created.

</domain>

<decisions>
## Implementation Decisions

### Lifestage Enum & Backfill
- **D-01:** Add `lifestage` column as VARCHAR with CHECK constraint for values: Seed, Sapling, Tree, Harvest. Not a PostgreSQL ENUM type (easier to extend later).
- **D-02:** Backfill mapping based on project maturity:
  - `completed` status → Harvest (Project DragonPunk id=1)
  - `paused` status → Seed (Project Noosphere Ghosts id=3)
  - Long-running active projects with standing orders → Tree (id=5,6,7,9,10,12,13,14,16,17,51)
  - Recently created active projects → Sapling (id=56 GSD-E2E-Test, id=59 Innatescript)
- **D-03:** Forward-only transition enforced by database trigger (not CHECK constraint — CHECK can't reference prior row values). Trigger prevents: Harvest→anything, Tree→Seed/Sapling, Sapling→Seed. Allows: Seed→Sapling→Tree→Harvest (forward only).

### Goals FK Migration
- **D-04:** Add `project_id INTEGER REFERENCES projects(id)` column to goals table (nullable — allows orphaned goals during transition).
- **D-05:** Migration script parses existing `project` text values (wikilink format: `[[Project DragonPunk]]`, `{{"Project DragonPunk"}}`) and maps to projects.id by name match. Mapping:
  - `[[Project DragonPunk]]` and `{{"Project DragonPunk"}}` → projects.id = 1
  - `[[Project GOTCHA]]` → NULL (no matching project — GOTCHA was OpenClaw, now deprecated)
  - `[[Project Puppet Show]]` → NULL (no matching project in current roster)
- **D-06:** After migration, the original `project` text column is kept for reference but marked deprecated. Not dropped in this phase to avoid breaking any readers.

### Projects → Areas FK
- **D-07:** Add `area_id INTEGER REFERENCES areas(id)` column to projects table (nullable — standalone projects don't need an area).
- **D-08:** Backfill area assignments based on project domain:
  - Infrastructure/Systems area: Project Noosphere Ghosts (3), Noosphere Dispatch Pipeline (51), Modular Fortress (13), Project Digital Sovereignty (5)
  - EM Corp area: Project Complete Success (10), Cognitive Submission (12), Operation Normality (14), Project Dancing Contingency (6)
  - Orbis area: Sovereign Realms of Orbis (16), Historical Lore of Orbis (17), The Soulcrusher's Exodus (9), Project DragonPunk (1)
  - Living Room Music area: PROJECT MIDI-OSIN (7)
  - N8K99/Personal area: GSD-E2E-Test-Phase5 (56), Innatescript (59)

### Perception Enrichment
- **D-09:** Add `lifestage` and `area_name` fields to the projects section of perception responses. Minimal change — LEFT JOIN areas on projects.area_id, include both fields in the JSON output.
- **D-10:** No perception endpoint rewrite. Incremental addition only, per PROJECT.md out-of-scope constraint.

### Claude's Discretion
- Index choices for new FK columns
- Exact wikilink parsing regex in migration script
- Order of ALTER TABLE statements
- Whether to add a composite index on (area_id, status) for area-based project queries

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Schema Specification
- `.planning/REQUIREMENTS.md` — SCHEMA-05, SCHEMA-06, SCHEMA-07, API-05 (acceptance criteria for this phase)
- `PARAT-NoosphereSchema.docx` — Original PARAT architecture document

### Existing Code (Modification Targets)
- `/opt/dpn-api/src/handlers/af64_perception.rs` — Perception endpoint (lines 423-498 for projects section)
- `/opt/dpn-api/src/handlers/projects.rs` — Existing project handlers (will need lifestage/area support)
- `/root/dpn-core/src/db/projects.rs` — Project struct and CRUD functions (need new fields)
- `/opt/dpn-core/src/db/projects.rs` — Operational copy (must be synced)

### Phase 16 Outputs (Dependencies)
- `.planning/phases/16-foundation-tables-api/16-CONTEXT.md` — Areas table decisions (D-01/D-02 seed data, D-03 DB trigger philosophy)
- `/root/migrations/16-parat-foundation-tables.sql` — Areas table DDL (owner is VARCHAR(64) REFERENCES agents(id))

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `areas` table already exists with 5 seeded domains (Phase 16) — ready for FK references
- `Project` struct in dpn-core (`id, name, slug, status, description, owner, schedule`) — needs lifestage + area_id fields added
- `update_updated_at_column()` trigger function — reusable for updated_at on modified tables
- `ApiError::Conflict` variant (Phase 16) — available if needed for lifestage transition violations

### Established Patterns
- DB trigger enforcement for business rules (Phase 16: immutability, frozen, version history)
- dpn-core struct + async CRUD functions pattern (Phase 16: areas, archives, resources, templates)
- Perception endpoint enrichment via LEFT JOIN (existing pattern for projects section)

### Integration Points
- `projects` table: ALTER TABLE ADD COLUMN (lifestage, area_id)
- `goals` table: ALTER TABLE ADD COLUMN (project_id)
- `/opt/dpn-api/src/handlers/af64_perception.rs`: perception query modification
- `/root/dpn-core/src/db/projects.rs` + `/opt/dpn-core/src/db/projects.rs`: struct field additions

</code_context>

<specifics>
## Specific Ideas

- The goals.project text column currently uses mixed wikilink formats (`[[...]]` and `{{...}}`). Migration must handle both.
- 15 projects exist currently (not 14 as originally estimated in REQUIREMENTS.md — Innatescript id=59 was added recently).
- Project GOTCHA and Project Puppet Show referenced in goals have no matching projects table entries — these are historical/deprecated references.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 17-projects-goals-restructuring*
*Context gathered: 2026-03-28*
