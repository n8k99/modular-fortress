# Roadmap: Noosphere Dispatch Pipeline

## Milestones

- v1.0 Noosphere Dispatch Pipeline (Phases 1-5) -- shipped 2026-03-26
- v1.1 Ghost Coordination Patterns (Phases 6-10) -- shipped 2026-03-27
- v1.2 Operational Readiness (Phases 11-15) -- shipped 2026-03-28
- v1.3 PARAT Noosphere Schema (Phases 16-20) -- shipped 2026-03-29
- v1.4 Ghost Sovereignty (Phases 21-25) -- in progress

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

### v1.4 Ghost Sovereignty (In Progress)

**Milestone Goal:** Ghosts speak directly to the noosphere via PostgreSQL and evaluate Innate .dpn Templates as executable instructions -- removing the HTTP middleman and giving ghosts a native language.

- [ ] **Phase 21: Direct PostgreSQL Foundation** - Lisp tick engine connects to PostgreSQL and runs perception + state updates as SQL, replacing HTTP calls
- [ ] **Phase 22: Conversations & Tasks Direct** - Conversations and task mutations run as SQL from Lisp, completing the HTTP-to-SQL migration
- [ ] **Phase 23: Noosphere Resolver** - Innate's @, (), {} symbols resolve against master_chronicle tables via a Lisp resolver module
- [ ] **Phase 24: Template Evaluation & Execution** - Ghosts evaluate .dpn Template bodies during cognition and Daily Note (agent){action} patterns trigger real tool invocations
- [ ] **Phase 25: Ghost Expression Generation** - Ghosts compose valid Innate .dpn expressions to create or modify Templates

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

### Phase 21: Direct PostgreSQL Foundation
**Goal**: Ghosts perceive the noosphere and update their own state via direct SQL, eliminating HTTP round-trips for the core tick cycle
**Depends on**: Phase 20 (v1.3 complete)
**Requirements**: DB-01, DB-02
**Success Criteria** (what must be TRUE):
  1. The Lisp tick engine opens a persistent PostgreSQL connection at startup using native SBCL socket I/O (no Quicklisp, following AF64 zero-deps convention)
  2. `SELECT` queries from Lisp return the same perception data shape (messages, tasks, projects, documents, team activity) as the current `/api/perception/:agent_id` HTTP endpoint, verifiable by comparing JSON output
  3. Agent state updates (energy, tier, last_tick_at) written via `UPDATE` from Lisp are immediately visible in `SELECT agent_state` queries
  4. The tick engine completes a full perceive-rank-classify cycle using SQL instead of HTTP without exceeding the current tick interval
**Plans**: 3 plans
Plans:
- [x] 21-01-PLAN.md -- libpq FFI bindings, connection pool, db-query/db-execute wrappers
- [x] 21-02-PLAN.md -- Perception SQL queries, fetch-agents/fetch-fitness via SQL, rewire perception.lisp
- [x] 21-03-PLAN.md -- State update SQL functions, rewire energy.lisp and tick-engine phase-update-state

### Phase 22: Conversations & Tasks Direct
**Goal**: All ghost-to-noosphere communication (conversations and task mutations) runs as SQL, completing the removal of HTTP from the ghost tick path
**Depends on**: Phase 21
**Requirements**: DB-03, DB-04
**Success Criteria** (what must be TRUE):
  1. Ghosts read unread conversations via SQL with `read_by` array filtering identical to the current API behavior
  2. Ghosts write new conversation messages and mark messages as read via SQL, with `read_by` array append operations working correctly
  3. Task creation, status updates, completion, and `blocked_by` management execute as SQL from Lisp with the same semantics as the current HTTP endpoints
  4. After this phase, zero HTTP calls from the tick engine to dpn-api remain in the ghost-to-noosphere path (dpn-api still serves frontends)
**Plans**: 3 plans
Plans:
- [x] 22-01-PLAN.md -- SQL wrapper functions (db-conversations, db-tasks, db-auxiliary) + ASDF/package wiring
- [x] 22-02-PLAN.md -- Rewire action-executor.lisp (39 calls) and tick-engine.lisp (2 calls) to SQL
- [x] 22-03-PLAN.md -- Rewire action-planner.lisp (11 calls) and 7 auxiliary files (11 calls) to SQL

### Phase 23: Noosphere Resolver
**Goal**: Innate's symbolic references (@, (), {}) resolve against master_chronicle tables, connecting the language to the noosphere
**Depends on**: Phase 21
**Requirements**: INNATE-01
**Success Criteria** (what must be TRUE):
  1. `@project_name` resolves to the matching project row from `projects` table, returning its id, status, goals, and owner
  2. `@area_name`, `@template_name`, `@agent_name` resolve to their respective table rows (areas, templates, agents)
  3. `(agent_name)` resolves to the agent record with id, department, energy, tier, and current assignments
  4. `{scope_filter}` narrows queries -- e.g., `@projects{status=active}` returns only active projects
  5. Resolution errors (missing entity, ambiguous match) return structured error values that the Innate interpreter handles without crashing
**Plans**: 2 plans
Plans:
- [x] 23-01-PLAN.md -- Cross-repo wiring, package definition, resolve-reference, resolve-search
- [x] 23-02-PLAN.md -- deliver-commission, resolve-wikilink, resolve-context, load-bundle, integration verification

### Phase 24: Template Evaluation & Execution
**Goal**: Ghosts read .dpn Templates from the noosphere, evaluate their Innate expressions, and Daily Note (agent){action} patterns trigger real tool invocations during operations
**Depends on**: Phase 22, Phase 23
**Requirements**: INNATE-02, INNATE-04
**Success Criteria** (what must be TRUE):
  1. A ghost's cognition job includes evaluated Template content -- Innate expressions in the Template body are resolved to concrete values before the LLM prompt is built
  2. Template evaluation results inform ghost planning: an executive reading a Template with `@projects{status=blocked}` sees the actual blocked projects, not the raw expression
  3. `(sarah_lin){sync_calendar}` in a Daily Note template triggers the calendar sync tool invocation during ghost operations, producing real output
  4. `(kathryn){finance_positions}` triggers the trading positions tool, with output attributed to Kathryn in the conversations table
  5. Evaluation errors in a Template do not crash the tick -- the ghost receives an error context and can skip or report the failure
**Plans**: 2 plans
Plans:
- [x] 24-01-PLAN.md -- Evaluator loading, package imports, evaluate-template-for-project helper, build-project-review-job integration
- [x] 24-02-PLAN.md -- Test template insertion, SBCL compilation verification, commission delivery end-to-end test

### Phase 25: Ghost Expression Generation
**Goal**: Ghosts compose valid Innate .dpn expressions to create or modify Templates, closing the loop where ghosts both read and write their native language
**Depends on**: Phase 24
**Requirements**: INNATE-03
**Success Criteria** (what must be TRUE):
  1. A ghost can generate a syntactically valid Innate expression (e.g., `@project_name{status=active}`) as part of its cognition output
  2. Generated expressions pass the Innate interpreter's parser without errors
  3. A ghost can create a new Template row in the `templates` table with a body containing Innate expressions, and that Template is evaluable by other ghosts in subsequent ticks
  4. A ghost can modify an existing Template's body, and the updated version evaluates correctly on the next read
**Plans**: 2 plans
Plans:
- [x] 25-01-PLAN.md -- Builder functions, validation, slug generation, template CRUD, package wiring
- [x] 25-02-PLAN.md -- Cognition pipeline integration (action-planner prompts + action-executor extraction/persistence)

## Progress

| Phase | Milestone | Plans | Status | Completed |
|-------|-----------|-------|--------|-----------|
| 1-5 | v1.0 | 11/11 | Complete | 2026-03-26 |
| 6-10 | v1.1 | 12/12 | Complete | 2026-03-27 |
| 11-15 | v1.2 | 8/8 | Complete | 2026-03-28 |
| 16-20 | v1.3 | 14/14 | Complete | 2026-03-29 |
| 21. Direct PostgreSQL Foundation | v1.4 | 3/3 | Complete    | 2026-03-29 |
| 22. Conversations & Tasks Direct | v1.4 | 3/3 | Complete    | 2026-03-29 |
| 23. Noosphere Resolver | v1.4 | 2/2 | Complete    | 2026-03-29 |
| 24. Template Evaluation & Execution | v1.4 | 2/2 | Complete   | 2026-03-29 |
| 25. Ghost Expression Generation | v1.4 | 2/2 | Complete   | 2026-03-30 |
