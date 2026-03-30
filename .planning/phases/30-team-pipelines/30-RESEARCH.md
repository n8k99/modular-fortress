# Phase 30: Team Pipelines - Research

**Researched:** 2026-03-30
**Domain:** Common Lisp tick engine pipeline advancement, PostgreSQL area_content, InnateScipt expressions
**Confidence:** HIGH

## Summary

Phase 30 replaces three hardcoded data structures in the Lisp tick engine -- `*pipeline-advancement*` (action-executor.lisp line 175), `*prev-stage-map*` (action-planner.lisp line 389), and `detect-pipeline-type` (action-executor.lisp line 295) -- with functions that load pipeline definitions from master_chronicle. The `area_content` table (created in Phase 27) already has the right schema for storing department Assignments.md entries with pipeline definitions embedded as structured data in the `body` or `metadata` JSONB column.

The current codebase has 4 hardcoded pipelines (engineering, investment, editorial, modular-fortress) with 28 stage-to-next-stage mappings, 2 fork definitions, and a parallel `prev-stage-map` of 17 entries. All call sites are contained: `advance-pipeline` is called from one location (action-executor.lisp line 580), `load-predecessor-stage-output` from one location (action-planner.lisp line 449), and `detect-pipeline-type` from two locations (lines 296, 447). Function signatures remain unchanged per D-14.

**Primary recommendation:** Store pipeline definitions as JSONB in the `metadata` column of `area_content` rows (one per department, content_type='pipeline'), load them into equivalent alist structures at tick startup, and replace the three defparameters with functions that read from the loaded cache. The existing `noosphere-resolver.lisp` area content resolution pattern shows exactly how to query area_content by slug and content_type.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01: Pipeline definitions live in master_chronicle, not filesystem
- D-02: Department pipelines stored as area_content entries (Assignments.md per department)
- D-03: Project-specific pipelines as InnateScipt on the project
- D-04: Assignments.md is markdown with embedded InnateScipt -- living playbook, not pure data
- D-05: Ghost YAML responsibilities echo DB assignments; DB authoritative, YAML is runtime cache
- D-06: Pipeline steps are ordered sequence with stage name and assigned ghost per step
- D-07: Stage names are pipeline-scoped -- (pipeline, stage) compound key
- D-08: Stage names feed em-org-graph visualization
- D-09: Forks supported (one stage unblocks multiple downstream)
- D-10: Joins deferred -- no waiting-for-multiple-predecessors logic
- D-11: Existing task columns sufficient, no schema migration
- D-12: Pipeline type from DB lookup via goal_id -> project -> pipeline definition
- D-13: Replace *pipeline-advancement* defparameter with DB-loaded function
- D-14: Keep advance-pipeline, build-pipeline-task-job, load-predecessor-stage-output signatures intact
- D-15: detect-pipeline-type and prev-stage-map also replaced by DB lookups
- D-16: Python tools for pipeline CRUD live in noosphere
- D-17: Project hierarchy via ownership/delegation, not parent_id column

### Claude's Discretion
- Exact InnateScipt expression syntax for pipeline step definitions
- Which departments get initial Assignments.md entries (recommend: all 8 departments with active pipelines)
- Loading/caching strategy (per-tick reload vs cached with invalidation)
- Exact format of pipeline data for em-org-graph color-coded visualization

### Deferred Ideas (OUT OF SCOPE)
- Join logic (waiting for multiple predecessors)
- Parent-child project relationships as schema (parent_id column)
- Full tool-registry.json retirement (Phase 31)
- Python tool InnateScipt wrappers (Phase 31)
- em-org-graph visual redesign for pipeline color coding
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PIPE-01 | Department/team YAML has assignments: section defining pipeline handoff chains | CONTEXT.md evolved this: DB is authoritative, YAML echoes it. The area_content table stores pipeline definitions; ghost YAML responsibilities reference them. |
| PIPE-02 | Pipeline definitions specify step sequence with ghost assignment per step | Ordered list of `(stage-name, assignee, pipeline-order)` triples stored as JSONB in area_content.metadata or body. Fork targets stored as additional downstream entries on a stage. |
| PIPE-03 | Tick engine routes pipeline handoffs using DB definitions instead of hardcoded *pipeline-advancement* | Replace defparameter with `get-pipeline-advancement` function that queries cached pipeline data loaded from area_content at tick startup. |
| PIPE-04 | Pipeline state tracked per-task (current step, next ghost) in task metadata | Already implemented: tasks.stage, tasks.goal_id, tasks.pipeline_order, tasks.stage_notes columns exist and are used by advance-pipeline. No schema change needed. |
</phase_requirements>

## Architecture Patterns

### Current Pipeline Data Flow (Being Replaced)

```
*pipeline-advancement* (defparameter, hardcoded alist)
  |
  v
advance-pipeline (action-executor.lisp:342)
  reads: (assoc current-stage *pipeline-advancement* :test #'string-equal)
  returns: (next-stage . next-assignee) or nil
  also: hardcoded fork-targets cond block (lines 403-427)
  also: iterates *pipeline-advancement* for energy rewards (line 438)
  |
detect-pipeline-type (action-executor.lisp:295)
  reads: hardcoded member lists per pipeline name
  returns: "engineering" | "investment" | "editorial" | "modular-fortress" | nil
  |
*prev-stage-map* (local let* in load-predecessor-stage-output, action-planner.lisp:389)
  reads: (assoc current-stage prev-stage-map :test #'string-equal)
  returns: predecessor stage name string
```

### New Pipeline Data Flow (Target)

```
area_content table (content_type='pipeline', one row per department pipeline)
  + project metadata JSONB (for project-specific pipelines)
  |
  v
load-all-pipelines (new function in db-client or new pipeline module)
  queries: area_content WHERE content_type = 'pipeline'
  queries: projects with pipeline metadata
  builds: *loaded-pipeline-advancement* (runtime alist, same shape as current)
  builds: *loaded-prev-stage-map* (runtime alist, same shape as current)
  builds: *loaded-pipeline-types* (stage -> pipeline-name mapping)
  builds: *loaded-fork-targets* (stage -> list of (fork-stage . fork-assignee))
  |
  v
get-pipeline-advancement (replaces direct *pipeline-advancement* access)
  returns: same (next-stage . next-assignee) shape
  |
get-prev-stage (replaces local prev-stage-map)
  returns: same predecessor stage name string
  |
get-pipeline-type (replaces detect-pipeline-type)
  returns: same pipeline type string
  |
get-fork-targets (replaces hardcoded cond block)
  returns: same list of (fork-stage . fork-assignee)
```

### Recommended Project Structure

```
lisp/runtime/
  pipeline-definitions.lisp  # NEW: pipeline loading, caching, accessor functions
  action-executor.lisp       # MODIFIED: replace defparameter + detect-pipeline-type + fork cond
  action-planner.lisp        # MODIFIED: replace local prev-stage-map
  ghost-capabilities.lisp    # UNCHANGED: model for YAML<->DB loading pattern
  noosphere-resolver.lisp    # UNCHANGED: model for area_content querying pattern
  db-client.lisp             # UNCHANGED: provides db-query, db-escape
lisp/packages.lisp           # MODIFIED: add pipeline-definitions package
```

### Pattern: Pipeline Definition Storage in area_content

The `area_content` table schema:

| Column | Type | Use for Pipelines |
|--------|------|-------------------|
| id | integer PK | auto |
| area_id | integer FK -> areas | 1 (EM Corp) for all department pipelines |
| content_type | varchar(64) | `'pipeline'` |
| title | varchar(512) | `'Engineering Pipeline'`, `'Editorial Pipeline'` etc. |
| body | text | Markdown Assignments.md content with embedded InnateScipt |
| metadata | jsonb | Structured pipeline definition (stages, assignees, forks) |
| status | varchar(32) | `'active'` |

**Recommended metadata JSONB format:**

```json
{
  "pipeline_name": "engineering",
  "department": "Engineering",
  "stages": [
    {"name": "spec", "assignee": "isaac", "order": 1},
    {"name": "infra-review", "assignee": "casey", "order": 2},
    {"name": "design", "assignee": "casey", "order": 3},
    {"name": "build", "assignee": "devin", "order": 4},
    {"name": "security-review", "assignee": "sanjay", "order": 5},
    {"name": "test", "assignee": "danielle", "order": 6},
    {"name": "deploy", "assignee": "morgan", "order": 7}
  ],
  "forks": [
    {"from_stage": "discovery", "to_stages": [
      {"name": "pattern-analysis", "assignee": "ibrahim_hassan"},
      {"name": "architecture-research", "assignee": "felix_wu"}
    ]}
  ],
  "terminal_assignee": "eliana"
}
```

This shape can be mechanically converted to the same alist format used by the current `*pipeline-advancement*`:

```
("spec" . ("infra-review" . "isaac"))
("infra-review" . ("design" . "casey"))
...
```

### Anti-Patterns to Avoid
- **Parsing pipeline definitions from markdown body at runtime:** The body field is for human-readable Assignments.md content. Structured pipeline data goes in the metadata JSONB column. Do not regex-parse markdown.
- **Loading pipelines from the DB on every advance-pipeline call:** This is called per-task per-tick. Load once at tick startup, cache in a defvar, reload per tick (30s-10min interval makes per-tick reload acceptable).
- **Changing advance-pipeline function signature:** D-14 explicitly locks this. The data source changes, the interface stays the same.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JSONB querying | Custom JSON string parsing | `metadata->>'pipeline_name'` SQL + `parse-json` | PostgreSQL JSONB operators are battle-tested |
| Pipeline alist construction | Manual alist building from raw SQL rows | Transform metadata JSONB through `parse-json` to hash-table to alist | The AF64 JSON parser already converts to hash-tables with keyword keys |
| Area content queries | New query functions | Existing `resolve-from-area-content` pattern in noosphere-resolver.lisp | Same table, same query pattern, just different content_type filter |

## Current Code Inventory (Complete)

### `*pipeline-advancement*` (action-executor.lisp:175-209)

Shape: `((stage-string . (next-stage-string . next-assignee-string)) ...)`

4 pipelines, 28 entries total:
- Engineering: spec -> infra-review -> design -> build -> security-review -> test -> deploy -> done
- Investment: thesis -> research -> analysis -> compliance -> documentation -> approval -> done
- Editorial: collection -> research -> curation -> composition -> editing -> polish -> publish -> done
- Modular Fortress: discovery -> pattern-analysis/architecture-research (fork) -> security-audit -> synthesis -> tool-audit/security-standards (fork) -> module-standards -> done

**Problem with current alist:** Stage name "research" appears in BOTH editorial and investment pipelines. The alist lookup returns the FIRST match, which means editorial "research" -> "curation" is shadowed by investment "research" -> "analysis". The `detect-pipeline-type` function has a special case for this (line 313: default to "investment"). The new DB-scoped lookup with (pipeline, stage) compound key eliminates this collision.

### `detect-pipeline-type` (action-executor.lisp:295-315)

Used at: line 447 (inside advance-pipeline, when pipeline is done, to determine where to persist deliverable)

Hardcoded stage-name-to-pipeline-type mapping. With DB-loaded pipelines, each pipeline definition carries its own `pipeline_name`, so this becomes a simple lookup from the loaded data.

### `prev-stage-map` (action-planner.lisp:389-416)

Local let* variable inside `load-predecessor-stage-output`. Shape: `((current-stage . predecessor-stage) ...)`. 17 entries covering all 4 pipelines. This is the inverse of `*pipeline-advancement*` -- for each stage, what came before it.

Can be mechanically derived from the pipeline stages array: stage N's predecessor is stage N-1. For forks, the predecessor is the fork source stage.

### Fork handling (action-executor.lisp:403-427)

Hardcoded cond block inside `advance-pipeline`:
- "discovery" forks to: `(("architecture-research" . "felix_wu"))`
- "synthesis" forks to: `(("security-standards" . "sanjay"))`

The fork targets are unblocked in addition to the primary next-stage from `*pipeline-advancement*`. This needs to be representable in the DB format (the `forks` array in the recommended JSONB structure handles this).

### Call site: advance-pipeline invocation (action-executor.lisp:580)

```lisp
(advance-pipeline (gethash :id task) stage agent-id content
                  :goal-id (gethash :goal-id task)
                  :task-text (gethash :text task))
```

Called when `validate-stage-output` passes. The task's `goal-id` links to the parent goal task, which links to a project via `project_id`. This chain enables DB pipeline lookup: task -> goal_id -> project_id -> project -> pipeline definition.

### Energy reward iteration (action-executor.lisp:438)

```lisp
(dolist (entry *pipeline-advancement*)
  (let ((participant (cddr entry))) ...))
```

When pipeline reaches "done", iterates ALL entries in `*pipeline-advancement*` to find participants for +30 energy reward. With DB-loaded data, this needs to iterate the pipeline-specific entries only (better, actually -- currently rewards participants from ALL pipelines).

## Pipeline Type Detection via goal_id Chain

Per D-12, pipeline type detection moves from stage-name lookup to DB lookup. The chain:

1. Task has `goal_id` (integer, FK to parent goal task)
2. Goal task has `project_id` (integer, FK to projects table)
3. Project links to a pipeline definition via:
   a. Department pipelines: project's `owner` -> agent's `department` -> area_content with matching department
   b. Project-specific pipelines: project metadata JSONB could carry pipeline_name

For the initial implementation, the simplest approach: the `metadata` JSONB on area_content pipeline rows includes the project IDs or project names it applies to, OR pipeline lookup goes through the project's department.

**Recommended approach:** Load all pipeline definitions at tick startup. Build a lookup table mapping `(project_id -> pipeline_name)` from: (a) area_content metadata listing associated project IDs, (b) project owner -> department -> department pipeline as fallback. When `advance-pipeline` needs the pipeline for a task, look up the task's goal -> project -> pipeline_name -> cached pipeline stages.

## Loading/Caching Strategy

**Recommendation: Per-tick reload (simple, sufficient).**

Tick interval is 30s-10min. Pipeline definitions change rarely (minutes to hours). Loading 4-8 pipeline definitions from area_content is a single SQL query returning a few KB of JSONB. Cost: ~1ms per tick. No cache invalidation complexity needed.

Pattern (following ghost-capabilities.lisp model):

```lisp
(defvar *pipeline-cache* nil
  "Cached pipeline definitions loaded from area_content. Refreshed each tick.")

(defun reload-pipeline-definitions ()
  "Load all active pipeline definitions from area_content.
   Called once per tick in the perceive phase."
  (let ((rows (db-query "SELECT title, metadata FROM area_content
                         WHERE content_type = 'pipeline' AND status = 'active'")))
    (setf *pipeline-cache* (build-pipeline-structures rows))))
```

## InnateScipt Expression Format for Pipeline Steps

Per D-04, Assignments.md is markdown with embedded InnateScipt. The pipeline data lives in JSONB metadata, but the human-readable Assignments.md body references it:

```markdown
# Engineering Department Assignments

## Pipeline: Engineering Build

The engineering pipeline progresses work from specification through deployment.

@Pipeline(Engineering Build)

### Stages
1. **Spec** -- assigned to @Person(Isaac): Write detailed specifications
2. **Infra Review** -- assigned to @Person(Casey): Review infrastructure
3. **Design** -- assigned to @Person(Casey): Architecture design
4. **Build** -- assigned to @Person(Devin): Implementation
5. **Security Review** -- assigned to @Person(Sanjay): Security audit
6. **Test** -- assigned to @Person(Danielle): Testing
7. **Deploy** -- assigned to @Person(Morgan): Deployment

### Fork Points
- After **Discovery**: fork to @Person(Ibrahim Hassan) for pattern-analysis AND @Person(Felix Wu) for architecture-research
```

The InnateScipt `@Person()` references are decorative in the body -- they make the document readable and linkable. The actual structured data is in the metadata JSONB column.

## Common Pitfalls

### Pitfall 1: Stage Name Collision Between Pipelines
**What goes wrong:** "research" appears in both editorial and investment pipelines. Current code has this bug (masked by detect-pipeline-type special case).
**Why it happens:** Global alist lookup finds first match regardless of pipeline.
**How to avoid:** The (pipeline, stage) compound key in the DB format eliminates this. Pipeline lookup functions must accept pipeline-name as parameter, not just stage name.
**Warning signs:** Tests that work with engineering pipeline but fail with editorial.

### Pitfall 2: Missing Pipeline for Orphan Tasks
**What goes wrong:** A task has a `goal_id` but the goal task's `project_id` is NULL or links to a project with no pipeline definition.
**Why it happens:** Legacy tasks or manually created tasks may not have proper project linkage.
**How to avoid:** `get-pipeline-advancement` returns nil gracefully (same as current behavior when stage not found in alist). Log a warning.
**Warning signs:** `advance-pipeline` silently does nothing for tasks that should advance.

### Pitfall 3: Lisp JSON Key Conventions
**What goes wrong:** JSONB metadata keys are snake_case in PostgreSQL, but the AF64 JSON parser converts underscores to hyphens. `"pipeline_name"` becomes `:PIPELINE-NAME`.
**Why it happens:** Established AF64 convention (documented in CLAUDE.md).
**How to avoid:** Access parsed JSONB with hyphenated keyword keys: `(gethash :PIPELINE-NAME metadata)`, `(gethash :FROM-STAGE fork)`.
**Warning signs:** nil returns from gethash on keys you know exist in the JSON.

### Pitfall 4: Fork Target Loading
**What goes wrong:** Forks currently hardcoded as a cond block. If the DB format doesn't capture fork targets with assignees, forks silently stop working.
**Why it happens:** The fork logic is deeply nested in advance-pipeline, easy to forget.
**How to avoid:** Forks must be first-class in the JSONB schema. The `forks` array format explicitly captures `from_stage`, `to_stages` with name + assignee.
**Warning signs:** Tasks in fork branches stay "blocked" forever.

### Pitfall 5: Energy Reward Scope
**What goes wrong:** Current code iterates ALL entries in `*pipeline-advancement*` when ANY pipeline completes, rewarding agents from unrelated pipelines.
**Why it happens:** Global alist contains all pipelines mixed together.
**How to avoid:** With per-pipeline definitions, only iterate the completing pipeline's stages for reward distribution.
**Warning signs:** Agents getting +30 energy for pipelines they never participated in (existing bug, actually).

## Code Examples

### Loading pipeline definitions from area_content

```lisp
;; Source: noosphere-resolver.lisp resolve-from-area-content pattern
(defun load-pipeline-definitions ()
  "Load all active pipeline definitions from area_content.
   Returns list of parsed metadata hash-tables."
  (handler-case
      (let* ((sql "SELECT ac.title, ac.metadata
                   FROM area_content ac
                   WHERE ac.content_type = 'pipeline'
                   AND ac.status = 'active'")
             (rows (db-query sql)))
        (loop for i from 0 below (length rows)
              for row = (aref rows i)
              for meta = (gethash :METADATA row)
              when (hash-table-p meta)
              collect meta))
    (error (e)
      (format t "[pipeline] Error loading definitions: ~a~%" e)
      nil)))
```

### Building advancement alist from loaded definitions

```lisp
;; Build the same alist shape as current *pipeline-advancement*
;; but scoped per pipeline-name for collision-free lookup
(defun build-advancement-alist (pipeline-meta)
  "Convert a single pipeline metadata hash-table into advancement alist entries.
   Returns list of ((pipeline-name . stage) . (next-stage . next-assignee))."
  (let* ((stages (gethash :STAGES pipeline-meta))
         (pipeline-name (gethash :PIPELINE-NAME pipeline-meta))
         (stage-list (if (vectorp stages) (coerce stages 'list) stages))
         (result '()))
    ;; Build sequential advancement from ordered stages
    (loop for (current next) on stage-list
          while next
          when (and (hash-table-p current) (hash-table-p next))
          do (push (cons (gethash :NAME current)
                         (cons (gethash :NAME next)
                               (gethash :ASSIGNEE next)))
                   result))
    ;; Terminal stage -> done
    (when stage-list
      (let ((last (car (last stage-list))))
        (when (hash-table-p last)
          (let ((terminal (or (gethash :TERMINAL-ASSIGNEE pipeline-meta)
                              (gethash :ASSIGNEE last))))
            (push (cons (gethash :NAME last) (cons "done" terminal)) result)))))
    (nreverse result)))
```

### Replacing detect-pipeline-type with DB lookup

```lisp
;; *pipeline-stage-type-map* built at load time: stage -> pipeline-name
;; For compound-key lookup: (pipeline-name, stage) -> next
(defun get-pipeline-type-for-task (task)
  "Determine pipeline type for a task via goal_id -> project -> pipeline lookup."
  (let* ((goal-id (gethash :GOAL-ID task))
         (goal (when goal-id (db-get-task-by-id goal-id)))
         (project-id (when (hash-table-p goal) (gethash :PROJECT-ID goal))))
    (when project-id
      (gethash project-id *project-pipeline-map*))))
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Hardcoded *pipeline-advancement* | DB-loaded pipeline definitions | Phase 30 (this phase) | Pipelines configurable without code changes |
| Global stage-name alist | (pipeline, stage) compound key | Phase 30 | Eliminates "research" collision bug |
| Hardcoded fork cond block | Declarative fork definitions in JSONB | Phase 30 | Forks configurable per pipeline |
| detect-pipeline-type by stage name | goal_id -> project -> pipeline lookup | Phase 30 | Pipeline type determined by context, not naming convention |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual SBCL REPL verification (no automated test framework in ghost codebase) |
| Config file | None -- SBCL load-and-run |
| Quick run command | `pm2 restart noosphere-ghosts && sleep 5 && pm2 logs noosphere-ghosts --lines 50` |
| Full suite command | Trigger a pipeline task, observe advance-pipeline log output |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PIPE-01 | area_content rows exist with pipeline definitions | smoke | `psql -c "SELECT count(*) FROM area_content WHERE content_type = 'pipeline'"` | N/A (SQL) |
| PIPE-02 | Pipeline stages define sequence + assignees | smoke | `psql -c "SELECT metadata->'stages' FROM area_content WHERE content_type = 'pipeline' LIMIT 1"` | N/A (SQL) |
| PIPE-03 | Tick engine loads and uses DB pipelines | manual | Restart ghosts, observe `[pipeline]` log messages showing DB-loaded data | N/A |
| PIPE-04 | Task tracks current step via existing columns | manual | Query tasks with active pipeline stages, verify stage/goal_id populated | N/A |

### Sampling Rate
- **Per task commit:** `pm2 logs noosphere-ghosts --lines 20` (check for load errors)
- **Per wave merge:** Restart ghosts, verify pipeline loading log messages
- **Phase gate:** At least one pipeline task advances through DB-loaded definitions

### Wave 0 Gaps
- None -- no automated test framework exists; verification is via REPL and log inspection (established pattern for all prior phases)

## Open Questions

1. **Project-to-pipeline mapping for existing tasks**
   - What we know: Tasks have goal_id -> project_id chain. Projects have owner -> department.
   - What's unclear: Should the mapping be stored on the area_content metadata (explicit project_id list) or derived from project owner's department?
   - Recommendation: Derive from department. Store `department` on the pipeline metadata. Map project owner -> agent department -> pipeline. Explicit project_id list on area_content for project-specific overrides (Modular Fortress).

2. **Initial department Assignments.md content**
   - What we know: 8 executive departments exist, 4 have defined pipelines in the current code.
   - What's unclear: Which departments should get initial entries? Only the 4 with existing pipelines, or all 8?
   - Recommendation: Start with the 4 existing pipelines (engineering, investment, editorial, modular-fortress). Other departments can be added when they define pipelines.

3. **Project-specific pipelines (D-03) storage**
   - What we know: Projects can have InnateScipt pipeline definitions on the project itself.
   - What's unclear: Where exactly? Projects table has no `metadata` JSONB column currently.
   - Recommendation: For this phase, use area_content with a project-specific content_type or title referencing the project. Defer adding metadata column to projects table.

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` lines 175-466 -- complete pipeline advancement code
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` lines 386-510 -- prev-stage-map and build-pipeline-task-job
- `/opt/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` lines 335-392 -- area_content query pattern
- `/opt/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp` -- YAML loading/writing pattern (model for DB loading)
- `/opt/project-noosphere-ghosts/lisp/util/yaml.lisp` -- YAML parser capabilities and limits
- PostgreSQL `\d area_content` -- verified schema with columns: id, area_id, content_type, title, body, metadata(jsonb), status
- PostgreSQL `tasks` table -- verified columns: stage, goal_id, pipeline_order, stage_notes(jsonb), project_id
- PostgreSQL `areas` table -- verified: area_id=1 is EM Corp (slug=em-corp), 5 areas total
- PostgreSQL `agents.department` -- verified: 19 distinct department values

### Secondary (MEDIUM confidence)
- InnateScipt syntax from `/opt/innatescript/README.md` -- @, (), {} container patterns

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - all code paths traced, DB schemas verified, patterns established in prior phases
- Architecture: HIGH - straightforward data source swap following D-14 (signatures intact)
- Pitfalls: HIGH - stage name collision identified from actual code, Lisp JSON quirk from project CLAUDE.md

**Research date:** 2026-03-30
**Valid until:** 2026-04-30 (stable -- Common Lisp ecosystem, internal codebase)
