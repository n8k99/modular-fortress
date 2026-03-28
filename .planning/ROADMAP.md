# Roadmap: Noosphere Dispatch Pipeline

## Milestones

- v1.0 Noosphere Dispatch Pipeline (Phases 1-5) -- shipped 2026-03-26
- v1.1 Ghost Coordination Patterns (Phases 6-10) -- shipped 2026-03-27
- v1.2 Operational Readiness (Phases 11-15) -- shipped 2026-03-28
- v1.3 PARAT Noosphere Schema (Phases 16-20) -- in progress

## Phases

<details>
<summary>v1.0 Noosphere Dispatch Pipeline (Phases 1-5) - SHIPPED 2026-03-26</summary>

- [x] **Phase 1: Schema & Dispatch** - Fix tasks table schema and dispatch_to_db.py so GSD plans persist correctly to master_chronicle
- [x] **Phase 2: Perception Pipeline** - Verify and fix perception endpoint so ghosts see dispatched projects and tasks
- [x] **Phase 3: Executive Cognition** - Executives perceive projects, decompose into staff tasks, and delegate via LLM cognition
- [x] **Phase 4: Tool Execution** - Staff ghosts execute real work using code, DB, API, and external tools
- [x] **Phase 5: Feedback & Reporting** - Close the loop with task completion reporting, wave advancement, and blocker escalation

</details>

<details>
<summary>v1.1 Ghost Coordination Patterns (Phases 6-10) - SHIPPED 2026-03-27</summary>

- [x] **Phase 6: Task Dependency Chains** - Wire blocked_by into perception filtering, auto-unblock on completion, and dependency-aware task creation
- [x] **Phase 7: Structured Artifact Passing** - Typed output schemas per pipeline stage replace untyped stage_notes with validated structured JSON
- [x] **Phase 8: Decisions Brain** - Executives consult and log project decisions before acting, queryable via API
- [x] **Phase 9: Verification Levels** - Quality severity classification on task completion with urgency escalation for critical issues
- [x] **Phase 10: Lifecycle Signals** - Staff signal availability after task completion, executives perceive idle agents for delegation

</details>

<details>
<summary>v1.2 Operational Readiness (Phases 11-15) - SHIPPED 2026-03-28</summary>

- [x] **Phase 11: Message Hygiene** - Stop token bleed from stale messages by filtering read messages and marking them processed
- [x] **Phase 12: Standing Orders** - Cron-scheduled project pipelines that ghosts perceive and execute on a recurring basis
- [x] **Phase 13: Operations Pipeline** - Daily health checks, notes, synthesis, podcast watching, and temporal compression under Project #14
- [x] **Phase 14: Editorial Pipeline** - Nightly Thought Police generation under Project #12 owned by Sylvia
- [x] **Phase 15: Financial Pipeline** - Trading briefings across three sessions plus calendar sync under Project #10 owned by Kathryn

</details>

### v1.3 PARAT Noosphere Schema (In Progress)

**Milestone Goal:** Restructure master_chronicle into the PARAT five-pillar architecture with organizational structure, temporal compression, and validated ghost memory injection.

- [ ] **Phase 16: Foundation Tables & API** - Create areas, archives, resources, templates tables with full CRUD endpoints
- [ ] **Phase 17: Projects & Goals Restructuring** - Add lifestage arc, goals FK migration, area linkage, perception enrichment
- [ ] **Phase 18: Memories Rename** - Rename vault_notes to memories with view bridge, compression columns, full Rust migration
- [ ] **Phase 19: Ghost Organizational Structure** - Teams, relationships, multi-area assignment, agent aliases
- [ ] **Phase 20: Nexus Import & Temporal Compression** - Deduplicate, import, compress, inject into Nova memory

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (16.1, 16.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

## Phase Details

<details>
<summary>v1.0 Phase Details (Phases 1-5)</summary>

### Phase 1: Schema & Dispatch
**Goal**: GSD plans persist correctly to master_chronicle
**Depends on**: Nothing (first phase)
**Requirements**: DISP-01, DISP-02, DISP-03
**Plans**: 2/2 plans complete
Plans:
- [x] 01-01-PLAN.md
- [x] 01-02-PLAN.md

### Phase 2: Perception Pipeline
**Goal**: Ghosts see dispatched projects and tasks
**Depends on**: Phase 1
**Requirements**: PERC-01, PERC-02, PERC-03, PERC-04, PERC-05
**Plans**: 2/2 plans complete
Plans:
- [x] 02-01-PLAN.md
- [x] 02-02-PLAN.md

### Phase 3: Executive Cognition
**Goal**: Executives perceive projects, decompose, and delegate
**Depends on**: Phase 2
**Requirements**: EXEC-01, EXEC-02, EXEC-03
**Plans**: 3/3 plans complete
Plans:
- [x] 03-01-PLAN.md
- [x] 03-02-PLAN.md
- [x] 03-03-PLAN.md

### Phase 4: Tool Execution
**Goal**: Staff ghosts execute real work
**Depends on**: Phase 3
**Requirements**: TOOL-01, TOOL-02, TOOL-03
**Plans**: 2/2 plans complete
Plans:
- [x] 04-01-PLAN.md
- [x] 04-02-PLAN.md

### Phase 5: Feedback & Reporting
**Goal**: Close the loop with task completion and escalation
**Depends on**: Phase 4
**Requirements**: REPT-01, REPT-02, REPT-03, REPT-04, REPT-05, REPT-06
**Plans**: 2/2 plans complete
Plans:
- [x] 05-01-PLAN.md
- [x] 05-02-PLAN.md

</details>

<details>
<summary>v1.1 Phase Details (Phases 6-10)</summary>

### Phase 6: Task Dependency Chains
**Goal**: Tasks block and unblock each other automatically
**Depends on**: Phase 5
**Requirements**: DEP-01, DEP-02, DEP-03
**Plans**: 3/3 plans complete
Plans:
- [x] 06-01-PLAN.md
- [x] 06-02-PLAN.md
- [x] 06-03-PLAN.md

### Phase 7: Structured Artifact Passing
**Goal**: Pipeline stages pass typed data, not raw text
**Depends on**: Phase 6
**Requirements**: ART-01, ART-02, ART-03
**Plans**: 3/3 plans complete
Plans:
- [x] 07-01-PLAN.md
- [x] 07-02-PLAN.md
- [x] 07-03-PLAN.md

### Phase 8: Decisions Brain
**Goal**: Executives consult and log decisions
**Depends on**: Phase 7
**Requirements**: DEC-01, DEC-02
**Plans**: 2/2 plans complete
Plans:
- [x] 08-01-PLAN.md
- [x] 08-02-PLAN.md

### Phase 9: Verification Levels
**Goal**: Quality severity classification with urgency escalation
**Depends on**: Phase 8
**Requirements**: VER-01, VER-02
**Plans**: 2/2 plans complete
Plans:
- [x] 09-01-PLAN.md
- [x] 09-02-PLAN.md

### Phase 10: Lifecycle Signals
**Goal**: Staff signal availability, executives perceive idle agents
**Depends on**: Phase 9
**Requirements**: LIFE-01, LIFE-02
**Plans**: 2/2 plans complete
Plans:
- [x] 10-01-PLAN.md
- [x] 10-02-PLAN.md

</details>

<details>
<summary>v1.2 Phase Details (Phases 11-15)</summary>

### Phase 11: Message Hygiene
**Goal**: Ghosts stop wasting tokens on stale messages they have already processed
**Depends on**: Phase 10 (v1.1 complete)
**Requirements**: SPAM-01, SPAM-02, SPAM-03, FIX-01, FIX-02
**Success Criteria** (what must be TRUE):
  1. A ghost that has already processed a message does not see it again in its next perception call
  2. After cognition completes, the agent's ID appears in the read_by array for every message it processed
  3. An agent with zero unread messages after filtering gets no cognition job and burns no tokens
  4. dpn-api can read and write JSONB metadata fields without sqlx errors
  5. A mark-as-read API endpoint exists and correctly appends agent IDs to the read_by array
**Plans:** 2/2 plans complete
Plans:
- [x] 11-01-PLAN.md -- Mark-read API endpoint, perception filter, sqlx verify, historical cleanup
- [x] 11-02-PLAN.md -- Lisp tick-engine mark-read integration and end-to-end verification

### Phase 12: Standing Orders
**Goal**: Ghosts execute recurring project work on a cron schedule without manual dispatch
**Depends on**: Phase 11
**Requirements**: STAND-01, STAND-02, STAND-03
**Success Criteria** (what must be TRUE):
  1. A project with a cron schedule field triggers ghost perception at the scheduled time
  2. The tick engine creates a cognition job for the owning executive when a scheduled project fires
  3. Standing order execution produces conversation output attributed to the responsible ghost, not a system account
**Plans:** 2/2 plans complete
Plans:
- [x] 12-01-PLAN.md -- DB migration, API PATCH endpoint, perception schedule metadata, Lisp cron matcher
- [x] 12-02-PLAN.md -- Tick engine schedule integration, action planner prompt enrichment, E2E verification

### Phase 13: Operations Pipeline
**Goal**: Nova's daily operational cadence runs autonomously as ghost work under Project #14
**Depends on**: Phase 12
**Requirements**: OPS-01, OPS-02, OPS-03, OPS-04
**Success Criteria** (what must be TRUE):
  1. Daily system health check runs on schedule and produces a status report attributed to Nova
  2. Daily notes are populated and nightly synthesis completes as ghost work under Project #14
  3. Podcast watcher checks feeds on schedule and posts new episodes to the appropriate destination
  4. Weekly and monthly finalization (temporal compression) executes with specific agent attribution
**Plans:** 2/2 plans complete
Plans:
- [x] 13-01-PLAN.md -- Tool registry registration, project review tool execution, podcast schedule
- [x] 13-02-PLAN.md -- Action planner label-to-tool mapping, end-to-end verification

### Phase 14: Editorial Pipeline
**Goal**: Sylvia's nightly editorial pipeline runs autonomously as ghost work under Project #12
**Depends on**: Phase 12
**Requirements**: EDIT-01, EDIT-02
**Success Criteria** (what must be TRUE):
  1. Nightly editorial pipeline fires on schedule under Project #12 with Sylvia as the owning executive
  2. Editorial output follows the existing Thought Police format and posts to the correct destination
**Plans:** 1/1 plans complete
Plans:
- [x] 14-01-PLAN.md -- Tool registration, script auth patch, dynamic label-to-tool mapping generalization

### Phase 15: Financial Pipeline
**Goal**: Kathryn's trading briefings and calendar sync run autonomously as ghost work under Project #10
**Depends on**: Phase 12
**Requirements**: FIN-01, FIN-02, OPS-05
**Success Criteria** (what must be TRUE):
  1. Trading briefings for Tokyo, London, and NYC sessions execute on their respective schedules under Project #10
  2. Each briefing session produces structured output posted to the appropriate channel
  3. Wave calendar sync executes as scheduled ghost work under the financial project
**Plans:** 1/1 plans complete
Plans:
- [x] 15-01-PLAN.md -- Tool registry updates, label-to-tool mappings, Calendar Sync schedule

</details>

### Phase 16: Foundation Tables & API
**Goal**: The four new PARAT pillars exist as live tables with seed data and working API endpoints
**Depends on**: Phase 15 (v1.2 complete)
**Requirements**: SCHEMA-01, SCHEMA-02, SCHEMA-03, SCHEMA-04, API-01, API-02, API-03, API-04
**Success Criteria** (what must be TRUE):
  1. Areas table exists with five seeded domains (EM Corp, Orbis, Living Room Music, N8K99/Personal, Infrastructure/Systems) and GET/POST/PATCH endpoints return correct data
  2. Archives table exists with immutability trigger that prevents content UPDATE, and POST creates records while PATCH is blocked on content fields
  3. Resources table exists as a curated index referencing documents/media via source_type/source_id without duplicating data, with frozen enforcement on API
  4. Templates table exists with .dpn body field, JSONB parameters, version history tracking on body changes, and full CRUD endpoints
  5. dpn-api starts cleanly with all new endpoints registered and dpn-core has module files for all four tables
**Plans**: TBD

### Phase 17: Projects & Goals Restructuring
**Goal**: Projects have a growth lifecycle and proper relational integrity with goals and areas
**Depends on**: Phase 16
**Requirements**: SCHEMA-05, SCHEMA-06, SCHEMA-07, API-05
**Success Criteria** (what must be TRUE):
  1. Every project has a lifestage value (Seed/Sapling/Tree/Harvest) with forward-only transition enforced at the database level, and all 14 existing projects are backfilled
  2. All 44 existing goals have integer project_id FK referencing projects table with zero orphaned goals (verified by LEFT JOIN query returning no NULLs)
  3. Projects table has area_id FK to areas, and projects can be queried by area
  4. Perception endpoint includes project lifestage and area context in ghost perception responses
**Plans**: TBD

### Phase 18: Memories Rename
**Goal**: The ghost memory substrate operates under its PARAT-native name with compression metadata and zero disruption to live ghost operations
**Depends on**: Phase 17
**Requirements**: MEM-01, MEM-02, MEM-03, MEM-04, MEM-05, MEM-06
**Success Criteria** (what must be TRUE):
  1. Table is named memories with a vault_notes view providing backward compatibility -- Lisp tick engine and Python tools continue working without changes
  2. Every memory row has a compression_tier value (daily/weekly/monthly/quarterly/yearly) backfilled from existing note_type, and compressed_from INTEGER[] tracks source entries for compressed records
  3. Departments are normalized via lookup table with proper FK from agents, consolidating the 19 inconsistent values into canonical entries
  4. All dpn-api handlers and dpn-core queries reference memories directly (not the view), and dpn-api compiles and starts cleanly
**Plans**: TBD

### Phase 19: Ghost Organizational Structure
**Goal**: Ghosts have formal team membership, typed relationships, multi-area assignments, and identity aliases within the noosphere
**Depends on**: Phase 18
**Requirements**: ORG-01, ORG-02, ORG-03, ORG-04
**Success Criteria** (what must be TRUE):
  1. Teams table exists with lead_id and area_id FKs, and team_members junction table tracks role_in_team for each ghost
  2. ghost_relationships table formalizes reports_to/mentor/mentee/collaborators/liaises_with as typed from_agent/to_agent rows replacing text arrays
  3. agent_areas junction table allows multi-area ghost assignment (e.g., Nova assigned to Operations AND cross-functional areas)
  4. Agents table has aliases text[] column and Nova's record includes T.A.S.K.S. as an alias
**Plans**: TBD

### Phase 20: Nexus Import & Temporal Compression
**Goal**: Historical ChatGPT conversations are archived, temporally compressed, and injected into Nova's ghost memory as operational context
**Depends on**: Phase 16, Phase 18
**Requirements**: IMPORT-01, IMPORT-02, IMPORT-03, IMPORT-04, IMPORT-05
**Success Criteria** (what must be TRUE):
  1. Nexus Chat AI documents are deduplicated across Archive paths with a canonical set identified and duplicate paths documented
  2. Deduplicated conversations exist in the archives table with source_type='chatgpt_import', extracted dates, and topic metadata
  3. Monthly, quarterly, and yearly summary memories exist in the memories table generated from grouped imported conversations via deterministic compression
  4. Nova/T.A.S.K.S. ghost memory columns contain synthesized perspectives on imported content at each temporal tier
  5. Relevant daily/weekly notes contain markdown links to imported archive content without corruption of existing note data
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 16 -> 17 -> 18 -> 19 -> 20

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Schema & Dispatch | v1.0 | 2/2 | Complete | 2026-03-26 |
| 2. Perception Pipeline | v1.0 | 2/2 | Complete | 2026-03-26 |
| 3. Executive Cognition | v1.0 | 3/3 | Complete | 2026-03-26 |
| 4. Tool Execution | v1.0 | 2/2 | Complete | 2026-03-26 |
| 5. Feedback & Reporting | v1.0 | 2/2 | Complete | 2026-03-26 |
| 6. Task Dependency Chains | v1.1 | 3/3 | Complete | 2026-03-26 |
| 7. Structured Artifact Passing | v1.1 | 3/3 | Complete | 2026-03-26 |
| 8. Decisions Brain | v1.1 | 2/2 | Complete | 2026-03-26 |
| 9. Verification Levels | v1.1 | 2/2 | Complete | 2026-03-26 |
| 10. Lifecycle Signals | v1.1 | 2/2 | Complete | 2026-03-27 |
| 11. Message Hygiene | v1.2 | 2/2 | Complete | 2026-03-27 |
| 12. Standing Orders | v1.2 | 2/2 | Complete | 2026-03-28 |
| 13. Operations Pipeline | v1.2 | 2/2 | Complete | 2026-03-28 |
| 14. Editorial Pipeline | v1.2 | 1/1 | Complete | 2026-03-28 |
| 15. Financial Pipeline | v1.2 | 1/1 | Complete | 2026-03-28 |
| 16. Foundation Tables & API | v1.3 | 0/? | Not started | - |
| 17. Projects & Goals Restructuring | v1.3 | 0/? | Not started | - |
| 18. Memories Rename | v1.3 | 0/? | Not started | - |
| 19. Ghost Organizational Structure | v1.3 | 0/? | Not started | - |
| 20. Nexus Import & Temporal Compression | v1.3 | 0/? | Not started | - |
