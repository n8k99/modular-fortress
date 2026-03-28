# Requirements: Noosphere Dispatch Pipeline v1.2

**Defined:** 2026-03-27
**Core Value:** GSD-dispatched projects must flow through to ghost execution and back without human intervention -- executives plan, staff execute, results report themselves.

## v1.2 Requirements

### Message Hygiene

- [x] **SPAM-01**: Perception endpoint filters out messages already in the agent's read_by array
- [x] **SPAM-02**: Action executor marks processed messages as read (appends agent_id to read_by) after cognition completes
- [x] **SPAM-03**: Agent with zero actionable items after read_by filtering is classified as idle (no cognition job, no token spend)

### Infrastructure Fixes

- [x] **FIX-01**: dpn-api Cargo.toml includes sqlx "json" feature so JSONB metadata reads/writes work correctly
- [x] **FIX-02**: Mark-as-read API endpoint exists to update read_by array on conversations

### Standing Orders Framework

- [x] **STAND-01**: Projects table supports a schedule field (cron expression) that triggers ghost perception at scheduled times
- [ ] **STAND-02**: Tick engine recognizes scheduled projects and creates cognition jobs for the owning executive at the scheduled time
- [ ] **STAND-03**: Standing order execution produces conversation output attributed to the responsible ghost

### Pipeline Migration -- Financial

- [ ] **FIN-01**: Trading briefings (Tokyo/London/NYC) execute as ghost pipeline under Project #10, owned by Kathryn
- [ ] **FIN-02**: Each briefing session produces structured output posted to the appropriate channel

### Pipeline Migration -- Editorial

- [ ] **EDIT-01**: Nightly editorial pipeline executes as ghost pipeline under Project #12, owned by Sylvia
- [ ] **EDIT-02**: Editorial output follows the existing Thought Police format and posts to the correct destination

### Pipeline Migration -- Operations

- [ ] **OPS-01**: Daily system health check executes as ghost work under Project #14, owned by Nova
- [ ] **OPS-02**: Daily note population and nightly synthesis execute as ghost work attributed to Nova
- [ ] **OPS-03**: Podcast watcher runs on schedule, checks feeds, posts new episodes
- [ ] **OPS-04**: Weekly and monthly finalization (temporal compression) execute as ghost work with specific agent attribution
- [ ] **OPS-05**: Wave calendar sync executes as ghost work under the financial project

## Future Requirements

### From v1.0/v1.1 Deferred

- **TOOL-04**: External tools (web search, URL fetch, embedding generation) -- no implementations exist yet

### Potential v2.0

- **AUTO-01**: Cross-department task handoffs
- **AUTO-02**: Ghost-initiated subtask creation (staff break own tasks)
- **AUTO-03**: Automatic wave dependency graph construction
- **AUTO-04**: Cost tracking per project (aggregate LLM spend by project_id)
- **DYN-01**: Relationship-aware delegation (dynamic teams)
- **DYN-02**: Org graph project team visualization

## Out of Scope

| Feature | Reason |
|---------|--------|
| Discord bot migration | Part of Project #5 Digital Sovereignty, not this pipeline |
| Ghost-to-ghost negotiation | Deadlock risk; executives mediate |
| Real-time activity streaming | Async DB reporting sufficient |
| Frontend/UI changes | Backend pipeline only |
| Tick engine rewrite | Extend, don't replace |
| Dynamic agent spawning | Fixed roster of 64 ghosts |
| OpenClaw conversation polling | Ghosts handle this natively via perception |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SPAM-01 | Phase 11 | Planned |
| SPAM-02 | Phase 11 | Planned |
| SPAM-03 | Phase 11 | Planned |
| FIX-01 | Phase 11 | Planned |
| FIX-02 | Phase 11 | Planned |
| STAND-01 | Phase 12 | Planned |
| STAND-02 | Phase 12 | Planned |
| STAND-03 | Phase 12 | Planned |
| OPS-01 | Phase 13 | Planned |
| OPS-02 | Phase 13 | Planned |
| OPS-03 | Phase 13 | Planned |
| OPS-04 | Phase 13 | Planned |
| EDIT-01 | Phase 14 | Planned |
| EDIT-02 | Phase 14 | Planned |
| FIN-01 | Phase 15 | Planned |
| FIN-02 | Phase 15 | Planned |
| OPS-05 | Phase 15 | Planned |

**Coverage:**
- v1.2 requirements: 17 total
- Mapped to phases: 17
- Unmapped: 0

---
*Requirements defined: 2026-03-27*
*Last updated: 2026-03-27*
