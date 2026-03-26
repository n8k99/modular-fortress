# Requirements: Noosphere Dispatch Pipeline

**Defined:** 2026-03-26
**Core Value:** GSD-dispatched projects must flow through to ghost execution and back without human intervention

## v1 Requirements

### Schema & Dispatch

- [x] **SCHM-01**: Tasks table has columns for project linkage (`project_id`), source tracking (`source`), and GSD context (`context`)
- [x] **SCHM-02**: dispatch_to_db.py successfully writes project records to projects table with owner, goals, and status
- [x] **SCHM-03**: dispatch_to_db.py successfully writes task records to tasks table with project linkage and wave metadata
- [x] **SCHM-04**: Dispatched tasks include department routing derived from project owner's domain
- [x] **SCHM-05**: dispatch_to_db.py `--status` shows accurate project and task status from DB

### Perception

- [x] **PERC-01**: /api/perception/:agent_id returns dispatched project-linked tasks for the agent
- [x] **PERC-02**: Executive agents perceive projects they own with current status and goals
- [x] **PERC-03**: Staff agents perceive tasks assigned to them with project context and must_haves
- [x] **PERC-04**: Project ownership triggers urgency boost (+15/project) in tick engine ranking
- [x] **PERC-05**: Perception filters tasks by scheduled_at so ghosts only see ready work

### Executive Planning

- [x] **EXEC-01**: Executive ghost receives dispatched project and uses LLM cognition to decompose into staff-suitable tasks
- [x] **EXEC-02**: Executive task breakdown respects wave ordering from GSD dispatch context
- [ ] **EXEC-03**: Executive assigns decomposed tasks to staff ghosts based on domain expertise and tool_scope
- [ ] **EXEC-04**: Decomposed subtasks are written to tasks table via API with project_id linkage
- [ ] **EXEC-05**: Executive monitors progress of delegated tasks across ticks and re-prioritizes as needed

### Tool Execution

- [ ] **TOOL-01**: Staff ghosts execute code tools via Claude Code CLI (read/write files, run commands, git ops)
- [ ] **TOOL-02**: Staff ghosts execute DB tools (query/mutate master_chronicle via dpn-api)
- [ ] **TOOL-03**: Staff ghosts execute API tools (call dpn-api endpoints for doc creation, task updates, messages)
- [ ] **TOOL-04**: Staff ghosts execute external tools (web search, URL fetch, embedding generation)
- [ ] **TOOL-05**: Tool execution respects agent tool_scope -- ghosts only use tools they're authorized for
- [ ] **TOOL-06**: Tool execution results are validated before task is marked complete (anti-hallucination)

### Reporting & Feedback

- [ ] **REPT-01**: Task completion posts a report to conversations table (from staff, to executive)
- [ ] **REPT-02**: Project/task status in DB reflects actual execution state (open → in_progress → done)
- [ ] **REPT-03**: Wave advancement: when all tasks in wave N complete, wave N+1 tasks become perceivable
- [ ] **REPT-04**: Blocker escalation: staff ghost posts blocker to conversations, executive perceives with high urgency
- [ ] **REPT-05**: /gsd:progress (or dispatch --status) shows real execution state of dispatched projects
- [ ] **REPT-06**: Nathan only receives conversation notifications for blockers and strategic decisions

## v2 Requirements

### Advanced Autonomy

- **AUTO-01**: Cross-department task handoffs (e.g., Eliana's engineering task needs Vincent's visual assets)
- **AUTO-02**: Ghost-initiated subtask creation (staff can break their own tasks further)
- **AUTO-03**: Automatic wave dependency graph construction from task metadata
- **AUTO-04**: Cost tracking per project (aggregate LLM token spend by project_id)

### Hardening

- **HARD-01**: File write sandboxing beyond path allowlists
- **HARD-02**: Dollar-denominated budget cap per project in cognition broker
- **HARD-03**: Automatic rollback on failed tool execution sequences
- **HARD-04**: Semantic coherence checks between pipeline stages (not just format validation)

## Out of Scope

| Feature | Reason |
|---------|--------|
| Ghost-to-ghost negotiation | Deadlock risk; ghosts execute dispatched work, don't create projects |
| Real-time activity streaming | Async DB reporting sufficient for 30s-10m tick intervals |
| Frontend/UI changes (dpn-kb, org graph) | Backend pipeline only; existing frontends read same DB |
| Tick engine rewrite | Extend existing architecture, don't replace it |
| Multi-droplet distribution | Single node constraint; all services on one droplet |
| Complex DAG workflow engine | Simple wave numbers (1, 2, 3) sufficient for v1 |
| Dynamic agent spawning | Destroys persistent identity model; fixed roster of 64 ghosts |
| Retry/backoff systems | Tick engine + cognitive winter + energy gating = natural backoff |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SCHM-01 | Phase 1 | Complete |
| SCHM-02 | Phase 1 | Complete |
| SCHM-03 | Phase 1 | Complete |
| SCHM-04 | Phase 1 | Complete |
| SCHM-05 | Phase 1 | Complete |
| PERC-01 | Phase 2 | Complete |
| PERC-02 | Phase 2 | Complete |
| PERC-03 | Phase 2 | Complete |
| PERC-04 | Phase 2 | Complete |
| PERC-05 | Phase 2 | Complete |
| EXEC-01 | Phase 3 | Complete |
| EXEC-02 | Phase 3 | Complete |
| EXEC-03 | Phase 3 | Pending |
| EXEC-04 | Phase 3 | Pending |
| EXEC-05 | Phase 3 | Pending |
| TOOL-01 | Phase 4 | Pending |
| TOOL-02 | Phase 4 | Pending |
| TOOL-03 | Phase 4 | Pending |
| TOOL-04 | Phase 4 | Pending |
| TOOL-05 | Phase 4 | Pending |
| TOOL-06 | Phase 4 | Pending |
| REPT-01 | Phase 5 | Pending |
| REPT-02 | Phase 5 | Pending |
| REPT-03 | Phase 5 | Pending |
| REPT-04 | Phase 5 | Pending |
| REPT-05 | Phase 5 | Pending |
| REPT-06 | Phase 5 | Pending |

**Coverage:**
- v1 requirements: 27 total
- Mapped to phases: 27
- Unmapped: 0

---
*Requirements defined: 2026-03-26*
*Last updated: 2026-03-26 after roadmap creation*
