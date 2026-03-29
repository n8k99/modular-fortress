# Phase 23: Noosphere Resolver - Research

**Researched:** 2026-03-29
**Domain:** Common Lisp CLOS resolver protocol, PostgreSQL query dispatch, cross-repo ASDF integration
**Confidence:** HIGH

## Summary

Phase 23 implements a `noosphere-resolver` CLOS class in the noosphere-ghosts repo that subclasses the Innate interpreter's `resolver` protocol. This resolver bridges Innate's symbolic language (`@`, `()`, `{}`, `[[]]`) to live master_chronicle PostgreSQL tables. The implementation is primarily a dispatch layer: the resolver protocol's 6 generic functions each map to SQL queries executed via the Phase 21/22 `db-client` infrastructure (`db-query`, `db-execute`, `db-escape`).

The key technical challenge is cross-repo integration: innatescript defines the resolver base class and type structs (`innate-result`, `resistance`) in its own packages, while noosphere-ghosts has the DB infrastructure. The launch system uses direct file loading (not ASDF `load-system`), so the innatescript packages must be loaded into the SBCL image before the noosphere-resolver file.

Most of the SQL queries needed are straightforward SELECTs. Several entity lookups (agents, projects) already exist as `db-get-agent-by-id` and `db-get-project-by-id` in db-auxiliary.lisp, though the resolver needs name-based lookup (not ID-based), so new query functions are needed.

**Primary recommendation:** Implement noosphere-resolver.lisp as a single file with all 6 method specializations, add a new package definition, and wire the innatescript packages into launch.sh's load sequence before the resolver file.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** The noosphere-resolver lives in the noosphere-ghosts repo (`/opt/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp`), NOT in the innatescript repo. It needs the db-client SQL infrastructure and PG connection pool from Phase 21/22. The innatescript repo stays pure (no DB dependencies).
- **D-02:** The noosphere-resolver subclasses `innate.eval.resolver:resolver` (the CLOS base class). It must specialize all 6 generic functions defined in `/opt/innatescript/src/eval/resolver.lisp`.
- **D-03:** The noosphere-resolver needs access to the innatescript package at load time. Wire via ASDF dependency: af64.asd depends-on innatescript.asd, or load innatescript packages before noosphere-resolver.
- **D-04:** `@name` resolves by searching tables in priority order: agents -> projects -> areas -> templates -> resources. First match wins.
- **D-05:** `@table.name` syntax narrows to a specific table: `@projects.nova_pipeline` searches only the projects table.
- **D-06:** Entity matching: case-insensitive, match against `name` or `title` column (projects use `name`, agents use `id` or `full_name`, areas/templates/resources use `name`). Use ILIKE for fuzzy matching or exact match.
- **D-07:** Qualifier chain: `@entity:property` returns a specific field from the matched row. Uses the stub-resolver pattern: intern qualifier as keyword, getf from the plist result.
- **D-08:** `{key=value}` in a search context maps to SQL `WHERE key = $value`. Multiple filters: `{status=active,owner=nova}` -> `WHERE status='active' AND owner='nova'`.
- **D-09:** The search directive `![type]{filters}` maps search-type to a table name.
- **D-10:** Filter values are always strings. SQL comparison is case-insensitive via ILIKE or LOWER().
- **D-11:** `(agent_name){instruction}` inserts a conversation message using `db-insert-conversation` from Phase 22. Fields: from_agent="system", to_agent=[agent_id], channel="commission", content=instruction text.
- **D-12:** Agent name resolution: match against agents table `id` column (case-insensitive). If not found, return resistance.
- **D-13:** `[[Title]]` resolves against the memories table by title. Uses `SELECT * FROM memories WHERE title ILIKE $1 LIMIT 1`.
- **D-14:** `{bundle_name}` loads from the templates table. `SELECT body FROM templates WHERE name = $1`. The body is parsed by the Innate parser into AST nodes.
- **D-15:** `[context[verb[args]]]` is Phase 24 scope. For Phase 23, implement a basic version that returns raw context/verb/args as structured result, or delegate to default resistance. Claude's discretion.
- **D-16:** All resolution failures return Innate's `resistance` struct (via `make-resistance`) with `:message` and `:source` fields.
- **D-17:** Ambiguous matches: return first match by priority order (D-04). Do NOT return resistance for ambiguous matches.

### Claude's Discretion
- Exact SQL queries for each resolution function
- Whether to cache resolved entities within a single evaluation pass
- How to handle NULL fields in resolved entity plists
- Whether resolve-context gets a basic implementation or pure resistance in Phase 23
- ASDF/package wiring details between innatescript and noosphere-ghosts

### Deferred Ideas (OUT OF SCOPE)
- Template evaluation with Innate expressions -- Phase 24
- Ghost expression generation -- Phase 25
- Caching resolved entities across ticks -- future optimization
- Full resolve-context implementation -- Phase 24
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| INNATE-01 | Noosphere resolver connects Innate's @, (), {} symbols to master_chronicle tables | All 6 resolver generic functions mapped to SQL queries against agents, projects, areas, templates, resources, memories tables via Phase 21/22 db-client infrastructure |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| SBCL | Installed | Common Lisp compiler | AF64 runtime standard |
| libpq (FFI) | Phase 21 | PostgreSQL connection | Zero-deps convention, already working |
| innatescript | 0.1.0 | Resolver protocol, types, parser | Defines the interface being implemented |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| db-client.lisp | Phase 21 | db-query, db-execute, db-escape | All SQL operations |
| db-conversations.lisp | Phase 22 | db-insert-conversation | Commission delivery (D-11) |
| db-auxiliary.lisp | Phase 22 | db-get-agent-by-id, db-get-project-by-id | Reference for existing query patterns |

No new dependencies needed. This phase uses only existing infrastructure.

## Architecture Patterns

### File Structure
```
/opt/project-noosphere-ghosts/lisp/
  packages.lisp                          # ADD: af64.runtime.noosphere-resolver package
  runtime/
    noosphere-resolver.lisp              # NEW: all 6 method specializations
/opt/innatescript/
  src/                                   # NO CHANGES (stays pure)
launch.sh                               # MODIFY: load innatescript packages + files before resolver
```

### Pattern 1: CLOS Generic Function Specialization
**What:** The resolver protocol uses CLOS defgeneric/defmethod dispatch. The noosphere-resolver subclasses `resolver` and specializes all 6 methods.
**When to use:** This is the ONLY pattern for resolver implementation.
**Example:**
```common-lisp
;; Source: /opt/innatescript/src/eval/stub-resolver.lisp (reference implementation)
(defclass noosphere-resolver (resolver)
  ((db-pool :initarg :db-pool :accessor resolver-db-pool))
  (:documentation "Resolves Innate symbols against master_chronicle PostgreSQL tables."))

(defmethod resolve-reference ((r noosphere-resolver) name qualifiers)
  ;; Priority: agents -> projects -> areas -> templates -> resources
  (let ((entity (or (resolve-from-agents r name)
                    (resolve-from-projects r name)
                    (resolve-from-areas r name)
                    (resolve-from-templates r name)
                    (resolve-from-resources r name))))
    (if entity
        (if (null qualifiers)
            (make-innate-result :value entity :context :query)
            ;; Qualifier chain: intern as keyword, getf from plist
            (let* ((qual (first qualifiers))
                   (key (intern (string-upcase qual) :keyword))
                   (val (getf entity key)))
              (if val
                  (make-innate-result :value val :context :query)
                  (make-resistance :message (format nil "~a has no property ~a" name qual)
                                   :source (format nil "~a:~a" name qual)))))
        (make-resistance :message (format nil "Entity not found: ~a" name)
                         :source name))))
```

### Pattern 2: Resistance for Errors (NOT Exceptions)
**What:** Resolution failures return `resistance` structs, not signals. The evaluator decides whether to signal `innate-resistance` condition.
**When to use:** Every resolution function must return either `innate-result` or `resistance` (except `load-bundle` which returns list or nil, and `deliver-commission` which always returns `innate-result`).
**Example:**
```common-lisp
;; Source: /opt/innatescript/src/eval/resolver.lisp
(make-resistance :message "Entity not found: nova" :source "nova")
(make-innate-result :value '(:id "nova" :full-name "Nova" :department "operations") :context :query)
```

### Pattern 3: DB Query Result as Plist
**What:** `db-query` returns a vector of hash-tables with hyphenated keyword keys (`:full-name`, `:agent-id`). These must be converted to plists for the resolver protocol (stub-resolver uses plists, evaluator uses `getf`).
**When to use:** Every DB result must be converted from hash-table to plist before returning as innate-result value.
**Example:**
```common-lisp
(defun hash-to-plist (ht)
  "Convert a hash-table with keyword keys to a plist."
  (when ht
    (let ((plist '()))
      (maphash (lambda (k v)
                 (when v  ;; Skip NULL fields
                   (push v plist)
                   (push k plist)))
               ht)
      plist)))
```

### Pattern 4: Table-Dot-Name Dispatch
**What:** `@table.name` syntax splits on `.` to narrow search to a specific table (D-05).
**When to use:** In resolve-reference, check if name contains `.` before cascading search.
**Example:**
```common-lisp
(let ((dot-pos (position #\. name)))
  (if dot-pos
      (let ((table (subseq name 0 dot-pos))
            (entity-name (subseq name (1+ dot-pos))))
        (resolve-from-table r table entity-name))
      ;; No dot: cascade through priority order
      (resolve-cascade r name)))
```

### Pattern 5: Cross-Repo Package Loading
**What:** launch.sh loads files sequentially. Innatescript packages and files must load BEFORE noosphere-resolver.lisp.
**When to use:** At boot time.
**Critical detail:** launch.sh uses direct `(load ...)` calls, NOT ASDF `load-system`. The innatescript packages.lisp defines all innate.* packages. The source files must be loaded in dependency order: packages -> types -> conditions -> resolver (from eval/).

### Anti-Patterns to Avoid
- **Signaling errors in resolver methods:** Always return `resistance` structs, never `(error ...)`. The evaluator handles signaling.
- **Returning hash-tables as innate-result values:** The stub-resolver tests and evaluator expect plists. Convert DB hash-tables to plists.
- **Byte-slicing strings for dot-name parsing:** Use `(position #\. name)` and `(subseq ...)` which work on characters, not bytes. Respects the UTF-8 rule.
- **Using db-get-agent-by-id for name resolution:** Existing helpers take ID as exact match. The resolver needs name-based ILIKE lookup. Write new query functions.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| SQL injection prevention | Manual string escaping | `db-escape` from db-client.lisp | Already handles nil, strings, integers, keywords, hash-tables |
| PostgreSQL connection | New connection code | `*db-pool*` / `db-query` / `db-execute` | Phase 21 pool with health checks, acquire timeouts |
| Conversation insertion | Raw INSERT SQL | `db-insert-conversation` from db-conversations.lisp | Handles array types, thread IDs, metadata, RETURNING |
| AST parsing for bundles | Custom parser | `innate.parser:parse` | Bundle body text must be parsed by the Innate parser, not hand-rolled |
| Result/error types | Custom structs | `make-innate-result` / `make-resistance` from innate.types | Evaluator type-checks these; custom types will break dispatch |
| Qualifier resolution | Custom property accessor | `(intern (string-upcase qual) :keyword)` + `(getf plist key)` | Matches stub-resolver pattern exactly, evaluator expects it |

**Key insight:** The noosphere-resolver is a thin dispatch layer. Every building block already exists: DB queries in db-client, conversation insertion in db-conversations, result types in innate.types, and the parser in innate.parser. The resolver's job is ONLY to map Innate protocol calls to existing DB operations.

## Common Pitfalls

### Pitfall 1: JSON Underscore-to-Hyphen Conversion
**What goes wrong:** AF64's JSON parser converts underscores to hyphens in keys. DB query results from `db-query` return hash-tables with hyphenated keyword keys like `:full-name`, `:agent-id`, `:created-at`.
**Why it happens:** The `af64.utils.json:json-keyword` function hyphenates all keys.
**How to avoid:** When converting hash-table rows to plists, the keys are ALREADY hyphenated keywords. Qualifier lookup must use the same convention: `@nova:full-name` (not `@nova:full_name`). The intern-as-keyword pattern from stub-resolver handles this correctly.
**Warning signs:** If `(getf plist :full-name)` returns nil but the hash-table has the value -- check key format.

### Pitfall 2: Projects Use `name` Not `title`
**What goes wrong:** CONTEXT.md D-06 says "projects use `title`" but the actual schema shows `projects.name` (varchar 256) -- there is no `title` column on the projects table.
**Why it happens:** The discussion may have used "title" generically.
**How to avoid:** Use the actual column names from the verified schemas:
- `agents`: match on `id` (primary key) or `full_name` column
- `projects`: match on `name` column
- `areas`: match on `name` column
- `templates`: match on `name` column
- `resources`: match on `name` column
- `memories`: match on `title` column (this one DOES have title)
**Warning signs:** Empty results when querying projects by title.

### Pitfall 3: db-query Returns Vector, Not List
**What goes wrong:** `db-query` returns `#()` (vector of hash-tables), not a list. Calling `car`/`first` on a vector signals an error.
**Why it happens:** The pg.lisp FFI builds result vectors.
**How to avoid:** Use `(aref results 0)` for first element, `(length results)` for count, and check `(> (length results) 0)` before accessing.
**Warning signs:** Type error on `first`/`car` calls.

### Pitfall 4: Load Order Between Repos
**What goes wrong:** Noosphere-resolver.lisp references `innate.eval.resolver:resolver`, `innate.types:make-innate-result`, etc. If innatescript packages aren't loaded first, SBCL signals "package not found" at compile time.
**Why it happens:** launch.sh loads files sequentially. The innatescript files are in a separate directory (`/opt/innatescript/src/`).
**How to avoid:** Add innatescript file loads to launch.sh BEFORE noosphere-resolver.lisp. Load order must be: innatescript/src/packages.lisp -> types.lisp -> conditions.lisp -> eval/resolver.lisp (minimum needed). The parser is also needed for `load-bundle` (D-14).
**Warning signs:** "Package INNATE.EVAL.RESOLVER does not exist" at SBCL load time.

### Pitfall 5: Agent ID Case Sensitivity
**What goes wrong:** Agent IDs in the database are lowercase strings like "nova", "eliana", "sarah_lin". The resolver receives names from Innate expressions which may have different casing.
**Why it happens:** Innate expressions might be `@Nova` or `(Sarah_Lin)`.
**How to avoid:** Use `LOWER()` or `ILIKE` for all agent lookups: `WHERE LOWER(id) = LOWER($1)`.
**Warning signs:** `@Nova` returns resistance but `@nova` works.

### Pitfall 6: Bundle Body Parsing Requires Full Parser
**What goes wrong:** `load-bundle` (D-14) must return a list of AST nodes. The template `body` column contains raw Innate text that needs parsing.
**Why it happens:** The resolver protocol specifies: "Returns: list of AST nodes (the bundle's parsed contents) on success, NIL if not found."
**How to avoid:** Load the full innatescript parser chain (tokenizer + parser) at boot time. Call `(innate.parser:parse (innate.parser.tokenizer:tokenize body))` to convert body text to AST nodes. Extract children from the resulting program node.
**Warning signs:** `load-bundle` returning raw text instead of AST nodes.

## Code Examples

### Minimum Viable resolve-reference (agents table)
```common-lisp
;; Query pattern for agent lookup by name (case-insensitive)
(defun resolve-from-agents (name)
  "Look up agent by id or full_name. Returns plist or nil."
  (handler-case
      (let* ((esc (db-escape (string-downcase name)))
             (sql (format nil
                    "SELECT id, full_name, role, department, status, agent_tier, ~
                     description, reports_to ~
                     FROM agents WHERE LOWER(id) = LOWER(~a) ~
                     OR LOWER(full_name) = LOWER(~a) LIMIT 1"
                    esc esc))
             (results (db-query sql)))
        (when (> (length results) 0)
          (hash-to-plist (aref results 0))))
    (error (e)
      (format t "  [resolve-from-agents] error: ~a~%" e)
      nil)))
```

### resolve-search with Dynamic WHERE Clauses
```common-lisp
;; Search directive: ![projects]{status=active,owner=nova}
(defmethod resolve-search ((r noosphere-resolver) search-type terms)
  (let* ((table (search-type-to-table search-type))
         (where-clauses
           (loop for term in terms
                 when (consp term)
                 collect (format nil "LOWER(~a) = LOWER(~a)"
                                 (car term) (db-escape (if (consp (cdr term))
                                                           (cadr term)
                                                           (cdr term)))))))
    (if (and table where-clauses)
        (let* ((where (format nil " WHERE ~{~a~^ AND ~}" where-clauses))
               (sql (format nil "SELECT * FROM ~a~a LIMIT 50" table where))
               (results (db-query sql)))
          (if (> (length results) 0)
              (make-innate-result
               :value (map 'list #'hash-to-plist results)
               :context :query)
              (make-resistance
               :message (format nil "No ~a match filters" table)
               :source (format nil "~a" terms))))
        (make-resistance
         :message (format nil "Unknown search type: ~a" search-type)
         :source (format nil "~a" search-type)))))
```

### deliver-commission Using Existing db-insert-conversation
```common-lisp
;; Commission: (nova){review the pipeline status}
(defmethod deliver-commission ((r noosphere-resolver) agent-name instruction)
  (let ((agent-id (resolve-agent-id agent-name)))
    (if agent-id
        (progn
          (db-insert-conversation
            "system"                    ; from_agent
            (list agent-id)             ; to_agents (list of strings)
            (if (stringp instruction) instruction (format nil "~a" instruction))
            :channel "commission")
          (make-innate-result :value t :context :commission))
        ;; Agent not found -- commissions should still return innate-result per protocol
        ;; But D-12 says return resistance if not found
        (make-resistance :message (format nil "Agent not found: ~a" agent-name)
                         :source agent-name))))
```

**Note on D-12 vs protocol:** The resolver protocol docstring says `deliver-commission` "Never returns resistance." But D-12 says "If not found, return resistance." The CONTEXT.md decision (D-12) takes precedence as a locked decision. However, the evaluator may not expect resistance from `deliver-commission`. Resolution: return `make-innate-result :value nil :context :commission` with a log warning, consistent with the base class default. This preserves protocol safety while logging the error. Alternatively, follow D-12 literally. The planner should note this tension.

### load-bundle Using Innate Parser
```common-lisp
;; Bundle: {daily_standup}
(defmethod load-bundle ((r noosphere-resolver) name)
  (handler-case
      (let* ((esc (db-escape name))
             (sql (format nil "SELECT body FROM templates WHERE LOWER(name) = LOWER(~a) LIMIT 1" esc))
             (results (db-query sql)))
        (when (> (length results) 0)
          (let* ((row (aref results 0))
                 (body (gethash :body row)))
            (when body
              ;; Parse template body into AST nodes
              (let* ((tokens (innate.parser.tokenizer:tokenize body))
                     (program (innate.parser:parse tokens)))
                ;; Return children of the program node (the actual content nodes)
                (innate.types:node-children program))))))
    (error (e)
      (format t "  [load-bundle] error: ~a~%" e)
      nil)))
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| HTTP API for all DB ops | Direct SQL via libpq FFI | Phase 21/22 (v1.4) | Resolver uses db-query directly, no HTTP |
| vault_notes table | memories table (with view bridge) | v1.3 PARAT | Wikilink resolution queries `memories`, not `vault_notes` |
| stub-resolver only | noosphere-resolver (this phase) | Phase 23 | First live resolver connecting Innate to real data |

## Open Questions

1. **deliver-commission Protocol Tension**
   - What we know: The resolver protocol docstring says deliver-commission "Never returns resistance." D-12 says return resistance if agent not found.
   - What's unclear: Whether the evaluator handles resistance from deliver-commission gracefully.
   - Recommendation: Check evaluator.lisp handling of deliver-commission return value. If it doesn't check for resistance, return innate-result with nil value and log the error instead. If it does handle resistance, follow D-12.

2. **Agent State Join for Energy/Tier**
   - What we know: The success criteria say `(agent_name)` should resolve to "id, department, energy, tier, and current assignments." Energy and tier live in `agent_state` table, not `agents`.
   - What's unclear: Whether to JOIN agents + agent_state in the query.
   - Recommendation: Yes, JOIN. `SELECT a.*, s.energy, s.tier FROM agents a LEFT JOIN agent_state s ON a.id = s.agent_id WHERE ...`. This gives the full picture in one query.

3. **Innatescript Loading Strategy**
   - What we know: launch.sh loads files directly, not via ASDF. Innatescript has its own ASDF system at `/opt/innatescript/innatescript.asd`.
   - What's unclear: Whether to use ASDF `load-system` for innatescript or direct file loads.
   - Recommendation: Direct file loads (matching launch.sh's existing pattern). Load the minimum needed: packages.lisp, types.lisp, conditions.lisp, parser/tokenizer.lisp, parser/parser.lisp, eval/resolver.lisp. This avoids ASDF configuration complexity and keeps the boot process consistent.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Custom Lisp test framework (innatescript/tests/test-framework.lisp) + manual SBCL REPL verification |
| Config file | None -- tests run via SBCL load |
| Quick run command | `cd /opt/innatescript && sbcl --eval '(require :asdf)' --eval '(asdf:load-system "innatescript/tests")' --eval '(innate.tests:run-all)' --quit` |
| Full suite command | Same as quick run + manual REPL verification of noosphere-resolver against live DB |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| INNATE-01a | @project_name resolves to project row | integration | Manual REPL: load resolver, call resolve-reference | No -- Wave 0 |
| INNATE-01b | @area_name, @template_name, @agent_name resolve | integration | Manual REPL | No -- Wave 0 |
| INNATE-01c | (agent_name) resolves + delivers commission | integration | Manual REPL + check conversations table | No -- Wave 0 |
| INNATE-01d | {scope_filter} narrows queries | integration | Manual REPL | No -- Wave 0 |
| INNATE-01e | Resolution errors return structured resistance | unit | SBCL eval with missing entities | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** Load noosphere-resolver in SBCL REPL, test against live DB
- **Per wave merge:** Full innatescript test suite + resolver integration test
- **Phase gate:** All 5 success criteria verified against live master_chronicle

### Wave 0 Gaps
- [ ] Test script that loads innatescript + noosphere-resolver and runs resolution against live DB
- [ ] Test cases for each success criterion (project, area, template, agent, filter, error)
- [ ] Verify innatescript's existing 175/176 tests still pass after loading noosphere packages

## Database Schema Reference

### Entity Resolution Target Tables

| Table | Match Column(s) | Key Fields to Return | Notes |
|-------|-----------------|---------------------|-------|
| agents | `id`, `full_name` | id, full_name, role, department, status, agent_tier, description | JOIN agent_state for energy, tier |
| projects | `name` | id, name, status, owner, description, goals, lifestage | No `title` column -- use `name` |
| areas | `name` | id, name, slug, description, owner, status | |
| templates | `name` | id, name, slug, category, description, body, parameters | `body` used by load-bundle |
| resources | `name` | id, name, slug, resource_type, description, tags | |
| memories | `title` | id, path, title, content, note_type | For wikilink resolution |

### Search Type to Table Mapping
| Search Type | Table | Valid Filter Columns |
|-------------|-------|---------------------|
| "projects" | projects | status, owner, lifestage, area_id |
| "agents" | agents | department, status, agent_tier, role |
| "areas" | areas | owner, status |
| "templates" | templates | category |
| "resources" | resources | resource_type, area_id, frozen |
| "tasks" | tasks | status, assigned_to, project_id, department |

## Sources

### Primary (HIGH confidence)
- `/opt/innatescript/src/eval/resolver.lisp` -- Resolver protocol (6 generic functions, exact signatures)
- `/opt/innatescript/src/eval/stub-resolver.lisp` -- Reference implementation showing correct return patterns
- `/opt/innatescript/src/types.lisp` -- innate-result and resistance struct definitions
- `/opt/innatescript/src/packages.lisp` -- All package definitions and exports
- `/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp` -- db-query, db-execute, db-escape
- `/opt/project-noosphere-ghosts/lisp/runtime/db-auxiliary.lisp` -- Existing entity query patterns
- `/opt/project-noosphere-ghosts/lisp/runtime/db-conversations.lisp` -- db-insert-conversation
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` -- AF64 package definitions
- `/opt/project-noosphere-ghosts/launch.sh` -- Boot sequence (direct file loading)
- PostgreSQL `\d` output for agents, projects, areas, templates, resources, memories tables

### Secondary (MEDIUM confidence)
- `/opt/innatescript/src/eval/evaluator.lisp` -- How evaluator calls resolver (verified first 80 lines)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all components are existing, verified in the codebase
- Architecture: HIGH -- resolver protocol is well-defined with reference implementation
- Pitfalls: HIGH -- verified against actual DB schemas and code patterns
- Cross-repo integration: MEDIUM -- launch.sh direct loading is straightforward but untested with innatescript

**Research date:** 2026-03-29
**Valid until:** 2026-04-28 (stable -- all components are local codebase, no external dependencies)

## Project Constraints (from CLAUDE.md)

- **Stack**: Common Lisp/SBCL only for ghost code. No new languages.
- **DB is the OS**: All state in master_chronicle. Resolver queries go to PostgreSQL.
- **AF64 zero-deps**: No Quicklisp. Innatescript packages loaded via direct file loads.
- **Lisp naming**: kebab-case functions, `*earmuffs*` for specials, dot-separated package names.
- **Error handling**: handler-case wrapping, error states logged but engine continues.
- **JSON quirk**: Parser converts underscores to hyphens in keys.
- **Single droplet**: All on 144.126.251.126. Resource-conscious.
