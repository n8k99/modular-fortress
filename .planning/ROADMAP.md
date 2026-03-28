# Roadmap: Noosphere Dispatch Pipeline

## Milestones

- v1.0 Noosphere Dispatch Pipeline (Phases 1-5) -- shipped 2026-03-26
- v1.1 Ghost Coordination Patterns (Phases 6-10) -- shipped 2026-03-27
- v1.2 Operational Readiness (Phases 11-15) -- in progress

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

### v1.2 Operational Readiness (In Progress)

- [x] **Phase 11: Message Hygiene** - Stop token bleed from stale messages by filtering read messages and marking them processed (completed 2026-03-27)
- [x] **Phase 12: Standing Orders** - Cron-scheduled project pipelines that ghosts perceive and execute on a recurring basis (completed 2026-03-28)
- [x] **Phase 13: Operations Pipeline** - Daily health checks, notes, synthesis, podcast watching, and temporal compression under Project #14 (completed 2026-03-28)
- [ ] **Phase 14: Editorial Pipeline** - Nightly Thought Police generation under Project #12 owned by Sylvia
- [ ] **Phase 15: Financial Pipeline** - Trading briefings across three sessions plus calendar sync under Project #10 owned by Kathryn

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (6.1, 6.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

## Phase Details

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
**Plans**: TBD

### Phase 15: Financial Pipeline
**Goal**: Kathryn's trading briefings and calendar sync run autonomously as ghost work under Project #10
**Depends on**: Phase 12
**Requirements**: FIN-01, FIN-02, OPS-05
**Success Criteria** (what must be TRUE):
  1. Trading briefings for Tokyo, London, and NYC sessions execute on their respective schedules under Project #10
  2. Each briefing session produces structured output posted to the appropriate channel
  3. Wave calendar sync executes as scheduled ghost work under the financial project
**Plans**: TBD

## Progress

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
| 11. Message Hygiene | v1.2 | 2/2 | Complete    | 2026-03-27 |
| 12. Standing Orders | v1.2 | 2/2 | Complete    | 2026-03-28 |
| 13. Operations Pipeline | v1.2 | 2/2 | Complete   | 2026-03-28 |
| 14. Editorial Pipeline | v1.2 | 0/? | Not started | - |
| 15. Financial Pipeline | v1.2 | 0/? | Not started | - |
