---
phase: quick
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - /opt/project-noosphere-ghosts/README.md
  - /opt/project-noosphere-ghosts/PROJECT_NOOSPHERE_GHOSTS.md
autonomous: true
requirements: []
must_haves:
  truths:
    - "README.md reflects the 22-module AF64 runtime as fully operational"
    - "README.md documents Claude Code CLI as primary LLM provider with 3-provider chain"
    - "README.md documents PostgreSQL direct integration, standing orders, tool registry, and empirical rollups"
    - "PROJECT_NOOSPHERE_GHOSTS.md has no Python file references"
    - "PROJECT_NOOSPHERE_GHOSTS.md checklists reflect completed workstreams (A.1-5,A.7; C.1-5; D.1-4)"
    - "PROJECT_NOOSPHERE_GHOSTS.md current status is updated to 2026-03-29"
  artifacts:
    - path: "/opt/project-noosphere-ghosts/README.md"
      provides: "Accurate documentation of current AF64 runtime"
    - path: "/opt/project-noosphere-ghosts/PROJECT_NOOSPHERE_GHOSTS.md"
      provides: "Updated project tracking with correct completion status"
  key_links: []
---

<objective>
Update Noosphere Ghosts README.md and PROJECT_NOOSPHERE_GHOSTS.md to reflect the current state of the codebase — a fully operational 22-module Common Lisp runtime with Claude Code CLI provider, direct PostgreSQL integration, standing orders, tool registry, and empirical rollups.

Purpose: Documentation is stale — README says "Redesign in progress" and lists hypothetical modules; PROJECT doc references Python files that no longer exist and has unmarked completed items.
Output: Two updated documentation files ready for PR.
</objective>

<execution_context>
@$HOME/.claude/get-shit-done/workflows/execute-plan.md
@$HOME/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@/opt/project-noosphere-ghosts/af64.asd
@/opt/project-noosphere-ghosts/README.md
@/opt/project-noosphere-ghosts/PROJECT_NOOSPHERE_GHOSTS.md
@/opt/project-noosphere-ghosts/config/provider-config.json
</context>

<tasks>

<task type="auto">
  <name>Task 1: Update README.md to reflect current 22-module AF64 runtime</name>
  <files>/opt/project-noosphere-ghosts/README.md</files>
  <action>
Update README.md with the following changes:

1. **Status line**: Change `**Status:** Redesign in progress` to `**Status:** Operational — Common Lisp AF64 runtime`

2. **Tick lifecycle** (line ~156-174): Update the tick steps to match actual implementation:
   - 1. perceive environment (perception.lisp)
   - 2. evaluate drives and energy (drive.lisp, energy.lisp)
   - 3. plan actions / schedule tasks (action-planner.lisp, task-scheduler.lisp)
   - 4. submit cognition requests to broker (cognition-broker.lisp)
   - 5. execute actions with results (action-executor.lisp)
   - 6. report tick results (tick-reporting.lisp)

3. **System Components** (line ~296-338): Replace the hypothetical module list and descriptions with the actual 22-module inventory from af64.asd, organized by layer:

   **Foundation:**
   - `packages.lisp` — Package definitions
   - `util/json.lisp` — JSON encoder/decoder (zero-deps, no Quicklisp)
   - `util/pg.lisp` — PostgreSQL client via libpq FFI (SB-ALIEN)
   - `util/http.lisp` — HTTP client via curl subprocess

   **Runtime Core:**
   - `runtime/rules.lisp` — Constitutional ruleset (Rules for Being a Ghost)
   - `runtime/runtime-paths.lisp` — File path resolution
   - `runtime/self-mod.lisp` — Mutable behavior registries for ghost self-rewrite
   - `runtime/api-client.lisp` — DPN API HTTP client
   - `runtime/db-client.lisp` — Direct PostgreSQL query layer
   - `runtime/cognition-types.lisp` — CognitionJob/CognitionResult schemas
   - `runtime/user-profile.lisp` — Primary user handle resolution

   **LLM Providers:**
   - `runtime/provider-adapters.lisp` — Provider adapter interface
   - `runtime/claude-code-provider.lisp` — Claude Code CLI provider (`claude -p`)
   - `runtime/cognition-broker.lisp` — Shared cognition pool with queue, cache, priority, winter/thaw

   **Perception and Drives:**
   - `runtime/perception.lisp` — Tier-aware substrate scans
   - `runtime/drive.lisp` — Drive ticking and pressure queries
   - `runtime/energy.lisp` — Energy economy helpers

   **Planning and Execution:**
   - `runtime/action-planner.lisp` — Deterministic cognition job planning
   - `runtime/task-scheduler.lisp` — Task prioritization and scheduling
   - `runtime/cron-matcher.lisp` — Standing orders / cron expression matching
   - `runtime/action-executor.lisp` — Applies cognition results to side effects
   - `runtime/tool-socket.lisp` — Tool registry and execution framework

   **Reporting:**
   - `runtime/empirical-rollups.lisp` — Daily/weekly/monthly/quarterly/yearly rollups
   - `runtime/tick-reporting.lisp` — Tick report generation and persistence
   - `runtime/tick-engine.lisp` — Tick orchestrator

   - `main.lisp` — Entry point

4. **Module comparison table** (line ~380-388): Replace the old 6-row Lisp-vs-Python table with a full table covering all 22 modules. Since Python is fully retired, change column header from "Former Python file" to "Capability" and list what each module provides.

5. **Provider config section** (line ~396-427): Add Claude Code CLI as the primary provider. Update to describe the 3-provider chain:
   - **claude-code** (primary): Uses `claude -p` CLI with `--output-format json`, $0.50/request budget, per-tier model selection
   - **anthropic** (fallback): Direct Anthropic HTTP API via local proxy
   - **stub** (last resort): Deterministic fallback for cognitive winter

   Reference `config/provider-config.json` as the canonical configuration file. Keep the existing generic HTTP provider docs but add the claude-code type docs above them.

6. **New section after Memory Model**: Add "Standing Orders and Cron Matching" section describing cron-matcher.lisp — ghosts can have recurring scheduled work matched via cron expressions.

7. **New section after Substrate Contract**: Add "Tool Registry" section describing tool-socket.lisp — extensible tool execution framework for ghost actions.

8. **Memory Model section** (line ~179-205): Update from "expected to exist" language to "implemented" language. Note that empirical-rollups.lisp generates daily/weekly/monthly/quarterly/yearly rollups from tick reports.

9. **Dependencies list** (line ~362-365): Update to reflect current deps:
   - `curl` for HTTP egress
   - `libpq.so.5` (PostgreSQL client library) for direct DB access via SB-ALIEN FFI
   - Access to master_chronicle PostgreSQL database
   - No Quicklisp — zero external Lisp dependencies

10. **Development Status** (line ~356): Change from transitional language to operational. State the legacy Python engine is fully retired and all 22 modules are operational.
  </action>
  <verify>
    <automated>cd /opt/project-noosphere-ghosts && grep -c "Redesign in progress" README.md | grep -q "^0$" && grep -q "claude-code-provider" README.md && grep -q "pg.lisp" README.md && grep -q "empirical-rollups" README.md && grep -q "cron-matcher" README.md && grep -q "tool-socket" README.md && echo "PASS" || echo "FAIL"</automated>
  </verify>
  <done>README.md accurately documents all 22 modules, Claude Code CLI provider chain, PostgreSQL direct integration, standing orders, tool registry, empirical rollups, and shows status as operational</done>
</task>

<task type="auto">
  <name>Task 2: Update PROJECT_NOOSPHERE_GHOSTS.md with current completion status</name>
  <files>/opt/project-noosphere-ghosts/PROJECT_NOOSPHERE_GHOSTS.md</files>
  <action>
Update PROJECT_NOOSPHERE_GHOSTS.md with the following changes:

1. **Python references**: Replace ALL Python file references with Lisp equivalents:
   - `cognition_engine.py` → `runtime/cognition-broker.lisp`
   - `tick_engine.py` → `runtime/tick-engine.lisp`
   - `action_planner.py` → `runtime/action-planner.lisp`
   - `action_executor.py` → `runtime/action-executor.lisp`
   - `perception.py` → `runtime/perception.lisp`
   - `drive_model.py` → `runtime/drive.lisp`
   - `energy.py` → `runtime/energy.lisp`

2. **Workstream A checklist** (line ~282-289): Mark completed items:
   - [x] A.1 through A.5 (already marked)
   - [ ] A.6 stays unchecked (broker persistence still in-memory)
   - [x] A.7 (already marked)

3. **Workstream B checklist** (line ~292-298): Add note that Phase 21 (direct PostgreSQL via libpq FFI) is superseding these API-based items. The Lisp runtime now talks directly to PostgreSQL rather than going through dpn-api HTTP endpoints for cognition state. Keep items unchecked but add a note: "Note: Phase 21 direct PostgreSQL integration supersedes HTTP API approach — ghosts access DB directly via pg.lisp/db-client.lisp"

4. **Workstream C checklist** (line ~300-306): All 5 items already marked [x] — verify and keep.

5. **Workstream D checklist** (line ~308-313): All 4 items already marked [x] — verify and keep.

6. **Phase H milestones** (line ~160-165): Update Python references:
   - `cognition_engine.py scaffold created` → `cognition-broker.lisp operational`

7. **Phase I milestones** (line ~175-181): Update Python references:
   - `tick_engine.py` → `tick-engine.lisp`

8. **Phase H Success Criteria** (line ~254-259): Update `cognition_engine.py` → `cognition-broker.lisp`

9. **Phase I Success Criteria** (line ~262-267 and ~328-332): Update `tick_engine.py` → `tick-engine.lisp`

10. **Current Status section** (line ~368-396): Replace entirely with updated status as of 2026-03-29:

    ```
    ### Status as of 2026-03-29

    **Phase Progress**: Workstreams A, C, D substantially complete. B superseded by direct DB.
    **Current Phase**: Phase 21 (Direct PostgreSQL Foundation) — executing

    **Completed**:
    * Full Common Lisp port — all 22 AF64 modules operational
    * Claude Code CLI integrated as primary LLM provider (3-provider chain)
    * Cognition broker with queue, cache, priority, winter/thaw mechanics
    * Standing orders via cron-matcher for recurring ghost work
    * Tool registry and execution framework (tool-socket.lisp)
    * Empirical rollups: daily/weekly/monthly/quarterly/yearly from tick reports
    * Direct PostgreSQL access via libpq FFI (pg.lisp, zero Quicklisp deps)
    * Action executor applies cognition results to side effects
    * Task scheduler with priority-based scheduling
    * Tick reporting with broker telemetry
    * Legacy Python engine fully retired

    **In Progress**:
    * Phase 21: Direct PostgreSQL queries replacing dpn-api HTTP calls for perception/state
    * A.6: Moving broker state from in-memory to DB-backed persistence

    **Blocked**:
    * None currently
    ```

11. **Progress Log** (line ~422-427): Add entries after the existing 2026-03-11 entry:

    ```
    ### 2026-03-15 - Common Lisp Port Complete
    Full AF64 runtime ported to Common Lisp. All modules operational under ASDF system definition. Legacy Python engine retired.

    ### 2026-03-17 - Claude Code CLI Provider Integrated
    claude-code-provider.lisp added as primary LLM provider using `claude -p` with JSON output. 3-provider chain established: Claude Code CLI -> Anthropic HTTP -> stub fallback.

    ### 2026-03-20 - Standing Orders and Tool Registry
    cron-matcher.lisp enables recurring scheduled work via cron expressions. tool-socket.lisp provides extensible tool execution framework.

    ### 2026-03-24 - Empirical Rollups Operational
    empirical-rollups.lisp generates daily/weekly/monthly/quarterly/yearly summaries from tick report data. Memory model now empirically grounded.

    ### 2026-03-28 - Direct PostgreSQL via libpq FFI
    pg.lisp provides direct PostgreSQL access using SB-ALIEN FFI to libpq.so.5. Zero Quicklisp dependencies maintained. Phase 21 begun to migrate perception and state queries from dpn-api HTTP to direct DB.
    ```

12. **Runtime Stack diagram** (line ~121-148): Update the provider boxes to show the actual 3-provider chain:
    - `claude-code CLI` (primary)
    - `anthropic HTTP` (fallback)
    - `stub / fallback` (cognitive winter)
  </action>
  <verify>
    <automated>cd /opt/project-noosphere-ghosts && grep -c "cognition_engine\.py\|tick_engine\.py\|action_planner\.py\|perception\.py" PROJECT_NOOSPHERE_GHOSTS.md | grep -q "^0$" && grep -q "2026-03-29" PROJECT_NOOSPHERE_GHOSTS.md && grep -q "Phase 21" PROJECT_NOOSPHERE_GHOSTS.md && grep -q "libpq FFI" PROJECT_NOOSPHERE_GHOSTS.md && echo "PASS" || echo "FAIL"</automated>
  </verify>
  <done>PROJECT_NOOSPHERE_GHOSTS.md has zero Python file references, all completed workstream items are checked, current status reflects 2026-03-29, Phase 21 noted, and progress log has milestone entries for Lisp port, Claude Code provider, standing orders, empirical rollups, and PostgreSQL FFI</done>
</task>

</tasks>

<verification>
1. README.md: No mention of "Redesign in progress", all 22 modules listed, Claude Code CLI documented, pg.lisp documented, empirical rollups documented
2. PROJECT_NOOSPHERE_GHOSTS.md: Zero Python file references (`grep -c ".py" should find none in code contexts`), workstream completions marked, status dated 2026-03-29, progress log has 5 new entries
3. Both files are internally consistent and cross-reference correctly
</verification>

<success_criteria>
- README.md status shows operational, not "redesign in progress"
- All 22 modules from af64.asd are documented in README.md
- Claude Code CLI 3-provider chain is documented
- PostgreSQL direct integration (pg.lisp, db-client.lisp) is documented
- Standing orders, tool registry, and empirical rollups are documented
- PROJECT_NOOSPHERE_GHOSTS.md contains no Python file references
- Workstream completion checkboxes match actual state
- Current status section dated 2026-03-29 with accurate progress
- Progress log has entries for major milestones since 2026-03-11
</success_criteria>

<output>
After completion, create `.planning/quick/260329-nkq-update-noosphere-ghosts-readme-md-and-pr/260329-nkq-SUMMARY.md`
</output>
