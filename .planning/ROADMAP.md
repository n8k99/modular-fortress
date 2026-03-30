# Roadmap: Noosphere Dispatch Pipeline

## Milestones

- v1.0 Noosphere Dispatch Pipeline (Phases 1-5) -- shipped 2026-03-26
- v1.1 Ghost Coordination Patterns (Phases 6-10) -- shipped 2026-03-27
- v1.2 Operational Readiness (Phases 11-15) -- shipped 2026-03-28
- v1.3 PARAT Noosphere Schema (Phases 16-20) -- shipped 2026-03-29
- v1.4 Ghost Sovereignty (Phases 21-25) -- shipped 2026-03-30
- v1.5 InnateScipt Capabilities (Phases 26-31) -- in progress

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

<details>
<summary>v1.3 PARAT Noosphere Schema (Phases 16-20) - SHIPPED 2026-03-29</summary>

See `.planning/milestones/v1.3-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v1.4 Ghost Sovereignty (Phases 21-25) - SHIPPED 2026-03-30</summary>

- [x] **Phase 21: Direct PostgreSQL Foundation** - Lisp tick engine connects to PostgreSQL via libpq FFI and runs perception + state updates as SQL
- [x] **Phase 22: Conversations & Tasks Direct** - All 63 HTTP calls replaced with direct SQL, completing zero-HTTP tick engine
- [x] **Phase 23: Noosphere Resolver** - Innate's @, (), {} symbols resolve against master_chronicle tables via CLOS resolver
- [x] **Phase 24: Template Evaluation & Execution** - Ghosts evaluate .dpn Templates during cognition, commission delivery triggers real tools
- [x] **Phase 25: Ghost Expression Generation** - Ghosts compose valid Innate expressions via LLM prompts with parse-round-trip validation

</details>

### v1.5 InnateScipt Capabilities (In Progress)

**Milestone Goal:** Replace the static tool registry with InnateScipt-defined ghost capabilities -- every ghost's YAML declares what it can do as live InnateScipt expressions, with executive oversight and team pipeline definitions.

- [x] **Phase 26: Runtime Stability** - Fix the execute-work-task paren bug and commit all tick engine fixes before building on top
- [x] **Phase 27: Area Content Tables** - Structured area-scoped content tables for Eckenrode Muziekopname replacing flat documents
- [ ] **Phase 28: Ghost Capabilities** - YAML-defined InnateScipt responsibilities replace tool-registry.json for ghost capability declaration and cognition
- [ ] **Phase 29: Orbis Foundation** - YAML coordinates, ship assignment, and RPG persona fields for ghost spatial identity
- [ ] **Phase 30: Team Pipelines** - YAML-defined pipeline handoff chains replace hardcoded pipeline advancement logic
- [ ] **Phase 31: Tool Migration** - Existing Python tools wrapped as InnateScipt expressions, tool-registry.json retired

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
<details>
<summary>v1.3 Phase Details (Phases 16-20)</summary>

See `.planning/milestones/v1.3-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v1.4 Phase Details (Phases 21-25)</summary>

See `.planning/milestones/v1.4-ROADMAP.md` for full phase details.

</details>

### v1.5 Phase Details (Phases 26-31)

### Phase 26: Runtime Stability
**Goal**: Tick engine runs without known bugs so subsequent phases build on a solid foundation
**Depends on**: Phase 25 (v1.4 complete)
**Requirements**: STAB-01, STAB-02
**Success Criteria** (what must be TRUE):
  1. The execute-work-task function returns its json-object result from the correct let* scope without paren mismatch
  2. All 7 tick engine fixes from the 2026-03-29 session (UTF-8 pg-escape, NULL handling, tilde SQL, type coercion, description column) are committed and loadable by SBCL without errors
  3. A full tick cycle completes without runtime errors on the live system
**Plans**: 3 plans
Plans:
- [x] 26-01-PLAN.md -- Fix paren bug and commit all 9 tick engine fixes
- [x] 26-02-PLAN.md -- Live tick cycle verification and human sign-off
- [x] 26-03-PLAN.md -- Gap closure: fix outer let* scope in execute-work-task

### Phase 27: Area Content Tables
**Goal**: Eckenrode Muziekopname has structured content tables that the noosphere resolver can query by area scope
**Depends on**: Phase 26
**Requirements**: AREA-01, AREA-02, AREA-03
**Success Criteria** (what must be TRUE):
  1. EM area has one or more dedicated content tables with columns appropriate to its content domain
  2. Content records are linked to the EM area via FK relationship and queryable by area scope
  3. An InnateScipt expression like {em.content} or equivalent resolves to area-scoped content via the noosphere resolver
**Plans**: 2 plans
Plans:
- [x] 27-01-PLAN.md -- Create area_content table and populate from EM documents
- [x] 27-02-PLAN.md -- Extend noosphere resolver load-bundle for area content queries

### Phase 28: Ghost Capabilities
**Goal**: Ghosts declare what they can do as InnateScipt expressions in their YAML, replacing the static tool registry for capability discovery and cognition
**Depends on**: Phase 26
**Requirements**: CAP-01, CAP-02, CAP-03, CAP-04, CAP-05, CAP-06, CAP-07
**Success Criteria** (what must be TRUE):
  1. A ghost's YAML file contains a responsibilities: section with valid InnateScipt expressions describing its capabilities
  2. The tick engine reads capabilities from ghost YAML instead of tool-registry.json when determining what a ghost can do
  3. The action planner includes the ghost's InnateScipt capabilities in LLM cognition prompts so the LLM knows what tools/actions are available
  4. A ghost can add, edit, or remove its own responsibility expressions via cognition output, with parse-round-trip validation before persistence
  5. An executive ghost can modify a subordinate's responsibility expressions (add new capabilities, prune outdated ones)
**Plans**: 4 plans
Plans:
- [x] 28-01-PLAN.md -- YAML parser, ghost-capabilities module, 9 agent YAML files
- [x] 28-02-PLAN.md -- Action planner capability injection in all 4 prompt builders
- [x] 28-03-PLAN.md -- Responsibility self-modification and executive override
- [ ] 28-04-PLAN.md -- Gap closure: wire mutation support into proactive-work path

### Phase 29: Orbis Foundation
**Goal**: Ghosts have spatial identity in the Orbis world via YAML-defined coordinates, ship assignment, and RPG persona
**Depends on**: Phase 26
**Requirements**: ORBIS-01, ORBIS-02, ORBIS-03
**Success Criteria** (what must be TRUE):
  1. Each ghost's YAML has starting_point with x/y coordinates derived from Pantheon Formation ship assignment
  2. Ghost YAML includes ship_assignment and rpg_persona fields that are loadable by the tick engine
  3. Trust and energy thresholds for Orbis access are defined in ghost YAML and readable at runtime
**Plans**: [To be planned]

### Phase 30: Team Pipelines
**Goal**: Department and team pipelines are defined in YAML with explicit handoff chains, replacing hardcoded pipeline advancement
**Depends on**: Phase 28
**Requirements**: PIPE-01, PIPE-02, PIPE-03, PIPE-04
**Success Criteria** (what must be TRUE):
  1. Department/team YAML files contain an assignments: section defining pipeline steps with ghost assignments per step
  2. The tick engine reads pipeline definitions from YAML and routes handoffs between ghosts according to the defined step sequence
  3. Each task in a pipeline tracks its current step and next ghost in task metadata, advancing automatically on step completion
**Plans**: [To be planned]

### Phase 31: Tool Migration
**Goal**: All existing Python tools are accessible as InnateScipt expressions, and tool-registry.json is retired
**Depends on**: Phase 27, Phase 28
**Requirements**: TOOL-01, TOOL-02, TOOL-03, TOOL-04
**Success Criteria** (what must be TRUE):
  1. Kalshi, trading, ops, and other Python scripts have InnateScipt expression wrappers in ghost YAML responsibilities
  2. The noosphere resolver can invoke Python scripts when evaluating an InnateScipt tool expression (e.g., a commission triggers script execution)
  3. tool-registry.json is deleted and no code path references it -- all tool discovery flows through InnateScipt capabilities
  4. Tool execution results flow back through the cognition pipeline as conversation output attributed to the executing ghost
**Plans**: [To be planned]

## Progress

| Phase | Milestone | Plans | Status | Completed |
|-------|-----------|-------|--------|-----------|
| 1-5 | v1.0 | 11/11 | Complete | 2026-03-26 |
| 6-10 | v1.1 | 12/12 | Complete | 2026-03-27 |
| 11-15 | v1.2 | 8/8 | Complete | 2026-03-28 |
| 16-20 | v1.3 | 14/14 | Complete | 2026-03-29 |
| 21-25 | v1.4 | 12/12 | Complete | 2026-03-30 |
| 26. Runtime Stability | v1.5 | 3/3 | Complete    | 2026-03-30 |
| 27. Area Content Tables | v1.5 | 2/2 | Complete    | 2026-03-30 |
| 28. Ghost Capabilities | v1.5 | 3/4 | Gap closure | 2026-03-30 |
| 29. Orbis Foundation | v1.5 | 0/? | Not started | - |
| 30. Team Pipelines | v1.5 | 0/? | Not started | - |
| 31. Tool Migration | v1.5 | 0/? | Not started | - |
