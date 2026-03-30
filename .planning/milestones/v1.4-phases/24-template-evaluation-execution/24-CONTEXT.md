# Phase 24: Template Evaluation & Execution - Context

**Gathered:** 2026-03-29
**Status:** Ready for planning
**Source:** Auto-mode (--auto)

<domain>
## Phase Boundary

Wire the Innate evaluator + noosphere-resolver into the ghost cognition pipeline so that:
1. Templates from the `templates` table are loaded and their Innate expressions evaluated before LLM prompts are built
2. Evaluated template content (with resolved @references, search results, etc.) enriches cognition job input-context
3. `(agent){action}` commission patterns in templates trigger real tool invocations via the existing conversation→perception→execution pipeline
4. Evaluation errors are caught and reported without crashing the tick

This connects the Innate language (Phase 23's resolver) to the ghost cognition pipeline (action-planner → cognition-broker → LLM → action-executor).

</domain>

<decisions>
## Implementation Decisions

### Template Loading (INNATE-02)
- **D-01:** Templates are loaded in action-planner.lisp when building cognition jobs for standing-order/operations kinds. The planner checks if the project has an associated template (via project metadata or a templates table lookup by project name/label).
- **D-02:** Template body is fetched using `load-bundle` from the noosphere-resolver (Phase 23), which queries the templates table and returns parsed AST nodes.
- **D-03:** If no template exists for a project/task, the planner proceeds with its current behavior (no template enrichment). Template evaluation is additive, not required.

### Innate Evaluation Integration (INNATE-02)
- **D-04:** The Innate evaluator is called on template AST nodes with an eval-env containing `*noosphere-resolver*` as the :resolver. This happens in action-planner.lisp BEFORE the cognition job's input-context is built.
- **D-05:** Evaluated output (a string of resolved content) is injected into the cognition job's `:system-prompt` or appended as a `:template-context` key in input-context. The LLM sees concrete data (e.g., "Blocked projects: Project Alpha, Project Beta") not raw Innate expressions.
- **D-06:** The evaluator uses `:scope :query` for template evaluation — this is a read-only context where `@references` resolve but commissions are deferred until the action-executor processes the LLM's output.

### Commission-to-Tool Mapping (INNATE-04)
- **D-07:** `(agent){action}` commissions in Daily Note templates are NOT executed during template evaluation. Instead, they are collected during evaluation (the resolver's deliver-commission inserts a conversation message) and the target ghost perceives and executes them on its next tick cycle.
- **D-08:** For the specific success criteria — `(sarah_lin){sync_calendar}` and `(kathryn){finance_positions}` — these commissions insert conversations with channel="commission" that the target agent perceives. The action-planner for that agent then creates a cognition job with the commission content, and the action-executor invokes the matching tool.
- **D-09:** Tool label mapping already exists in action-planner.lisp (label-to-tool mapping from Phase 13/14). Commissions reference these same labels. Claude's discretion on exact wiring.

### Error Handling
- **D-10:** Template evaluation is wrapped in `handler-case`. If the Innate evaluator signals an error (missing entity, parse failure, etc.), the error is caught and a fallback string is used: `"[Template evaluation error: {error-message}]"`. The cognition job proceeds with this error context rather than crashing.
- **D-11:** Resistance values from the resolver (missing entities, ambiguous matches) produce inline error markers in the evaluated output: `"[unresolved: @missing_entity]"`. The LLM sees these and can report them.

### Claude's Discretion
- Exact location in action-planner.lisp where template loading/evaluation is inserted
- Whether template content goes into :system-prompt or a separate :template-context key
- How to associate projects with templates (by name match, metadata field, or explicit FK)
- Whether to cache evaluated template results within a single tick
- Exact eval-env construction (scope, decrees, bindings)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Ghost Cognition Pipeline (where evaluation integrates)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — Creates cognition jobs with input-context; template evaluation goes HERE
- `/opt/project-noosphere-ghosts/lisp/runtime/cognition-types.lisp` — cognition-job struct definition with input-context hash-table
- `/opt/project-noosphere-ghosts/lisp/runtime/cognition-broker.lisp` — Manages job queue, dispatches to LLM provider
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Processes LLM results, executes tools, handles conversation posting

### Innate Interpreter (evaluator to call)
- `/opt/innatescript/src/eval/evaluator.lisp` — The evaluate function that takes AST + eval-env and produces results
- `/opt/innatescript/src/eval/resolver.lisp` — eval-env struct definition with :resolver, :scope, :decrees, :bindings slots

### Phase 23 Resolver (the bridge)
- `/opt/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` — All 6 methods + *noosphere-resolver* global + init function

### Existing Tool Infrastructure
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — Contains label-to-tool mapping from Phase 13/14 (standing orders)

### Requirements
- `.planning/REQUIREMENTS.md` — INNATE-02 (Template evaluation), INNATE-04 (Commission execution)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `*noosphere-resolver*` global from Phase 23 — already initialized at tick-engine startup
- `load-bundle` method on noosphere-resolver — fetches template body from templates table, parses into AST
- `make-eval-env` from innatescript — creates evaluation environment with resolver
- `deliver-commission` method — already inserts conversation messages for commissions
- `load-hard-prompt` in action-planner.lisp — existing pattern for loading prompt content from DB

### Established Patterns
- Cognition job input-context is a `json-object` hash-table with :system-prompt and :messages keys
- action-planner creates jobs with `make-cognition-job` — template evaluation slots in before this call
- `handler-case` wrapping all DB/eval operations with error recovery

### Integration Points
- action-planner.lisp `plan-*` functions — each creates a cognition job; template evaluation enriches input-context
- Standing order projects — already have labels that map to tools; these are natural template carriers
- The `load-hard-prompt` function already loads prompt templates from memories/documents — this is the existing pattern to extend

</code_context>

<specifics>
## Specific Ideas

- Template evaluation makes ghost cognition context-aware: instead of "check blocked projects", the LLM prompt says "3 blocked projects: Alpha (auth issue), Beta (waiting on Vincent), Gamma (resource constraint)"
- Commissions are fire-and-forget from the evaluator's perspective — they create conversations that target ghosts perceive on their next tick, maintaining the existing async architecture
- The success criteria's specific examples (sarah_lin/sync_calendar, kathryn/finance_positions) test the end-to-end path: template → evaluate → commission → conversation → perception → cognition → tool execution

</specifics>

<deferred>
## Deferred Ideas

- Ghost expression generation (composing valid Innate expressions) — Phase 25
- Template creation/modification by ghosts — Phase 25
- Real-time template evaluation during tick (vs. pre-computed at job creation) — future optimization
- Template versioning and change tracking — future milestone

</deferred>

---

*Phase: 24-template-evaluation-execution*
*Context gathered: 2026-03-29 via auto-mode*
