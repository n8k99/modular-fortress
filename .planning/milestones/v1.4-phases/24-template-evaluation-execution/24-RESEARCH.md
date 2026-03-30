# Phase 24: Template Evaluation & Execution - Research

**Researched:** 2026-03-29
**Domain:** Innate evaluator integration into AF64 ghost cognition pipeline (Common Lisp)
**Confidence:** HIGH

## Summary

Phase 24 wires the Innate evaluator into the ghost cognition pipeline so that Templates from `master_chronicle.templates` are loaded, their Innate expressions evaluated (resolving @references to concrete data), and the resulting enriched content injected into cognition job prompts. Additionally, `(agent){action}` commission patterns in Daily Note templates trigger real tool invocations by creating conversation messages that target agents perceive on their next tick.

The primary integration point is `action-planner.lisp`, specifically the `build-project-review-job` function (for standing orders / project reviews) and the `default-job-builder` function chain. The Innate evaluator (`innate.eval:evaluate`) needs to be loaded at runtime (currently missing from `launch.sh`) and called with an `eval-env` containing the `*noosphere-resolver*` before cognition job input-context is built.

**Primary recommendation:** Add evaluator.lisp to launch.sh load sequence, create an `evaluate-template` helper function in action-planner.lisp that wraps the Innate evaluator with handler-case error recovery, and inject evaluated content into cognition job :system-prompt or :template-context.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Templates loaded in action-planner.lisp when building cognition jobs for standing-order/operations kinds. Planner checks if project has associated template (via project metadata or templates table lookup by project name/label).
- **D-02:** Template body fetched using `load-bundle` from noosphere-resolver (Phase 23), which queries templates table and returns parsed AST nodes.
- **D-03:** If no template exists for a project/task, planner proceeds with current behavior (no template enrichment). Template evaluation is additive, not required.
- **D-04:** Innate evaluator called on template AST nodes with eval-env containing `*noosphere-resolver*` as :resolver. Happens in action-planner.lisp BEFORE cognition job input-context is built.
- **D-05:** Evaluated output injected into cognition job's `:system-prompt` or appended as `:template-context` key in input-context. LLM sees concrete data not raw Innate expressions.
- **D-06:** Evaluator uses `:scope :query` for template evaluation -- read-only context where @references resolve but commissions are deferred.
- **D-07:** `(agent){action}` commissions in Daily Note templates are NOT executed during template evaluation. Instead, collected during evaluation (resolver's deliver-commission inserts conversation message) and target ghost perceives/executes on next tick.
- **D-08:** For `(sarah_lin){sync_calendar}` and `(kathryn){finance_positions}` -- commissions insert conversations with channel="commission" that target agent perceives. Action-planner creates cognition job with commission content, action-executor invokes matching tool.
- **D-09:** Tool label mapping already exists in action-planner.lisp (label-to-tool mapping from Phase 13/14). Commissions reference same labels.
- **D-10:** Template evaluation wrapped in `handler-case`. Errors caught, fallback string used: `"[Template evaluation error: {error-message}]"`. Cognition job proceeds with error context.
- **D-11:** Resistance values produce inline error markers: `"[unresolved: @missing_entity]"`. LLM sees these and can report them.

### Claude's Discretion
- Exact location in action-planner.lisp where template loading/evaluation is inserted
- Whether template content goes into :system-prompt or separate :template-context key
- How to associate projects with templates (by name match, metadata field, or explicit FK)
- Whether to cache evaluated template results within a single tick
- Exact eval-env construction (scope, decrees, bindings)

### Deferred Ideas (OUT OF SCOPE)
- Ghost expression generation (composing valid Innate expressions) -- Phase 25
- Template creation/modification by ghosts -- Phase 25
- Real-time template evaluation during tick (vs. pre-computed at job creation)
- Template versioning and change tracking
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| INNATE-02 | Ghosts evaluate .dpn Template bodies during cognition, receiving resolved content as actionable context that informs their planning and execution | Template loading via load-bundle, evaluation via innate.eval:evaluate with noosphere-resolver, injection into cognition job input-context |
| INNATE-04 | Daily Note template Innate expressions -- (agent){action} patterns like (sarah_lin){sync_calendar} and (kathryn){finance_positions} -- execute during ghost operations, triggering real tool invocations | deliver-commission already inserts conversations with channel="commission"; target ghost perceives on next tick, action-planner builds job, action-executor invokes tool via tool-mapping-for-label |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

- **Stack**: Common Lisp (SBCL) for all ghost runtime code. No new languages.
- **DB is the OS**: All state in master_chronicle. Templates live in the `templates` table.
- **AF64 zero-deps convention**: No Quicklisp. All dependencies are vendored or written from scratch.
- **Ghost LLM**: Claude Code CLI with $0.50/request budget. Template evaluation adds overhead to prompts.
- **Lisp naming**: kebab-case functions, `*earmuffs*` for specials, `handler-case` for error recovery.
- **JSON quirk**: Parser converts underscores to hyphens (`:is-error` not `:is_error`).

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| innate.eval:evaluate | v1.0 (local) | Two-pass AST evaluator (decrees then nodes) | The Innate interpreter -- this phase's target |
| innate.eval.resolver:make-eval-env | v1.0 (local) | Creates evaluation environment struct | Required to configure resolver + scope for evaluate |
| noosphere-resolver | Phase 23 (local) | Connects Innate symbols to master_chronicle | Already deployed, provides *noosphere-resolver* global |
| innate.parser.tokenizer:tokenize | v1.0 (local) | Tokenizes .dpn text to token stream | Already loaded in launch.sh |
| innate.parser:parse | v1.0 (local) | Parses token stream to AST | Already loaded in launch.sh |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| load-bundle (noosphere-resolver) | Phase 23 | Fetches template body from DB, parses to AST | Called to load template content before evaluation |
| deliver-commission (noosphere-resolver) | Phase 23 | Inserts conversation for (agent){action} | Called automatically by evaluator during commission adjacency detection |
| tool-mapping-for-label (action-planner) | Phase 13/14 | Maps standing order labels to tool names | Used by project-review jobs when standing orders fire |

## Architecture Patterns

### Critical Gap: Evaluator Not Loaded at Runtime

The `launch.sh` load sequence currently loads these innatescript files:
```
packages.lisp, types.lisp, conditions.lisp, tokenizer.lisp, parser.lisp, resolver.lisp
```

**Missing:** `evaluator.lisp` is NOT loaded. The `innate.eval:evaluate` function is unavailable at runtime. This must be fixed as Wave 0 / prerequisite work.

The fix: add `/opt/innatescript/src/eval/evaluator` to the `--eval` block in `launch.sh` that loads innatescript files, between `resolver` and the closing paren.

### Integration Point: action-planner.lisp

The cognition job build chain:
```
build-cognition-job
  -> dispatch-ghost-behavior (self-mod hook)
  -> default-job-builder
    -> load-persona
    -> build-message-job       (messages priority)
    -> build-request-job       (requests priority)
    -> build-task-job           (tasks priority)
    -> build-project-review-job (project review / standing orders)
```

Template evaluation slots into `build-project-review-job` (primary target for standing orders that fire templates) and optionally `build-task-job` for task-associated templates.

### Recommended Architecture

```
action-planner.lisp
  |
  +-- evaluate-template-for-project (new function)
  |     Takes: project-name or project-id
  |     1. Look up template by project name match in templates table
  |     2. Use load-bundle to get AST nodes
  |     3. Wrap AST in :program node (evaluate requires this)
  |     4. Create eval-env with *noosphere-resolver*, :scope :query
  |     5. Call innate.eval:evaluate with handler-case
  |     6. Flatten results to string
  |     7. Return evaluated string or error marker
  |
  +-- build-project-review-job (modified)
        Insert evaluated template content into system-prompt
        before or after project summaries
```

### Pattern: Template-to-Project Association

There is no FK from `projects` to `templates`. Three options (Claude's discretion):

1. **Name match** (recommended): Query `templates WHERE LOWER(name) = LOWER(project_name)` -- simplest, uses existing `resolve-from-templates` pattern
2. **Metadata field**: Store `template_id` in `projects.current_context` or a metadata JSON field
3. **Category match**: Use `templates.category` to match project type

Recommendation: **Name match** for v1, with fallback to `templates.category` matching the project's area. This requires zero schema changes and follows the noosphere-resolver's existing case-insensitive name matching pattern.

### Pattern: Evaluated Output Injection

Two approaches for injecting template context into cognition jobs:

**Option A: Append to :system-prompt** (recommended)
- Simpler, guaranteed to be seen by LLM
- Follows existing pattern (persona + reality-anchor + schedule-context all concatenated into system-prompt)

**Option B: Separate :template-context key**
- Cleaner separation of concerns
- Requires cognition-broker / LLM provider to know about the new key

Recommendation: **Option A** -- append to system-prompt with clear section header (`## Template Context`). Follows the existing pattern used by `schedule-context`, `team-roster`, and `decisions-context` in `build-project-review-job`.

### Anti-Patterns to Avoid
- **Evaluating commissions during template eval**: Per D-07, commissions are fire-and-forget via deliver-commission. The evaluator already handles this via commission adjacency detection. Do NOT try to intercept or defer commissions -- let the existing mechanism work.
- **Blocking on commission results**: Commissions are async. The commissioning ghost does not wait for results. The target ghost perceives on its next tick.
- **Wrapping evaluate in ignore-errors**: Use `handler-case` to catch specific conditions (`innate-resistance`, `innate-parse-error`, `error`) and produce meaningful fallback strings, not silent nil.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Template body parsing | Custom parser | `load-bundle` on noosphere-resolver | Already parses body to AST nodes via tokenize+parse |
| Commission delivery | Custom conversation insert | `deliver-commission` on noosphere-resolver | Already handles agent lookup, conversation creation, resistance on unknown agents |
| Tool invocation from commissions | Custom tool executor | Existing perception -> action-planner -> action-executor pipeline | Commissions create conversations; target ghost perceives and acts naturally |
| AST evaluation | Custom tree walker | `innate.eval:evaluate` | Full two-pass evaluator with decree collection, commission adjacency, fulfillment fallback |
| Result serialization to string | Custom formatter | `(format nil "~a" result)` or `(princ-to-string result)` | Innate results are strings, numbers, plists -- standard CL printing handles them |

## Common Pitfalls

### Pitfall 1: evaluate Expects a :program Node, Not a List of Children
**What goes wrong:** `load-bundle` returns a list of child AST nodes (not a :program node). Calling `(evaluate children env)` directly will fail because `evaluate` expects `(node-kind ast)` to be `:program`.
**Why it happens:** `load-bundle` strips the program wrapper and returns `(node-children program)`.
**How to avoid:** Wrap the returned children in a program node: `(make-node :kind :program :children (load-bundle ...))`
**Warning signs:** Error mentioning "BUG: :program node should not reach eval-node"

### Pitfall 2: innate-resistance Is a Condition, Not an Error
**What goes wrong:** Using `(handler-case (evaluate ...) (error (e) ...))` catches parse errors but not resistance. Unhandled resistance propagates silently (it's signaled, not errored).
**Why it happens:** `innate-resistance` inherits from `condition`, not `error`. It uses `signal`, not `error`.
**How to avoid:** Catch both: `(handler-case (evaluate ...) (innate-resistance (r) ...) (error (e) ...))`
**Warning signs:** Missing @references silently vanish from output

### Pitfall 3: Commission Adjacency Only Works at Top Level
**What goes wrong:** `(agent){action}` inside nested brackets or bundles may not trigger commissions.
**Why it happens:** Commission adjacency detection only operates in the `evaluate` main loop, checking consecutive `:agent` + `:bundle` nodes in top-level children. Inside nested bundles, `eval-node` handles `:agent` nodes by returning the name string and `:bundle` nodes by loading and evaluating the bundle content -- no commission adjacency check.
**How to avoid:** Ensure Daily Note templates place commission patterns at the top level: `(sarah_lin){sync_calendar}` as a standalone line, not nested inside brackets or other constructs.
**Warning signs:** Commissions in templates don't create conversations

### Pitfall 4: Package Imports for evaluate
**What goes wrong:** `action-planner.lisp` is in `:af64.runtime.action-planner` package which does NOT import from innate packages. Calling `innate.eval:evaluate` without proper import fails.
**Why it happens:** Phase 23 only set up imports for `noosphere-resolver` package, not for action-planner.
**How to avoid:** Either (a) add `:import-from :innate.eval #:evaluate` to action-planner package, or (b) use fully qualified `innate.eval:evaluate` in the code (less clean), or (c) add a wrapper function on noosphere-resolver that calls evaluate.
**Warning signs:** Package/symbol errors at load time

### Pitfall 5: Result Flattening -- evaluate Returns a List
**What goes wrong:** `evaluate` returns a list of results (one per top-level child node). Injecting a list into a format string produces `(value1 value2 ...)` instead of readable text.
**Why it happens:** `evaluate` uses progn-like semantics, collecting results from each child.
**How to avoid:** Flatten results to string: `(format nil "~{~a~^ ~}" (remove nil results))` or join with newlines for multi-part templates.
**Warning signs:** LLM prompt contains literal Lisp list syntax

### Pitfall 6: Plist Results from @references
**What goes wrong:** Resolving `@projects{status=blocked}` returns a list of plists. Format-ing a plist produces `(:ID 5 :NAME "Alpha" ...)` which is not LLM-friendly.
**Why it happens:** `resolve-from-projects` returns raw plists from hash-to-plist.
**How to avoid:** Build a result formatter that converts plists to human-readable strings, e.g., `"Project Alpha (ID: 5, status: blocked)"`.
**Warning signs:** LLM receives raw plist/hash-table notation

## Code Examples

### Loading evaluator at runtime (launch.sh fix)
```bash
# Current line 9 loads packages through resolver:
--eval '(dolist (f (list ... "/opt/innatescript/src/eval/resolver")) (load ...))'

# Must become (add evaluator after resolver):
--eval '(dolist (f (list ... "/opt/innatescript/src/eval/resolver" "/opt/innatescript/src/eval/evaluator")) (load ...))'
```

### Template evaluation helper function
```lisp
;; In action-planner.lisp
(defun evaluate-template-for-project (project-name)
  "Load and evaluate a template associated with PROJECT-NAME.
   Returns evaluated string or nil if no template / error."
  (handler-case
      (let* ((resolver af64.runtime.noosphere-resolver:*noosphere-resolver*))
        (when resolver
          (let ((children (innate.eval.resolver:load-bundle resolver project-name)))
            (when children
              (let* ((program (innate.types:make-node
                               :kind :program
                               :children children))
                     (env (innate.eval.resolver:make-eval-env
                           :resolver resolver
                           :scope :query))
                     (results (innate.eval:evaluate program env)))
                ;; Flatten results to human-readable string
                (format nil "~{~a~^~%~}" (remove nil results)))))))
    (innate.conditions:innate-resistance (r)
      (format nil "[unresolved: ~a]"
              (innate.conditions:resistance-condition-message r)))
    (error (e)
      (format nil "[Template evaluation error: ~a]" e))))
```

### Injecting template context into project-review job
```lisp
;; Inside build-project-review-job, after project-summaries and before schedule-context:
(template-context
  (let ((first-project-name (gethash :name (elt projects 0))))
    (when first-project-name
      (let ((evaluated (evaluate-template-for-project first-project-name)))
        (when (and evaluated (> (length evaluated) 0))
          (format nil "~%~%## Template Context~%~a" evaluated))))))
```

### Result flattening for plist values
```lisp
(defun format-innate-value (val)
  "Convert an Innate evaluation result to LLM-friendly string."
  (cond
    ((null val) "")
    ((stringp val) val)
    ((numberp val) (princ-to-string val))
    ((and (listp val) (keywordp (first val)))
     ;; Plist: format key-value pairs
     (with-output-to-string (s)
       (loop for (k v) on val by #'cddr
             do (format s "~a: ~a  " (string-downcase (symbol-name k)) v))))
    ((listp val)
     ;; List of items (search results)
     (format nil "~{~a~^~%~}" (mapcar #'format-innate-value val)))
    (t (princ-to-string val))))
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Static prompt templates | Innate-evaluated dynamic templates | Phase 24 (this phase) | Ghosts see resolved data, not raw expressions |
| Manual tool invocation scheduling | Commission-driven tool execution via (agent){action} | Phase 24 (this phase) | Daily Note templates trigger real tools automatically |
| HTTP-based perception | Direct PostgreSQL perception | Phase 21-22 | Foundation that makes template evaluation fast (no HTTP overhead) |
| Resolver only (Phase 23) | Resolver + evaluator | Phase 24 | Full Innate pipeline operational in ghost runtime |

## Open Questions

1. **Template association method**
   - What we know: No FK from projects to templates. Name matching is simplest.
   - What's unclear: Whether projects with spaces/special chars in names will match template names reliably.
   - Recommendation: Use case-insensitive name match (already the pattern in resolve-from-templates). For the initial implementation, create specific templates named after standing order projects (e.g., "EM Operations", "EM Financial").

2. **Commission scope during evaluation**
   - What we know: D-06 says use `:scope :query` (read-only). D-07 says commissions are NOT executed during evaluation but ARE collected (deliver-commission fires).
   - What's unclear: There is a tension -- `:scope :query` suggests read-only, but the evaluator's commission adjacency detection calls `deliver-commission` regardless of scope. The resolver's `deliver-commission` inserts a conversation message (a write operation).
   - Recommendation: Accept the write -- this is the intended behavior per D-07/D-08. The commission IS delivered during evaluation, but the actual tool execution happens when the target ghost perceives on its next tick. The scope constraint means @references are read-only, not that all operations are read-only.

3. **Multiple templates per project**
   - What we know: A project could match multiple templates (e.g., by category).
   - What's unclear: Should we evaluate all matching templates or just the first?
   - Recommendation: First match only for v1. KISS.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual SBCL REPL testing + PM2 integration test |
| Config file | None -- SBCL loads and runs |
| Quick run command | `pm2 start noosphere-ghosts && sleep 30 && pm2 logs noosphere-ghosts --lines 50` |
| Full suite command | Same + DB verification queries |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| INNATE-02a | Cognition job includes evaluated template content | integration | Check PM2 logs for template evaluation output | N/A -- manual |
| INNATE-02b | Executive sees resolved data not raw expressions | integration | DB query: check cognition job input contains resolved data | N/A -- manual |
| INNATE-04a | (sarah_lin){sync_calendar} creates commission conversation | integration | `psql -c "SELECT * FROM conversations WHERE channel='commission' AND to_agents @> '{sarah}' ORDER BY id DESC LIMIT 1"` | N/A -- manual |
| INNATE-04b | (kathryn){finance_positions} triggers tool execution | integration | Check PM2 logs for tool execution by kathryn after commission | N/A -- manual |
| ERR-01 | Evaluation errors don't crash tick | integration | Intentionally create template with bad @reference, verify tick continues | N/A -- manual |

### Sampling Rate
- **Per task commit:** SBCL load test -- ensure files compile without errors
- **Per wave merge:** PM2 restart + single tick verification
- **Phase gate:** Full tick cycle with template-enriched standing order + commission delivery verified in DB

### Wave 0 Gaps
- [ ] `launch.sh` -- add evaluator.lisp to load sequence
- [ ] Test template in DB -- insert a real template with Innate expressions for an existing standing order project
- [ ] Package imports -- action-planner needs access to innate.eval:evaluate and innate.eval.resolver:make-eval-env

## Sources

### Primary (HIGH confidence)
- `/opt/innatescript/src/eval/evaluator.lisp` -- evaluate function, commission adjacency detection, two-pass architecture
- `/opt/innatescript/src/eval/resolver.lisp` -- eval-env struct, resolver protocol, make-eval-env
- `/opt/innatescript/src/types.lisp` -- node struct, innate-result, resistance
- `/opt/innatescript/src/conditions.lisp` -- innate-resistance condition (signal not error)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` -- all job builders, tool-mapping-for-label, build-project-review-job
- `/opt/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` -- all 6 resolver methods, load-bundle, deliver-commission, *noosphere-resolver* global
- `/opt/project-noosphere-ghosts/lisp/runtime/cognition-types.lisp` -- cognition-job struct, make-cognition-job with input-context hash-table
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` -- package definitions, action-planner imports, noosphere-resolver conditional package
- `/opt/project-noosphere-ghosts/launch.sh` -- SBCL load sequence (evaluator.lisp MISSING)
- `master_chronicle.templates` schema -- id, name, slug, category, body, parameters, metadata columns

### Secondary (MEDIUM confidence)
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` -- schedule-fired-labels, noosphere-resolver init at startup
- `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` -- execute-tool-call, get-tools-for-agent, process-tool-calls

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all code is local, fully inspected
- Architecture: HIGH -- integration points verified by reading all source files
- Pitfalls: HIGH -- derived from actual code analysis (evaluate expects :program node, innate-resistance is condition not error, commission adjacency is top-level only)

**Research date:** 2026-03-29
**Valid until:** 2026-04-28 (stable -- local codebase, no external dependencies)
