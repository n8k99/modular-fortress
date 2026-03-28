# Requirements: Noosphere Dispatch Pipeline

**Defined:** 2026-03-28
**Core Value:** GSD-dispatched projects must flow through to ghost execution and back without human intervention

## v1.3 Requirements

Requirements for PARAT Noosphere Schema milestone. Each maps to roadmap phases.

### Schema Foundation

- [x] **SCHEMA-01**: Areas table exists with id, name, slug, description, owner (agent FK), status, created_at, updated_at
- [x] **SCHEMA-02**: Archives table exists with immutability enforcement (trigger prevents content UPDATE), source_type/source_id tracking, period_start/period_end dates, full-text search
- [x] **SCHEMA-03**: Resources table exists as curated index referencing documents/media via source_type/source_id (no data duplication), with type categorization and frozen flag
- [x] **SCHEMA-04**: Templates table exists with body field for .dpn expressions, parameters JSONB schema, category, version tracking, and templates_history for previous versions
- [x] **SCHEMA-05**: Projects table has lifestage enum column (Seed/Sapling/Tree/Harvest) with forward-only transition constraint and backfilled values for existing 14 projects
- [x] **SCHEMA-06**: Goals table has proper project_id integer FK to projects, migrated from text project field, with all 44 existing goals mapped
- [x] **SCHEMA-07**: Projects table has area_id FK to areas table (nullable for standalone projects)

### Memory Migration

- [x] **MEM-01**: vault_notes table renamed to memories with VIEW bridge (vault_notes view) preserving backward compatibility for Lisp and Python code
- [x] **MEM-02**: memories table has compression_tier enum (daily/weekly/monthly/quarterly/yearly) backfilled from existing note_type values
- [x] **MEM-03**: memories table has compressed_from INTEGER[] column tracking which source entries each compressed entry summarizes
- [x] **MEM-04**: Departments normalized via lookup table with proper FK from agents, consolidating 19 inconsistent values
- [x] **MEM-05**: All dpn-api Rust endpoints updated from vault_notes to memories (structs, queries, handlers)
- [x] **MEM-06**: All dpn-core Rust queries updated from vault_notes to memories

### Ghost Organization

- [x] **ORG-01**: Teams table exists with name, department, lead_id (agent FK), area_id (area FK), plus team_members junction table with role_in_team
- [x] **ORG-02**: ghost_relationships table exists formalizing reports_to, mentor, mentee, collaborators, liaises_with from existing text arrays into typed from_agent/to_agent/relationship_type rows
- [x] **ORG-03**: agent_areas junction table exists for multi-area ghost assignment (Nova touches Operations AND cross-functional)
- [x] **ORG-04**: Agents table has aliases text[] column supporting dual identities (Nova IS T.A.S.K.S.)

### Nexus Import + Temporal Compression

- [ ] **IMPORT-01**: Nexus Chat AI documents deduplicated across Archive/Retired Nebulab/ and Archive/backup-Nebulab/ paths, with canonical set identified
- [ ] **IMPORT-02**: Deduplicated Nexus Chat AI conversations imported into archives table with source_type='chatgpt_import', extracted dates, and topic metadata
- [ ] **IMPORT-03**: Imported archive entries temporally cascaded into memories: monthly summaries generated from grouped conversations, then quarterly and yearly
- [ ] **IMPORT-04**: Nova/T.A.S.K.S. ghost memory columns populated with synthesized perspectives on imported content at each temporal tier
- [ ] **IMPORT-05**: Relevant daily/weekly notes receive markdown links to imported archive content without corrupting existing data

### API Surface

- [x] **API-01**: dpn-api has CRUD endpoints for areas table (GET list, GET by id, POST create, PATCH update)
- [x] **API-02**: dpn-api has CRUD endpoints for archives table (GET list, GET by id, POST create — no UPDATE on content fields, respecting immutability)
- [x] **API-03**: dpn-api has CRUD endpoints for resources table (GET list, GET by id, POST create, PATCH update, frozen enforcement)
- [x] **API-04**: dpn-api has CRUD endpoints for templates table (GET list, GET by id, POST create, PATCH update) with version history on body changes
- [x] **API-05**: Perception endpoint includes area context and project lifestage in ghost perception responses

## Previous Milestone Requirements (v1.0-v1.2)

All shipped. See MILESTONES.md for details.

## Future Requirements

### Frontend (v1.4)

- **FRONT-01**: em-site surfaces executive blog content authored by ghosts
- **FRONT-02**: em-site displays organizational structure (teams, departments, org chart)
- **FRONT-03**: dpn-tui updated for PARAT table access
- **FRONT-04**: dpn-api-client updated for new endpoints

### Ghost Direct DB (v1.5)

- **DIRECT-01**: Tick engine uses direct PostgreSQL instead of dpn-api HTTP layer
- **DIRECT-02**: Perception queries run as SQL from Lisp, not via REST endpoint

### Potential Future

- **AUTO-01**: Cross-department task handoffs
- **AUTO-02**: Ghost-initiated subtask creation
- **AUTO-03**: Automatic wave dependency graph construction
- **AUTO-04**: Cost tracking per project (aggregate LLM spend by project_id)
- **DYN-01**: Relationship-aware delegation (dynamic teams)
- **DYN-02**: Org graph project team visualization

## Out of Scope

| Feature | Reason |
|---------|--------|
| Migrating all 48K documents into resources | Resources is a curated index, not a warehouse |
| Renaming agents table to ghosts | 8+ FK references, excessive blast radius -- add ghost columns instead |
| Automated lifestage transitions | Lifestage is human/executive judgment, not rule-based |
| Deep Innate interpreter integration | Separate project -- templates store .dpn text but evaluation is future |
| Normalizing ghost memory columns into rows | Wide-table with 64 columns works at current scale (~64 agents) |
| Perception endpoint rewrite | Too risky during schema migration -- incremental additions only |
| Frontend UI for PARAT | Backend-only milestone, deferred to v1.4 |
| Migrating 39K Archive/ documents to archives table | Most are Orbis world lore, not temporal records |
| Importing Nexus Chat into all ghosts | Only Nova needs historical context as operations/memory ghost |
| Discord bot migration | Part of Project #5 Digital Sovereignty |
| Ghost-to-ghost negotiation | Deadlock risk; executives mediate |
| Real-time activity streaming | Async DB reporting sufficient |
| Tick engine rewrite | Extend, don't replace |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| SCHEMA-01 | Phase 16 | Complete |
| SCHEMA-02 | Phase 16 | Complete |
| SCHEMA-03 | Phase 16 | Complete |
| SCHEMA-04 | Phase 16 | Complete |
| SCHEMA-05 | Phase 17 | Complete |
| SCHEMA-06 | Phase 17 | Complete |
| SCHEMA-07 | Phase 17 | Complete |
| MEM-01 | Phase 18 | Complete |
| MEM-02 | Phase 18 | Complete |
| MEM-03 | Phase 18 | Complete |
| MEM-04 | Phase 18 | Complete |
| MEM-05 | Phase 18 | Complete |
| MEM-06 | Phase 18 | Complete |
| ORG-01 | Phase 19 | Complete |
| ORG-02 | Phase 19 | Complete |
| ORG-03 | Phase 19 | Complete |
| ORG-04 | Phase 19 | Complete |
| IMPORT-01 | Phase 20 | Pending |
| IMPORT-02 | Phase 20 | Pending |
| IMPORT-03 | Phase 20 | Pending |
| IMPORT-04 | Phase 20 | Pending |
| IMPORT-05 | Phase 20 | Pending |
| API-01 | Phase 16 | Complete |
| API-02 | Phase 16 | Complete |
| API-03 | Phase 16 | Complete |
| API-04 | Phase 16 | Complete |
| API-05 | Phase 17 | Complete |

**Coverage:**
- v1.3 requirements: 27 total
- Mapped to phases: 27
- Unmapped: 0

---
*Requirements defined: 2026-03-28*
*Last updated: 2026-03-28 after roadmap creation*
