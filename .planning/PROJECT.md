# Noosphere Dispatch Pipeline

## What This Is

The autonomous execution pipeline connecting GSD planning to Noosphere Ghost action. Nathan plans projects and phases in GSD, dispatches them to master_chronicle, and executive ghosts perceive, decompose, delegate, and drive work through their teams — reporting results back through the conversation table and project status updates. The goal is a system where ghosts do real work autonomously, and Nathan only intervenes for strategy and blockers.

## Core Value

GSD-dispatched projects must flow through to ghost execution and back without human intervention — executives plan, staff execute, results report themselves.

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
- ✓ Tasks table schema supports project linkage, source tracking, and GSD context — Phase 1
- ✓ dispatch_to_db.py writes valid hierarchical records (parent+subtask) with owner, department, and context — Phase 1

- ✓ Perception endpoint returns GSD fields (project_id, source, context, assigned_to, scheduled_at) for all agent types — Phase 2
- ✓ Ghosts perceive dispatched projects and assigned tasks through perception API — Phase 2
- ✓ Project ownership urgency boost (+15/project) code path verified end-to-end — Phase 2

- ✓ Executive ghosts use LLM cognition to decompose projects into staff-suitable tasks — Phase 3
- ✓ Executives delegate and assign tasks to staff via CREATE_TASK parser — Phase 3
- ✓ Task creation API supports ghost-originated tasks with auto-generated task_id — Phase 3

- ✓ Staff ghosts execute work via tool socket (DB, task, code, memory tools) — Phase 4
- ✓ Claude Code CLI registered as ghost tool with scope enforcement — Phase 4
- ✓ Tool results persisted in stage_notes for executive audit — Phase 4
- ✓ Agent tool_scope values audited and corrected (64 agents) — Phase 4

- ✓ Task completion reports to executive conversations with GSD context (project, must_haves, stage_notes) — Phase 5
- ✓ Wave advancement: DB trigger auto-opens next wave when current wave completes — Phase 5
- ✓ Project completion auto-notifies Nathan via conversations INSERT — Phase 5
- ✓ Blocker escalation to executive + ESCALATE: @nathan parser — Phase 5
- ✓ dispatch --status shows per-wave progress for all active projects — Phase 5

### Active

(All v1 requirements complete — see v2 Requirements below)

### Out of Scope

- Ghost-to-ghost negotiation or autonomous project creation — ghosts execute dispatched work, they don't create new projects
- Real-time streaming of ghost activity — async reporting through DB is sufficient
- Frontend UI changes to dpn-kb or org graph — this project is backend pipeline only
- Changing the tick engine architecture — extend it, don't rewrite it
- Multi-droplet distribution — everything stays on single node for now

## Context

**Current state:** The GSD→Ghost pipeline is architecturally defined but broken at every junction:
- `dispatch_to_db.py` tries to INSERT into columns that don't exist (`project_id`, `source`, `context`, `department`)
- `/api/perception/:agent_id` is called by ghosts but doesn't exist in dpn-api
- Project ownership boost (+15/project) in tick engine is dead code (perception returns empty)
- Ghost action-executor has hardcoded pipelines disconnected from GSD planning
- No feedback loop from execution back to project status

**What works:** The tick engine runs, cognition broker manages LLM calls, agents have energy/tiers, conversations flow, the DB schema is mostly there. The plumbing exists — the connections are missing.

**Key files involved:**
- `gotcha-workspace/tools/gsd/dispatch_to_db.py` — broken bridge script
- `dpn-core/src/db/projects.rs` — project CRUD
- `dpn-core/src/db/tasks.rs` — task CRUD (Obsidian-focused, needs GSD columns)
- `dpn-api/src/main.rs` — needs perception route
- `project-noosphere-ghosts/lisp/runtime/perception.lisp` — expects /api/perception/:agent_id
- `project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — needs project-aware planning
- `project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — needs tool expansion
- `project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — project boost already coded

**Executive roster and domains:**
- Nova (COO): Operations, automation, droplet
- Eliana (CTO): Engineering, infrastructure, repos
- Kathryn (CSO): Strategy, marketing, prediction markets
- Sylvia (Content Chief): Writing, narrative, content
- Vincent (Creative Director): Visual, images, art
- JMax (Head of Legal): Canon, legal, ethics, lore
- LRM (Head of Musicology): Music, audio, composition
- Sarah Lin (Executive PA): Routing, orchestration

## Constraints

- **Stack**: Rust (dpn-api, dpn-core), Common Lisp/SBCL (ghosts), Python (dispatch tools), PostgreSQL — no new languages
- **DB is the OS**: All state in master_chronicle. No file-based state for ghost work.
- **UTF-8 Rule**: Never mix character positions with byte indices in Rust code
- **Ghost LLM**: Claude Code CLI (`claude -p`) with `--output-format json`, $0.50/request budget
- **Tick interval**: Currently 30s-10min configurable. Tool execution must complete within tick bounds.
- **Single droplet**: All services on 144.126.251.126. Resource-conscious design.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| LLM cognition for executive planning | Executives should think like managers, not follow rules | — Pending |
| All tool types for staff ghosts | Autonomy requires real capabilities (code, DB, API, external) | — Pending |
| Nathan only for blockers + strategy | The whole point is autonomous execution | — Pending |
| Dispatch→perceive as first milestone | Prove the pipeline before building on it | — Pending |
| Dual feedback (status + conversations) | Status for tracking, conversations for notable events | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd:transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd:complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-03-26 after Phase 5 completion — ALL PHASES COMPLETE (TOOL-04 external tools deferred to v2)*
