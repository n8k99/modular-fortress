# Phase 10: Lifecycle Signals - Research

**Researched:** 2026-03-27
**Domain:** Common Lisp tick engine (idle detection, energy), Rust API (agent state), PostgreSQL (agent_state table)
**Confidence:** HIGH

## Summary

This phase adds lifecycle signal awareness to the ghost tick engine. Staff ghosts that complete all their assigned tasks should automatically be flagged as idle, executives reviewing projects should see which staff are available for delegation, and idle agents should receive an energy boost to be immediately ready for new work.

The implementation is well-scoped: all three changes touch existing, well-understood code paths. The tick engine already classifies agents as idle (Phase 3), the `format-team-roster` function already fetches all department agents, and the energy system already has a reward mechanism. The work is adding a `lifecycle_state` field to `agent_state.metadata` JSONB, enriching `format-team-roster` output, and applying a one-time energy boost on idle transition.

**Primary recommendation:** Store lifecycle_state in the existing `agent_state.metadata` JSONB column (no schema migration needed), detect idle transition in tick engine Phase 3 by comparing previous tick status, apply +12 energy boost on transition, and enrich `format-team-roster` with status/energy/task-count using a single batch query.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Automatic detection -- when a staff ghost completes its last assigned task (task queue becomes empty), the tick engine automatically sets an 'idle' flag on the agent record. No explicit LLM command needed.
- **D-02:** The idle flag is set during Phase 5 (update state) of the tick engine, after classification. If an agent was classified as "idle" (not in acting set, has no actionable items), update the agent state to reflect this.
- **D-03:** The idle flag is cleared when the agent next has actionable items in perception (tasks, messages, requests).
- **D-04:** Enhance `format-team-roster` to include status (idle/working/dormant), energy level, and open task count per agent. Executive sees: `casey (systems-engineer) -- IDLE, energy: 65, tasks: 0`.
- **D-05:** Uses existing agent API data (tier, energy) plus a task count query per agent. No new API endpoint needed -- enrich the existing roster formatting.
- **D-06:** One-time energy boost (+10-15) when an agent transitions to idle state (last task completed). This ensures idle agents quickly reach working/prime tier energy levels and are prioritized for delegation.
- **D-07:** The boost applies only on transition to idle, not on every idle tick. Regular +5/tick rest continues as normal for sustained idle periods.

### Claude's Discretion
- Exact energy boost value on idle transition (within 10-15 range)
- Whether to add a `lifecycle_state` column to agent_state or use existing tier/status fields
- How to query task count per agent efficiently in format-team-roster (single query vs per-agent)
- Whether idle agents should appear first in the team roster (sorted by availability)

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| LIFE-01 | Staff ghost signals IDLE after completing assigned work (no more tasks in queue) | Tick engine Phase 3 already classifies idle agents; Phase 5 already PATCHes state. Add lifecycle_state to metadata JSONB during Phase 5 update. |
| LIFE-02 | Executive perceives staff availability (idle agents listed in project review context) | `format-team-roster` already fetches all dept agents via `/api/agents`. Enrich with energy, lifecycle_state from same response + task count from batch query. |
| LIFE-03 | Energy system reflects lifecycle state (idle agents have energy available for new work) | Energy reward system supports named deltas. Add `:idle-transition` reward (+12) applied once when agent transitions from acting to idle. |
</phase_requirements>

## Standard Stack

No new libraries needed. This phase modifies existing code only.

### Core (Existing)
| Component | Location | Purpose | Modification |
|-----------|----------|---------|--------------|
| tick-engine.lisp | `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` | Tick orchestration | Phase 5: write lifecycle_state to metadata |
| energy.lisp | `/opt/project-noosphere-ghosts/lisp/runtime/energy.lisp` | Energy costs/rewards | Add `:idle-transition` reward |
| action-planner.lisp | `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` | Team roster formatting | Enrich `format-team-roster` |
| af64_agents.rs | `/opt/dpn-api/src/handlers/af64_agents.rs` | Agent state PATCH | Already supports metadata JSONB -- no change needed |

## Architecture Patterns

### Lifecycle State Storage Decision

**Recommendation: Use `agent_state.metadata` JSONB column.**

The `agent_state` table already has a `metadata` JSONB column (currently `'{}'::jsonb` for all agents). Storing `lifecycle_state` here avoids a schema migration and follows the existing pattern where agent_state stores tick-computed state.

```sql
-- What the metadata will look like after this phase:
-- {"lifecycle_state": "idle", "idle_since": "2026-03-27T12:00:00Z"}
-- {"lifecycle_state": "active"}
```

**Why not a dedicated column:**
- Adding a column requires an ALTER TABLE migration + Rust struct change + API handler update
- The metadata JSONB is already read/written by the state update flow
- lifecycle_state is tick-derived state, not a core schema concept

**Why not use existing `tier` field:**
- Tier (dormant/base/working/prime/opus) reflects energy+fitness, not task availability
- An agent can be "idle" (no tasks) but at "working" tier (energy 40, fitness 30)
- These are orthogonal dimensions

### Pattern 1: Idle Transition Detection

**What:** Compare current tick classification with previous lifecycle_state to detect transitions.
**When to use:** Phase 3 (classify) or between Phase 3 and Phase 5.

The tick engine already tracks which agents are in the acting set vs idle. The key insight: we need to detect the *transition* to idle (was active, now idle) to apply the one-time energy boost. The current metadata value tells us the previous state.

```lisp
;; In phase-update-state, after determining the agent was idle this tick:
;; 1. Read current metadata from agent (already fetched in phase 1)
;; 2. Check if lifecycle_state was NOT "idle" previously
;; 3. If transitioning TO idle: apply boost + set lifecycle_state = "idle"
;; 4. If already idle: just ensure lifecycle_state stays "idle"
;; 5. If agent was active this tick: set lifecycle_state = "active"
```

### Pattern 2: Efficient Task Count Query

**What:** Get open task counts for all agents in a department with a single query.
**When to use:** In `format-team-roster` to avoid N+1 queries.

```lisp
;; Option A: Single API call to /api/tasks with department filter
;; Option B: Query task counts alongside agent fetch
;;
;; The /api/agents endpoint already returns all agents.
;; Task counts need a separate query. Best approach:
;; Fetch tasks for department, aggregate client-side in Lisp.
```

Since `format-team-roster` already calls `api-get "/api/agents"`, and the tasks API exists, the most pragmatic approach is to fetch open tasks and count by assignee in Lisp. This avoids adding a new API endpoint (per D-05).

### Pattern 3: Metadata PATCH via Existing API

The `StateUpdate` struct in `af64_agents.rs` (line 114-124) already accepts `metadata: Option<Value>`. However, the current handler does NOT merge metadata -- it would need to. Looking at the handler code, `metadata` is not currently handled in the PATCH endpoint (no `if let Some(ref metadata) = body.metadata` block exists).

**This means we need to add metadata PATCH support to the Rust handler**, or use a different approach. Two options:

1. **Add metadata merge to PATCH handler** (small Rust change -- ~5 lines)
2. **Store lifecycle_state in the Lisp layer only** (read from agent data, don't persist to DB metadata)

**Recommendation:** Option 1. Adding metadata PATCH support is a small, reusable change. The field already exists in the `StateUpdate` struct.

### Anti-Patterns to Avoid
- **Polling task counts per-agent:** N+1 API calls in format-team-roster. Use batch approach instead.
- **Overloading tier for lifecycle:** Tier = energy+fitness model. Lifecycle = task availability. Keep separate.
- **Boost on every idle tick:** D-07 explicitly says transition-only. Must track previous state to detect transition.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| State transition detection | Custom state machine | Compare metadata JSONB previous vs current classification | Simple comparison, no new abstractions needed |
| Task count aggregation | New API endpoint | Fetch tasks via existing API, aggregate in Lisp | D-05 says no new endpoints |
| Energy boost | New energy function | Add entry to `+energy-rewards+` hash table | Existing pattern handles cap/floor |

## Common Pitfalls

### Pitfall 1: Metadata Merge vs Overwrite
**What goes wrong:** PATCHing metadata replaces the entire JSONB blob instead of merging keys.
**Why it happens:** Naive `SET metadata = $1` overwrites. Need `metadata || $1` or `jsonb_set`.
**How to avoid:** In the Rust handler, use `SET metadata = COALESCE(metadata, '{}'::jsonb) || $1::jsonb` to merge.
**Warning signs:** Other metadata keys disappearing after lifecycle updates.

### Pitfall 2: Idle Transition Fires Every Tick
**What goes wrong:** Energy boost applied repeatedly because transition detection reads stale data.
**Why it happens:** If lifecycle_state isn't persisted before the next tick reads it, the agent appears to "transition" to idle every tick.
**How to avoid:** Persist lifecycle_state in Phase 5 state update (same PATCH call as tier/ticks). Read previous state from agent data fetched in Phase 1.
**Warning signs:** Idle agents gaining energy much faster than +5/tick rest.

### Pitfall 3: Lisp JSON Keyword Quirk
**What goes wrong:** Metadata field names with underscores become hyphens in Lisp.
**Why it happens:** The Lisp JSON parser converts `lifecycle_state` to `:lifecycle-state`.
**How to avoid:** Use `:lifecycle-state` when reading in Lisp. The JSON encoder will convert back to `lifecycle_state` for the API call.
**Warning signs:** nil values when reading metadata fields, or fields not being recognized.

### Pitfall 4: Executive Agents Have No Tasks (False Idle)
**What goes wrong:** Executives get flagged as idle because they have no assigned tasks (they have projects, not tasks).
**Why it happens:** Lifecycle state checks task queue only.
**How to avoid:** Only set lifecycle_state for staff agents (role != executive). Or: include project ownership in the "has work" check. The `has-actionable-items` function already checks projects -- use it as the idle criterion.
**Warning signs:** Executives showing as IDLE despite having active projects.

### Pitfall 5: format-team-roster API Call Fails Silently
**What goes wrong:** Task count fetch fails, roster shows without task counts.
**Why it happens:** handler-case swallows errors in format-team-roster already (line 756).
**How to avoid:** Default task count to "?" on error, not 0. Zero implies "confirmed no tasks."
**Warning signs:** All agents showing 0 tasks when they actually have work.

## Code Examples

### Energy Reward Addition (energy.lisp)
```lisp
;; Add to +energy-rewards+ hash table initialization:
(setf (gethash :idle-transition table) 12)
```

### Lifecycle State in Phase 5 (tick-engine.lisp)
```lisp
;; In phase-update-state, extend the state-update json-object:
;; Need to determine lifecycle from agent-summaries (built in Phase 3)
(defun phase-update-state (agents now agent-summaries)
  "PATCH each agent's tier, tick counters, and lifecycle state."
  (dolist (agent agents)
    (let* ((aid (gethash :id agent))
           (fitness (fetch-fitness aid))
           (current-energy (get-energy aid))
           (new-tier (determine-tier fitness current-energy aid))
           ;; Get this tick's classification from agent-summaries
           (summary (gethash aid agent-summaries))
           (tick-status (when summary (gethash :status summary)))
           ;; Previous lifecycle from metadata
           (prev-metadata (or (gethash :metadata agent) (make-hash-table)))
           (prev-lifecycle (when (hash-table-p prev-metadata)
                            (gethash :lifecycle-state prev-metadata)))
           ;; Determine new lifecycle state
           (new-lifecycle (if (member tick-status '("idle" "winter_idle") :test #'string-equal)
                             "idle" "active"))
           ;; Detect transition to idle
           (transitioning-to-idle (and (string-equal new-lifecycle "idle")
                                       (not (string-equal prev-lifecycle "idle"))))
           ;; Apply idle transition boost
           (_ (when transitioning-to-idle
                (update-energy aid 12)  ; +12 one-time boost
                (setf current-energy (get-energy aid))
                (setf new-tier (determine-tier fitness current-energy aid))))
           (state-update (json-object
                          :tier new-tier
                          :last-tick-at now
                          :ticks-alive (+ 1 (or (gethash :ticks-alive agent) 0))
                          :ticks-at-current-tier (if (string= new-tier (gethash :tier agent))
                                                     (+ 1 (or (gethash :ticks-at-current-tier agent) 0))
                                                     0)
                          :metadata (json-object :lifecycle-state new-lifecycle))))
      (handler-case
          (api-patch (format nil "/api/agents/~a/state" aid) state-update)
        (error (e)
          (format t "  [state-update-error] ~a: ~a~%" aid e))))))
```

### Enhanced format-team-roster (action-planner.lisp)
```lisp
(defun format-team-roster (agent-id agent-info)
  "Fetch staff agents in the executive's department with availability info."
  (handler-case
      (let* ((dept (or (gethash :department agent-info) ""))
             (all-agents (api-get "/api/agents"))
             (agent-list (if (vectorp all-agents) (coerce all-agents 'list)
                            (if (listp all-agents) all-agents nil)))
             ;; Fetch open tasks to count per agent
             (open-tasks (handler-case
                           (vector->list (api-get "/api/tasks" (list :status "pending,in_progress")))
                           (error () nil)))
             (task-counts (make-hash-table :test #'equal)))
        ;; Build task count map
        (dolist (tk open-tasks)
          (let ((assignee (gethash :assignee tk)))
            (when assignee
              (setf (gethash assignee task-counts)
                    (1+ (or (gethash assignee task-counts) 0))))))
        (with-output-to-string (s)
          (format s "~%## Your Team (~a)~%" dept)
          (let ((members nil))
            (dolist (a agent-list)
              (when (and (string-equal dept (or (gethash :department a) ""))
                         (not (string-equal agent-id (or (gethash :id a) ""))))
                (push a members)))
            ;; Sort: idle agents first, then by energy descending
            (setf members (sort members
                                (lambda (a b)
                                  (let ((a-meta (or (gethash :metadata a) (make-hash-table)))
                                        (b-meta (or (gethash :metadata b) (make-hash-table))))
                                    (let ((a-idle (string-equal "idle" (or (gethash :lifecycle-state a-meta) "")))
                                          (b-idle (string-equal "idle" (or (gethash :lifecycle-state b-meta) ""))))
                                      (if (eq a-idle b-idle)
                                          (> (or (gethash :energy a) 0) (or (gethash :energy b) 0))
                                          a-idle))))))
            (if members
                (dolist (a members)
                  (let* ((aid (or (gethash :id a) "?"))
                         (meta (or (gethash :metadata a) (make-hash-table)))
                         (lifecycle (or (gethash :lifecycle-state meta) "unknown"))
                         (energy (or (gethash :energy a) 0))
                         (tasks (or (gethash aid task-counts) 0)))
                    (format s "- ~a (~a) -- ~a, energy: ~a, tasks: ~a~%"
                            aid
                            (or (gethash :role a) "staff")
                            (string-upcase lifecycle)
                            (round energy)
                            tasks)))
                (format s "  (no staff found in department)~%")))))
    (error (e)
      (format nil "~%[error loading team: ~a]" e))))
```

### Metadata PATCH Support (af64_agents.rs)
```rust
// Add to update_state handler, after the ticks_alive block:
if let Some(ref metadata) = body.metadata {
    sqlx::query(
        "UPDATE agent_state SET metadata = COALESCE(metadata, '{}'::jsonb) || $1::jsonb WHERE agent_id = $2"
    )
    .bind(metadata)
    .bind(&id)
    .execute(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| No lifecycle tracking | Tier-based only (dormant/base/working/prime) | v1.0 | Executives can't see task availability |
| format-team-roster: id + name + role | Same (minimal info) | v1.0 | No delegation intelligence |
| No idle transition boost | +5/tick rest only | v1.0 | Idle agents recover slowly |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual verification via tick engine logs + DB queries |
| Config file | None (Lisp REPL + psql) |
| Quick run command | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle master_chronicle -c "SELECT agent_id, metadata FROM agent_state WHERE metadata != '{}'::jsonb LIMIT 10"` |
| Full suite command | Run tick engine, verify idle agents get lifecycle_state in metadata |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| LIFE-01 | Staff ghost signals IDLE after task queue empty | integration | `psql -c "SELECT agent_id, metadata->>'lifecycle_state' FROM agent_state WHERE metadata->>'lifecycle_state' = 'idle'"` | N/A (DB query) |
| LIFE-02 | Executive sees idle staff in project review | manual | Run tick with executive project review, check team roster output in tick log | N/A |
| LIFE-03 | Idle agents get energy boost on transition | integration | `psql -c "SELECT agent_id, energy, metadata FROM agent_state"` after tick run | N/A |

### Sampling Rate
- **Per task commit:** Verify Lisp compiles (`sbcl --eval '(load "af64.asd")' --quit`), verify Rust compiles (`cargo check`)
- **Per wave merge:** Run one tick cycle, check DB state
- **Phase gate:** Full tick cycle with at least one idle agent visible in executive review

### Wave 0 Gaps
None -- testing is via DB queries and tick engine observation, no test framework to set up.

## Open Questions

1. **Does `/api/agents` return metadata from agent_state?**
   - What we know: The `list_agents` handler (af64_agents.rs line 15-49) does NOT include `metadata` in its response. It joins agent_state but only extracts energy, tier, last_tick_at, ticks_at_current_tier, ticks_alive.
   - What's unclear: format-team-roster uses this endpoint. Without metadata in the response, Lisp can't read lifecycle_state from the list endpoint.
   - Recommendation: Add `metadata` to the `list_agents` response JSON. Small change (~1 line in Rust handler). This is required for format-team-roster to show lifecycle state.

2. **Task count query -- does the tasks API support status filtering?**
   - What we know: The tasks endpoint exists but specific query parameters need verification.
   - Recommendation: If tasks API doesn't support status filter, query tasks via a simpler approach or add a minimal filter. Alternatively, count tasks from the perception data already fetched in Phase 1 (each agent's perception includes their tasks).

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` -- full tick engine read, Phase 3 classification (idle detection), Phase 5 state update
- `/opt/project-noosphere-ghosts/lisp/runtime/energy.lisp` -- energy costs/rewards system, `update-energy` function
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` -- `format-team-roster` (lines 735-757), `build-project-review-job` (lines 759-842)
- `/opt/project-noosphere-ghosts/lisp/runtime/perception.lisp` -- `has-actionable-items` function
- `/opt/dpn-api/src/handlers/af64_agents.rs` -- Agent state PATCH handler, StateUpdate struct, list_agents query
- PostgreSQL `agent_state` table schema -- verified via `\d agent_state`, metadata JSONB column confirmed

### Secondary (MEDIUM confidence)
- DB query for task counts by assignee -- verified small dataset (1 row currently)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - all existing code, no new dependencies
- Architecture: HIGH - metadata JSONB approach verified against actual schema
- Pitfalls: HIGH - identified from direct code reading (metadata merge, JSON quirk, transition detection)

**Key risk:** The `list_agents` handler needs to include `metadata` in its response for format-team-roster to work. This is a small but necessary Rust change that must be done first.

**Research date:** 2026-03-27
**Valid until:** 2026-04-27 (stable codebase, no external dependencies)
