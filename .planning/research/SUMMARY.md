# Research Summary: Noosphere Dispatch Pipeline

**Domain:** Agentic dispatch and execution pipeline
**Researched:** 2026-03-26
**Overall confidence:** HIGH

## Executive Summary

The Noosphere Dispatch Pipeline is not a build-from-scratch project. It is a plumbing repair project. Every major component already exists -- the Rust API, the Lisp tick engine, the Python dispatch script, the PostgreSQL state store, the perception endpoint, the action executor, the cognition broker. The problem is that the connections between these components are broken at specific, identifiable points.

The tasks table schema does not match what dispatch_to_db.py tries to write. The dispatch script references columns (`source`, `context`) that don't exist. The perception endpoint already returns tasks and projects comprehensively (401 lines of working code in af64_perception.rs), but if no tasks are successfully dispatched, it returns empty results. The project ownership boost (+15/project urgency) is coded in the tick engine but never fires because perception returns no projects for agents that don't own any.

The technology stack requires zero changes. Axum 0.7, sqlx 0.8, SBCL, psycopg2-binary -- all correct for the job. The only new capability needed is extending the Lisp action-executor with new action types for project decomposition and tool execution. Everything else is fixing broken INSERT statements, adding missing columns, and verifying that data flows end-to-end.

The critical risk is ghost hallucination of progress -- agents claiming to complete work without actually executing tools. This is already partially mitigated by the existing `validate-stage-output` function and `tool_scope` checks in perception, but needs explicit enforcement in the new action types.

## Key Findings

**Stack:** No technology changes needed. Fix the plumbing between existing components.
**Architecture:** DB-centric, tick-driven, perception-based. All patterns are sound and well-implemented.
**Critical pitfall:** Ghost hallucination of progress -- agents marking tasks done without tool execution.

## Implications for Roadmap

Based on research, suggested phase structure:

1. **Fix the Pipe** - Schema repair + dispatch fix + perception verification
   - Addresses: Schema alignment, dispatch_to_db.py fix, perception returns dispatched tasks
   - Avoids: Schema mismatch cascade (Pitfall #2)
   - This is the cheapest, highest-leverage work. Everything downstream depends on it.

2. **Executive Cognition** - Executives perceive projects, decompose into tasks, assign to staff
   - Addresses: Executive decomposition, domain-routed assignment, wave ordering
   - Avoids: Hallucination of progress (Pitfall #1), wave ordering ignored (Pitfall #6)
   - Requires: Prompt engineering for structured cognition output. LLM cost monitoring.

3. **Tool Execution** - Staff ghosts execute real work using tools
   - Addresses: Tool dispatch table, code/DB/API tool types
   - Avoids: Budget exhaustion (Pitfall #3)
   - Start with deterministic tools (DB queries, API calls), add LLM-powered tools carefully.

4. **Close the Loop** - Task completion, wave advancement, progress reporting, blocker escalation
   - Addresses: Feedback loop, /gsd:progress, blocker escalation
   - Avoids: Orphaned subtasks (Pitfall #7)

**Phase ordering rationale:**
- Phase 1 must come first because everything else depends on tasks actually existing in the DB.
- Phase 2 before Phase 3 because executives must decompose work before staff can execute it.
- Phase 4 last because feedback loops only matter when there's real execution to report on.

**Research flags for phases:**
- Phase 2: Needs prompt engineering research -- how to make LLM produce structured task breakdown.
- Phase 3: Needs design work on tool sandboxing, especially file write operations.
- Phases 1 and 4: Standard patterns, unlikely to need additional research.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Direct codebase inspection. All dependencies verified on crates.io and PyPI. |
| Features | HIGH | Feature list derived from PROJECT.md requirements + existing code capabilities. |
| Architecture | HIGH | 401-line perception endpoint inspected. Tick engine, action executor, dispatch script all read. |
| Pitfalls | HIGH | Based on actual bugs found in code (schema mismatch) + known LLM agent failure modes. |

## Gaps to Address

- **Actual tasks table schema** -- Need to run `\d tasks` against the live DB to see exactly which columns exist vs which are missing. Research identified the mismatch from code inspection but could not verify against the live schema.
- **Cognition prompt engineering** -- How to make Claude Code CLI produce structured JSON task breakdowns reliably. This is a Phase 2 research topic.
- **Tool scope definitions** -- Which agents have which tool_scope values. Determines who can execute what. Need a DB query to audit this.
- **Budget monitoring** -- No research on cost tracking mechanisms. The cognition broker logs provider and model used, but aggregating cost per project needs design.
