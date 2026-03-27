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

- [x] Fix ghost message spam — perception filters read messages, action executor marks messages read after processing — Phase 11
- [x] sqlx JSONB fix — "json" feature confirmed in dpn-api Cargo.toml — Phase 11
- [ ] Standing order framework — recurring project pipelines that ghosts perceive and execute on schedule
- [ ] Trading briefings pipeline — Tokyo/London/NYC sessions via Project #10 "Complete Success" (Kathryn)
- [ ] Editorial pipeline — nightly Thought Police generation via Project #12 "Cognitive Submission" (Sylvia)
- [ ] Operational cadence — daily notes, health checks, synthesis, calendar sync via Project #14 "Operation Normality" (Nova)
- [ ] Podcast watcher — check feeds, post new episodes to Discord
- [ ] Weekly/monthly finalization — temporal compression attributed to specific ghosts

## Current Milestone: v1.2 Operational Readiness

**Goal:** Make ghosts production-trustworthy by fixing the spam problem, then migrate OpenClaw's standing orders into ghost-executed project pipelines so OpenClaw can be retired.

**Target features:**
- Message read-marking (stop token bleed from stale messages)
- sqlx JSONB fix (lifecycle metadata persistence)
- Standing order pipelines (cron-triggered project work for ghosts)
- OpenClaw cron migration (trading, editorial, ops, podcasts, daily notes)

## Shipped Milestones

### v1.1 Ghost Coordination Patterns (shipped 2026-03-27)
- Task dependency chains, structured artifact passing, decisions brain, verification levels, lifecycle signals

### v1.0 Noosphere Dispatch Pipeline (shipped 2026-03-26)
- Schema & dispatch, perception pipeline, executive cognition, tool execution, feedback & reporting

### Out of Scope

- Ghost-to-ghost negotiation or autonomous project creation — ghosts execute dispatched work, they don't create new projects
- Real-time streaming of ghost activity — async reporting through DB is sufficient
- Frontend UI changes to dpn-kb or org graph — this project is backend pipeline only
- Changing the tick engine architecture — extend it, don't rewrite it
- Multi-droplet distribution — everything stays on single node for now

## Context

**Current state (post v1.1):** The full GSD→Ghost pipeline works end-to-end with coordination patterns:
- dispatch_to_db.py writes hierarchical tasks with owner, department, wave context, and blocked_by dependencies
- Perception returns all GSD fields, filters blocked tasks, and includes structured artifacts from predecessors
- Executives decompose projects via LLM, consult shared decision history, delegate with dependency chains
- Staff execute using 67+ tools (DB, API, code via Claude CLI, memory) with typed artifact outputs
- Quality verification with severity levels, lifecycle signals for staff availability, auto-unblock triggers
- Wave advancement, completion reporting, blocker escalation all wired via DB triggers

**Known operational issues:**
- Ghost message spam: agents with stale unread messages get cognition jobs every tick, burning tokens with repetitive "nothing to report" messages. Root cause: `read_by` column never updated, perception returns same messages indefinitely.
- Phase 10 sqlx fix: missing `"json"` feature in dpn-api Cargo.toml prevents metadata JSONB persistence for lifecycle state.

**Key files:**
- `gotcha-workspace/tools/gsd/dispatch_to_db.py` — GSD-to-DB bridge
- `dpn-api/src/handlers/af64_perception.rs` — perception endpoint
- `project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — tick cycle
- `project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — cognition job construction
- `project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — tool execution + message posting

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
| LLM cognition for executive planning | Executives should think like managers, not follow rules | ✓ Good — v1.0 |
| All tool types for staff ghosts | Autonomy requires real capabilities (code, DB, API, external) | ✓ Good — v1.0 (external deferred) |
| Nathan only for blockers + strategy | The whole point is autonomous execution | ✓ Good — v1.0 |
| Dispatch→perceive as first milestone | Prove the pipeline before building on it | ✓ Good — v1.0 |
| Dual feedback (status + conversations) | Status for tracking, conversations for notable events | ✓ Good — v1.0 |
| Incorporate patterns from Squad/ATL/ClawTeam | Proven coordination patterns strengthen existing concepts | ✓ Good — v1.1 |

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
*Last updated: 2026-03-27 after Phase 11 completion*
