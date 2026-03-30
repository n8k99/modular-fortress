# Requirements: Noosphere Dispatch Pipeline

**Defined:** 2026-03-29
**Core Value:** GSD-dispatched projects must flow through to ghost execution and back without human intervention -- executives plan, staff execute, results report themselves.

## v1.5 Requirements

Requirements for InnateScipt Capabilities milestone. Each maps to roadmap phases.

### Ghost Capabilities

- [x] **CAP-01**: Ghost YAML has responsibilities: section with InnateScipt expressions defining its capabilities
- [x] **CAP-02**: Tick engine evaluates ghost YAML capabilities instead of looking up tool-registry.json
- [x] **CAP-03**: Action planner injects ghost's InnateScipt capabilities into LLM cognition prompts
- [x] **CAP-04**: Ghost can write new InnateScipt expressions to its own responsibilities via cognition output
- [x] **CAP-05**: Ghost can edit/remove its own responsibility expressions via cognition output
- [x] **CAP-06**: Executive ghost can edit subordinate ghost responsibilities (add/prune capabilities)
- [x] **CAP-07**: Capability changes validated via InnateScipt parse-round-trip before persistence

### Team Pipelines

- [ ] **PIPE-01**: Department/team YAML has assignments: section defining pipeline handoff chains
- [ ] **PIPE-02**: Pipeline definitions specify step sequence with ghost assignment per step
- [ ] **PIPE-03**: Tick engine routes pipeline handoffs using YAML definitions instead of hardcoded *pipeline-advancement*
- [ ] **PIPE-04**: Pipeline state tracked per-task (current step, next ghost) in task metadata

### Area Content

- [x] **AREA-01**: Eckenrode Muziekopname area has structured table(s) for its content
- [x] **AREA-02**: Content records scoped under areas via FK relationships
- [x] **AREA-03**: Noosphere resolver can query area-scoped content via InnateScipt

### Tool Migration

- [ ] **TOOL-01**: Existing Python tools (Kalshi, trading, ops) wrapped as InnateScipt expressions
- [ ] **TOOL-02**: Noosphere resolver can invoke Python scripts when evaluating InnateScipt tool expressions
- [ ] **TOOL-03**: tool-registry.json retired -- all tool access flows through InnateScipt capabilities
- [ ] **TOOL-04**: Tool execution results flow back through the same cognition pipeline

### Orbis Foundation

- [ ] **ORBIS-01**: Ghost YAML has starting_point coordinates (x, y) from Pantheon Formation ship assignment
- [ ] **ORBIS-02**: Ghost YAML has ship_assignment and rpg_persona fields
- [ ] **ORBIS-03**: Trust and energy thresholds for Orbis access defined in ghost YAML

### Runtime Stability

- [x] **STAB-01**: execute-work-task paren scope bug fixed -- return json-object executes in correct let* scope
- [x] **STAB-02**: All 7 tick engine fixes from 2026-03-29 session committed to project-noosphere-ghosts

## Future Requirements (v1.6+)

### Orbis Exploration

- **ORBIS-04**: Drunkard's Walk movement per tick for high-energy ghosts
- **ORBIS-05**: Ghosts encounter and edit/append world objects (biomes, routes, rivers, burgs, provinces, markers)
- **ORBIS-06**: Ghost movement paths tracked and visualizable on Orbis map (~v1.9)

### Dynamic Teams

- **TEAM-01**: Relationship-aware delegation based on ghost_relationships table
- **TEAM-02**: Org graph project team visualization

## Out of Scope

| Feature | Reason |
|---------|--------|
| Drunkard's Walk movement system | v1.6+ -- v1.5 lays YAML foundation only |
| Orbis map visualization of ghost paths | ~v1.9 -- "grits and shingles" |
| Ghost-to-ghost negotiation | Ghosts execute dispatched work, don't create projects |
| Discord output bridge | External delivery concern, not noosphere |
| Multi-droplet distribution | Single node constraint |
| Frontend UI changes | Backend pipeline only |
| dpn-tui / dpn-api-client | Deferred downstream effects |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| STAB-01 | Phase 26 | Complete |
| STAB-02 | Phase 26 | Complete |
| AREA-01 | Phase 27 | Complete |
| AREA-02 | Phase 27 | Complete |
| AREA-03 | Phase 27 | Complete |
| CAP-01 | Phase 28 | Complete |
| CAP-02 | Phase 28 | Complete |
| CAP-03 | Phase 28 | Complete |
| CAP-04 | Phase 28 | Complete |
| CAP-05 | Phase 28 | Complete |
| CAP-06 | Phase 28 | Complete |
| CAP-07 | Phase 28 | Complete |
| ORBIS-01 | Phase 29 | Pending |
| ORBIS-02 | Phase 29 | Pending |
| ORBIS-03 | Phase 29 | Pending |
| PIPE-01 | Phase 30 | Pending |
| PIPE-02 | Phase 30 | Pending |
| PIPE-03 | Phase 30 | Pending |
| PIPE-04 | Phase 30 | Pending |
| TOOL-01 | Phase 31 | Pending |
| TOOL-02 | Phase 31 | Pending |
| TOOL-03 | Phase 31 | Pending |
| TOOL-04 | Phase 31 | Pending |

**Coverage:**
- v1.5 requirements: 23 total
- Mapped to phases: 23
- Unmapped: 0

---
*Requirements defined: 2026-03-29*
*Last updated: 2026-03-29 after roadmap creation*
