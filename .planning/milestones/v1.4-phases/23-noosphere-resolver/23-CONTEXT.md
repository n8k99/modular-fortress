# Phase 23: Noosphere Resolver - Context

**Gathered:** 2026-03-29
**Status:** Ready for planning
**Source:** Auto-mode (--auto)

<domain>
## Phase Boundary

Create a `noosphere-resolver` CLOS class that subclasses Innate's `resolver` protocol and implements all 6 generic functions (`resolve-reference`, `resolve-search`, `deliver-commission`, `resolve-wikilink`, `resolve-context`, `load-bundle`) by querying master_chronicle tables via the Phase 21/22 direct SQL infrastructure. This connects Innate's symbolic language to the live noosphere — `@project_name` returns real project data, `(agent_name){instruction}` delivers real commissions, `{status=active}` filters real records.

</domain>

<decisions>
## Implementation Decisions

### Resolver Architecture
- **D-01:** The noosphere-resolver lives in the noosphere-ghosts repo (`/opt/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp`), NOT in the innatescript repo. It needs the db-client SQL infrastructure and PG connection pool from Phase 21/22. The innatescript repo stays pure (no DB dependencies).
- **D-02:** The noosphere-resolver subclasses `innate.eval.resolver:resolver` (the CLOS base class). It must specialize all 6 generic functions defined in `/opt/innatescript/src/eval/resolver.lisp`.
- **D-03:** The noosphere-resolver needs access to the innatescript package at load time. Wire via ASDF dependency: af64.asd depends-on innatescript.asd, or load innatescript packages before noosphere-resolver.

### Entity-to-Table Mapping (resolve-reference @)
- **D-04:** `@name` resolves by searching tables in priority order: agents → projects → areas → templates → resources. First match wins. This covers the most common use case (agents are referenced most often).
- **D-05:** `@table.name` syntax narrows to a specific table: `@projects.nova_pipeline` searches only the projects table. The part before the dot is the table name (pluralized), after is the entity name.
- **D-06:** Entity matching: case-insensitive, match against `name` or `title` column (projects use `title`, agents use `id` or `full_name`, areas/templates/resources use `name`). Use ILIKE for fuzzy matching or exact match — Claude's discretion.
- **D-07:** Qualifier chain: `@entity:property` returns a specific field from the matched row. E.g., `@nova:department` returns Nova's department. Uses the stub-resolver pattern: intern qualifier as keyword, getf from the plist result.

### Scope Filters (resolve-search {})
- **D-08:** `{key=value}` in a search context maps to SQL `WHERE key = $value`. Multiple filters: `{status=active,owner=nova}` → `WHERE status='active' AND owner='nova'`.
- **D-09:** The search directive `![type]{filters}` maps search-type to a table name. E.g., `![projects]{status=active}` queries `SELECT * FROM projects WHERE status='active'`.
- **D-10:** Filter values are always strings (no type coercion). SQL comparison is case-insensitive via ILIKE or LOWER().

### Commission Delivery (deliver-commission)
- **D-11:** `(agent_name){instruction}` inserts a conversation message using `db-insert-conversation` from Phase 22. Fields: from_agent="system", to_agent=[agent_id], channel="commission", content=instruction text.
- **D-12:** Agent name resolution: match against agents table `id` column (case-insensitive). If not found, return resistance.

### Wikilink Resolution (resolve-wikilink)
- **D-13:** `[[Title]]` resolves against the memories table (formerly vault_notes) by title. Uses `SELECT * FROM memories WHERE title ILIKE $1 LIMIT 1`.

### Bundle Loading (load-bundle)
- **D-14:** `{bundle_name}` loads from the templates table. `SELECT body FROM templates WHERE name = $1`. The body is parsed by the Innate parser into AST nodes.

### Context Resolution (resolve-context)
- **D-15:** `[context[verb[args]]]` is Phase 24 scope (template evaluation). For Phase 23, implement a basic version that returns the raw context/verb/args as a structured result, or delegate to the default resistance. Claude's discretion.

### Error Handling
- **D-16:** All resolution failures return Innate's `resistance` struct (via `make-resistance`) with `:message` and `:source` fields. This matches the existing stub-resolver pattern and the evaluator's error handling.
- **D-17:** Ambiguous matches (multiple entities with same name across tables): return the first match by priority order (D-04). Do NOT return resistance for ambiguous matches — the priority order IS the disambiguation.

### Claude's Discretion
- Exact SQL queries for each resolution function
- Whether to cache resolved entities within a single evaluation pass
- How to handle NULL fields in resolved entity plists
- Whether resolve-context gets a basic implementation or pure resistance in Phase 23
- ASDF/package wiring details between innatescript and noosphere-ghosts

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Innate Resolver Protocol (THE interface to implement)
- `/opt/innatescript/src/eval/resolver.lisp` — Base resolver class + 6 generic function signatures
- `/opt/innatescript/src/eval/stub-resolver.lisp` — Reference implementation showing correct return types (innate-result vs resistance)
- `/opt/innatescript/src/types.lisp` — innate-result and resistance struct definitions
- `/opt/innatescript/src/conditions.lisp` — innate-resistance condition for error propagation

### Innate Evaluator (consumer of resolver)
- `/opt/innatescript/src/eval/evaluator.lisp` — How the evaluator calls resolver methods and handles results

### Phase 21/22 SQL Infrastructure (reuse these)
- `/opt/project-noosphere-ghosts/lisp/util/pg.lisp` — libpq FFI bindings, connection pool
- `/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp` — db-query, db-execute, db-escape
- `/opt/project-noosphere-ghosts/lisp/runtime/db-conversations.lisp` — db-insert-conversation (for commissions)
- `/opt/project-noosphere-ghosts/lisp/runtime/db-tasks.lisp` — db-get-tasks-by-filter (for task queries)
- `/opt/project-noosphere-ghosts/lisp/runtime/db-auxiliary.lisp` — db-get-agent, db-get-project-by-id, etc.

### PARAT Tables (query targets)
- Master chronicle tables: agents, projects, areas, templates, resources, archives, memories

### Requirements
- `.planning/REQUIREMENTS.md` — INNATE-01 (Noosphere Resolver)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `resolver` base class from innatescript — subclass with `noosphere-resolver`
- `stub-resolver` — exact reference implementation showing return value patterns
- `make-innate-result` / `make-resistance` — struct constructors for return types
- All `db-*` SQL wrapper functions from Phase 21/22 — agents, projects, conversations, tasks, etc.
- `db-auxiliary.lisp` already has `db-get-agent`, `db-get-project-by-id`, `db-get-agents` — many resolution queries are already written

### Established Patterns
- CLOS generic function dispatch for resolver protocol
- Resistance structs for error propagation (not exceptions)
- eval-env struct carries resolver instance through evaluation
- Entity plists as the standard data format (keyword keys, string values)

### Integration Points
- `eval-env` struct's `:resolver` slot — where the noosphere-resolver instance goes
- Ghost tick engine — must create noosphere-resolver at startup and pass to evaluator
- af64.asd — needs to load innatescript as a dependency (or at least the resolver/types packages)
- packages.lisp — new `af64.runtime.noosphere-resolver` package

</code_context>

<specifics>
## Specific Ideas

- The noosphere-resolver is the bridge between two repos: it lives in noosphere-ghosts but implements the innatescript protocol. This is the first cross-repo integration.
- Most resolution queries already exist as db-auxiliary functions — the resolver is largely a dispatch layer mapping Innate symbols to existing SQL wrappers.
- The stub-resolver has 175 passing tests — the noosphere-resolver should pass the same protocol contract (correct return types, resistance for missing entities, etc.)

</specifics>

<deferred>
## Deferred Ideas

- Template evaluation with Innate expressions — Phase 24
- Ghost expression generation — Phase 25
- Caching resolved entities across ticks — future optimization
- Full resolve-context implementation — Phase 24

</deferred>

---

*Phase: 23-noosphere-resolver*
*Context gathered: 2026-03-29 via auto-mode*
