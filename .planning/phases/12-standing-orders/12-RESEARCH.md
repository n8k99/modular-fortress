# Phase 12: Standing Orders - Research

**Researched:** 2026-03-27
**Domain:** Cron-based project scheduling in Lisp tick engine + Rust API + PostgreSQL
**Confidence:** HIGH

## Summary

Standing orders require changes across three layers: (1) a PostgreSQL schema migration adding a `schedule JSONB` column to the `projects` table, (2) a Rust API update to expose the schedule field through existing project endpoints and the perception response, and (3) a Common Lisp cron matcher in the tick engine that evaluates project schedules each tick and injects matching project owners into the acting-set.

The architecture is well-suited for this. The tick engine already runs on a timer, already fetches projects via perception, and already has urgency boosting for project ownership (+15). The new schedule trigger is conceptually identical to the message boost (+50) -- it forces the project owner into the acting-set and creates a cognition job. The `build-project-review-job` function in action-planner.lisp already constructs project review jobs; it just needs the schedule label injected into the prompt.

**Primary recommendation:** Implement a simple cron field matcher in Lisp (5 fields: minute, hour, dom, month, dow), add `last_schedule_fire` JSONB to projects for double-fire prevention, and modify the ranking phase to check schedules before the acting-set selection.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Add `schedule JSONB` column to the `projects` table. No separate schedules table.
- **D-02:** Schedule format is a JSON array of cron objects: `[{"expr": "0 8 * * 1-5", "label": "NYC Session"}, ...]`. Each fires independently.
- **D-03:** Empty array or null means no schedule.
- **D-04:** Cron expression evaluation happens in Lisp (tick engine), not in the API.
- **D-05:** Simple cron matcher in Lisp -- parse `minute hour day-of-month month day-of-week`, supports ranges, lists, `*/N`. No external dependencies.
- **D-06:** Schedule check happens once per tick in the ranking phase. Matching project owners get boosted into the acting-set.
- **D-07:** When a schedule fires, inject the project owner into the acting-set (same as message boost). Executive gets a cognition job with the project and schedule label.
- **D-08:** Cognition prompt includes the schedule label so the executive knows which standing order fired.
- **D-09:** No automatic task creation -- executive reviews project and decides action.
- **D-10:** API needs GET/PATCH for project schedule field, plus dispatch bridge support.
- **D-11:** Perception endpoint should include schedule metadata when returning projects.

### Claude's Discretion
- Exact Lisp cron parser implementation (match function signature, edge cases)
- How to handle missed schedules (if ghost was down during scheduled time)
- Whether to add a `last_fired_at` timestamp to prevent double-firing within the same cron window

### Deferred Ideas (OUT OF SCOPE)
None
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| STAND-01 | Projects table supports a schedule field (cron expression) that triggers ghost perception at scheduled times | DB migration adds `schedule JSONB` column; perception endpoint includes schedule metadata in project response; API PATCH endpoint allows setting schedules |
| STAND-02 | Tick engine recognizes scheduled projects and creates cognition jobs for the owning executive at the scheduled time | Lisp cron matcher evaluates schedules in ranking phase; matching owner gets acting-set injection (+50 boost); `build-project-review-job` enhanced with schedule label |
| STAND-03 | Standing order execution produces conversation output attributed to the responsible ghost | Existing cognition flow already attributes output to the acting agent; project review jobs post conversations as the executive agent, not a system account |
</phase_requirements>

## Architecture Patterns

### Current Flow (no schedules)
```
tick-engine.lisp:run-tick
  -> phase-perceive: fetch perception for each agent (calls /api/perception/:agent_id)
  -> phase-rank: compute urgency, select acting-set
     - project-boost = 15 * num_owned_projects (always-on for executives)
  -> phase-classify-agents: acting-set agents get cognition jobs
  -> action-planner.lisp:build-cognition-job
     -> default-job-builder: messages > requests > tasks > project_review
     -> build-project-review-job: constructs review job for owned projects
  -> phase-process-cognition: execute results, mark messages read
```

### New Flow (with schedules)
```
tick-engine.lisp:run-tick
  -> phase-perceive: fetch perception (now includes schedule metadata per project)
  -> [NEW] check-project-schedules: for each agent's projects, evaluate cron expressions
     - If any schedule matches current time window, add schedule-boost (+50) to that agent
     - Store which schedule label(s) fired for prompt enrichment
  -> phase-rank: compute urgency (schedule-boost now included alongside msg/task/project boosts)
     - Schedule-boosted agents guaranteed into acting-set
  -> phase-classify-agents: acting-set agents get cognition jobs
  -> action-planner.lisp:build-project-review-job [MODIFIED]
     - Include schedule label in prompt: "Standing order fired: NYC Session"
     - Executive sees which schedule triggered and acts accordingly
```

### Recommended File Organization
```
lisp/runtime/
  cron-matcher.lisp          # NEW: cron expression parser and matcher
  tick-engine.lisp           # MODIFIED: schedule check in ranking phase
  action-planner.lisp        # MODIFIED: schedule label in project review prompt
  perception.lisp            # NO CHANGE (perception data comes from API)
  packages.lisp              # MODIFIED: new package + exports

dpn-api/src/handlers/
  af64_perception.rs         # MODIFIED: include schedule field in project response
  projects.rs                # MODIFIED: add PATCH endpoint, include schedule in responses

dpn-core/src/db/
  projects.rs                # MODIFIED: add schedule field to Project struct + queries
```

### Pattern: Cron Expression Matching in Lisp

The cron matcher is the core new code. Standard 5-field cron: `minute hour day-of-month month day-of-week`.

**Field matching rules (per D-05):**
- `*` -- matches any value
- `N` -- exact match (e.g., `8` matches hour 8)
- `N-M` -- range match (e.g., `1-5` matches 1,2,3,4,5)
- `N,M,O` -- list match (e.g., `1,3,5`)
- `*/N` -- step match (e.g., `*/15` for every 15 minutes)

**Key design decision:** The matcher should check if the current time falls within the tick's time window, not exact minute match. Since ticks are 60s-600s apart, a schedule for minute 8 should fire if the tick runs at minute 9 but the previous tick was at minute 7.

**Recommended approach:** Compare against the current minute/hour/dow/etc at tick time. Use a `last_schedule_fire` tracking mechanism (JSONB on project or separate tracking in tick engine state) to prevent double-firing if two ticks fall within the same cron minute.

### Pattern: Schedule Boost Integration

The ranking phase (tick-engine.lisp lines 134-218) already computes urgency as:
```
urgency = (pressure * energy/100) + msg_boost + req_boost + task_boost + project_boost + deadline_boost + quality_issue_boost
```

Add `schedule-boost` (+50, same weight as messages) for agents whose owned projects have a schedule matching the current tick time. This guarantees the executive enters the acting-set.

### Anti-Patterns to Avoid
- **Evaluating cron in the API:** The API should be stateless. The tick engine is the scheduler.
- **Creating tasks automatically:** Per D-09, the executive decides what to do. Standing orders are scheduled project reviews, not task generators.
- **Timezone conversion in Lisp:** All cron expressions should be stored and evaluated in UTC. The OpenClaw jobs use `America/New_York` timezone -- these must be converted to UTC before storing.
- **Matching exact seconds:** Cron only has minute granularity. Compare at minute level only.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Timezone conversion | Lisp TZ library | Store all crons in UTC; convert at entry time (API or dispatch) | CL has no standard TZ support; UTC-only avoids DST bugs entirely |
| Project CRUD | Custom SQL per handler | Extend existing dpn-core Project struct + sqlx queries | Consistent with all other DB access patterns |
| Schedule persistence | File-based state | JSONB column on projects table | DB is the OS -- all state in master_chronicle |

## Common Pitfalls

### Pitfall 1: Double-Firing Within Same Cron Window
**What goes wrong:** Tick interval is 60-600 seconds. A cron expression for `0 8 * * 1-5` could fire on two consecutive ticks if both fall within the 08:00 minute.
**Why it happens:** Ticks aren't synchronized to minute boundaries.
**How to avoid:** Track `last_schedule_fire` per project-schedule pair. Store as JSONB on the project row or in a transient hash table in the tick engine. Only fire if last fire was more than 60 seconds ago (cron minimum granularity).
**Warning signs:** Executive getting duplicate project review jobs for the same standing order.

### Pitfall 2: Missed Schedules When Ghost Is Down
**What goes wrong:** If noosphere-ghosts is stopped during a scheduled time, the schedule fires on restart even if the window is long past.
**Why it happens:** No tracking of "last checked at" -- the matcher just checks current time.
**How to avoid:** On restart, only fire schedules whose window includes the current time. Don't retroactively fire missed schedules from hours ago. This is the correct behavior for periodic reviews (you don't need 3 missed health checks -- just do the next one).
**Recommendation:** The `last_schedule_fire` timestamp naturally handles this -- if the ghost was down for 6 hours, only fire schedules matching the current minute, not all missed ones.

### Pitfall 3: Lisp JSON Key Conversion
**What goes wrong:** The Lisp JSON parser converts underscores to hyphens. `schedule_label` becomes `:schedule-label`, `last_fired_at` becomes `:last-fired-at`.
**Why it happens:** Standard Lisp JSON convention in this codebase.
**How to avoid:** Use hyphenated keys consistently in Lisp code. When building API requests, the `json-object` macro handles conversion back to underscored JSON keys.
**Warning signs:** Hash table lookups returning nil for keys that definitely exist.

### Pitfall 4: Project Owner Missing
**What goes wrong:** Projects #12 (Cognitive Submission) and #14 (Operation Normality) currently have no owner set.
**Why it happens:** Owner was never assigned during initial project setup.
**How to avoid:** The migration or a setup step must set owners: #12 -> sylvia, #14 -> nova. Without an owner, the schedule has no agent to inject into the acting-set.
**Warning signs:** Schedule fires but nobody gets a cognition job.

### Pitfall 5: UTF-8 Safety in Rust
**What goes wrong:** JSONB schedule field contains labels like "NYC Session" -- no UTF-8 risk. But any string truncation in perception must use `.chars().take(N).collect()`, never byte slicing.
**Why it happens:** CLAUDE.md Rust UTF-8 Rule.
**How to avoid:** Follow existing patterns in af64_perception.rs which already use `.chars().take(N).collect()`.

### Pitfall 6: Perception Returning Schedule for Non-Owned Projects
**What goes wrong:** If the perception endpoint returns schedule metadata for ALL projects (not just owned), agents without projects see schedule data they can't act on.
**Why it happens:** The projects query in perception already filters by `WHERE p.owner = $1` (line 429 of af64_perception.rs).
**How to avoid:** The existing filter is correct. Just add `p.schedule` to the SELECT -- it naturally scopes to owned projects only.

## Code Examples

### Lisp Cron Matcher (recommended implementation)

```lisp
;; Source: New file cron-matcher.lisp
(in-package :af64.runtime.cron-matcher)

(defun parse-cron-field (field-str)
  "Parse a single cron field into a matcher function.
   Returns a function that takes an integer and returns T if it matches."
  (cond
    ;; Wildcard
    ((string= field-str "*") (lambda (val) (declare (ignore val)) t))
    ;; Step: */N
    ((and (> (length field-str) 2) (string= (subseq field-str 0 2) "*/"))
     (let ((step (parse-integer (subseq field-str 2))))
       (lambda (val) (zerop (mod val step)))))
    ;; Range: N-M
    ((position #\- field-str)
     (let* ((parts (split-string field-str #\-))
            (lo (parse-integer (first parts)))
            (hi (parse-integer (second parts))))
       (lambda (val) (and (>= val lo) (<= val hi)))))
    ;; List: N,M,O
    ((position #\, field-str)
     (let ((nums (mapcar #'parse-integer (split-string field-str #\,))))
       (lambda (val) (member val nums))))
    ;; Exact number
    (t (let ((num (parse-integer field-str)))
         (lambda (val) (= val num))))))

(defun cron-matches-p (expr minute hour dom month dow)
  "Return T if cron expression matches the given time components.
   expr is a string like '0 8 * * 1-5'."
  (let* ((fields (split-string expr #\Space))
         (matchers (mapcar #'parse-cron-field fields)))
    (and (funcall (nth 0 matchers) minute)
         (funcall (nth 1 matchers) hour)
         (funcall (nth 2 matchers) dom)
         (funcall (nth 3 matchers) month)
         (funcall (nth 4 matchers) dow))))
```

### Rust: Adding Schedule to Perception Response

```rust
// Source: af64_perception.rs, projects query (line ~424)
// Add p.schedule to the SELECT:
let project_rows = sqlx::query(
    r#"SELECT p.id, p.name, p.status, p.description, p.goals, p.blockers, p.current_context,
              p.schedule,
              (SELECT COUNT(*) FROM tasks t WHERE t.project_id = p.id AND t.status IN ('open', 'pending', 'in-progress')) as open_tasks,
              (SELECT COUNT(*) FROM tasks t WHERE t.project_id = p.id AND t.status IN ('done', 'completed')) as completed_tasks
       FROM projects p
       WHERE p.owner = $1 AND p.status = 'active'
       ORDER BY p.updated_at DESC"#
)
.bind(&agent_id)
.fetch_all(&pool).await.unwrap_or_default();

// In the project JSON construction, add schedule:
"schedule": r.get::<Option<serde_json::Value>, _>("schedule"),
```

### SQL Migration

```sql
-- Add schedule JSONB column to projects
ALTER TABLE projects ADD COLUMN schedule JSONB DEFAULT NULL;

-- Set initial schedules for existing projects (converted to UTC)
-- Project #10 (Complete Success / Financial) - Kathryn
UPDATE projects SET
  schedule = '[
    {"expr": "0 22 * * 0-4", "label": "Tokyo Session"},
    {"expr": "0 6 * * 1-5", "label": "London Session"},
    {"expr": "0 12 * * 1-5", "label": "NYC Session"}
  ]'::jsonb,
  owner = 'kathryn'
WHERE id = 10;

-- Project #12 (Cognitive Submission / Editorial) - Sylvia
UPDATE projects SET
  schedule = '[{"expr": "0 1 * * *", "label": "Nightly Editorial"}]'::jsonb,
  owner = 'sylvia'
WHERE id = 12;

-- Project #14 (Operation Normality / Ops) - Nova
UPDATE projects SET
  schedule = '[
    {"expr": "0 13 * * *", "label": "Daily Health Check"},
    {"expr": "5 4 * * *", "label": "Nightly Synthesis"},
    {"expr": "50 3 * * *", "label": "Daily Note Population"},
    {"expr": "30 4 * * 6", "label": "Weekly Finalization"},
    {"expr": "0 5 1 * *", "label": "Monthly Finalization"}
  ]'::jsonb,
  owner = 'nova'
WHERE id = 14;
```

**Note on timezone conversion:** The OpenClaw jobs use `America/New_York` (ET). During EDT (current, March-November), ET = UTC-4. Conversions:
- 18:00 ET = 22:00 UTC (Tokyo briefing)
- 02:00 ET = 06:00 UTC (London briefing)
- 08:00 ET = 12:00 UTC (NYC briefing)
- 09:00 ET = 13:00 UTC (Health check)
- 21:00 ET = 01:00 UTC (Nightly editorial)
- 00:05 ET = 04:05 UTC (Nightly synthesis)
- 23:50 ET = 03:50 UTC (Daily note)
- 00:30 ET Sat = 04:30 UTC Sat (Weekly)
- 01:00 ET 1st = 05:00 UTC 1st (Monthly)

**DST Warning:** These UTC values shift by 1 hour when EDT->EST (November) and back (March). Since cron expressions are fixed, the schedules will be off by 1 hour during EST. This is acceptable for now -- the alternative (timezone-aware cron evaluation) adds significant complexity. Can be addressed later if needed.

### Rust: PATCH Endpoint for Projects

```rust
// Source: projects.rs -- new PATCH handler
#[derive(Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub goals: Option<String>,
    pub blockers: Option<String>,
    pub current_context: Option<String>,
    pub schedule: Option<serde_json::Value>,
}

pub async fn update_project(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateProjectRequest>,
) -> Result<Json<Value>, ApiError> {
    // Dynamic query builder pattern (same as tasks.rs)
    // ...
}
```

### Lisp: Schedule Check in Ranking Phase

```lisp
;; In phase-rank, after computing project-boost, before urgency sum:
;; Check if any owned project has a schedule matching current time
(schedule-boost
  (if projects
      (let ((fired-labels '()))
        (loop for p across projects
              do (let ((schedule (gethash :schedule p)))
                   (when (and schedule (vectorp schedule) (> (length schedule) 0))
                     (loop for entry across schedule
                           do (let ((expr (gethash :expr entry))
                                    (label (gethash :label entry)))
                                (when (and expr (cron-matches-now-p expr))
                                  (push label fired-labels)))))))
        ;; Store fired labels for later use in prompt building
        (when fired-labels
          (setf (gethash aid *schedule-fired-labels*) fired-labels))
        (if fired-labels 50 0))
      0))
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| OpenClaw cron jobs (jobs.json) | Standing orders in projects table | Phase 12 | Cron jobs move from OpenClaw file to DB JSONB |
| Manual project dispatch via GSD | Scheduled auto-dispatch via tick engine | Phase 12 | Executives get periodic reviews without Nathan intervention |
| Fixed +15 project boost only | +15 always + 50 when schedule fires | Phase 12 | Scheduled projects temporarily dominate acting-set |

## Open Questions

1. **DST handling**
   - What we know: Storing cron expressions in UTC avoids all TZ complexity in Lisp. Current EDT offset is UTC-4.
   - What's unclear: When EDT->EST shift happens in November, all schedules shift by 1 hour. The 08:00 ET NYC briefing becomes 13:00 UTC instead of 12:00 UTC.
   - Recommendation: Accept 1-hour shift during DST transitions. Document in project schedule comments. Revisit only if Nathan reports it as a problem.

2. **Podcast watcher and Discord archive jobs**
   - What we know: OpenClaw has 14 cron jobs. 3 map to trading (#10), 1 to editorial (#12), 5 to operations (#14). The remaining jobs (podcast watcher, Discord archive, pipeline wakeup, conversations poll) are infrastructure, not standing orders.
   - What's unclear: Whether podcast watcher and Discord archive should become standing orders on a project, or remain as deterministic cron scripts.
   - Recommendation: Out of scope for Phase 12. Those jobs are Phase 13+ territory (OPS-03 specifically addresses podcast watcher).

3. **`last_schedule_fire` storage location**
   - What we know: Need to prevent double-firing within the same cron window.
   - Option A: JSONB field on projects table (persistent across restarts, but requires API writes each tick).
   - Option B: In-memory hash table in tick engine (fast, lost on restart, but restart = fresh state anyway).
   - Recommendation: Option B (in-memory). Simpler, faster, and the consequence of a double-fire on restart is just one extra project review -- harmless.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual testing via `curl` + REPL verification |
| Config file | none -- no automated test infrastructure exists |
| Quick run command | `curl http://localhost:8080/api/perception/kathryn \| jq .projects` |
| Full suite command | N/A |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| STAND-01 | Schedule field on projects, visible in perception | smoke | `curl localhost:8080/api/projects/10 \| jq .schedule` | N/A |
| STAND-02 | Tick engine creates cognition job when schedule fires | manual | Start ghosts, observe tick log for schedule-triggered project review | N/A |
| STAND-03 | Conversation output attributed to ghost | manual | Check conversations table after schedule-triggered review | N/A |

### Sampling Rate
- **Per task commit:** `curl localhost:8080/api/projects/10 | jq .schedule` (verify API)
- **Per wave merge:** Restart noosphere-ghosts, observe one tick cycle with schedule
- **Phase gate:** All 3 projects have schedules, one full schedule fire observed in tick logs

### Wave 0 Gaps
- No automated test framework for Lisp tick engine or Rust API integration
- Manual verification is the existing pattern across the entire codebase
- dpn-core has `cargo test` but it tests DB functions, not API handlers

## Sources

### Primary (HIGH confidence)
- `/opt/dpn-api/src/handlers/af64_perception.rs` -- full perception endpoint code reviewed
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` -- full tick engine code reviewed
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` -- `build-project-review-job` reviewed (lines 817-900)
- `/opt/project-noosphere-ghosts/lisp/runtime/perception.lisp` -- `has-actionable-items` reviewed
- `/opt/project-noosphere-ghosts/lisp/runtime/task-scheduler.lisp` -- existing scheduling patterns reviewed
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` -- package structure reviewed
- `/root/.openclaw/cron/jobs.json` -- all 14 OpenClaw cron jobs reviewed
- `/opt/dpn-api/src/handlers/projects.rs` -- existing project CRUD reviewed (no PATCH exists)
- `/root/dpn-core/src/db/projects.rs` -- Project struct and DB functions reviewed
- PostgreSQL `\d projects` -- current schema verified directly

### Secondary (MEDIUM confidence)
- UTC timezone conversions for cron expressions (manual calculation, verified against OpenClaw job times)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all code directly inspected, no external libraries needed
- Architecture: HIGH -- extending well-understood existing patterns (urgency boost, perception, project review)
- Pitfalls: HIGH -- derived from direct code inspection and known codebase quirks (JSON key conversion, UTF-8 rule, missing owners)

**Research date:** 2026-03-27
**Valid until:** 2026-04-27 (stable -- no external dependencies changing)
