# Requirements: Noosphere Dispatch Pipeline

**Defined:** 2026-03-29
**Core Value:** GSD-dispatched projects must flow through to ghost execution and back without human intervention

## v1.4 Requirements

Requirements for Ghost Sovereignty milestone. Each maps to roadmap phases.

### Direct PostgreSQL

- [x] **DB-01**: Perception queries run as SQL from Lisp tick engine, returning the same data shape as /api/perception/:agent_id — messages, tasks, projects, documents, team activity
- [x] **DB-02**: Agent state updates (energy, tier, last_tick_at) written directly via SQL from Lisp, bypassing HTTP PATCH
- [x] **DB-03**: Conversations (read, write, mark-read) executed via SQL from Lisp without HTTP — including read_by array operations
- [x] **DB-04**: Task mutations (create, update status, complete, blocked_by management) executed via SQL from Lisp

### Innate Integration

- [x] **INNATE-01**: Noosphere resolver connects Innate's @, (), {} symbols to master_chronicle tables — @ resolves entities (projects, agents, areas, templates), () addresses agents, {} provides scope/filtering
- [ ] **INNATE-02**: Ghosts evaluate .dpn Template bodies during cognition, receiving resolved content as actionable context that informs their planning and execution
- [ ] **INNATE-03**: Ghosts compose valid Innate .dpn expressions to create or modify Templates via the interpreter's generation capabilities
- [ ] **INNATE-04**: Daily Note template Innate expressions — (agent){action} patterns like (sarah_lin){sync_calendar} and (kathryn){finance_positions} — execute during ghost operations, triggering real tool invocations

## Previous Milestone Requirements (v1.0-v1.3)

All shipped. See .planning/milestones/ for details.

## Future Requirements

### Frontend (v1.5)

- **FRONT-01**: em-site surfaces executive blog content authored by ghosts
- **FRONT-02**: em-site displays organizational structure (teams, departments, org chart)
- **FRONT-03**: dpn-tui updated for PARAT table access
- **FRONT-04**: dpn-api-client updated for new endpoints

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
| Removing dpn-api entirely | dpn-api still serves Next.js frontends, MCP tools — only ghost-to-DB path changes |
| Full Innate language v2 features | Scope is noosphere resolver + template eval + generation, not language evolution |
| Multi-substrate resolver | Only noosphere (master_chronicle) resolver needed — generic resolvers are future |
| Frontend UI for Innate | Backend-only — no .dpn editor or template UI this milestone |
| Innate error recovery UX | Ghosts handle evaluation errors via existing error handling, not new UX |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| DB-01 | Phase 21 | Complete |
| DB-02 | Phase 21 | Complete |
| DB-03 | Phase 22 | Complete |
| DB-04 | Phase 22 | Complete |
| INNATE-01 | Phase 23 | Complete |
| INNATE-02 | Phase 24 | Pending |
| INNATE-03 | Phase 25 | Pending |
| INNATE-04 | Phase 24 | Pending |

**Coverage:**
- v1.4 requirements: 8 total
- Mapped to phases: 8
- Unmapped: 0

---
*Requirements defined: 2026-03-29*
*Last updated: 2026-03-29 after v1.4 roadmap creation*
