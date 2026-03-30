# Phase 25: Ghost Expression Generation - Context

**Gathered:** 2026-03-30
**Status:** Ready for planning
**Source:** Auto-mode (--auto)

<domain>
## Phase Boundary

Ghosts compose valid Innate .dpn expressions as part of their cognition output and persist them as new or modified Templates in the `templates` table. This closes the read-write loop: Phase 24 gave ghosts the ability to READ and EVALUATE templates; Phase 25 gives them the ability to WRITE them. A ghost can create a Template with Innate expressions (e.g., `@projects{status=blocked}`, `(nova){health_check}`), and that Template is immediately evaluable by other ghosts in subsequent ticks.

</domain>

<decisions>
## Implementation Decisions

### Expression Construction (INNATE-03)
- **D-01:** Ghosts construct Innate expressions as plain text strings using Lisp builder/helper functions. No AST-to-string serializer needed — Innate syntax is simple enough that string concatenation with validated parts works. Examples: `(build-reference "project_name")` → `"@project_name"`, `(build-commission "nova" "health_check")` → `"(nova){health_check}"`.
- **D-02:** LLM cognition can also emit raw Innate expression strings directly (the syntax is human-readable). Builder functions are for programmatic generation; the LLM path produces strings naturally.

### Validation Before Persistence
- **D-03:** All generated expressions MUST pass parse-round-trip validation before being written to the templates table. Validation: `(parse (tokenize expr))` — if it returns a valid AST without signaling an error, the expression is valid. This leverages the existing 175-test-passing parser as the authoritative syntax checker.
- **D-04:** If validation fails, the ghost receives an error context (via the existing handler-case pattern from Phase 24 D-10) and can report the failure without crashing. Invalid expressions are never persisted.

### Template CRUD from Lisp
- **D-05:** Ghosts write new templates via direct SQL `INSERT INTO templates (name, slug, category, description, body, parameters) VALUES (...)` using `db-execute` from Phase 21/22. This follows the established direct-SQL convention — no HTTP calls to dpn-api.
- **D-06:** Ghosts modify existing template bodies via `UPDATE templates SET body = ... WHERE id = ...` using `db-execute`. The DB trigger handles version history automatically (from Phase 16 D-08).
- **D-07:** Slug generation: derive from name using simple kebab-case conversion in Lisp. Claude's discretion on exact implementation.

### LLM Integration (Cognition Pipeline)
- **D-08:** The action-planner's system prompt instructs the LLM to include generated Innate expressions in a structured JSON field (e.g., `"innate_expressions": [{"name": "...", "body": "..."}]`) when the ghost's task involves template creation/modification.
- **D-09:** The action-executor parses the `innate_expressions` field from LLM output, validates each expression via parse-round-trip (D-03), and persists valid ones to the templates table.
- **D-10:** Not all cognition jobs produce expressions — only jobs where the ghost is explicitly tasked with creating or modifying templates. The expression generation path is additive, following the Phase 24 convention (D-03: template evaluation is additive, not required).

### Agent IDs
- **D-11:** Agent IDs in generated expressions use DB `id` column (e.g., `sarah`, `kathryn`), not compound names. Consistent with Phase 24 D-03 (Agent IDs use DB id column).

### Claude's Discretion
- Exact builder function signatures and whether they live in a new file or extend existing ones
- Exact JSON schema for the `innate_expressions` field in LLM output
- Whether to add a "generate-template" tool to the tool registry or handle via action-executor directly
- Exact system prompt additions for template generation tasks
- Whether templates created by ghosts get a specific category (e.g., "ghost-generated")

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Innate Parser (validation layer)
- `/opt/innatescript/src/parser/tokenizer.lisp` — `tokenize` function: string → token list
- `/opt/innatescript/src/parser/parser.lisp` — `parse` function: tokens → AST nodes
- `/opt/innatescript/src/types.lisp` — node, innate-result, resistance struct definitions

### Innate Evaluator (consumer of generated templates)
- `/opt/innatescript/src/eval/evaluator.lisp` — `evaluate` function that processes AST + eval-env

### Phase 23 Resolver (already writes to templates table)
- `/opt/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` — `load-bundle` reads templates; `resolve-from-templates` queries templates table

### Phase 24 Template Evaluation (the read side of the loop)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — `evaluate-template-for-project` function; `build-project-review-job` integration point

### Ghost Cognition Pipeline (where generation integrates)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — Creates cognition jobs; template generation instructions go in system prompt
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Processes LLM output; template persistence goes here
- `/opt/project-noosphere-ghosts/lisp/runtime/cognition-types.lisp` — cognition-job struct

### Direct SQL Infrastructure (for template CRUD)
- `/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp` — db-query, db-execute, db-escape
- `/opt/project-noosphere-ghosts/lisp/runtime/db-auxiliary.lisp` — existing entity query patterns

### Templates Table (persistence target)
- dpn-api templates handler: `/opt/dpn-api/src/handlers/templates.rs` — Shows table schema: name, slug, category, description, body, parameters

### Requirements
- `.planning/REQUIREMENTS.md` — INNATE-03 (Ghost expression generation)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `tokenize` + `parse` from innatescript — parse-round-trip validation (D-03)
- `db-execute` from db-client.lisp — direct SQL for INSERT/UPDATE templates
- `db-escape` from db-client.lisp — SQL injection prevention via PQescapeLiteral
- `resolve-from-templates` in noosphere-resolver — shows exact query pattern for templates table
- `evaluate-template-for-project` in action-planner — shows how templates are loaded and evaluated (the read path that generation must satisfy)
- `make-node` from types.lisp — AST node construction (if programmatic AST building is needed)

### Established Patterns
- LLM output is JSON (claude -p with --output-format json) — action-executor already parses JSON responses
- Tool calls in LLM output are already extracted and executed by action-executor
- handler-case wrapping all DB/eval operations with error recovery (Phase 24 pattern)
- Template body is plain text with Innate expressions; parsed on read, not on write

### Integration Points
- action-executor.lisp — add expression extraction + validation + persistence after LLM response processing
- action-planner.lisp — add system prompt instructions for template generation tasks
- packages.lisp — may need new package or extend existing for builder functions
- templates table — INSERT/UPDATE targets; version history trigger fires automatically

</code_context>

<specifics>
## Specific Ideas

- The generation loop is: LLM produces expression string → Lisp validates via parse-round-trip → Lisp persists to templates table → next tick, any ghost can evaluate the new template via Phase 24 infrastructure
- Builder functions serve two purposes: (1) programmatic generation for deterministic patterns, (2) examples in LLM prompts showing valid syntax
- The existing tool-call pattern in action-executor (extract JSON → validate → execute) directly maps to expression extraction (extract JSON → validate via parser → persist via SQL)

</specifics>

<deferred>
## Deferred Ideas

- Template versioning UI (view/diff template changes in dpn-kb) — future frontend milestone
- Template sharing between ghosts (access control, ownership) — future milestone
- Innate language v2 features (new node types, expression evolution) — out of scope per REQUIREMENTS.md
- Template creation wizard for Nathan in dpn-tui — FRONT-03 scope

</deferred>

---

*Phase: 25-ghost-expression-generation*
*Context gathered: 2026-03-30 via auto-mode*
