# Phase 30: Team Pipelines - Context

**Gathered:** 2026-03-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Department and team pipelines are defined in the noosphere (master_chronicle) with explicit handoff chains, replacing hardcoded `*pipeline-advancement*` logic. Pipeline definitions live as area_content entries (Assignments.md per department) and project-level InnateScipt expressions. The tick engine reads pipeline definitions from the DB, not from filesystem YAML or defparameters.

</domain>

<decisions>
## Implementation Decisions

### Pipeline storage — DB-native, not filesystem
- **D-01:** Pipeline definitions live in master_chronicle, not as YAML/config files on disk. The noosphere is the substrate — all ghost instructions, assignments, and pipelines pull from it.
- **D-02:** Department-level pipelines are stored as area_content entries under the EM area. Each department/team gets an `Assignments.md` entry — a markdown document with embedded InnateScipt expressions defining the operational pipeline (step sequence, ghost assignments, handoff chains).
- **D-03:** Project-specific pipelines (e.g., @Project(Modular Fortress)) carry their own InnateScipt pipeline definitions on the project itself. The owning executive delegates to the executing executive — JMax owns Digital Sovereignty, Eliana executes via Modular Fortress.

### Assignments.md content format
- **D-04:** Assignments.md is a markdown document with embedded InnateScipt expressions — a living playbook for the department. Not structured YAML-in-markdown, not pure data. Ghosts and the org graph both read from it.
- **D-05:** Ghost YAML `responsibilities:` section echoes what the department's Assignments.md says they do. DB is authoritative, YAML is the runtime cache that the tick engine reads at load time.

### Pipeline step definition
- **D-06:** Pipeline steps are an ordered sequence with stage name and assigned ghost per step. The YAML parser and `advance-pipeline` function read from DB-loaded pipeline data instead of the hardcoded `*pipeline-advancement*` alist.
- **D-07:** Stage names are pipeline-scoped, not globally unique. A stage called "research" in editorial is distinct from "research" in investment. The `(pipeline, stage)` tuple is the compound key.
- **D-08:** Stage names feed the em-org-graph visualization — color-coded progression of tasks through pipeline stages.

### Fork/join support
- **D-09:** Forks supported (one stage unblocks multiple downstream stages) — this preserves existing modular-fortress behavior where `discovery` unblocks both `pattern-analysis` and `architecture-research`.
- **D-10:** Joins deferred — no waiting-for-multiple-predecessors logic in this phase. Linear progression plus forks only.

### Task metadata
- **D-11:** Existing `stage`, `goal_id`, `stage_notes`, `pipeline_order` columns on tasks table are sufficient. No schema migration needed for pipeline state tracking.
- **D-12:** Pipeline type identification moves from `detect-pipeline-type` function to a DB lookup — the task's `goal_id` links to a project which has its pipeline definition.

### Integration approach
- **D-13:** Replace `*pipeline-advancement*` defparameter in action-executor.lisp with a function that loads pipeline definitions from the DB (via area_content for department pipelines, via project metadata for project-specific pipelines).
- **D-14:** Keep `advance-pipeline`, `build-pipeline-task-job`, and `load-predecessor-stage-output` function signatures intact — this is a data source swap, not a refactor.
- **D-15:** `detect-pipeline-type` and `prev-stage-map` defparameters also replaced by DB lookups.

### Python tooling
- **D-16:** Python tools for pipeline management (CRUD on Assignments.md entries, pipeline visualization data for org graph) live colocated with the operational noosphere, accessible via InnateScipt expressions.

### Project hierarchy
- **D-17:** Project relationships (Digital Sovereignty → Modular Fortress) expressed through noosphere ownership and department routing, not a new parent_id schema column. The relationship is implicit in project ownership + department head delegation.

### Claude's Discretion
- Exact InnateScipt expression syntax for pipeline step definitions within Assignments.md
- Which departments get initial Assignments.md entries (recommend: all 8 departments with active pipelines)
- Loading/caching strategy for pipeline definitions (per-tick reload vs cached with invalidation)
- Exact format of pipeline data passed to em-org-graph for color-coded visualization

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Pipeline advancement (being replaced)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` lines 175-209 — `*pipeline-advancement*` hardcoded alist
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` lines 295-315 — `detect-pipeline-type` function
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` lines 342-466 — `advance-pipeline` with fork handling
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` lines 386-432 — `prev-stage-map` and `build-pipeline-task-job`

### YAML capabilities (Phase 28 pattern)
- `/opt/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp` — YAML loading pattern to reuse for DB-loaded pipelines
- `/opt/project-noosphere-ghosts/lisp/util/yaml.lisp` — YAML parser (nested sections, lists)

### Area content (Phase 27)
- master_chronicle `area_content` table — where Assignments.md entries will live
- `/opt/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` — resolves area-scoped content via InnateScipt

### DB layer
- `/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp` — direct PostgreSQL queries via libpq FFI

### Requirements
- `.planning/REQUIREMENTS.md` — PIPE-01 through PIPE-04

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `ghost-capabilities.lisp` — Pattern for loading structured data and injecting into action planner prompts. Pipeline loading can follow same pattern but read from DB instead of YAML files.
- `noosphere-resolver.lisp` — Already resolves area-scoped content. Can be extended to resolve pipeline definitions from Assignments.md entries.
- `db-client.lisp` — Direct SQL queries via libpq. Pipeline loading adds new query functions here.
- `area_content` table — Phase 27 created this for EM structured content. Department Assignments.md entries are a natural fit.

### Established Patterns
- Action planner composes system prompts from: persona + capabilities + responsibilities + hard prompts. Pipeline context is another layer.
- Action executor parses cognition output for structured blocks (tool_call, innate_expression, responsibility mutations). Pipeline advancement is another output type.
- `advance-pipeline` already handles forks via `fork-targets` — the logic stays, the data source changes.

### Integration Points
- `action-executor.lisp` — `*pipeline-advancement*`, `*prev-stage-map*`, `detect-pipeline-type` all replaced by DB-loaded equivalents
- `action-planner.lisp` — `build-pipeline-task-job` reads predecessor output using DB pipeline definitions
- `area_content` table — new entries for department Assignments.md documents
- em-org-graph — pipeline stage names and progression feed the visualization

</code_context>

<specifics>
## Specific Ideas

- Nathan thinks in Obsidian vault metaphors — "virtual directories" for departments with Assignments.md entries. The area_content table IS the vault, departments are folders, Assignments.md is a note.
- The noosphere is the substrate for everything — ghost identity, instructions, backstories, behaviors, pipelines. External tools are just "hands" that InnateScipt operates through.
- The system builds itself recursively — ghosts read their assignments from the noosphere, execute work, and the results flow back into the noosphere.
- em-org-graph visualization shows pipeline progression with color-coded stages — this is how Nathan sees the system working at a glance.
- Project hierarchy example: JMax owns @Project(Digital Sovereignty), Eliana executes @Project(Modular Fortress) as a subproject. Pipeline definitions on the project, delegation through department heads.

</specifics>

<deferred>
## Deferred Ideas

- Join logic (waiting for multiple predecessors) — future phase when a pipeline needs convergence
- Parent-child project relationships as schema (parent_id column) — express through noosphere ownership for now
- Full tool-registry.json retirement — Phase 31
- Python tool InnateScipt wrappers — Phase 31
- em-org-graph visual redesign for pipeline color coding — may need its own phase if complex

</deferred>

---

*Phase: 30-team-pipelines*
*Context gathered: 2026-03-30*
