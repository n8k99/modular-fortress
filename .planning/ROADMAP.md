# Roadmap: Noosphere Dispatch Pipeline

## Milestones

- v1.0 Noosphere Dispatch Pipeline (Phases 1-5) -- shipped 2026-03-26
- v1.1 Ghost Coordination Patterns (Phases 6-10) -- in progress

## Phases

<details>
<summary>v1.0 Noosphere Dispatch Pipeline (Phases 1-5) - SHIPPED 2026-03-26</summary>

- [x] **Phase 1: Schema & Dispatch** - Fix tasks table schema and dispatch_to_db.py so GSD plans persist correctly to master_chronicle
- [x] **Phase 2: Perception Pipeline** - Verify and fix perception endpoint so ghosts see dispatched projects and tasks
- [x] **Phase 3: Executive Cognition** - Executives perceive projects, decompose into staff tasks, and delegate via LLM cognition
- [x] **Phase 4: Tool Execution** - Staff ghosts execute real work using code, DB, API, and external tools
- [x] **Phase 5: Feedback & Reporting** - Close the loop with task completion reporting, wave advancement, and blocker escalation

</details>

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (6.1, 6.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 6: Task Dependency Chains** - Wire blocked_by into perception filtering, auto-unblock on completion, and dependency-aware task creation
- [ ] **Phase 7: Structured Artifact Passing** - Typed output schemas per pipeline stage replace untyped stage_notes with validated structured JSON
- [ ] **Phase 8: Decisions Brain** - Executives consult and log project decisions before acting, queryable via API
- [ ] **Phase 9: Verification Levels** - Quality severity classification on task completion with urgency escalation for critical issues
- [ ] **Phase 10: Lifecycle Signals** - Staff signal availability after task completion, executives perceive idle agents for delegation

## Phase Details

### Phase 6: Task Dependency Chains
**Goal**: Ghosts only see tasks whose dependencies are satisfied, and completing a task automatically unblocks downstream work
**Depends on**: Phase 5 (v1.0 completion reporting and wave advancement)
**Requirements**: DEP-01, DEP-02, DEP-03, DEP-04
**Success Criteria** (what must be TRUE):
  1. A task with blocked_by pointing to an incomplete task does NOT appear in the perception response for its assigned agent
  2. When a blocking task is marked complete, all tasks referencing it in blocked_by become perceivable within the next tick
  3. An executive ghost creating subtasks via CREATE_TASK can specify blocked_by references and the dependency is persisted
  4. dispatch_to_db.py automatically sets blocked_by for wave 2+ subtasks based on wave ordering from GSD plans
**Plans:** 3 plans

Plans:
- [x] 06-01-PLAN.md -- Schema migration (INTEGER to INTEGER[]) and trigger extension for auto-unblocking
- [x] 06-02-PLAN.md -- Perception filtering for blocked tasks and task API INTEGER[] support
- [x] 06-03-PLAN.md -- Lisp CREATE_TASK parser blocked_by= syntax and dispatch wave dependency population

### Phase 7: Structured Artifact Passing
**Goal**: Pipeline stage outputs are typed and validated, so the next assignee receives structured context instead of freeform text
**Depends on**: Phase 6 (dependency chains enable ordered pipeline progression)
**Requirements**: ART-01, ART-02, ART-03
**Success Criteria** (what must be TRUE):
  1. Each pipeline stage (spec, design, code, test, review) has a defined JSON schema, and stage_notes for completed tasks contain structured output matching that schema
  2. When a task advances to the next pipeline stage, the new assignee's perception context includes the structured output from the previous stage as input
  3. The action executor rejects a COMPLETE command if the stage output does not match the expected schema for that pipeline stage
**Plans**: TBD

### Phase 8: Decisions Brain
**Goal**: Executives have shared memory of project decisions so they act consistently and don't contradict prior choices
**Depends on**: Phase 5 (v1.0 executive cognition and project review)
**Requirements**: DEC-01, DEC-02, DEC-03
**Success Criteria** (what must be TRUE):
  1. When an executive reviews a project, the LLM prompt includes recent decisions from the decisions table for that project
  2. When an executive makes a decision during project review, it appears in the decisions table with the correct project_id and agent attribution
  3. Calling GET /api/decisions?project_id=X returns all decisions for that project in chronological order
**Plans**: TBD

### Phase 9: Verification Levels
**Goal**: Task completion quality is assessed with severity levels so executives can prioritize rework on critical issues
**Depends on**: Phase 5 (v1.0 completion reporting flow)
**Requirements**: VER-01, VER-02, VER-03
**Success Criteria** (what must be TRUE):
  1. When a staff ghost completes a task, the completion report in conversations includes a severity classification (CRITICAL, WARNING, or SUGGESTION) for any quality issues found
  2. An executive perceives tasks with CRITICAL verification issues at higher urgency than normal task updates
  3. Staff ghost output includes a structured quality assessment block alongside the COMPLETE command that the action executor parses and persists
**Plans**: TBD

### Phase 10: Lifecycle Signals
**Goal**: Executives know which staff are available for new work so they can delegate immediately instead of waiting for the next tick cycle
**Depends on**: Phase 5 (v1.0 energy system and tick engine)
**Requirements**: LIFE-01, LIFE-02, LIFE-03
**Success Criteria** (what must be TRUE):
  1. After a staff ghost completes its last assigned task, it signals IDLE and this state is visible in the system
  2. When an executive reviews a project, the context includes a list of idle staff agents available for delegation
  3. An idle agent's energy level reflects availability (not drained from recent work) so the tick engine correctly prioritizes it for new assignments
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 6 -> 7 -> 8 -> 9 -> 10

Note: Phases 8, 9, and 10 depend only on v1.0 (Phase 5), not on each other. They are sequenced for orderly execution but could theoretically parallelize.

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Schema & Dispatch | v1.0 | 2/2 | Complete | 2026-03-26 |
| 2. Perception Pipeline | v1.0 | 2/2 | Complete | 2026-03-26 |
| 3. Executive Cognition | v1.0 | 3/3 | Complete | 2026-03-26 |
| 4. Tool Execution | v1.0 | 2/2 | Complete | 2026-03-26 |
| 5. Feedback & Reporting | v1.0 | 2/2 | Complete | 2026-03-26 |
| 6. Task Dependency Chains | v1.1 | 0/3 | Planning | - |
| 7. Structured Artifact Passing | v1.1 | 0/0 | Not started | - |
| 8. Decisions Brain | v1.1 | 0/0 | Not started | - |
| 9. Verification Levels | v1.1 | 0/0 | Not started | - |
| 10. Lifecycle Signals | v1.1 | 0/0 | Not started | - |
