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

(Defined in REQUIREMENTS.md for v1.5)

## Current Milestone: v1.5 InnateScipt Capabilities

**Goal:** Replace the static tool registry with InnateScipt-defined ghost capabilities — every ghost's YAML declares what it can do as live InnateScipt expressions, with executive oversight and team pipeline definitions.

**Target features:**
- Ghost YAML responsibilities as InnateScipt expressions (replaces tool-registry.json)
- Ghost self-modification capability (write/edit own InnateScipt)
- Executive oversight of subordinate responsibilities (prune/add)
- Team/department pipeline definitions with handoff chains in YAML
- Area-scoped table for EM content (structured area tables replacing flat documents)
- InnateScipt wrappers for existing tools (Kalshi, trading, ops Python scripts)
- Orbis YAML foundation (starting_point coordinates, ship assignment, RPG persona)
- Runtime stability fixes (execute-work-task paren bug, tick engine errors)

### Recently Validated (v1.4)

- ✓ Direct PostgreSQL from Lisp via SB-ALIEN FFI to libpq (zero-deps, pool size 2) — Phase 21
- ✓ Full SQL migration: perception, energy, conversations, tasks, tick reports replace HTTP calls — Phase 22
- ✓ Noosphere resolver connecting @, (), {} Innate symbols to master_chronicle tables — Phase 23
- ✓ Template evaluation in ghost cognition: action-planner reads .dpn Templates, evaluates via Innate interpreter — Phase 24
- ✓ Ghost expression generation: innate-builder module with builder functions, validation, template CRUD — Phase 25
- ✓ Cognition pipeline integration: LLM system prompts include Innate syntax, action-executor extracts/validates/persists expressions — Phase 25

### Recently Validated (v1.3)

- ✓ PARAT foundation tables (areas, archives, resources, templates) with DB-level integrity enforcement — Phase 16
- ✓ CRUD API endpoints for all 4 PARAT tables with frozen/immutable 409 enforcement — Phase 16
- ✓ Project lifestage lifecycle (Seed/Sapling/Tree/Harvest) with forward-only DB trigger — Phase 17
- ✓ Goals FK migration from wikilink text to integer project_id — Phase 17
- ✓ Projects linked to areas via area_id FK, perception enriched with lifestage + area_name — Phase 17
- ✓ vault_notes renamed to memories with backward-compatible view bridge (INSTEAD OF triggers) — Phase 18
- ✓ Compression metadata (tier + compressed_from) on all 2831 memories — Phase 18
- ✓ Departments normalized: 8 canonical entries, all 64 agents linked via FK — Phase 18
- ✓ All Rust code (dpn-core + dpn-api) migrated from vault_notes to memories — Phase 18
- ✓ Ghost org structure: 13 teams, 500 typed relationships, 67 area assignments, 11 routines — Phase 19
- ✓ 64 EM Staff documents enriched with YAML frontmatter (agent_id, memory_column, dept, team, area) — Phase 19
- ✓ Nova aliases: {T.A.S.K.S.}, routines table backing Innate (agent){action} expressions — Phase 19
- ✓ Nexus Chat AI deduplicated: 990 canonical conversations from 1984 raw documents — Phase 20
- ✓ Temporal compression cascade: 18 monthly + 7 quarterly + 3 yearly memories from ChatGPT imports — Phase 20
- ✓ Ghost memory injection: 111 domain-routed perspectives across Nova, LRM, Vincent, Sylvia — Phase 20
- ✓ Daily/weekly note linking: 381 notes with ## Nexus Imports wikilink sections — Phase 20

## Shipped Milestones

### v1.4 Ghost Sovereignty (shipped 2026-03-30)
- Direct PostgreSQL via SB-ALIEN FFI to libpq: 63 HTTP calls replaced with SQL, zero-HTTP tick engine
- Noosphere resolver: Innate's @, (), {} symbols resolve against master_chronicle via CLOS protocol
- Template evaluation in cognition: ghosts read/evaluate .dpn Templates, commission delivery triggers real tools
- Ghost expression generation: LLM prompts include Innate syntax, action-executor extracts/validates/persists expressions
- 1,832 new LOC Common Lisp across 6 modules (db-client, db-conversations, db-tasks, db-auxiliary, noosphere-resolver, innate-builder)

### v1.3 PARAT Noosphere Schema (shipped 2026-03-29)
- PARAT five-pillar schema: areas, archives, resources, templates with DB-level integrity
- Project lifestage lifecycle, goals FK migration, memories rename with view bridge
- Ghost org structure: 13 teams, 500 typed relationships, 67 area assignments
- Nexus Chat AI: 990 conversations archived, temporally compressed, injected into 4 executive ghost memories

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
- em-site restructuring / public-facing content publishing — deferred to v1.5
- dpn-tui, dpn-api-client, other iron downstream effects — deferred to v1.5

## Context

**Current state (post v1.5 Phase 30):** The full pipeline is sovereign — ghosts speak directly to PostgreSQL via libpq FFI (zero HTTP in tick path) and speak Innate natively. master_chronicle has 85+ tables with PARAT five-pillar structure. Innate interpreter v1.0 at `/opt/innatescript/` is connected via noosphere-resolver — ghosts evaluate .dpn Templates during cognition and compose new expressions via LLM generation.
- Tick engine runs entirely over SQL: perception, conversations, tasks, energy, tiers — no dpn-api dependency
- Noosphere resolver connects @, (), {} symbols to master_chronicle tables
- Template evaluation enriches cognition with resolved expressions; commission delivery triggers real tools
- Ghosts generate valid Innate expressions with parse-round-trip validation before persistence
- Standing orders: Nova (6 ops tools), Sylvia (editorial), Kathryn (trading + calendar)
- All output attributed to the executing ghost via conversations table
- Ghost YAML capabilities replace tool-registry.json for capability discovery (Phase 28)
- Pipeline definitions live in master_chronicle area_content as JSONB metadata, not hardcoded defparameters (Phase 30)
- Tick engine reloads pipeline definitions from DB each tick; 4 pipelines (engineering, investment, editorial, modular-fortress) migrated
- Orbis spatial identity in ghost YAML (coordinates, ship assignment, RPG persona) — Phase 29
- **Runtime bugs from 2026-03-29:** All fixed (Phase 26), paren scope bug resolved
- **Financial tools in gotcha-workspace:** 4 Kalshi/trading Python scripts need InnateScipt wrappers (Phase 31)

**OpenClaw migration status:** 11 of 14 cron jobs migrated to ghost standing orders. Remaining 3 are OpenClaw-internal and will retire with OpenClaw itself.

**Key files:**
- `gotcha-workspace/tools/gsd/dispatch_to_db.py` — GSD-to-DB bridge
- `project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` — tick cycle with direct SQL
- `project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — cognition prompts with Innate generation instructions
- `project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — tool execution + expression extraction/persistence
- `project-noosphere-ghosts/lisp/runtime/db-client.lisp` — libpq FFI bindings and connection pool
- `project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` — CLOS resolver for Innate symbols
- `project-noosphere-ghosts/lisp/runtime/innate-builder.lisp` — expression constructors, validation, template CRUD
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
| SB-ALIEN FFI to libpq (not Quicklisp) | AF64 zero-deps convention; direct C bindings, 2-connection pool | ✓ Good — v1.4 |
| Direct SQL before Innate integration | DB is prerequisite for resolver; HTTP removal first, then language | ✓ Good — v1.4 |
| CLOS resolver protocol for Innate | Extensible method dispatch, clean separation from interpreter core | ✓ Good — v1.4 |
| LLM-generated expressions with validation | Parse-round-trip ensures only valid Innate persists to templates | ✓ Good — v1.4 |

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
*Last updated: 2026-03-30 — after Phase 30 (Team Pipelines) completed*
