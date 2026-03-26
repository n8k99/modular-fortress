# Architecture Patterns

**Domain:** Agentic dispatch pipeline (GSD -> Ghost execution)
**Researched:** 2026-03-26
**Confidence:** HIGH (based on direct codebase inspection)

## Recommended Architecture

The architecture already exists. The work is connecting the broken junctions, not redesigning the system.

```
Nathan (Claude Code + GSD)
  |
  | /gsd:dispatch (Python)
  v
dispatch_to_db.py ---> PostgreSQL master_chronicle
  |                     (projects + tasks tables)
  |                          |
  |                          | Perception polling (every tick)
  |                          v
  |                     dpn-api (Rust/Axum :8080)
  |                     GET /api/perception/:agent_id
  |                          |
  |                          | JSON response
  |                          v
  |                     AF64 Tick Engine (SBCL)
  |                     perceive -> rank -> classify -> execute
  |                          |
  |                          | Executive cognition (Claude Code CLI)
  |                          | "Break this project into tasks"
  |                          v
  |                     Action Executor
  |                     POST /api/af64/tasks (create subtasks)
  |                     POST /api/conversations (delegate to staff)
  |                          |
  |                          | Staff perceives assigned tasks (next tick)
  |                          v
  |                     Staff Tool Execution
  |                     uiop:run-program (code tools)
  |                     api-post (DB tools)
  |                          |
  |                          | Task completion
  |                          v
  |                     PATCH /api/af64/tasks/:id (status -> done)
  |                     POST /api/conversations (report results)
  |                          |
  v                          v
/gsd:progress <-------- PostgreSQL (aggregated status)
```

### Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| dispatch_to_db.py | Parse GSD .planning/ artifacts, INSERT into projects + tasks | PostgreSQL (direct) |
| dpn-api | REST gateway for all DB access. Auth, validation, query optimization. | PostgreSQL (sqlx), all consumers |
| af64_perception.rs | Aggregate per-agent snapshot: messages, tasks, projects, team, relationships | PostgreSQL (read-only queries) |
| af64_tasks.rs | CRUD for tasks: list, create, update, delete | PostgreSQL (read-write) |
| tick-engine.lisp | Orchestrate perceive-rank-classify-execute cycle | dpn-api (HTTP), cognition broker |
| cognition-broker.lisp | Rate-limit and route LLM calls. Manage cognitive winter. | Claude Code CLI (subprocess) |
| action-executor.lisp | Execute cognition results: post messages, advance pipelines, create tasks | dpn-api (HTTP) |
| perception.lisp | Call perception API, handle errors | dpn-api (HTTP) |
| action-planner.lisp | Classify perception into action types, build cognition jobs | tick-engine (in-process) |

### Data Flow

**Dispatch flow (Nathan -> DB):**
1. Nathan runs `/gsd:dispatch N` which calls `dispatch_to_db.py --phase N`
2. Script reads `.planning/phases/NN-*/PLAN.md` files
3. Parses frontmatter (wave, must_haves, depends_on)
4. UPSERTs project row, then INSERTs task rows with `source='gsd'`
5. Tasks land in DB with `status='open'`, `project_id` set

**Perception flow (DB -> Ghost):**
1. Tick engine calls `(perceive agent-id tier last-tick-at)`
2. Lisp HTTP client GETs `/api/perception/:agent_id?tier=X&since=Y`
3. af64_perception.rs builds snapshot: messages, tasks, projects, team, responsibilities
4. Tasks filtered by: triage sees unassigned, executives see department, staff see personal
5. Projects filtered by: owner = agent_id, status = 'active'

**Execution flow (Ghost -> DB):**
1. Tick engine ranks agents by urgency score
2. Top N agents get cognition jobs submitted to broker
3. Broker calls Claude Code CLI with structured prompt + perception context
4. LLM returns structured response (JSON with action type + content)
5. Action executor dispatches based on action type
6. Results written back to DB via dpn-api

## Patterns to Follow

### Pattern 1: Perception as Single Aggregated Snapshot

**What:** One API call per agent per tick returns everything the agent needs to decide what to do.

**When:** Always. This is the existing pattern and it works.

**Why:** Reduces HTTP round-trips. The tick engine processes 60+ agents per tick. 1 call each vs 5 calls each is the difference between a tick completing in time.

**Already implemented in:** `af64_perception.rs` (401 lines, comprehensive)

### Pattern 2: Action Types as Dispatch Table

**What:** The action executor maps action type strings to execution functions.

**When:** LLM cognition returns a response. The response includes an action type.

**Why:** Clean separation between "what to do" (LLM decides) and "how to do it" (deterministic dispatch).

**Current actions:**
- `respond-message` -- post a conversation reply
- `advance-pipeline` -- move task to next pipeline stage
- `complete-task` -- mark task done with validation

**New actions needed:**
- `decompose-project` -- executive breaks project into subtasks
- `assign-task` -- executive delegates to staff
- `execute-tool` -- staff runs a tool (code/db/api/file)
- `report-blocker` -- staff escalates to executive
- `update-project-status` -- executive updates project context/blockers

### Pattern 3: DB as the Only Bus

**What:** All inter-component communication goes through PostgreSQL tables. No in-memory queues, no message brokers, no file-based state.

**When:** Always. This is the foundational architecture decision.

**Why:** Crash resilience. If any component restarts, it picks up where it left off by reading DB state. No lost messages, no orphaned state.

**Tables that serve as buses:**
- `conversations` -- agent-to-agent messages (+50 urgency boost)
- `tasks` -- work queue (+25 urgency boost)
- `projects` -- portfolio awareness (+15/project urgency boost)
- `tick_log` -- empirical activity record
- `cognition_jobs` -- pending reasoning requests

### Pattern 4: Energy as Natural Rate Limiting

**What:** Every agent has an energy level (0-100). Actions cost energy. Low energy = fewer actions. Zero energy = dormant.

**When:** Every tick. Energy gates which agents get cognition time.

**Why:** Prevents runaway LLM spending. A ghost that burns through its energy doing bad work naturally stops. Combined with cognitive winter (backpressure on pending jobs), this creates organic flow control without explicit rate limits.

### Pattern 5: Validate Stage Output Before Advancing

**What:** The `validate-stage-output` function in action-executor.lisp checks that work products meet minimum quality bars before allowing pipeline advancement.

**When:** Any time a task moves to the next pipeline stage.

**Why:** Without validation, ghosts mark tasks "done" after producing garbage. The existing validators check: minimum length, presence of structured sections (for specs), reference to actual concerns (for reviews), etc.

**Extend this for:** New tool execution results. Validate that code tools produced parseable output, DB tools returned expected data shapes, etc.

## Anti-Patterns to Avoid

### Anti-Pattern 1: Individual UPDATE Statements

**What:** af64_tasks.rs `update_task()` runs separate UPDATE for each field.

**Why bad:** 10 optional fields = up to 10 round-trips to PostgreSQL. Should be a single UPDATE with COALESCE.

**Instead:** Build a single UPDATE statement with all non-null fields. Not critical for correctness but harms latency.

### Anti-Pattern 2: Hardcoded Pipeline Advancement

**What:** `*pipeline-advancement*` in action-executor.lisp is a static alist mapping stage -> (next-stage . assignee).

**Why bad:** Adding a new pipeline or changing routing requires editing Lisp source code and restarting the process.

**Instead:** For this project, the GSD pipeline stages should come from the dispatched task context (stored in `context` JSONB), not from hardcoded Lisp. The existing pipelines (engineering, investment, editorial) can stay hardcoded, but GSD-dispatched work should carry its own stage progression.

### Anti-Pattern 3: Ignoring the `source` Column

**What:** dispatch_to_db.py tries to write `source='gsd'` but the column doesn't exist.

**Why bad:** Without source tracking, there's no way to distinguish GSD-dispatched tasks from organically created ones. Perception queries can't filter for dispatched work.

**Instead:** Add the `source` column and use it in perception queries to give GSD tasks appropriate urgency.

## Scalability Considerations

| Concern | Current (64 agents) | At 200 agents | At 1000 agents |
|---------|---------------------|---------------|----------------|
| Perception queries | 64 API calls/tick, each ~10ms = <1s total | 200 calls, still <3s | Would need batch perception or async parallelism |
| Task table scans | Indexes on (assignee, status) handle it | Same, with index | Partition by status if slow |
| Cognition broker | 6 jobs/tick max, cognitive winter at 18 pending | Same limits apply | Would need multiple broker instances |
| LLM cost | $0.50/request budget, 6/tick = $3/tick max | Same budget constraint | Cost is the real scaling bottleneck |
| Tick interval | 60s-600s configurable | May need longer ticks | Tiered tick frequency by agent importance |

Scalability is NOT a concern for this project. 64 agents on a single droplet is well within capacity. The bottleneck is LLM cost ($0.50/request), not system architecture.

## Sources

- Direct codebase inspection of af64_perception.rs (401 lines), af64_tasks.rs (167 lines), action-executor.lisp, tick-engine.lisp, perception.lisp, dispatch_to_db.py
- [PostgreSQL LISTEN/NOTIFY docs](https://www.postgresql.org/docs/current/sql-notify.html)
- [Axum state management patterns](https://docs.rs/axum/latest/axum/response/sse/)
