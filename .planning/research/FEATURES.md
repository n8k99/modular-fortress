# Feature Landscape

**Domain:** Agentic dispatch and execution pipeline
**Researched:** 2026-03-26

## Table Stakes

Features that must work for the pipeline to function. Missing = pipeline is broken.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Schema alignment (tasks table) | dispatch_to_db.py fails on INSERT because `source`, `context` columns don't exist | Low | ALTER TABLE to add missing columns |
| dispatch_to_db.py writes valid records | Core bridge is broken at column level | Low | Fix column names, add error handling |
| Perception returns project-linked tasks | Ghosts can't act on work they can't see | Low | af64_perception.rs already queries tasks + projects. Schema fix may be all that's needed. |
| Project ownership urgency boost | Executives must notice new assigned projects | Low | Code exists in tick engine (+15/project). Perception already returns projects. Verify boost fires. |
| Task status mutations via API | Ghosts must mark tasks in-progress/done | Low | PATCH endpoint exists in af64_tasks.rs |
| GSD progress query | Nathan needs to see dispatched work status | Low | dispatch_to_db.py `--status` works if schema is correct |

## Differentiators

Features that make this genuinely autonomous, not a glorified TODO list.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Executive task decomposition via LLM | Executives receive project, use cognition to break into staff tasks | High | Core "thinking like a manager" feature. Requires structured prompts for cognition broker. |
| Domain-routed task assignment | Tasks auto-assigned by department and expertise | Medium | Executive knows staff roster. Cognition result includes assignment. Action executor creates subtasks via POST /api/af64/tasks. |
| Wave-based ordering | Phase plans have wave dependencies (wave 1 before 2) | Medium | dispatch_to_db.py already writes wave to context JSONB. Executives respect ordering when delegating. |
| Tool execution by staff ghosts | Staff DO work (code, DB, file ops) not just talk | High | Requires expanding action-executor.lisp. Currently only conversations + pipeline advancement. |
| Feedback loop (execution -> project status) | Task completions roll up to project progress | Medium | Last task in wave -> advance wave. All tasks done -> project complete. |
| Blocker escalation | Staff hits blocker, escalates to executive | Medium | Post to conversations with source='blocker'. Executive perceives with +50 urgency. Natural fit. |

## Anti-Features

Features to explicitly NOT build.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Ghost-to-ghost negotiation | Out of scope: ghosts execute dispatched work, don't create projects | Nathan creates projects via /gsd:new-project |
| Real-time streaming | Out of scope. Async DB reporting sufficient for 60s+ ticks | Poll via /gsd:progress |
| Frontend changes | Out of scope. Backend pipeline only | Existing dpn-kb reads same DB |
| Tick engine rewrite | Constraint: "extend it, don't rewrite it" | Add action types to existing executor |
| Multi-droplet distribution | Single node constraint | All on 144.126.251.126 |
| Complex dependency DAGs | Overkill for v1 | Simple wave numbers (1, 2, 3) |
| Retry/backoff systems | Tick engine already has cognitive winter + energy | Energy depletion = natural backoff |
| Tool sandboxing service | Separate sandboxing process is overengineered for single-droplet | Path allowlists in the tool dispatch table |

## Feature Dependencies

```
Schema Fix (tasks table)
  --> dispatch_to_db.py fix
    --> Dispatch writes valid records
      --> Perception returns dispatched tasks
        --> Ghosts see work
          --> Executive decomposition (LLM)
            --> Staff assignment
              --> Tool execution
                --> Task completion
                  --> Status rollup
                    --> Progress reporting
                      --> Blocker escalation (if needed)
```

## MVP Recommendation

Prioritize in three phases matching the dependency chain:

**Phase 1 -- Prove the pipe flows (table stakes):**
1. Fix tasks table schema (add `source`, `context`, verify `project_id`)
2. Fix dispatch_to_db.py to match actual schema
3. Verify perception returns dispatched tasks
4. Verify project ownership boost fires for executives
5. End-to-end test: dispatch a project, see it in perception output

**Phase 2 -- Ghosts act on work (core differentiators):**
6. Executive decomposition prompts (structured cognition for project breakdown)
7. Domain-routed task assignment (executive assigns to staff)
8. Action executor extension: new action type `execute-project-task`
9. Basic tool execution (code tools via Claude CLI, DB tools via API)

**Phase 3 -- Close the loop (feedback + autonomy):**
10. Task completion -> status rollup
11. Wave advancement (wave 1 done -> unlock wave 2)
12. Blocker escalation (staff -> executive conversations)
13. /gsd:progress shows real execution state

**Defer to v2:**
- File write sandboxing (beyond path allowlists)
- Cross-department task handoffs
- Automatic wave dependency graph construction
- Ghost-initiated subtask creation (currently executive-only)
