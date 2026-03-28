# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v1.2 -- Operational Readiness

**Shipped:** 2026-03-28
**Phases:** 5 | **Plans:** 8

### What Was Built
- Mark-as-read API endpoint with read_by perception filtering, eliminating ghost message spam (2229 stale messages cleaned)
- Standing orders framework: JSONB schedule on projects, Lisp cron matcher, +50 urgency boost on schedule fire
- 9 operational tools registered: 6 for Nova (ops), 1 for Sylvia (editorial), 2 for Kathryn (financial)
- Dynamic label-to-tool mapping generalized for any executive/project (11 mappings total)
- execute-project-review wired to process-tool-calls so standing orders can execute real tools

### What Worked
- Auto-advance pipeline (discuss -> plan -> execute per phase) completed 4 phases in a single session
- Phase 13/14/15 followed identical patterns -- tool registration + label mapping became formulaic
- Worktree isolation for executor agents prevented merge conflicts on source code
- Dynamic mapping generalization in Phase 14 paid off immediately in Phase 15

### What Was Inefficient
- STATE.md merge conflicts on every worktree merge (orchestrator and agent both update it)
- Researcher agents sometimes found issues (auth blocker, env vars) that didn't fully propagate into plans
- Checker warnings about env vars and Discord creds were noted but not acted on in auto mode

### Patterns Established
- Tool registration pattern: JSON entry in tool-registry.json + label mapping in action-planner.lisp
- Per-project standing orders: schedule JSONB array on projects table, tick engine evaluates each tick
- Dynamic tool-mapping-for-label: cond-based dispatch, extensible without modifying existing entries
- Ghost tool execution: existing Python scripts triggered via tool invocation, no rewrites needed

### Key Lessons
- **Formulaic phases are fast.** Phases 13-15 were structurally identical -- once the pattern was established, execution was near-mechanical.
- **Standing orders are identity.** Nova IS T.A.S.K.S. -- the schedules aren't jobs assigned to her, they're her operational rhythm.
- **Noosphere-native output.** Conversations table, not Discord. Ghosts live in the substrate; external delivery is a separate concern.
- **Auto mode works for well-understood patterns.** The discuss->plan->execute auto chain was effective because each phase followed a known template.

### Cost Observations
- Model mix: opus (executor, planner, researcher), sonnet (checker, verifier)
- Sessions: 1 session for all 5 phases (auto-advance chain)
- Notable: 8 plans executed in ~30 min total. Phases 14-15 were under 10 min each.

---

## Milestone: v1.1 -- Ghost Coordination Patterns

**Shipped:** 2026-03-27
**Phases:** 5 | **Plans:** 12

### What Was Built
- Task dependency chains with INTEGER[] blocked_by, auto-unblock triggers, and wave-ordered dispatch
- Structured artifact passing with JSONB stage_notes, JSON schema validation, and DB-sourced predecessor context
- Shared decisions brain with REST API and automatic capture/injection in executive cognition
- Verification severity levels with quality issue extraction and +40 CRITICAL urgency boost
- Lifecycle signals with idle detection, metadata persistence, and enriched team rosters

### What Worked
- Phase-per-session execution: each phase (discuss -> plan -> execute) completed in a single focused session
- GSD tooling automated most bookkeeping (summary extraction, state tracking, roadmap updates)
- Lisp/Rust split worked well: Rust for API + data, Lisp for behavior + cognition
- Small focused plans (1-3 tasks each) with clear must_haves made execution fast and verifiable

### What Was Inefficient
- ROADMAP.md phase checklist state got stale during execution (some phases showing wrong completion status)
- sqlx feature flag gap (missing `"json"`) wasn't caught during Phase 10 execution -- only found during live verification
- Ghost message spam not caught until post-milestone operational review -- need operational smoke testing as part of verification

### Patterns Established
- Two-pass dependency population in dispatch_to_db.py (create parents first, then populate blocked_by)
- COALESCE + JSONB merge pattern for non-destructive metadata updates in Rust
- Lifecycle state derivation from tick classification (not stored separately, computed from behavior)
- schema_version field in JSONB artifacts for forward compatibility

### Key Lessons
- **Verify against live system, not just unit tests.** The sqlx feature gap and message spam were invisible to code-level verification but immediately obvious against the running system.
- **read_by column must be used.** The perception API returning stale messages is the #1 token waste issue -- must be fixed in v1.2.
- **Execution velocity is high when plans are small.** Average 4 min/plan across 12 plans. No plan took more than 16 min.

### Cost Observations
- Model mix: 100% opus (executor and planner)
- Sessions: ~6 (1 per phase + milestone completion)
- Notable: 48 min total execution for 12 plans is very efficient

---

## Milestone: v1.0 -- Noosphere Dispatch Pipeline

**Shipped:** 2026-03-26
**Phases:** 5 | **Plans:** 11

### What Was Built
- dispatch_to_db.py bridge from GSD plans to master_chronicle
- Perception endpoint with GSD task fields and urgency boosts
- Executive cognition with LLM decomposition and CREATE_TASK delegation
- Staff tool execution (DB, API, code via Claude CLI, memory)
- Feedback loop with wave advancement, completion reports, blocker escalation

### What Worked
- DB-first architecture meant all coordination was through existing PostgreSQL -- no new infrastructure needed
- Extending the tick engine (not rewriting) preserved working functionality
- E2E test scripts against live API caught real integration issues

### What Was Inefficient
- dispatch_to_db.py had multiple rounds of fixes for schema mismatches discovered during execution
- Tool scope audit across 64 agents was manual and tedious

### Key Lessons
- **Start with the bridge, not the brain.** Getting data flowing (dispatch -> perceive) before adding intelligence (cognition -> execution) was the right order.
- **E2E scripts are essential.** Unit tests pass but integration breaks. Always verify against live DB + API.

---

## Cross-Milestone Trends

| Metric | v1.0 | v1.1 | v1.2 |
|--------|------|------|------|
| Phases | 5 | 5 | 5 |
| Plans | 11 | 12 | 8 |
| Avg plan duration | 4 min | 4 min | 4 min |
| Total execution | ~44 min | ~48 min | ~30 min |
| Requirements | 12 | 16 | 17 |
| Model | opus | opus | opus+sonnet |
