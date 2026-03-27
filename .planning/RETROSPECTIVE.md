# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

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

| Metric | v1.0 | v1.1 |
|--------|------|------|
| Phases | 5 | 5 |
| Plans | 11 | 12 |
| Avg plan duration | 4 min | 4 min |
| Total execution | ~44 min | ~48 min |
| Requirements | 12 | 16 |
| Model | opus | opus |
