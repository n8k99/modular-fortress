# Phase 16: Foundation Tables & API - Context

**Gathered:** 2026-03-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Create the four new PARAT pillar tables (areas, archives, resources, templates) with seed data, database-level integrity enforcement, and full CRUD REST endpoints in dpn-api backed by dpn-core query modules. No modifications to existing tables — this phase is purely additive.

</domain>

<decisions>
## Implementation Decisions

### Seed Data (Areas)
- **D-01:** Seed 5 area domains matching ROADMAP success criteria: EM Corp, Orbis, Living Room Music, N8K99/Personal, Infrastructure/Systems
- **D-02:** Area owners mapped by executive domain routing: Nova (Infrastructure/Systems), Kathryn (EM Corp), Sylvia (Orbis), LRM (Living Room Music), Nathan (N8K99/Personal)

### Archives Immutability
- **D-03:** Database trigger prevents UPDATE on content fields — not application-level enforcement. Consistent with DB-is-the-OS philosophy. Any client (Rust, Python, Lisp) gets the same guarantee.
- **D-04:** POST creates records; PATCH allowed on metadata fields (tags, topic) but blocked on content/body fields by the trigger.

### Resources Frozen Flag
- **D-05:** Database trigger enforces frozen=true rows — UPDATE blocked at DB level when frozen flag is set. API returns 409 Conflict for frozen resource updates.
- **D-06:** Resources are a curated index referencing documents/media via source_type/source_id. No data duplication — resources point to existing rows in documents, media, etc.

### Templates Version History
- **D-07:** Separate `templates_history` table (not JSONB array) — proper FK to templates, queryable, matches REQUIREMENTS SCHEMA-04 spec.
- **D-08:** On body field change in templates, a trigger (or application logic) inserts the previous version into templates_history with a version number and timestamp.
- **D-09:** Templates store .dpn expressions as inert text — no evaluation in v1.3. Innate interpreter is a separate project.

### Claude's Discretion
- Column types, index choices, and migration script structure
- Handler response format (follow existing `Json(serde_json::json!({ "tablename": data }))` pattern)
- Test strategy for triggers (SQL-level tests in migration scripts)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Schema Specification
- `PARAT-NoosphereSchema.docx` — Original PARAT five-pillar architecture document (source of truth for table design intent)
- `.planning/REQUIREMENTS.md` — SCHEMA-01 through SCHEMA-04, API-01 through API-04 (acceptance criteria for this phase)

### Existing Patterns (Code Templates)
- `/opt/dpn-api/src/handlers/projects.rs` — Reference handler pattern (GET list, GET by id, POST create, PATCH update)
- `/root/dpn-core/src/db/projects.rs` — Reference dpn-core module pattern (struct + async query functions)
- `/opt/dpn-api/src/main.rs` — Router registration pattern for new endpoints

### Research
- `.planning/research/SUMMARY.md` — v1.3 research summary with risk analysis and execution order
- `.planning/research/ARCHITECTURE.md` — Architecture decisions for PARAT schema
- `.planning/research/PITFALLS.md` — Known pitfalls (sqlx compile-time checking, view RULES, etc.)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `dpn_core::Project` struct pattern: `#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]` — use same derive stack for Area, Archive, Resource, Template structs
- `ApiError` enum in `/opt/dpn-api/src/error.rs` — existing error types (Database, NotFound, Unauthorized) cover most needs; may need to add Conflict variant for frozen/immutable violations
- Handler pattern: `State(pool) → Result<Json<Value>, ApiError>` with dpn_core function delegation

### Established Patterns
- dpn-core modules: one file per table in `src/db/`, re-exported from `lib.rs`
- dpn-api handlers: one file per domain in `src/handlers/`, registered in `main.rs` router
- SQL migrations: raw SQL files (no ORM migration framework — direct psql execution)
- All DB functions are async, use `sqlx::query_as` for typed results

### Integration Points
- `/opt/dpn-api/src/main.rs` — new routes added to router (`.route("/api/areas", get(...)...`)
- `/root/dpn-core/src/lib.rs` — new modules declared and re-exported
- PostgreSQL `master_chronicle` — new tables via SQL migration script

</code_context>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches following existing codebase patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 16-foundation-tables-api*
*Context gathered: 2026-03-28*
