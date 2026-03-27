# Roadmap: Noosphere Dispatch Pipeline

## Milestones

- v1.0 Noosphere Dispatch Pipeline (Phases 1-5) -- shipped 2026-03-26
- v1.1 Ghost Coordination Patterns (Phases 6-10) -- shipped 2026-03-27

## Phases

<details>
<summary>v1.0 Noosphere Dispatch Pipeline (Phases 1-5) - SHIPPED 2026-03-26</summary>

- [x] **Phase 1: Schema & Dispatch** - Fix tasks table schema and dispatch_to_db.py so GSD plans persist correctly to master_chronicle
- [x] **Phase 2: Perception Pipeline** - Verify and fix perception endpoint so ghosts see dispatched projects and tasks
- [x] **Phase 3: Executive Cognition** - Executives perceive projects, decompose into staff tasks, and delegate via LLM cognition
- [x] **Phase 4: Tool Execution** - Staff ghosts execute real work using code, DB, API, and external tools
- [x] **Phase 5: Feedback & Reporting** - Close the loop with task completion reporting, wave advancement, and blocker escalation

</details>

<details>
<summary>v1.1 Ghost Coordination Patterns (Phases 6-10) - SHIPPED 2026-03-27</summary>

- [x] **Phase 6: Task Dependency Chains** - Wire blocked_by into perception filtering, auto-unblock on completion, and dependency-aware task creation
- [x] **Phase 7: Structured Artifact Passing** - Typed output schemas per pipeline stage replace untyped stage_notes with validated structured JSON
- [x] **Phase 8: Decisions Brain** - Executives consult and log project decisions before acting, queryable via API
- [x] **Phase 9: Verification Levels** - Quality severity classification on task completion with urgency escalation for critical issues
- [x] **Phase 10: Lifecycle Signals** - Staff signal availability after task completion, executives perceive idle agents for delegation

</details>

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (6.1, 6.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Schema & Dispatch | v1.0 | 2/2 | Complete | 2026-03-26 |
| 2. Perception Pipeline | v1.0 | 2/2 | Complete | 2026-03-26 |
| 3. Executive Cognition | v1.0 | 3/3 | Complete | 2026-03-26 |
| 4. Tool Execution | v1.0 | 2/2 | Complete | 2026-03-26 |
| 5. Feedback & Reporting | v1.0 | 2/2 | Complete | 2026-03-26 |
| 6. Task Dependency Chains | v1.1 | 3/3 | Complete | 2026-03-26 |
| 7. Structured Artifact Passing | v1.1 | 3/3 | Complete | 2026-03-26 |
| 8. Decisions Brain | v1.1 | 2/2 | Complete | 2026-03-26 |
| 9. Verification Levels | v1.1 | 2/2 | Complete | 2026-03-26 |
| 10. Lifecycle Signals | v1.1 | 2/2 | Complete | 2026-03-27 |
