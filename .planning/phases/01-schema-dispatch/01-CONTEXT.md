# Phase 1: Schema & Dispatch - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Fix dispatch_to_db.py so GSD plans persist correctly to master_chronicle with all required metadata — project ownership, hierarchical tasks with subtasks, department routing, and wave ordering. The tasks table schema is already complete; the work is fixing the dispatch script and verifying end-to-end flow.

</domain>

<decisions>
## Implementation Decisions

### Critical Finding: Schema Already Exists
- **D-01:** The tasks table ALREADY has all GSD columns: `project_id`, `source`, `context`, `department`, `stage`, `pipeline_order`, `parent_id`, `assigned_to`, `assigned_by`, `scheduled_at`, `deadline`, `blocked_by`, `stage_notes`, `doc_source`. No ALTER TABLE needed.
- **D-02:** The projects table ALREADY has: `id`, `name`, `slug`, `status`, `description`, `goals`, `owner`, `current_context`, `blockers`, `document_id`. Schema is complete.
- **D-03:** The dispatch script (`dispatch_to_db.py`) already runs successfully — `--status` returns 10 active projects. The INSERT statements work against the live schema.

### Project Name Extraction
- **D-04:** Extract project name from first H1 heading (`# Title`) in PROJECT.md, not YAML frontmatter. This matches how GSD writes projects. Slug derived from name.

### Owner & Department Routing
- **D-05:** Project ownership REQUIRES `--owner` flag on dispatch. No auto-inference. Department is derived from the owner's executive domain (Eliana→engineering, Sylvia→content, etc.).
- **D-06:** `dispatch_project()` must set the `owner` column on the projects row. Currently it doesn't — this is the key fix.
- **D-07:** Tasks inherit department from project owner's domain mapping.

### Task Granularity — Hierarchical Model
- **D-08:** Dispatch creates a PARENT task per PLAN.md file (existing behavior), PLUS individual SUBTASKS for each must_have item, linked via `parent_id` FK.
- **D-09:** Wave numbers apply to parent tasks only. Subtasks inherit wave context implicitly. Executives reorder subtasks as needed during delegation.
- **D-10:** Subtask task_ids follow pattern: `gsd-phase{N}-plan{P}-mh{M}` where M is the must_have index.

### End-to-End Verification
- **D-11:** Two verification approaches: (1) Test fixture with known values for automated validation, (2) Live dispatch of the real Noosphere Dispatch Pipeline project as smoke test.
- **D-12:** Verification checks: project row has owner set, tasks have project_id linkage, subtasks have parent_id, source='gsd', context JSON populated with wave/must_haves, department derived from owner.

### Claude's Discretion
- Error handling improvements in dispatch_to_db.py (connection retries, better error messages)
- Exact department mapping table (owner→department) — standard from CLAUDE.md executive roster
- Whether to add `--dry-run` flag for testing dispatch without writing

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Dispatch Bridge
- `gotcha-workspace/tools/gsd/dispatch_to_db.py` — The dispatch script to modify (265 lines)

### Database Schema
- Live `tasks` table schema — 39 columns including GSD fields (project_id, source, context, department, stage, parent_id, etc.)
- Live `projects` table schema — 12 columns with owner, goals, current_context, blockers
- `gotcha-workspace/tools/db/migrate_to_postgres.sql` — Original migration (may be outdated vs live schema)

### Executive Domain Mapping
- `CLAUDE.md` §Executive Agent Roster — Nova(ops), Eliana(eng), Kathryn(strategy), Sylvia(content), Vincent(creative), JMax(legal), LRM(music), Sarah Lin(routing)

### GSD Planning Artifacts
- `.planning/PROJECT.md` — Example of H1 heading format for name extraction
- `.planning/ROADMAP.md` — Phase structure with plan stubs

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `dispatch_to_db.py` is 265 lines and already functional — modify, don't rewrite
- `parse_frontmatter()` function handles YAML extraction from plans
- `psycopg2.extras.RealDictCursor` pattern already established
- Tasks table has triggers: `task_assigned_notify`, `task_completed_trigger`, `task_rejected_trigger`, `task_set_completed` — these fire automatically

### Established Patterns
- Upsert via `ON CONFLICT (task_id) DO UPDATE` — prevents duplicates on re-dispatch
- `ON CONFLICT (slug) DO UPDATE` for projects — idempotent project creation
- Context stored as JSON string in text column
- Priority mapping: wave 1→high, 2→medium, 3→low

### Integration Points
- Tasks FK to projects via `project_id`
- Tasks FK to parent via `parent_id` (self-referential for subtask hierarchy)
- `assigned_to` is text[] (array) — supports multi-assignment
- `source` defaults to 'obsidian' — GSD tasks should set 'gsd'
- `stage` defaults to 'open' — matches expected initial state
- DB triggers fire on INSERT/UPDATE — notification system already wired

</code_context>

<specifics>
## Specific Ideas

- The Noosphere Dispatch Pipeline project itself should be dispatched as the live smoke test
- Executive domain mapping should match exactly what's in CLAUDE.md (strict domain routing rules)
- The parent_id subtask model leverages existing FK constraint — no schema changes needed

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 01-schema-dispatch*
*Context gathered: 2026-03-26*
