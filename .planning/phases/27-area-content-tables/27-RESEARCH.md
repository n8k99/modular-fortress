# Phase 27: Area Content Tables - Research

**Researched:** 2026-03-30
**Domain:** PostgreSQL schema design, Common Lisp CLOS resolver, InnateScipt bundle evaluation
**Confidence:** HIGH

## Summary

Phase 27 creates a structured `area_content` table in master_chronicle, populates it from the existing 1,027 EM area documents, and extends the noosphere resolver's `load-bundle` method to resolve `{em.content}` and `{em.content.podcast}` expressions. The work is entirely backend: one SQL migration, one data migration script, one new Lisp query function, and one extension to the existing `load-bundle` CLOS method.

The existing codebase patterns are well-established. The `noosphere-resolver.lisp` already handles table dispatch via dot-notation in `resolve-reference`. The `load-bundle` method currently only queries the `templates` table. Extending it to detect area-slug dot-notation (e.g., `em.content`) and route to `area_content` is a clean, minimal change. The DB layer (`db-client.lisp`, `db-auxiliary.lisp`) provides all needed primitives: `db-query`, `db-execute`, `db-escape`, `db-query-single`.

**Primary recommendation:** Single `area_content` table with content_type discriminator and JSONB metadata. Extend `load-bundle` in noosphere-resolver.lisp to detect `{area-slug.content}` and `{area-slug.content.type}` patterns. Migration script maps document paths to content types using the verified path-prefix mapping from the documents table.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Single `area_content` table with `content_type` VARCHAR discriminator and JSONB `metadata` column -- avoids per-domain table proliferation
- **D-02:** FK to `areas` table via `area_id` column for area scoping
- **D-03:** Core columns: id, area_id, content_type, title, body, metadata (JSONB), status, created_at, updated_at
- **D-04:** Optional FK to `documents` table (`source_document_id`) for traceability back to original vault documents
- **D-05:** Content types derived from existing EM document paths: podcast, blog, branding, engineering, thought-police, morning-pages, label, speaking, systems, collaboration
- **D-06:** New CLOS resolver method for `area_content` table -- `{em.content}` resolves to area-scoped content via area_id FK where area slug = 'em-corp'
- **D-07:** Filterable by content_type: `{em.content.podcast}` resolves to area_content WHERE area_id = EM AND content_type = 'podcast'
- **D-08:** Follows existing CLOS resolver pattern from Phase 23 (noosphere-resolver.lisp)
- **D-09:** Create DB migration (SQL) for area_content table with proper indexes
- **D-10:** Populate from existing documents table for EM area (path prefix 'Areas/Eckenrode Muziekopname/')
- **D-11:** Map document path segments to content_type values per D-05 mapping
- **D-12:** Documents table remains untouched -- area_content is additive, not destructive

### Claude's Discretion
- Exact JSONB metadata schema per content_type
- Index strategy (GIN on metadata, B-tree on area_id + content_type)
- Whether to add dpn-api endpoints for area_content CRUD or rely on direct SQL from tick engine
- Batch size for migration script

### Deferred Ideas (OUT OF SCOPE)
- Area content tables for Orbis, LRM, N8K99, Infrastructure -- future phases after EM proves the pattern
- Full-text search on area_content.body -- could use pg_trgm or tsvector, but not Phase 27 scope
- Embedding vectors on area_content -- valuable for semantic search but separate concern
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| AREA-01 | EM area has structured table(s) for its content | SQL migration creates `area_content` table with D-03 columns, D-09 indexes. Verified areas table exists with EM at id=1, slug='em-corp' |
| AREA-02 | Content records scoped under areas via FK relationships | `area_id` FK to `areas(id)` per D-02. Existing `areas` table already referenced by projects, resources, teams. Pattern proven. |
| AREA-03 | Noosphere resolver can query area-scoped content via InnateScipt | Extend `load-bundle` in noosphere-resolver.lisp to handle `{em.content}` and `{em.content.podcast}` dot-notation. Returns plist results via existing `db-query` + `hash-to-plist` pattern. |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

- **Stack**: Rust (dpn-api, dpn-core), Common Lisp/SBCL (ghosts), Python (dispatch tools), PostgreSQL -- no new languages
- **DB is the OS**: All state in master_chronicle. No file-based state for ghost work.
- **Lisp JSON quirk**: Parser converts underscores to hyphens (`:is-error` not `:is_error`)
- **Single droplet**: All services on 144.126.251.126. Resource-conscious design.
- **Common Lisp naming**: `kebab-case` functions, `*earmuffs*` for specials, `af64.runtime.*` package hierarchy
- **GSD Workflow Enforcement**: Work through GSD commands

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| PostgreSQL | 15+ (installed) | area_content table, migration | Already the noosphere substrate |
| SBCL | installed | noosphere-resolver extension | AF64 runtime language |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| af64.utils.pg | in-tree | `pg-query`, `pg-execute`, `pg-escape` | All DB operations from Lisp |
| af64.runtime.db | in-tree | `db-query`, `db-escape`, `db-query-single` | High-level query wrapper |
| af64.runtime.noosphere-resolver | in-tree | `load-bundle`, `resolve-reference` | Innate expression evaluation |

No new dependencies required. This phase uses only existing infrastructure.

## Architecture Patterns

### Recommended File Structure
```
/opt/project-noosphere-ghosts/lisp/runtime/
  noosphere-resolver.lisp     # MODIFIED: extend load-bundle for area content
  db-area-content.lisp        # NEW: SQL queries for area_content table (optional, could go in db-auxiliary)
  packages.lisp               # MODIFIED: export new symbols if new module

SQL migration (location TBD):
  migrations/027_area_content.sql   # NEW: CREATE TABLE + indexes + migration INSERT
```

### Pattern 1: Table Query Function (existing pattern from db-auxiliary.lisp)
**What:** Each domain gets dedicated query functions that build SQL and call `db-query`/`db-execute`
**When to use:** Any new table access from the tick engine
**Example:**
```lisp
;; Source: noosphere-resolver.lisp lines 34-44 (resolve-from-agents pattern)
(defun resolve-from-area-content (area-slug &optional content-type)
  "Query area_content by area slug and optional content_type. Returns list of plists."
  (handler-case
      (let* ((esc-slug (db-escape area-slug))
             (sql (if content-type
                      (format nil
                        "SELECT ac.id, ac.title, ac.content_type, ac.status, ac.metadata, ac.created_at ~
                         FROM area_content ac ~
                         JOIN areas a ON ac.area_id = a.id ~
                         WHERE a.slug = ~a AND ac.content_type = ~a ~
                         ORDER BY ac.created_at DESC LIMIT 50"
                        esc-slug (db-escape content-type))
                      (format nil
                        "SELECT ac.id, ac.title, ac.content_type, ac.status, ac.metadata, ac.created_at ~
                         FROM area_content ac ~
                         JOIN areas a ON ac.area_id = a.id ~
                         WHERE a.slug = ~a ~
                         ORDER BY ac.created_at DESC LIMIT 50"
                        esc-slug)))
             (results (db-query sql)))
        (loop for i from 0 below (length results)
              collect (hash-to-plist (aref results i))))
    (error (e)
      (format t "[noosphere-resolver] area_content lookup error: ~a~%" e)
      nil)))
```

### Pattern 2: Bundle Load Extension (extending load-bundle)
**What:** Extend `load-bundle` to detect `{area-slug.content}` and `{area-slug.content.type}` patterns
**When to use:** When `{em.content}` or `{em.content.podcast}` is evaluated
**Example:**
```lisp
;; Extend load-bundle to handle area content dot-notation
;; Before falling through to template lookup, check if name matches area-slug.content pattern
(defmethod load-bundle ((r noosphere-resolver) name)
  "Load a bundle by name. First checks for area.content pattern, then falls through to templates."
  ;; Check for area.content or area.content.type pattern
  (let ((parts (split-string name #\.)))
    (cond
      ;; {em.content} -> area_content WHERE area slug = 'em-corp'
      ;; {em.content.podcast} -> area_content WHERE slug='em-corp' AND content_type='podcast'
      ((and (>= (length parts) 2)
            (string= (second parts) "content"))
       (let* ((area-slug-prefix (first parts))  ;; "em" -> need to resolve to "em-corp"
              (content-type (when (>= (length parts) 3) (third parts)))
              (results (resolve-from-area-content
                         (resolve-area-slug area-slug-prefix) content-type)))
         ;; Return results as innate-result, not AST nodes
         ;; (This differs from template bundles which return parsed AST)
         (when results
           results)))  ;; Return list of plists
      ;; Default: template lookup (existing behavior)
      (t
       ;; ... existing template lookup code ...
       ))))
```

**Key design decision:** `{em.content}` returns **data** (list of plists), not **AST nodes**. This is different from template bundles which return parsed InnateScipt. The evaluator already handles non-node return values from `eval-node` (it can return strings, plists, etc.). The bundle case in the evaluator signals resistance if nil is returned, which is correct for "not found". For area content, returning data directly is the right approach.

### Pattern 3: Area Slug Resolution
**What:** Map short area prefix (e.g., "em") to full area slug (e.g., "em-corp")
**When to use:** When parsing `{em.content}` expressions
**Example:**
```lisp
(defun resolve-area-slug (prefix)
  "Resolve a short area prefix to full area slug. 'em' -> 'em-corp', etc.
   Falls back to using prefix as-is if no match."
  (handler-case
      (let* ((esc (db-escape (format nil "~a%" prefix)))
             (sql (format nil "SELECT slug FROM areas WHERE slug LIKE ~a LIMIT 1" esc))
             (result (db-query-single sql)))
        (if result (gethash :slug result) prefix))
    (error (e)
      (declare (ignore e))
      prefix)))
```

### Anti-Patterns to Avoid
- **Modifying the documents table:** D-12 explicitly locks this. area_content is additive.
- **Per-content-type tables:** D-01 locks single table with discriminator. Do not create `podcast_content`, `blog_content`, etc.
- **Returning AST nodes from area content queries:** Unlike template bundles, area content returns data (plists). Do not try to parse content body as InnateScipt.
- **Hardcoding area ID:** Use the FK relationship and slug lookup, not `WHERE area_id = 1`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| SQL escaping | Manual string quoting | `db-escape` from db-client.lisp | PQescapeLiteral handles all edge cases |
| DB connection | Raw libpq calls | `db-query`/`db-execute` via `*db-pool*` | Connection pooling, health checks built in |
| JSON column handling | String concatenation | `db-escape` with hash-table input | Handles JSONB encoding + escaping |
| PostgreSQL arrays | Manual array syntax | `pg-text-array`/`pg-int-array` from db-auxiliary | Proper escaping and type casting |

## Common Pitfalls

### Pitfall 1: JSON Key Hyphenation
**What goes wrong:** Lisp JSON parser converts underscores to hyphens. `content_type` becomes `:content-type` in hash-tables returned by `db-query`.
**Why it happens:** af64.utils.json `json-keyword` function does this conversion globally.
**How to avoid:** Always use hyphenated keywords when accessing query results: `(gethash :content-type row)`, NOT `(gethash :content_type row)`. SQL column names stay snake_case in the SQL string, but result access uses kebab-case keywords.
**Warning signs:** NIL returns from `gethash` on query results.

### Pitfall 2: Bundle vs Data Return Types
**What goes wrong:** `load-bundle` currently returns AST nodes (from template parsing). Area content should return data plists. The evaluator calls `eval-node` on bundle children, which would fail on raw data.
**Why it happens:** Template bundles contain InnateScipt text that gets parsed. Area content is structured data, not code.
**How to avoid:** Two approaches: (a) Override `load-bundle` to return an `innate-result` directly for area content (bypass the evaluator's node iteration), or (b) Create a separate method/function called from `load-bundle` that returns data wrapped in `make-innate-result`. Approach (b) is cleaner -- detect area content pattern in `load-bundle`, return early with a result rather than falling through to the AST evaluation path.
**Warning signs:** Type errors when evaluator tries to call `node-kind` on a plist.

### Pitfall 3: Migration Data Integrity
**What goes wrong:** Some documents have NULL titles or empty content. Path-to-content_type mapping misses edge cases (156 "other" documents that don't match any of the 10 defined content types).
**Why it happens:** Document paths are not perfectly regular. Some are top-level department files, staff profiles, or miscellaneous.
**How to avoid:** Use `COALESCE` for nullable fields. Assign unmapped documents to a catch-all content type like `general` or skip them. The 156 unmapped docs include: EM Staff profiles (64), Art dept reports (7), support docs (7), lore (6), hard prompts (7), D&D tech stack (5), and various singletons.
**Warning signs:** NULL constraint violations during INSERT, content_type values not matching the defined set.

### Pitfall 4: Large Result Sets
**What goes wrong:** `{em.content}` without type filter returns 1,027 records. Returning all as plists could be expensive.
**Why it happens:** No pagination in the resolver pattern.
**How to avoid:** Apply LIMIT 50 (consistent with existing `resolve-search` pattern). For area-wide queries, return summary data (id, title, content_type, status) rather than full body text.
**Warning signs:** Slow query response, large memory allocation in SBCL.

### Pitfall 5: Area Slug Prefix Ambiguity
**What goes wrong:** Short prefix "em" could match multiple areas if future areas start with "em".
**Why it happens:** Using LIKE 'em%' for prefix matching.
**How to avoid:** Use exact slug mapping. Current areas: em-corp, orbis, living-room-music, n8k99-personal, infrastructure-systems. A simple hardcoded or configurable map is safe for now. Or query `WHERE slug = 'em-corp'` directly, with the prefix-to-slug mapping done in Lisp code.
**Warning signs:** Wrong area returned from ambiguous prefix.

## Code Examples

### SQL Migration (area_content table)
```sql
-- Source: Verified against areas table schema (id, name, slug, metadata)
-- and documents table schema (id, path, title, content, status)
CREATE TABLE area_content (
    id SERIAL PRIMARY KEY,
    area_id INTEGER NOT NULL REFERENCES areas(id),
    content_type VARCHAR(64) NOT NULL,
    title VARCHAR(512),
    body TEXT,
    metadata JSONB DEFAULT '{}'::jsonb,
    status VARCHAR(32) DEFAULT 'active',
    source_document_id INTEGER REFERENCES documents(id),
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

-- Primary query pattern: area + content_type
CREATE INDEX idx_area_content_area_type ON area_content(area_id, content_type);
-- Status filtering
CREATE INDEX idx_area_content_status ON area_content(status);
-- Source document traceability
CREATE INDEX idx_area_content_source_doc ON area_content(source_document_id);
-- JSONB metadata queries (discretionary)
CREATE INDEX idx_area_content_metadata ON area_content USING gin(metadata);
-- Updated_at trigger (matches areas table pattern)
CREATE TRIGGER update_area_content_updated_at
    BEFORE UPDATE ON area_content
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

### Data Migration (populate from documents)
```sql
-- Source: Verified document path counts from master_chronicle
INSERT INTO area_content (area_id, content_type, title, body, source_document_id, status)
SELECT
    1 AS area_id,  -- EM Corp (verified: id=1, slug='em-corp')
    CASE
        WHEN d.path LIKE '%/Musicology/01 Podcast%' THEN 'podcast'
        WHEN d.path LIKE '%/Executive/03 Blog%' THEN 'blog'
        WHEN d.path LIKE '%/ContentandBrandingOffice%' THEN 'branding'
        WHEN d.path LIKE '%/Engineering/01 Engineering%' THEN 'engineering'
        WHEN d.path LIKE '%/Executive/Thought Police%' THEN 'thought-police'
        WHEN d.path LIKE '%/Musicology/Morning Pages%' THEN 'morning-pages'
        WHEN d.path LIKE '%/Musicology/02 Label%' THEN 'label'
        WHEN d.path LIKE '%/Executive/04 CEO Speak%' THEN 'speaking'
        WHEN d.path LIKE '%/Engineering/03 Systems%' THEN 'systems'
        WHEN d.path LIKE '%/Success/Collaboration%' THEN 'collaboration'
        ELSE 'general'
    END AS content_type,
    COALESCE(d.title, SPLIT_PART(d.path, '/', -1)) AS title,
    d.content AS body,
    d.id AS source_document_id,
    COALESCE(d.status, 'active') AS status
FROM documents d
WHERE d.path LIKE 'Areas/Eckenrode Muziekopname/%';
```

### Lisp: load-bundle Extension Pattern
```lisp
;; Source: noosphere-resolver.lisp load-bundle method (lines 314-332)
;; and resolve-reference dot-notation pattern (lines 155-173)

;; Helper: split string on delimiter
(defun split-on-dot (s)
  "Split string S on '.' returning list of substrings."
  (let ((parts '()) (start 0))
    (loop for i from 0 below (length s)
          when (char= (char s i) #\.)
          do (push (subseq s start i) parts)
             (setf start (1+ i)))
    (push (subseq s start) parts)
    (nreverse parts)))

;; Area slug map -- maps short prefixes to full slugs
(defparameter *area-slug-map*
  '(("em" . "em-corp")
    ("orbis" . "orbis")
    ("lrm" . "living-room-music")
    ("n8k99" . "n8k99-personal")
    ("infra" . "infrastructure-systems"))
  "Map of short area prefixes to full area slugs.")

(defun lookup-area-slug (prefix)
  "Map short prefix to full area slug. Returns slug string or nil."
  (cdr (assoc prefix *area-slug-map* :test #'string-equal)))
```

## Verified Data

### EM Document Counts by Content Type (verified 2026-03-30)
| Content Type | Count | Path Pattern |
|--------------|-------|-------------|
| podcast | 321 | Musicology/01 Podcast Episodes/ |
| blog | 163 | Executive/03 Blog/ |
| branding | 106 | ContentandBrandingOffice/ |
| engineering | 101 | Engineering/01 Engineering Projects/ |
| thought-police | 52 | Executive/Thought Police/ |
| morning-pages | 39 | Musicology/Morning Pages/ |
| label | 30 | Musicology/02 Label Artists/ |
| speaking | 23 | Executive/04 CEO Speaks/ |
| systems | 22 | Engineering/03 Systems/ |
| collaboration | 14 | Success/CollaborationFramework.../ |
| **general** (unmapped) | **156** | EM Staff (64), Art (7), Support (7), Lore (6), misc |
| **Total** | **1,027** | Areas/Eckenrode Muziekopname/ |

### Areas Table (verified 2026-03-30)
| ID | Name | Slug |
|----|------|------|
| 1 | EM Corp | em-corp |
| 2 | Orbis | orbis |
| 3 | Living Room Music | living-room-music |
| 4 | N8K99/Personal | n8k99-personal |
| 5 | Infrastructure/Systems | infrastructure-systems |

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Flat documents with path-prefix queries | Structured area_content with FK scoping | Phase 27 (this phase) | Enables typed content queries, area-scoped resolution |
| Template-only bundle loading | Bundle loading with area content detection | Phase 27 (this phase) | `{em.content}` syntax works alongside `{template-name}` |

## Open Questions

1. **What to do with 156 unmapped "general" documents?**
   - What we know: They include EM Staff profiles (64), Art dept (7), Support (7), Lore (6), Hard Prompts (7), D&D Tech Stack (5), and misc.
   - What's unclear: Whether to assign them `general` content type or create more specific types (staff, art, support, lore)
   - Recommendation: Use `general` as catch-all for Phase 27. Future phases can refine content types. This keeps the migration simple and matches D-05 which only defines 10 types.

2. **dpn-api endpoints for area_content CRUD?**
   - What we know: Tick engine can query directly via `db-query`. dpn-api has no area_content handlers.
   - What's unclear: Whether frontend sites need area_content access.
   - Recommendation: Skip dpn-api endpoints in Phase 27. Direct SQL from tick engine is sufficient for resolver needs. Add API endpoints in a future phase if frontends need it.

3. **JSONB metadata schema per content type?**
   - What we know: Documents table has `frontmatter` (TEXT) and some have structured content.
   - Recommendation: For Phase 27, populate metadata with extracted frontmatter as JSON where parseable, empty `{}` otherwise. Content-type-specific metadata schemas (e.g., podcast: episode_number, publish_date; blog: author, tags) can evolve organically.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual SQL verification + SBCL REPL |
| Config file | None -- Lisp runtime tests run in REPL |
| Quick run command | `PGPASSWORD=chronicle2026 psql -U chronicle -d master_chronicle -h 127.0.0.1 -c "SELECT count(*) FROM area_content;"` |
| Full suite command | See below per requirement |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| AREA-01 | area_content table exists with correct schema | smoke | `PGPASSWORD=chronicle2026 psql -U chronicle -d master_chronicle -h 127.0.0.1 -c "\d area_content"` | N/A (SQL) |
| AREA-01 | EM content records populated | smoke | `PGPASSWORD=chronicle2026 psql -U chronicle -d master_chronicle -h 127.0.0.1 -c "SELECT content_type, COUNT(*) FROM area_content WHERE area_id = 1 GROUP BY content_type ORDER BY count DESC;"` | N/A (SQL) |
| AREA-02 | FK relationship to areas table works | smoke | `PGPASSWORD=chronicle2026 psql -U chronicle -d master_chronicle -h 127.0.0.1 -c "SELECT a.name, COUNT(ac.id) FROM area_content ac JOIN areas a ON ac.area_id = a.id GROUP BY a.name;"` | N/A (SQL) |
| AREA-03 | {em.content} resolves via load-bundle | integration | Load noosphere-ghosts in SBCL, call `(load-bundle *noosphere-resolver* "em.content")` | N/A (REPL) |
| AREA-03 | {em.content.podcast} filters by type | integration | SBCL REPL: `(load-bundle *noosphere-resolver* "em.content.podcast")` | N/A (REPL) |

### Sampling Rate
- **Per task commit:** Run SQL smoke tests for table/data integrity
- **Per wave merge:** Run all SQL smoke tests + verify SBCL resolver loads without error
- **Phase gate:** All 5 test cases pass, including REPL integration tests

### Wave 0 Gaps
None -- no test framework needed. Verification is SQL queries (table exists, data correct) and REPL evaluation (resolver methods work). Both are manual but deterministic.

## Sources

### Primary (HIGH confidence)
- master_chronicle `areas` table schema -- verified via `\d areas` query
- master_chronicle `documents` table schema -- verified via `\d documents` query
- `/opt/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` -- full source read, 349 lines
- `/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp` -- full source read, db-query/db-escape/db-execute patterns
- `/opt/project-noosphere-ghosts/lisp/runtime/db-auxiliary.lisp` -- first 50 lines, pg-text-array/pg-int-array patterns
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` -- full source read, all package definitions
- `/opt/innatescript/src/eval/resolver.lisp` -- full source read, CLOS generic function protocol
- `/opt/innatescript/src/eval/evaluator.lisp` -- bundle evaluation path (lines 119-131, 190-203)
- Document path counts verified via SQL queries against master_chronicle

### Secondary (MEDIUM confidence)
- None needed -- all findings verified against source code and database

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all components exist and are verified in-tree
- Architecture: HIGH -- extending existing patterns (load-bundle, resolve-from-*, db-query)
- Pitfalls: HIGH -- verified JSON hyphenation quirk, bundle return type difference, data counts

**Research date:** 2026-03-30
**Valid until:** 2026-04-30 (stable codebase, no external dependencies)
