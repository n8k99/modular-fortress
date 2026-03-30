# Phase 27: Area Content Tables - Context

**Gathered:** 2026-03-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Create structured area-scoped content tables for Eckenrode Muziekopname that the noosphere resolver can query by area scope, replacing flat path-prefix matching against the documents table. EM area currently has ~900+ documents across Musicology, Executive, Engineering, and Success paths.

</domain>

<decisions>
## Implementation Decisions

### Table design
- **D-01:** Single `area_content` table with `content_type` VARCHAR discriminator and JSONB `metadata` column — avoids per-domain table proliferation
- **D-02:** FK to `areas` table via `area_id` column for area scoping
- **D-03:** Core columns: id, area_id, content_type, title, body, metadata (JSONB), status, created_at, updated_at
- **D-04:** Optional FK to `documents` table (`source_document_id`) for traceability back to original vault documents

### Content domain mapping
- **D-05:** Content types derived from existing EM document paths:
  - `podcast` — Musicology/01 Podcast (~321 docs)
  - `blog` — Executive/03 Blog (~161 docs)
  - `branding` — ContentandBrandingOffice (~106 docs)
  - `engineering` — Engineering/01 Engineering (~101 docs)
  - `thought-police` — Executive/Thought Police (~52 docs)
  - `morning-pages` — Musicology/Morning Pages (~39 docs)
  - `label` — Musicology/02 Label (~30 docs)
  - `speaking` — Executive/04 CEO Speaking (~23 docs)
  - `systems` — Engineering/03 Systems (~22 docs)
  - `collaboration` — Success/Collaborations (~14 docs)

### Resolver integration
- **D-06:** New CLOS resolver method for `area_content` table — `{em.content}` resolves to area-scoped content via area_id FK where area slug = 'em-corp'
- **D-07:** Filterable by content_type: `{em.content.podcast}` resolves to area_content WHERE area_id = EM AND content_type = 'podcast'
- **D-08:** Follows existing CLOS resolver pattern from Phase 23 (noosphere-resolver.lisp)

### Migration strategy
- **D-09:** Create DB migration (SQL) for area_content table with proper indexes
- **D-10:** Populate from existing documents table for EM area (path prefix 'Areas/Eckenrode Muziekopname/')
- **D-11:** Map document path segments to content_type values per D-05 mapping
- **D-12:** Documents table remains untouched — area_content is additive, not destructive

### Claude's Discretion
- Exact JSONB metadata schema per content_type
- Index strategy (GIN on metadata, B-tree on area_id + content_type)
- Whether to add dpn-api endpoints for area_content CRUD or rely on direct SQL from tick engine
- Batch size for migration script

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Noosphere resolver
- `/opt/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` — CLOS resolver protocol, existing resolve methods for documents/projects/areas
- `/opt/innatescript/` — Innate interpreter with pluggable resolver protocol

### Database schema
- master_chronicle `areas` table — id, name, slug, description, owner, status, metadata (JSONB)
- master_chronicle `documents` table — id, path, title, content, frontmatter, project_id, embedding

### Existing area content
- EM area (id=1, slug='em-corp') has ~900+ documents under path prefix 'Areas/Eckenrode Muziekopname/'

### Requirements
- `.planning/REQUIREMENTS.md` — AREA-01, AREA-02, AREA-03 requirement definitions

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `noosphere-resolver.lisp` — CLOS generic function `resolve-reference` with methods for different table types; new method for area_content follows same pattern
- `db-client.lisp` / `db-auxiliary.lisp` — existing SQL query patterns for direct PostgreSQL from Lisp via libpq FFI
- `pg.lisp` — pg-escape, pg-query functions for safe SQL execution

### Established Patterns
- DB migrations done as raw SQL files, applied manually or via script
- Resolver methods dispatch on reference type keyword (`:document`, `:project`, `:area`)
- JSONB used extensively (areas.metadata, projects.schedule, agent state)

### Integration Points
- `noosphere-resolver.lisp` — add new `resolve-reference` method for `:area-content` type
- `db-client.lisp` or new `db-area-content.lisp` — SQL queries for area_content table
- `packages.lisp` — export new symbols if adding new module

</code_context>

<specifics>
## Specific Ideas

- The 5 areas (EM Corp, Orbis, Living Room Music, N8K99/Personal, Infrastructure/Systems) all could eventually get area_content tables, but Phase 27 focuses on EM only per scope
- EM document paths are well-structured by department — the path-to-content_type mapping is clean
- The `{em.content.podcast}` syntax mirrors how Innate already handles dot-notation for nested resolution

</specifics>

<deferred>
## Deferred Ideas

- Area content tables for Orbis, LRM, N8K99, Infrastructure — future phases after EM proves the pattern
- Full-text search on area_content.body — could use pg_trgm or tsvector, but not Phase 27 scope
- Embedding vectors on area_content — valuable for semantic search but separate concern

</deferred>

---

*Phase: 27-area-content-tables*
*Context gathered: 2026-03-30*
