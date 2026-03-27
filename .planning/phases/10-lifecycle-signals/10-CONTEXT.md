# Phase 10: Lifecycle Signals - Context

**Gathered:** 2026-03-27
**Status:** Ready for planning

<domain>
## Phase Boundary

Staff ghosts automatically signal idle/ready-for-next when their task queue empties, executives perceive agent availability (status, energy, task count) in their team roster, and idle agents get an energy boost so they're ready for immediate delegation.

</domain>

<decisions>
## Implementation Decisions

### Idle Signal Mechanism
- **D-01:** Automatic detection — when a staff ghost completes its last assigned task (task queue becomes empty), the tick engine automatically sets an 'idle' flag on the agent record. No explicit LLM command needed.
- **D-02:** The idle flag is set during Phase 5 (update state) of the tick engine, after classification. If an agent was classified as "idle" (not in acting set, has no actionable items), update the agent state to reflect this.
- **D-03:** The idle flag is cleared when the agent next has actionable items in perception (tasks, messages, requests).

### Executive Availability View
- **D-04:** Enhance `format-team-roster` to include status (idle/working/dormant), energy level, and open task count per agent. Executive sees: `casey (systems-engineer) — IDLE, energy: 65, tasks: 0`.
- **D-05:** Uses existing agent API data (tier, energy) plus a task count query per agent. No new API endpoint needed — enrich the existing roster formatting.

### Energy-Availability Alignment
- **D-06:** One-time energy boost (+10-15) when an agent transitions to idle state (last task completed). This ensures idle agents quickly reach working/prime tier energy levels and are prioritized for delegation.
- **D-07:** The boost applies only on transition to idle, not on every idle tick. Regular +5/tick rest continues as normal for sustained idle periods.

### Claude's Discretion
- Exact energy boost value on idle transition (within 10-15 range)
- Whether to add a `lifecycle_state` column to agent_state or use existing tier/status fields
- How to query task count per agent efficiently in format-team-roster (single query vs per-agent)
- Whether idle agents should appear first in the team roster (sorted by availability)

### Carried From Prior Phases
- Phase 5: Completion reports and wave advancement
- Phase 9: Quality assessment and urgency boosts in tick engine
- Existing: Idle agents classified in Phase 3 of tick, rest at +5 energy/tick
- Existing: format-team-roster shows id, name, role — no availability info
- Existing: Tier thresholds: dormant ≤0, base 0-20, working 20-70, prime >70

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Tick Engine (Idle Classification)
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — Key: Phase 2 ranking (lines 134-218), Phase 3 classification (idle detection), Phase 5 state update (lines 366-383), `determine-tier` (lines 82-92)

### Energy System
- `/opt/project-noosphere-ghosts/lisp/runtime/energy.lisp` — Energy costs, rewards, cap mechanism, `update-energy` function

### Action Planner (Team Roster)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — Key: `format-team-roster` (lines 735-757), `build-project-review-job` (lines 759-842)

### Agent State API
- `/opt/dpn-api/src/handlers/` — Agent state PATCH endpoint for tier/status updates

### Perception
- `/opt/dpn-api/src/handlers/af64_perception.rs` — Executive perception response structure

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `format-team-roster` — already fetches all agents in department. Enhance with additional fields.
- `determine-tier` — already computes tier from energy + fitness. Can derive idle state.
- `update-energy` — existing function for applying energy deltas.
- Phase 5 state update — already PATCHes agent state per tick. Add idle flag here.
- `has-actionable-items` in perception.lisp — already checks if agent has messages/tasks/projects.

### Established Patterns
- Agent state update: Phase 5 of tick engine PATCHes tier, last-tick-at, ticks-alive
- Energy rewards: applied via `update-energy` with named deltas
- Perception-based classification: agent gets perception → ranked by urgency → classified as acting/idle/dormant
- Team roster: fetched via API, filtered by department, formatted as markdown

### Integration Points
- `tick-engine.lisp` Phase 3 (classify) — detect idle transition, apply energy boost
- `tick-engine.lisp` Phase 5 (update state) — set idle flag on agent record
- `action-planner.lisp` `format-team-roster` — add status, energy, task count
- Agent state API — accept and store idle/lifecycle fields

</code_context>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 10-lifecycle-signals*
*Context gathered: 2026-03-27*
