# Roadmap: Noosphere Dispatch Pipeline

## Overview

This project repairs the broken connections between GSD planning and Noosphere Ghost execution. The pipeline already exists architecturally -- the tick engine runs, the perception endpoint is coded (401 lines), the dispatch script exists, the DB schema is mostly there. The work is fixing broken INSERT statements, verifying data flows, then building the executive cognition and tool execution layers that turn ghosts from passive observers into autonomous workers. Five phases progress from schema repair through to a closed feedback loop where Nathan only intervenes for blockers.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Schema & Dispatch** - Fix tasks table schema and dispatch_to_db.py so GSD plans persist correctly to master_chronicle
- [ ] **Phase 2: Perception Pipeline** - Verify and fix perception endpoint so ghosts see dispatched projects and tasks
- [ ] **Phase 3: Executive Cognition** - Executives perceive projects, decompose into staff tasks, and delegate via LLM cognition
- [ ] **Phase 4: Tool Execution** - Staff ghosts execute real work using code, DB, API, and external tools
- [ ] **Phase 5: Feedback & Reporting** - Close the loop with task completion reporting, wave advancement, and blocker escalation

## Phase Details

### Phase 1: Schema & Dispatch
**Goal**: GSD-planned projects and tasks persist correctly to master_chronicle with all required metadata
**Depends on**: Nothing (first phase)
**Requirements**: SCHM-01, SCHM-02, SCHM-03, SCHM-04, SCHM-05
**Success Criteria** (what must be TRUE):
  1. Running `dispatch_to_db.py` against a GSD-planned project writes project and task records without errors
  2. Tasks in the DB have project_id linking them to their parent project, source field showing GSD origin, and context field with plan must_haves
  3. Dispatched tasks carry department routing derived from the project owner's executive domain
  4. Running `dispatch_to_db.py --status` returns accurate project and task counts and statuses from the live DB
**Plans**: 2 plans

Plans:
- [x] 01-01-PLAN.md — Fix dispatch_project with H1 extraction, owner column, department routing, and test scaffold
- [x] 01-02-PLAN.md — Fix dispatch_phase with hierarchical subtasks and enhanced show_status

### Phase 2: Perception Pipeline
**Goal**: Ghosts perceive dispatched work through the perception API with correct urgency and filtering
**Depends on**: Phase 1
**Requirements**: PERC-01, PERC-02, PERC-03, PERC-04, PERC-05
**Success Criteria** (what must be TRUE):
  1. Calling /api/perception/:agent_id for an executive returns their owned projects with status and goals
  2. Calling /api/perception/:agent_id for a staff agent returns tasks assigned to them with project context and must_haves
  3. Project ownership triggers the +15/project urgency boost in tick engine ranking (no longer dead code)
  4. Tasks with future scheduled_at dates are filtered out -- ghosts only see work that is ready now
**Plans**: 2 plans

Plans:
- [ ] 02-01: TBD
- [ ] 02-02: TBD

### Phase 3: Executive Cognition
**Goal**: Executive ghosts autonomously decompose dispatched projects into staff-suitable tasks and delegate them
**Depends on**: Phase 2
**Requirements**: EXEC-01, EXEC-02, EXEC-03, EXEC-04, EXEC-05
**Success Criteria** (what must be TRUE):
  1. An executive ghost that perceives a dispatched project produces a structured task breakdown via LLM cognition
  2. The task breakdown respects wave ordering from GSD dispatch context (wave 1 tasks before wave 2)
  3. Decomposed subtasks appear in the tasks table with correct project_id, assigned agent, and wave metadata
  4. Across multiple ticks, the executive monitors delegated task progress and adjusts priorities as staff complete or block on work
**Plans**: 2 plans

Plans:
- [ ] 03-01: TBD
- [ ] 03-02: TBD
- [ ] 03-03: TBD

### Phase 4: Tool Execution
**Goal**: Staff ghosts execute real work using authorized tools and validate results before marking tasks complete
**Depends on**: Phase 3
**Requirements**: TOOL-01, TOOL-02, TOOL-03, TOOL-04, TOOL-05, TOOL-06
**Success Criteria** (what must be TRUE):
  1. A staff ghost assigned a code task invokes Claude Code CLI to read/write files or run commands, and the result is persisted
  2. A staff ghost can query or mutate master_chronicle via dpn-api DB tools and API tools
  3. Tool execution respects agent tool_scope -- a ghost without code tool authorization cannot invoke Claude Code CLI
  4. Tool execution results are validated (output checked, not just "I did it") before the task status moves to done
  5. External tools (web search, URL fetch) are available to authorized ghosts for research-type tasks
**Plans**: 2 plans

Plans:
- [ ] 04-01: TBD
- [ ] 04-02: TBD
- [ ] 04-03: TBD

### Phase 5: Feedback & Reporting
**Goal**: Execution results flow back through the system so Nathan sees real progress and only gets pulled in for blockers
**Depends on**: Phase 4
**Requirements**: REPT-01, REPT-02, REPT-03, REPT-04, REPT-05, REPT-06
**Success Criteria** (what must be TRUE):
  1. When a staff ghost completes a task, a completion report appears in the conversations table addressed to the supervising executive
  2. Project and task status fields in the DB reflect actual execution state (open to in_progress to done) without manual updates
  3. When all tasks in wave N are done, wave N+1 tasks become perceivable to assigned ghosts (wave advancement works)
  4. A staff ghost that hits a blocker posts it to conversations, and the executive perceives it with elevated urgency
  5. Running /gsd:progress or dispatch --status shows real execution state including per-wave completion and blocker count
**Plans**: 2 plans

Plans:
- [ ] 05-01: TBD
- [ ] 05-02: TBD
- [ ] 05-03: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Schema & Dispatch | 0/2 | Not started | - |
| 2. Perception Pipeline | 0/2 | Not started | - |
| 3. Executive Cognition | 0/3 | Not started | - |
| 4. Tool Execution | 0/3 | Not started | - |
| 5. Feedback & Reporting | 0/3 | Not started | - |
