# Phase 1: Schema & Dispatch - Research

**Researched:** 2026-03-26
**Domain:** Python dispatch script + PostgreSQL schema (master_chronicle)
**Confidence:** HIGH

## Summary

Phase 1 modifies `gotcha-workspace/tools/gsd/dispatch_to_db.py` (265 lines) to correctly persist GSD planning artifacts to master_chronicle. The critical finding from research is that the database schema is already complete -- the `tasks` table has all 40 columns including `project_id`, `source`, `context`, `department`, `stage`, `parent_id`, `pipeline_order`, and more. The `projects` table has `owner`, `goals`, `current_context`, and `blockers`. No ALTER TABLE operations are needed.

The dispatch script currently has three bugs: (1) it extracts project name from YAML frontmatter instead of H1 heading, (2) it does not set the `owner` column on the projects row, and (3) it creates one task per PLAN.md but does not create subtasks for individual must_have items. The script also writes to `assignee` (varchar) but the modern routing uses `assigned_to` (text array). Department routing needs to map from the `--owner` agent ID to the department stored in the `agents` table.

**Primary recommendation:** Fix the three dispatch script bugs, add the hierarchical subtask model (parent + child tasks linked via parent_id), implement owner-to-department mapping using live DB lookup from agents table, and verify end-to-end with both fixture and live dispatch.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01: The tasks table ALREADY has all GSD columns: project_id, source, context, department, stage, pipeline_order, parent_id, assigned_to, assigned_by, scheduled_at, deadline, blocked_by, stage_notes, doc_source. No ALTER TABLE needed.
- D-02: The projects table ALREADY has: id, name, slug, status, description, goals, owner, current_context, blockers, document_id. Schema is complete.
- D-03: The dispatch script (dispatch_to_db.py) already runs successfully -- --status returns 10 active projects. The INSERT statements work against the live schema.
- D-04: Extract project name from first H1 heading (# Title) in PROJECT.md, not YAML frontmatter. This matches how GSD writes projects. Slug derived from name.
- D-05: Project ownership REQUIRES --owner flag on dispatch. No auto-inference. Department is derived from the owner's executive domain.
- D-06: dispatch_project() must set the owner column on the projects row. Currently it doesn't -- this is the key fix.
- D-07: Tasks inherit department from project owner's domain mapping.
- D-08: Dispatch creates a PARENT task per PLAN.md file (existing behavior), PLUS individual SUBTASKS for each must_have item, linked via parent_id FK.
- D-09: Wave numbers apply to parent tasks only. Subtasks inherit wave context implicitly. Executives reorder subtasks as needed during delegation.
- D-10: Subtask task_ids follow pattern: gsd-phase{N}-plan{P}-mh{M} where M is the must_have index.
- D-11: Two verification approaches: (1) Test fixture with known values for automated validation, (2) Live dispatch of the real Noosphere Dispatch Pipeline project as smoke test.
- D-12: Verification checks: project row has owner set, tasks have project_id linkage, subtasks have parent_id, source='gsd', context JSON populated with wave/must_haves, department derived from owner.

### Claude's Discretion
- Error handling improvements in dispatch_to_db.py (connection retries, better error messages)
- Exact department mapping table (owner to department) -- standard from CLAUDE.md executive roster
- Whether to add --dry-run flag for testing dispatch without writing

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SCHM-01 | Tasks table has columns for project linkage (project_id), source tracking (source), and GSD context (context) | VERIFIED: Live schema has all three columns. No DDL needed. |
| SCHM-02 | dispatch_to_db.py successfully writes project records to projects table with owner, goals, and status | dispatch_project() needs fix: add --owner flag passthrough and set owner column in INSERT/UPSERT |
| SCHM-03 | dispatch_to_db.py successfully writes task records to tasks table with project linkage and wave metadata | dispatch_phase() works for parent tasks but needs subtask creation for must_have items |
| SCHM-04 | Dispatched tasks include department routing derived from project owner's domain | Need owner-to-department lookup from agents table; mapping verified in DB |
| SCHM-05 | dispatch_to_db.py --status shows accurate project and task status from DB | show_status() already works; enhance to show task hierarchy and department |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

- **Tools must use** `tools/_config.py` for workspace-relative paths (never hardcoded absolute paths)
- **New tools must be** added to `tools/manifest.md`
- **Python venv** at `gotcha-workspace/.venv`
- **DB is the OS** -- state belongs in master_chronicle, not in files
- **Workspace portability** -- all tools must use relative paths
- **GOTCHA tool pattern** -- tools should be usable both as CLI scripts AND as importable functions

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| psycopg2-binary | 2.9.11 | PostgreSQL driver | Already installed in system Python and gotcha-workspace venv |
| Python 3 | 3.12.7 | Runtime | System Python on droplet |
| pytest | 9.0.2 | Test framework | Installed in gotcha-workspace venv |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| json (stdlib) | builtin | Context serialization | Encoding must_haves and wave metadata to context column |
| re (stdlib) | builtin | Slug generation, H1 extraction | Extracting project name from markdown H1 |
| argparse (stdlib) | builtin | CLI interface | Already used in dispatch_to_db.py |

### Alternatives Considered
None -- the stack is locked. This is a Python script modification, not a new tool.

**Installation:** No additional packages needed. All dependencies already present.

## Architecture Patterns

### Current Script Structure (dispatch_to_db.py)
```
dispatch_to_db.py (265 lines)
├── get_db()              # DB connection
├── find_planning_dir()   # Walk up to find .planning/
├── parse_frontmatter()   # YAML from markdown
├── dispatch_project()    # PROJECT.md → projects table
├── dispatch_phase()      # PLAN.md files → tasks table
├── show_status()         # Query DB status
└── main()                # CLI entrypoint
```

### Pattern 1: H1 Heading Extraction (replaces frontmatter name)
**What:** Extract project name from the first `# Title` line in PROJECT.md
**When to use:** Always -- GSD writes projects with H1 headings, not YAML name fields
**Example:**
```python
# Source: verified against /root/.planning/PROJECT.md
def extract_h1_name(text):
    """Extract project name from first H1 heading."""
    for line in text.split('\n'):
        line = line.strip()
        if line.startswith('# '):
            return line[2:].strip()
    return None
```

### Pattern 2: Owner-to-Department Mapping via DB Lookup
**What:** Query the agents table to get the department for a given owner ID
**When to use:** When --owner is provided, look up their department for task routing
**Example:**
```python
# Source: verified against live agents table
def get_owner_department(cur, owner_id):
    """Look up department from agents table for owner."""
    cur.execute("SELECT department FROM agents WHERE id = %s", (owner_id,))
    row = cur.fetchone()
    return row['department'] if row else None
```

### Pattern 3: Hierarchical Task Creation (parent + subtasks)
**What:** Create one parent task per PLAN.md, then individual subtasks per must_have item, linked via parent_id
**When to use:** During dispatch_phase() for every PLAN.md file
**Example:**
```python
# Parent task: gsd-phase1-plan01
# Subtask 1:  gsd-phase1-plan01-mh1
# Subtask 2:  gsd-phase1-plan01-mh2
# Subtasks reference parent via parent_id FK (tasks.id, not task_id)
```

### Pattern 4: Upsert with ON CONFLICT
**What:** INSERT ... ON CONFLICT (task_id) DO UPDATE for idempotent re-dispatch
**When to use:** Every INSERT -- already established pattern in the script
**Why:** Running dispatch twice should update, not duplicate

### Anti-Patterns to Avoid
- **Hardcoded department mapping:** Do NOT hardcode a dict of {agent_id: department}. The agents table is the source of truth. Query it.
- **Using `assignee` instead of `assigned_to`:** The `assignee` column is varchar (legacy Obsidian). The modern routing uses `assigned_to` (text array). Set both for compatibility, but `assigned_to` is what perception uses.
- **Byte slicing on markdown text:** Use string operations, never byte indices (Rust UTF-8 rule applies conceptually even in Python -- be explicit about character operations)

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Department mapping | Hardcoded Python dict | `SELECT department FROM agents WHERE id = %s` | DB is source of truth; agents table already has correct departments |
| Connection management | Custom retry logic | psycopg2 default with proper error messages | Script runs locally on the droplet; connection is always localhost |
| Slug generation | Custom slugify function | `re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")` | Already in the script, works correctly |
| YAML parsing | External YAML lib | Existing parse_frontmatter() | Only needs key:value pairs, already works |

## Common Pitfalls

### Pitfall 1: parent_id vs task_id Confusion
**What goes wrong:** Subtasks need `parent_id` set to the parent's **integer id** (primary key), not the parent's **task_id** (string). The FK constraint is `tasks_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES tasks(id)`.
**Why it happens:** The script works with task_id strings (like "gsd-phase1-plan01") but parent_id is an integer FK to tasks.id.
**How to avoid:** After inserting the parent task, capture the RETURNING id value and use that integer as parent_id for subtasks.
**Warning signs:** FK constraint violation error on subtask INSERT.

### Pitfall 2: assigned_to is text[] Not varchar
**What goes wrong:** Setting assigned_to as a plain string fails or gets stored wrong.
**Why it happens:** The column is `text[]` (PostgreSQL array type), not varchar.
**How to avoid:** Use `ARRAY[%s]::text[]` in the INSERT, or `'{owner_id}'` array literal.
**Warning signs:** Type mismatch error or perception endpoint not finding the task.

### Pitfall 3: Trigger Side Effects on INSERT
**What goes wrong:** The `task_assigned_notify` trigger fires on every INSERT. If assigned_to is populated, it sends a pg_notify event.
**Why it happens:** Trigger is AFTER INSERT FOR EACH ROW -- fires even during dispatch.
**How to avoid:** This is actually desired behavior -- it means ghosts can perceive newly dispatched tasks immediately. Just be aware it happens. If testing, don't be surprised by NOTIFY events.
**Warning signs:** None -- this is correct behavior, just worth knowing.

### Pitfall 4: Project Name Extraction Falls Back to "Untitled"
**What goes wrong:** Current code reads frontmatter for name, but PROJECT.md uses H1 heading format with no name in frontmatter. Gets "Untitled GSD Project".
**Why it happens:** `fm.get("name", fm.get("title", "Untitled GSD Project"))` -- frontmatter has no name/title key.
**How to avoid:** Parse H1 heading first, then fall back to frontmatter, then to "Untitled".
**Warning signs:** Project row with name "Untitled GSD Project" or wrong slug.

### Pitfall 5: must_have Extraction from PLAN.md
**What goes wrong:** must_haves are in the frontmatter as a YAML field, but the simple parser may not handle multi-line YAML lists.
**Why it happens:** `parse_frontmatter()` does simple `key: value` line splitting. Lists like `must_haves:\n  - item1\n  - item2` won't parse correctly.
**How to avoid:** Parse must_haves from the markdown body using regex to find the must_haves section, OR ensure PLAN.md frontmatter uses a single-line comma-separated format. Since PLAN.md files don't exist yet (TBD), the planner controls the format.
**Warning signs:** Empty must_haves in context JSON.

### Pitfall 6: Re-dispatch Overwrites Subtasks But Not Parent Link
**What goes wrong:** ON CONFLICT DO UPDATE on subtask task_ids will update text/context but if the parent task was also re-dispatched and got a NEW integer id, subtask parent_id still points to the old one.
**Why it happens:** Upsert doesn't update parent_id by default.
**How to avoid:** Include parent_id in the ON CONFLICT DO UPDATE SET clause. Or dispatch parent first, get its id, then upsert subtasks with the current parent id.
**Warning signs:** Subtasks orphaned from parent after re-dispatch.

## Code Examples

### Verified: Live Database Schema (tasks table key columns for GSD)
```sql
-- Source: \d tasks on live master_chronicle 2026-03-26
-- All columns below ALREADY EXIST -- no ALTER TABLE needed
project_id     integer          -- FK to projects(id)
source         varchar(32)      -- DEFAULT 'obsidian', set to 'gsd'
context        text             -- JSON string with wave/must_haves
department     varchar(64)      -- From owner's executive domain
stage          varchar(32)      -- DEFAULT 'open'
pipeline_order integer          -- DEFAULT 0
parent_id      integer          -- FK to tasks(id), self-referential
assigned_to    text[]           -- Array type for multi-assignment
assigned_by    varchar(64)      -- Who assigned (e.g., 'nathan' or 'gsd')
blocked_by     integer          -- FK to blocking task
stage_notes    text             -- Additional stage context
```

### Verified: Executive Agent ID to Department Mapping
```python
# Source: SELECT id, department FROM agents WHERE agent_tier = 'executive'
# Queried from live DB 2026-03-26
EXECUTIVE_DEPARTMENTS = {
    'eliana':  'Engineering',
    'jmax':    'Legal',
    'kathryn': 'Executive',      # Note: CSO but dept is "Executive"
    'lrm':    'Research',         # Note: Musicology head but dept is "Research"
    'nova':    'Systems',         # Note: COO but dept is "Systems"
    'sarah':   'Office of the CEO',  # Note: agent id is 'sarah' not 'sarah-lin'
    'sylvia':  'Content & Brand', # Note: includes "& Brand"
    'vincent': 'Creative',
}
# RECOMMENDATION: Do NOT hardcode this. Query agents table at runtime.
# This dict is for reference/testing only.
```

### Verified: Current dispatch_project() Bug (line 79)
```python
# CURRENT (broken for GSD):
name = fm.get("name", fm.get("title", "Untitled GSD Project"))
# PROJECT.md has no frontmatter name -- it uses H1: "# Noosphere Dispatch Pipeline"

# FIX:
# 1. Try H1 heading first
# 2. Fall back to frontmatter name/title
# 3. Last resort: "Untitled GSD Project"
```

### Verified: Current dispatch_project() Missing Owner (line 91-99)
```python
# CURRENT INSERT (no owner column):
cur.execute(
    """INSERT INTO projects (name, slug, status, description, goals, updated_at)
       VALUES (%s, %s, 'active', %s, %s, NOW())
       ON CONFLICT (slug) DO UPDATE SET ...
       RETURNING id, name""",
    (name, slug, description, goals),
)

# FIX: Add owner to INSERT and ON CONFLICT UPDATE
# dispatch_project() needs to accept owner parameter
```

### Verified: Context JSON Structure for Tasks
```python
# Current pattern (line 188):
json.dumps({
    "wave": wave,
    "must_haves": must_haves,
    "requirements": requirements,
    "depends_on": depends_on
})
# This is correct structure. Subtasks should carry their specific must_have text.
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Obsidian-only task sync | GSD dispatch + Obsidian | Phase 1 (now) | Tasks from two sources, distinguished by `source` column |
| Flat tasks (no hierarchy) | Parent + subtask via parent_id | Phase 1 (now) | Enables executive delegation model |
| No project ownership | Owner on project row + department routing | Phase 1 (now) | Enables domain-scoped perception |

## Open Questions

1. **PLAN.md frontmatter format for must_haves**
   - What we know: PLAN.md files don't exist yet (TBD in roadmap). The simple YAML parser handles single-line `key: value` pairs.
   - What's unclear: Will must_haves be a YAML list (multi-line) or comma-separated string?
   - Recommendation: The planner should define PLAN.md format with must_haves as either a markdown section (parsed from body) or single-line comma-separated in frontmatter. The dispatch script should handle both.

2. **--dry-run flag**
   - What we know: User gave Claude's discretion on this.
   - Recommendation: Add it. Wraps all DB operations in a transaction that gets rolled back. Useful for both testing and the verification test fixture.

3. **dispatch_to_db.py should use _config.py**
   - What we know: CLAUDE.md mandates `tools/_config.py` for all gotcha-workspace tools. Current dispatch script hardcodes DB credentials inline.
   - Recommendation: Refactor to import `PG_CONFIG` from `tools._config` for DB connection. This aligns with workspace conventions.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| PostgreSQL | All DB operations | Yes | (live on localhost:5432) | -- |
| Python 3 | dispatch_to_db.py | Yes | 3.12.7 | -- |
| psycopg2 | DB driver | Yes | 2.9.11 (system) / 2.9.9 (venv) | -- |
| pytest | Test fixture | Yes | 9.0.2 (gotcha-workspace venv) | -- |
| gotcha-workspace venv | Tool conventions | Yes | /root/gotcha-workspace/.venv | -- |

**Missing dependencies with no fallback:** None -- all dependencies available.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | pytest 9.0.2 |
| Config file | None -- see Wave 0 |
| Quick run command | `/root/gotcha-workspace/.venv/bin/pytest tests/test_dispatch.py -x` |
| Full suite command | `/root/gotcha-workspace/.venv/bin/pytest tests/ -x` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| SCHM-01 | Tasks table has project_id, source, context columns | smoke (DB schema check) | `pytest tests/test_dispatch.py::test_schema_columns -x` | No -- Wave 0 |
| SCHM-02 | dispatch_project() writes project with owner | integration | `pytest tests/test_dispatch.py::test_dispatch_project_with_owner -x` | No -- Wave 0 |
| SCHM-03 | dispatch_phase() writes parent + subtasks with project linkage | integration | `pytest tests/test_dispatch.py::test_dispatch_phase_hierarchy -x` | No -- Wave 0 |
| SCHM-04 | Tasks have department from owner's domain | integration | `pytest tests/test_dispatch.py::test_department_routing -x` | No -- Wave 0 |
| SCHM-05 | --status shows accurate counts | integration | `pytest tests/test_dispatch.py::test_status_output -x` | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** `/root/gotcha-workspace/.venv/bin/pytest tests/test_dispatch.py -x`
- **Per wave merge:** Full suite + live dispatch smoke test
- **Phase gate:** Full suite green + live dispatch of Noosphere Dispatch Pipeline project

### Wave 0 Gaps
- [ ] `tests/test_dispatch.py` -- covers SCHM-01 through SCHM-05
- [ ] `tests/conftest.py` -- shared fixtures (DB connection, test planning dir with fixture PROJECT.md and PLAN.md files)
- [ ] Test cleanup: dispatch test should create/delete its own test project and tasks (use unique slug like `test-gsd-dispatch-fixture`)

## Sources

### Primary (HIGH confidence)
- Live `\d tasks` query on master_chronicle -- 40 columns verified, all GSD columns present
- Live `\d projects` query on master_chronicle -- 12 columns verified, owner column exists
- Live `SELECT id, department FROM agents WHERE agent_tier = 'executive'` -- 7 executives + nathan mapped
- `gotcha-workspace/tools/gsd/dispatch_to_db.py` -- full source read (265 lines)
- `/root/.planning/PROJECT.md` -- H1 heading format verified ("# Noosphere Dispatch Pipeline")
- `gotcha-workspace/tools/_config.py` -- PG_CONFIG pattern verified

### Secondary (MEDIUM confidence)
- DB trigger `notify_task_assigned` source verified -- fires on INSERT with assigned_to populated
- `tasks_parent_id_fkey` FK constraint verified -- parent_id references tasks(id)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all libraries verified installed with exact versions
- Architecture: HIGH -- existing script read in full, DB schema queried live, all columns verified
- Pitfalls: HIGH -- derived from actual code bugs and live schema constraints

**Research date:** 2026-03-26
**Valid until:** 2026-04-26 (stable -- schema and script unlikely to change during project)
