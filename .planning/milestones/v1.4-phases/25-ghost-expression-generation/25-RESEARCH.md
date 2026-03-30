# Phase 25: Ghost Expression Generation - Research

**Researched:** 2026-03-30
**Domain:** Innate expression generation from ghost cognition pipeline (Common Lisp / SBCL)
**Confidence:** HIGH

## Summary

Phase 25 closes the read-write loop for Innate expressions. Phase 24 gave ghosts the ability to READ and EVALUATE templates; this phase gives them the ability to WRITE them. The implementation touches three areas: (1) Lisp builder/helper functions for constructing valid Innate expression strings, (2) parse-round-trip validation using the existing 175-test-passing parser, and (3) integration into the cognition pipeline so LLM output containing template definitions gets validated and persisted to the `templates` table.

The existing codebase provides strong foundations. The `tokenize` and `parse` functions from InnateScipt are already imported into the `af64.runtime.noosphere-resolver` package. The `db-execute` and `db-escape` functions handle SQL persistence safely. The action-executor already has a well-established pattern for extracting structured commands from LLM output (tool calls, CLASSIFY, DELEGATE, COMPLETE, CREATE_TASK). Expression generation follows the same extract-validate-execute pattern.

**Primary recommendation:** Implement expression generation as an additive extension to the existing action-executor pipeline, using the same handler-case error isolation pattern established in Phase 24. Builder functions go in a new file; expression extraction and persistence hooks into execute-project-review and execute-work-task.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Ghosts construct Innate expressions as plain text strings using Lisp builder/helper functions. No AST-to-string serializer needed.
- **D-02:** LLM cognition can also emit raw Innate expression strings directly. Builder functions for programmatic generation; LLM path produces strings naturally.
- **D-03:** All generated expressions MUST pass parse-round-trip validation before being written to templates table. Validation: `(parse (tokenize expr))` -- if it returns a valid AST without signaling an error, the expression is valid.
- **D-04:** If validation fails, the ghost receives error context via existing handler-case pattern. Invalid expressions are never persisted.
- **D-05:** Ghosts write new templates via direct SQL INSERT INTO templates using db-execute. No HTTP calls.
- **D-06:** Ghosts modify existing template bodies via UPDATE templates SET body = ... WHERE id = ... using db-execute. DB trigger handles version history automatically.
- **D-07:** Slug generation: derive from name using simple kebab-case conversion in Lisp.
- **D-08:** Action-planner system prompt instructs LLM to include generated Innate expressions in structured JSON field `"innate_expressions": [{"name": "...", "body": "..."}]`.
- **D-09:** Action-executor parses innate_expressions field from LLM output, validates each via parse-round-trip, persists valid ones.
- **D-10:** Expression generation is additive -- only jobs where ghost is explicitly tasked with creating/modifying templates.
- **D-11:** Agent IDs in generated expressions use DB id column (e.g., sarah, kathryn).

### Claude's Discretion
- Exact builder function signatures and whether they live in a new file or extend existing ones
- Exact JSON schema for the `innate_expressions` field in LLM output
- Whether to add a "generate-template" tool to the tool registry or handle via action-executor directly
- Exact system prompt additions for template generation tasks
- Whether templates created by ghosts get a specific category (e.g., "ghost-generated")

### Deferred Ideas (OUT OF SCOPE)
- Template versioning UI (view/diff template changes in dpn-kb)
- Template sharing between ghosts (access control, ownership)
- Innate language v2 features (new node types, expression evolution)
- Template creation wizard for Nathan in dpn-tui
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| INNATE-03 | Ghosts compose valid Innate .dpn expressions to create or modify Templates via the interpreter's generation capabilities | Builder functions for expression construction (D-01/D-02), parse-round-trip validation via existing parser (D-03), template CRUD via db-execute (D-05/D-06), LLM integration via action-planner/executor (D-08/D-09) |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| InnateScipt parser | v1.0 (175/176 tests) | Parse-round-trip validation of generated expressions | Authoritative syntax checker per D-03 |
| AF64 db-client | Current | db-execute, db-escape for template CRUD | Established direct-SQL convention from Phase 21/22 |
| AF64 action-executor | Current | LLM output processing, expression extraction | Established pattern for structured command extraction |
| AF64 action-planner | Current | System prompt injection for template generation context | Established job builder pattern |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| AF64 cognition-types | Current | cognition-result struct for content extraction | Every cognition result processing path |
| AF64 noosphere-resolver | Current | Already imports tokenize + parse from innate | Reuse existing package imports |

**No new external dependencies needed.** This phase is entirely within the existing Lisp ecosystem.

## Architecture Patterns

### Recommended File Structure
```
/opt/project-noosphere-ghosts/lisp/runtime/
  innate-builder.lisp          # NEW: builder functions + validation + template CRUD
  action-executor.lisp         # MODIFIED: add expression extraction to execute-* functions
  action-planner.lisp          # MODIFIED: add template generation instructions to system prompts
  packages.lisp                # MODIFIED: new package or extend noosphere-resolver exports
  noosphere-resolver.lisp      # EXISTING: already imports tokenize + parse (reuse)
```

### Pattern 1: Expression Builder Functions
**What:** Pure functions that construct valid Innate expression strings from parts.
**When to use:** Programmatic generation (not LLM path). Also useful as documentation/examples in system prompts.
**Recommendation:** Create a new file `innate-builder.lisp` with a new package `af64.runtime.innate-builder` or extend `af64.runtime.noosphere-resolver`.

Rationale for new file: Builder functions (write-path) are conceptually separate from resolver (read-path). Separation keeps noosphere-resolver focused on resolution. However, the builder needs `tokenize` and `parse` for validation, which are already imported in noosphere-resolver. Two options:

**Option A (recommended): New package `af64.runtime.innate-builder`**
- Clean separation of concerns
- Imports tokenize/parse directly from innate packages
- Exports builder + validation + template CRUD functions
- Action-executor imports from this package

**Option B: Extend noosphere-resolver**
- Fewer packages, no new defpackage
- But conflates read and write responsibilities

### Pattern 2: Parse-Round-Trip Validation
**What:** Validate expression string by tokenizing + parsing it. If no error is signaled, the expression is syntactically valid.
**When to use:** ALWAYS before persisting any expression to the templates table (D-03).

Key insight: The validation function must catch `innate-parse-error` (which is an error subtype, not just a condition). The existing `handler-case` pattern from Phase 24 works perfectly:

```lisp
(defun validate-innate-expression (expr)
  "Validate EXPR by parse-round-trip. Returns T if valid, (values nil error-message) if invalid."
  (handler-case
      (progn
        (parse (tokenize expr))
        t)
    (innate-parse-error (e)
      (values nil (format nil "Parse error: ~a" e)))
    (error (e)
      (values nil (format nil "Unexpected error: ~a" e)))))
```

### Pattern 3: LLM Output Expression Extraction
**What:** Parse structured JSON from LLM cognition output to extract template definitions.
**When to use:** In execute-project-review and execute-work-task, after existing processing.

The established pattern in action-executor is text-based parsing (e.g., `parse-classify-lines` scans for "CLASSIFY:" prefixes, `parse-handoff` scans for "HANDOFF:"). However, D-08 specifies a JSON field approach. Two integration strategies:

**Strategy A (recommended): JSON field extraction**
Since LLM output is processed as the `content` string from `cognition-result-content`, and the LLM is instructed to output JSON blocks, extract the `innate_expressions` array from a JSON code block in the content. This follows the same approach as `process-tool-calls` which extracts ```tool_call blocks.

**Strategy B: Text-based scanning**
Scan for "CREATE_TEMPLATE:" or "UPDATE_TEMPLATE:" prefixes like CLASSIFY/DELEGATE. Simpler but less structured.

JSON field extraction is more robust for multi-field template definitions (name + body + optional category).

### Pattern 4: Template CRUD via Direct SQL
**What:** INSERT and UPDATE templates table using db-execute + db-escape.
**When to use:** After successful parse-round-trip validation.

Key schema facts from the actual `templates` table:
- `name` VARCHAR(256) NOT NULL
- `slug` VARCHAR(256) NOT NULL, UNIQUE
- `category` VARCHAR(128), nullable
- `description` TEXT, nullable
- `body` TEXT NOT NULL
- `parameters` JSONB DEFAULT '[]'
- `metadata` JSONB DEFAULT '{}'
- `version` INTEGER DEFAULT 1
- `created_at` / `updated_at` auto-managed
- UNIQUE constraint on `slug` -- slug collisions will error
- Version history trigger fires automatically on UPDATE (Phase 16 D-08)

### Anti-Patterns to Avoid
- **Hand-rolling an AST-to-string serializer:** D-01 explicitly says this is unnecessary. Innate syntax is simple enough for string concatenation.
- **Bypassing parse-round-trip validation:** D-03 is non-negotiable. Never persist unvalidated expressions.
- **Using HTTP/dpn-api for template writes:** D-05 specifies direct SQL. All ghost DB operations use db-execute since Phase 21.
- **Making expression generation mandatory in all cognition paths:** D-10 says it is additive, following Phase 24 convention.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Innate syntax validation | Custom regex or string checks | `(parse (tokenize expr))` | Parser has 175 tests, handles all edge cases |
| SQL injection prevention | Manual string quoting | `db-escape` (wraps PQescapeLiteral) | Proven FFI-level escaping from Phase 21 |
| Template version history | Manual version tracking | Existing DB trigger `trg_template_version_history` | Fires automatically on UPDATE |
| UUID generation | External library | `generate-uuid` from cognition-types | Already used throughout codebase |
| JSON parsing from LLM output | Custom parser | `parse-json` from af64.utils.json | Battle-tested JSON parser |

**Key insight:** The validation infrastructure (parser), persistence infrastructure (db-client), and LLM output processing infrastructure (action-executor) all exist. This phase is primarily integration work, not new infrastructure.

## Common Pitfalls

### Pitfall 1: Slug Collision on Template INSERT
**What goes wrong:** Two ghosts create templates with the same name (or similar names that produce identical slugs). The UNIQUE constraint on `slug` causes an INSERT error.
**Why it happens:** kebab-case slug derivation is deterministic -- same name always produces same slug.
**How to avoid:** Wrap INSERT in handler-case. On unique violation, either append a numeric suffix or report the collision as an error to the ghost.
**Warning signs:** PostgreSQL error containing "duplicate key value violates unique constraint."

### Pitfall 2: LLM Generating Invalid Innate Syntax
**What goes wrong:** The LLM produces expressions with syntax errors (unclosed brackets, invalid characters, malformed references).
**Why it happens:** LLMs approximate syntax from examples. Even with good prompts, syntax errors are common.
**How to avoid:** Parse-round-trip validation (D-03) catches this. The error path should give useful feedback so the ghost knows what went wrong. Do NOT retry automatically -- let the ghost handle it on next tick.
**Warning signs:** `innate-parse-error` conditions from the parser.

### Pitfall 3: Expression Body Containing SQL-Unsafe Characters
**What goes wrong:** Innate expressions contain single quotes, backslashes, or other SQL metacharacters that break the INSERT statement.
**Why it happens:** Innate syntax uses curly braces, brackets, and @ symbols which are safe, but expression content (prose nodes, string literals) can contain anything.
**How to avoid:** Always use `db-escape` for ALL string values in SQL. Never concatenate raw expression strings into SQL.
**Warning signs:** PostgreSQL syntax errors on INSERT/UPDATE.

### Pitfall 4: JSON Extraction from Mixed LLM Output
**What goes wrong:** The LLM output contains both natural language text and the `innate_expressions` JSON block. Naive extraction fails because the content is not pure JSON.
**Why it happens:** LLM output is a mix of prose and structured blocks (same as tool_call blocks).
**How to avoid:** Use the same extraction pattern as `process-tool-calls` -- scan for a delimited code block (e.g., ```innate_expressions or a JSON object containing the key). Parse only the extracted block, not the entire content.
**Warning signs:** JSON parse errors when trying to parse the full content string.

### Pitfall 5: Forgetting to Import innate-parse-error in New Package
**What goes wrong:** handler-case for `innate-parse-error` doesn't catch the condition because the symbol isn't imported.
**Why it happens:** New package definition misses the import from :innate.conditions.
**How to avoid:** Ensure the builder package imports `innate-parse-error` from `:innate.conditions`, `tokenize` from `:innate.parser.tokenizer`, and `parse` from `:innate.parser`.
**Warning signs:** Unhandled condition errors during validation.

### Pitfall 6: Noosphere-Resolver Package Conditional Loading
**What goes wrong:** The noosphere-resolver package is wrapped in `handler-case` because it depends on InnateScipt being loaded first. If the new builder package has similar dependencies and isn't wrapped, it fails when innate isn't loaded.
**Why it happens:** packages.lisp loads innate via a separate --eval block. If innate loading fails, any package that imports innate symbols fails too.
**How to avoid:** Wrap the new package definition in the same handler-case pattern used for noosphere-resolver (line 208-227 of packages.lisp).
**Warning signs:** Package definition error at startup.

## Code Examples

### Expression Builder Functions (Recommended Implementation)
```lisp
;; Source: Derived from CONTEXT.md D-01 + existing codebase patterns

(defun build-reference (name &optional qualifiers)
  "Build an @reference expression. E.g., (build-reference \"project_name\") => \"@project_name\"
   With qualifiers: (build-reference \"projects\" '((\"status\" . \"active\"))) => \"@projects{status=active}\""
  (let ((base (format nil "@~a" name)))
    (if qualifiers
        (format nil "~a{~{~a=~a~^, ~}}" base
                (loop for (k . v) in qualifiers
                      collect k collect v))
        base)))

(defun build-commission (agent-id action)
  "Build an (agent){action} commission expression.
   E.g., (build-commission \"nova\" \"health_check\") => \"(nova){health_check}\""
  (format nil "(~a){~a}" agent-id action))

(defun build-search (search-type &optional qualifiers)
  "Build a ![type]{qualifiers} search expression.
   E.g., (build-search \"projects\" '((\"status\" . \"blocked\"))) => \"![projects]{status=blocked}\""
  (if qualifiers
      (format nil "![~a]{~{~a=~a~^, ~}}" search-type
              (loop for (k . v) in qualifiers
                    collect k collect v))
      (format nil "![~a]" search-type)))

(defun build-bundle (name)
  "Build a {bundle_name} bundle reference.
   E.g., (build-bundle \"daily-standup\") => \"{daily-standup}\""
  (format nil "{~a}" name))
```

### Parse-Round-Trip Validation
```lisp
;; Source: CONTEXT.md D-03 + existing handler-case pattern from Phase 24

(defun validate-innate-expression (expr)
  "Validate EXPR by tokenize+parse round-trip.
   Returns T if valid, (VALUES NIL error-string) if invalid."
  (handler-case
      (let ((ast (parse (tokenize expr))))
        (if (and ast (node-kind ast))
            t
            (values nil "Parser returned empty result")))
    (innate-parse-error (e)
      (values nil (format nil "~a" e)))
    (error (e)
      (values nil (format nil "Unexpected: ~a" e)))))
```

### Template INSERT via Direct SQL
```lisp
;; Source: CONTEXT.md D-05 + templates table schema + db-client patterns

(defun db-insert-template (name body &key category description parameters)
  "Insert a new template into the templates table. Returns T on success.
   Slug is derived from name via kebab-case conversion."
  (let* ((slug (name-to-slug name))
         (sql (format nil
                "INSERT INTO templates (name, slug, category, description, body, parameters) ~
                 VALUES (~a, ~a, ~a, ~a, ~a, ~a)"
                (db-escape name)
                (db-escape slug)
                (if category (db-escape category) "NULL")
                (if description (db-escape description) "NULL")
                (db-escape body)
                (if parameters (db-escape (encode-json parameters)) "'[]'::jsonb"))))
    (db-execute sql)))

(defun db-update-template-body (template-id new-body)
  "Update an existing template's body. Version history trigger fires automatically."
  (let ((sql (format nil
               "UPDATE templates SET body = ~a WHERE id = ~a"
               (db-escape new-body)
               template-id)))
    (db-execute sql)))
```

### Slug Generation
```lisp
;; Source: CONTEXT.md D-07

(defun name-to-slug (name)
  "Convert template name to kebab-case slug.
   'Operation Normality' => 'operation-normality'
   'My Template 2.0' => 'my-template-2-0'"
  (string-downcase
   (with-output-to-string (s)
     (loop for ch across name
           do (cond
                ((alphanumericp ch) (write-char ch s))
                ((or (char= ch #\Space) (char= ch #\_)) (write-char #\- s))
                ((char= ch #\.) (write-char #\- s))
                ;; Skip other characters
                )))))
```

### Expression Extraction from LLM Output
```lisp
;; Source: Derived from process-tool-calls pattern in tool-socket.lisp

(defun extract-innate-expressions (content)
  "Extract innate_expressions JSON from LLM output content.
   Looks for a JSON object or array containing template definitions.
   Returns list of hash-tables with :name and :body keys, or NIL."
  (handler-case
      (let ((start (search "\"innate_expressions\"" content)))
        (when start
          ;; Find the enclosing JSON object or the array value
          ;; Strategy: find [ after the key, extract until matching ]
          (let* ((arr-start (position #\[ content :start start))
                 (arr-end (when arr-start (find-matching-bracket content arr-start))))
            (when (and arr-start arr-end)
              (let* ((json-str (subseq content arr-start (1+ arr-end)))
                     (parsed (parse-json json-str)))
                (when (or (listp parsed) (vectorp parsed))
                  (coerce parsed 'list)))))))
    (error () nil)))
```

### Integration into execute-project-review / execute-work-task
```lisp
;; Source: Existing action-executor patterns (execute-work-task lines 394-469)

;; Add after existing tool-call processing in execute-project-review:
(let ((expressions (extract-innate-expressions content)))
  (when expressions
    (format t "  [innate-gen] ~a: ~a expression(s) found~%" agent-id (length expressions))
    (dolist (expr-def expressions)
      (let* ((name (gethash :name expr-def))
             (body (gethash :body expr-def))
             (category (or (gethash :category expr-def) "ghost-generated")))
        (when (and name body)
          (multiple-value-bind (valid err)
              (validate-innate-expression body)
            (if valid
                (handler-case
                    (progn
                      (db-insert-template name body :category category)
                      (format t "  [innate-gen] ~a: created template '~a'~%" agent-id name))
                  (error (e)
                    (format t "  [innate-gen-error] ~a: INSERT failed for '~a': ~a~%" agent-id name e)))
                (format t "  [innate-gen-invalid] ~a: expression for '~a' failed validation: ~a~%"
                        agent-id name err))))))))
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Templates read-only by ghosts | Phase 24: ghosts evaluate templates during cognition | Phase 24 (current) | Ghosts can READ Innate expressions |
| Manual template creation (SQL) | Phase 25: ghosts CREATE/UPDATE templates via cognition | This phase | Ghosts can WRITE Innate expressions |
| HTTP-based DB access | Direct SQL via SB-ALIEN FFI to libpq | Phase 21 | All ghost DB operations use db-execute |

## Open Questions

1. **JSON extraction robustness**
   - What we know: LLM output is mixed prose + structured blocks. tool_call extraction uses regex/string scanning.
   - What's unclear: Best approach for extracting `innate_expressions` JSON from mixed content. The LLM may not always wrap it in a code block.
   - Recommendation: Support both inline JSON and code-block-delimited JSON. Test with actual LLM outputs during validation.

2. **Template UPDATE identification**
   - What we know: D-06 specifies UPDATE by id. The ghost needs to know the template id to update.
   - What's unclear: How the ghost discovers which template id to update. The LLM would need the template id from perception or task context.
   - Recommendation: Support update-by-name (look up id from name) as well as update-by-id. Add a `"template_updates"` JSON field alongside `"innate_expressions"` for modifications.

3. **Category for ghost-generated templates**
   - What we know: Existing templates use categories like "standing-order" and empty string.
   - What's unclear: Whether to auto-tag ghost-created templates with "ghost-generated" for traceability.
   - Recommendation: Default to "ghost-generated" category. Allows easy querying of ghost-authored templates.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual Lisp REPL testing + SQL verification |
| Config file | None (AF64 zero-deps convention, no test framework) |
| Quick run command | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -c "SELECT id, name, slug, category FROM templates ORDER BY id DESC LIMIT 5"` |
| Full suite command | InnateScipt parser tests: `cd /opt/innatescript && sbcl --load run-tests.lisp` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| INNATE-03a | Ghost generates syntactically valid Innate expression | smoke | REPL: `(validate-innate-expression "@project_name{status=active}")` | No -- Wave 0 |
| INNATE-03b | Generated expressions pass parser without errors | unit | REPL: `(parse (tokenize "(nova){health_check}"))` | Yes (innate parser tests) |
| INNATE-03c | Ghost creates new Template row with Innate body | integration | SQL: `SELECT * FROM templates WHERE category='ghost-generated' ORDER BY id DESC LIMIT 1` | No -- Wave 0 |
| INNATE-03d | Ghost modifies existing Template body | integration | SQL: `SELECT version, body FROM templates WHERE id=X` + check templates_history | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** Verify builder functions produce valid syntax via REPL test
- **Per wave merge:** Run innate parser test suite + verify template CRUD via SQL queries
- **Phase gate:** Full round-trip test: builder -> validate -> INSERT -> load-bundle -> evaluate

### Wave 0 Gaps
- [ ] No formal test file for builder functions (REPL testing only, following AF64 convention)
- [ ] Template CRUD verification via SQL queries (manual, not automated)

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` -- Full package dependency graph, import structure
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` -- execute-cognition-result dispatch, process-tool-calls pattern
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` -- System prompt construction, build-project-review-job
- `/opt/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` -- load-bundle, resolve-from-templates, tokenize/parse imports
- `/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp` -- db-execute, db-escape API
- `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` -- process-tool-calls extraction pattern
- `/opt/project-noosphere-ghosts/lisp/runtime/cognition-types.lisp` -- cognition-result struct
- `/opt/innatescript/src/conditions.lisp` -- innate-parse-error, innate-resistance condition hierarchy
- `/opt/innatescript/src/parser/tokenizer.lisp` -- tokenize function signature
- `/opt/innatescript/src/parser/parser.lisp` -- parse function signature
- `/opt/innatescript/src/types.lisp` -- node struct, innate-result, resistance structs
- PostgreSQL `\d templates` -- actual table schema with triggers and constraints

### Secondary (MEDIUM confidence)
- Existing template data (3 rows: smoke test, version test, operation normality) -- shows real-world body format

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all components exist and are verified from source code
- Architecture: HIGH -- follows established patterns (tool-call extraction, handler-case, db-execute)
- Pitfalls: HIGH -- derived from actual codebase constraints (UNIQUE slug, handler-case wrapping, conditional package loading)

**Research date:** 2026-03-30
**Valid until:** 2026-04-30 (stable -- Lisp codebase, no external dependency churn)
