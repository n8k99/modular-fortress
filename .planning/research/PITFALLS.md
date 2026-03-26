# Domain Pitfalls

**Domain:** Agentic dispatch and execution pipeline
**Researched:** 2026-03-26

## Critical Pitfalls

Mistakes that cause rewrites or major issues.

### Pitfall 1: Ghost Hallucination of Progress

**What goes wrong:** Ghosts mark tasks "done" without actually doing real work. They produce conversation messages that DESCRIBE work instead of DOING work. The task status says "completed" but nothing actually changed in the codebase or database.

**Why it happens:** The current action-executor only has `respond-message` and `advance-pipeline` actions. When a ghost receives a task, its only tool is talking. So it talks about doing the work and calls it done.

**Consequences:** Project status shows 100% complete but nothing was built. Nathan discovers this and loses trust in the entire system.

**Prevention:**
1. The `validate-stage-output` function already exists and checks for minimum quality. Extend it to verify that tool execution actually occurred (check `tools-executed` count > 0 for execution tasks).
2. Only allow `complete-task` action after tool execution has produced verifiable output.
3. Agents without `tool_scope` should not receive execution tasks (af64_perception.rs already checks this).

**Detection:** Tasks marked "done" with no corresponding tool execution entries. Conversations saying "I have completed X" without any PATCH to task status or tool output logged.

### Pitfall 2: Schema Mismatch Cascade

**What goes wrong:** dispatch_to_db.py writes to columns that don't exist (`source`, `context`, `department` on tasks). The INSERT silently fails or throws, and no tasks are created. Ghosts see nothing to do. Nathan thinks dispatch worked because the script exited 0.

**Why it happens:** The script was written against a planned schema, not the actual schema. Nobody verified the columns exist.

**Consequences:** The entire pipeline is dead from step one. No tasks dispatched = nothing to perceive = ghosts idle.

**Prevention:**
1. Run the dispatch script against the actual DB and check for errors FIRST, before building anything else.
2. Add a `--dry-run` flag that validates the schema without inserting.
3. Wrap INSERTs in try/except and report clear errors about missing columns.

**Detection:** `dispatch_to_db.py --status` shows no tasks for a project that was "dispatched."

### Pitfall 3: Cognition Budget Exhaustion

**What goes wrong:** Executive decomposition + staff tool execution both require LLM calls via Claude Code CLI at $0.50/request. With 6 jobs/tick and a tick every 60 seconds, that is $3/minute = $180/hour if the system runs hot.

**Why it happens:** Every dispatched project triggers executive cognition (decompose) + multiple staff cognitions (execute tools). A project with 10 tasks means 10+ LLM calls just for execution.

**Consequences:** Unexpected bill, or cognitive winter kicks in and everything stalls.

**Prevention:**
1. Use deterministic (non-LLM) execution for simple, well-defined tasks. Not every task needs an LLM call.
2. Batch related tasks into a single cognition request where possible.
3. Executive decomposition should produce tasks that can be executed deterministically (e.g., "run this specific command" not "figure out how to do X").
4. Set hard budget caps in the cognition broker config.

**Detection:** Cognitive winter triggering frequently. `pending >= 18` in cognition_jobs table.

## Moderate Pitfalls

### Pitfall 4: Underscore-Hyphen JSON Quirk

**What goes wrong:** The Lisp JSON parser converts underscores to hyphens. So `project_id` in PostgreSQL becomes `:project-id` in Lisp. If code on the Lisp side references `:project_id`, it gets nil.

**Prevention:** Always use hyphenated keys in Lisp code (`:project-id`, `:from-agent`). Test JSON round-trips when adding new fields. Document this quirk prominently.

### Pitfall 5: Perception Since Filter Masking Old Messages

**What goes wrong:** The `since` parameter in perception filters out messages older than the timestamp. But if an agent was dormant for hours and then wakes up, it misses all messages from the dormant period.

**Prevention:** Already partially mitigated -- perception.lisp hardcodes `since` to "2026-01-01T00:00:00Z" (ignoring `last-tick-at`). The handoff detection also ignores `since` for unresponded handoffs. GSD-dispatched tasks are inherently safe because the perception query uses status-based filtering (WHERE status IN ('open', 'pending', 'in-progress')), not time-based filtering.

### Pitfall 6: Wave Ordering Ignored

**What goes wrong:** Executive decomposes a project and assigns all tasks simultaneously, ignoring wave ordering. Wave 2 tasks start before wave 1 completes. Dependencies break.

**Prevention:** When perception returns tasks for a project, include wave information from the `context` JSONB. The executive cognition prompt must explicitly state: "Only assign wave N+1 tasks after all wave N tasks are complete." Alternatively, filter wave 2 tasks out of perception until wave 1 is done.

### Pitfall 7: Orphaned Subtasks

**What goes wrong:** Executive creates subtasks via POST /api/af64/tasks but doesn't set `project_id`. The subtasks exist in the tasks table but are disconnected from the project. Progress queries miss them. Nobody perceives them correctly.

**Prevention:** The create_task endpoint should require `project_id` for any task where `source='ghost'`. Validate on the API side.

## Minor Pitfalls

### Pitfall 8: UTF-8 Boundary Errors

**What goes wrong:** Rust code uses byte slicing `[..N]` on strings with emoji or non-ASCII characters. Panics with "byte index is not a char boundary."

**Prevention:** Already documented in CLAUDE.md guardrails. af64_perception.rs correctly uses `.chars().take(N).collect()`. Continue this pattern everywhere.

### Pitfall 9: Pipeline Advancement for Non-Pipeline Tasks

**What goes wrong:** The action executor tries to advance a GSD-dispatched task through the hardcoded `*pipeline-advancement*` stages, but GSD tasks don't have matching stage names.

**Prevention:** Check `source` column: if source='gsd', use GSD-specific logic (wave-based advancement from `context` JSONB); otherwise use existing hardcoded pipelines.

### Pitfall 10: Dispatch Without Owner Assignment

**What goes wrong:** `dispatch_to_db.py --phase N` creates tasks but no `--owner` is specified. Tasks have no assignee. Only triage agents (sarah, lara) see them. No executive picks them up for decomposition.

**Prevention:** Auto-route to correct executive based on project domain. If `--owner` not provided, look up project domain and assign to matching executive (engineering -> eliana, content -> sylvia, ops -> nova, etc.).

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| Schema fix + dispatch repair | Schema mismatch cascade (#2) | Run dispatch against real DB first, verify columns exist |
| Perception verification | Wave ordering ignored (#6) | Test with multi-wave project, verify wave 2 tasks hidden until wave 1 done |
| Executive decomposition | Hallucination of progress (#1) | Validate decomposition output contains structured task specs, not prose |
| Staff tool execution | Budget exhaustion (#3) | Start with deterministic tools only, add LLM tools carefully |
| Feedback loop | Orphaned subtasks (#7) | Require project_id on all ghost-created tasks |
| Owner assignment | Dispatch without owner (#10) | Add domain-to-executive routing in dispatch script |

## Sources

- Direct inspection of action-executor.lisp (validate-stage-output, pipeline advancement table)
- Direct inspection of af64_perception.rs (tool_scope checks, since filtering, task routing by tier)
- Direct inspection of dispatch_to_db.py (column mismatch with actual schema)
- CLAUDE.md guardrails (UTF-8 rule, DB-is-the-OS principle)
- PROJECT.md constraints ($0.50/request budget, single droplet, no tick engine rewrite)
