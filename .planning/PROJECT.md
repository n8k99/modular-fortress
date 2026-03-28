# Noosphere Dispatch Pipeline

## What This Is

The autonomous execution pipeline connecting GSD planning to Noosphere Ghost action. Nathan plans projects and phases in GSD, dispatches them to the noosphere (master_chronicle), and executive ghosts perceive, decompose, delegate, and drive work through their teams. Standing orders give ghosts temporal autonomy — Nova/T.A.S.K.S. runs operational cadence, Sylvia publishes editorials, Kathryn delivers trading briefings, all on cron schedules without human dispatch.

## Core Value

GSD-dispatched projects must flow through to ghost execution and back without human intervention — executives plan, staff execute, results report themselves. Standing orders extend this to recurring work.

## Requirements

### Validated

- ✓ PostgreSQL master_chronicle as central state store — existing
- ✓ Projects table with owner, goals, status fields — existing
- ✓ Tasks table with assignment, status tracking — existing
- ✓ Conversations table for inter-agent messaging — existing
- ✓ Agent registry with 8 executives + 56 staff — existing
- ✓ Tick engine with perceive→rank→classify→execute→update cycle — existing
- ✓ Cognition broker with provider chain and cognitive winter — existing
- ✓ dpn-api REST gateway on port 8080 — existing
- ✓ GSD planning workflow (project→discuss→plan→execute) — existing
- ✓ Executive agent roster with domain routing — existing
- ✓ Tasks table schema supports project linkage, source tracking, and GSD context — v1.0
- ✓ dispatch_to_db.py writes valid hierarchical records — v1.0
- ✓ Perception endpoint returns GSD fields for all agent types — v1.0
- ✓ Ghosts perceive dispatched projects and assigned tasks — v1.0
- ✓ Executive ghosts decompose projects via LLM cognition — v1.0
- ✓ Staff execute via tool socket (DB, task, code, memory tools) — v1.0
- ✓ Task dependency chains with auto-unblock — v1.1
- ✓ Structured artifact passing between pipeline stages — v1.1
- ✓ Decisions brain for executive consultation — v1.1
- ✓ Verification levels with quality severity — v1.1
- ✓ Lifecycle signals for staff availability — v1.1
- ✓ Ghost message spam eliminated (read_by filtering + mark-as-read) — v1.2
- ✓ Standing order framework (cron schedules, tick engine evaluation, urgency boost) — v1.2
- ✓ Operations pipeline: 6 tools, Nova/T.A.S.K.S. executes health checks, notes, synthesis, podcasts — v1.2
- ✓ Editorial pipeline: Sylvia executes nightly Thought Police under Cognitive Submission — v1.2
- ✓ Financial pipeline: Kathryn executes trading briefings + calendar sync under Project Complete Success — v1.2
- ✓ Dynamic label-to-tool mapping supporting any executive/project (11 mappings) — v1.2

### Active

(None — all v1.0-v1.2 requirements shipped. Next milestone will define new requirements.)

## Shipped Milestones

### v1.2 Operational Readiness (shipped 2026-03-28)
- Message hygiene, standing orders framework, operations/editorial/financial pipeline migration from OpenClaw
- 3 executives with temporal autonomy: Nova (ops), Sylvia (editorial), Kathryn (financial)
- 11 label-to-tool mappings across 3 projects, dynamic per-executive mapping

### v1.1 Ghost Coordination Patterns (shipped 2026-03-27)
- Task dependency chains, structured artifact passing, decisions brain, verification levels, lifecycle signals

### v1.0 Noosphere Dispatch Pipeline (shipped 2026-03-26)
- Schema & dispatch, perception pipeline, executive cognition, tool execution, feedback & reporting

### Out of Scope

- Ghost-to-ghost negotiation or autonomous project creation — ghosts execute dispatched work, they don't create new projects
- Real-time streaming of ghost activity — async reporting through the noosphere is sufficient
- Frontend UI changes to dpn-kb or org graph — this project is backend pipeline only
- Changing the tick engine architecture — extend it, don't rewrite it
- Multi-droplet distribution — everything stays on single node for now
- Discord output bridge — ghosts post to the noosphere; Discord is an external delivery concern

## Context

**Current state (post v1.2):** The full pipeline works end-to-end with standing orders:
- dispatch_to_db.py writes hierarchical tasks with owner, department, wave context, and dependencies
- Perception returns all GSD fields, filters read messages, includes schedule metadata
- Cron schedules fire on projects, injecting owning executives into acting-set with +50 urgency boost
- Action planner maps schedule labels to specific tool invocations via dynamic `tool-mapping-for-label`
- `execute-project-review` calls `process-tool-calls` so standing orders execute real tools
- Nova runs 6 ops tools, Sylvia runs editorial, Kathryn runs trading briefings + calendar sync
- All output attributed to the executing ghost via conversations table

**OpenClaw migration status:** 11 of 14 cron jobs migrated to ghost standing orders. Remaining 3 are OpenClaw-internal (pipeline wakeup, conversations poll, tasks archive) and will retire with OpenClaw itself.

**Key files:**
- `gotcha-workspace/tools/gsd/dispatch_to_db.py` — GSD-to-DB bridge
- `dpn-api/src/handlers/af64_perception.rs` — perception endpoint with schedule metadata
- `project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — tick cycle with cron schedule evaluation
- `project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — cognition prompts with label-to-tool mapping
- `project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — tool execution in project reviews
- `project-noosphere-ghosts/config/tool-registry.json` — 9 registered operational tools

**Executive roster and standing order domains:**
- Nova/T.A.S.K.S. (COO): Operations — health check, daily notes, synthesis, podcasts, temporal compression
- Eliana (CTO): Engineering — no standing orders yet
- Kathryn (CSO): Financial — Tokyo/London/NYC trading briefings, calendar sync
- Sylvia (Content Chief): Editorial — nightly Thought Police generation
- Vincent (Creative Director): Visual — no standing orders yet
- JMax (Head of Legal): Canon — no standing orders yet
- LRM (Head of Musicology): Music — no standing orders yet
- Sarah Lin (Executive PA): Routing — no standing orders yet

## Constraints

- **Stack**: Rust (dpn-api, dpn-core), Common Lisp/SBCL (ghosts), Python (dispatch tools), PostgreSQL — no new languages
- **Noosphere is the OS**: All state in master_chronicle. No file-based state for ghost work.
- **UTF-8 Rule**: Never mix character positions with byte indices in Rust code
- **Ghost LLM**: Claude Code CLI (`claude -p`) with `--output-format json`, $0.50/request budget
- **Tick interval**: Currently 30s-10min configurable. Tool execution must complete within tick bounds.
- **Single droplet**: All services on 144.126.251.126. Resource-conscious design.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| LLM cognition for executive planning | Executives should think like managers, not follow rules | ✓ Good — v1.0 |
| All tool types for staff ghosts | Autonomy requires real capabilities (code, DB, API, external) | ✓ Good — v1.0 |
| Nathan only for blockers + strategy | The whole point is autonomous execution | ✓ Good — v1.0 |
| Dispatch→perceive as first milestone | Prove the pipeline before building on it | ✓ Good — v1.0 |
| Dual feedback (status + conversations) | Status for tracking, conversations for notable events | ✓ Good — v1.0 |
| Incorporate patterns from Squad/ATL/ClawTeam | Proven coordination patterns strengthen existing concepts | ✓ Good — v1.1 |
| Standing orders as scheduled project reviews | Reuse existing project review infrastructure, not new mechanism | ✓ Good — v1.2 |
| Run existing Python scripts via tool invocation | No rewrites, proven tools, ghost just triggers them | ✓ Good — v1.2 |
| Dynamic label-to-tool mapping | Generalized for any executive/project, not hardcoded per-agent | ✓ Good — v1.2 |
| Noosphere-native output (conversations, not Discord) | Ghosts live in the noosphere; external delivery is a separate concern | ✓ Good — v1.2 |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition:**
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone:**
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-03-28 after v1.2 Operational Readiness milestone*
