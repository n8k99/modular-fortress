# Phase 16: Foundation Tables & API - Research

**Researched:** 2026-03-28
**Domain:** PostgreSQL schema creation + Rust (dpn-core/dpn-api) CRUD implementation
**Confidence:** HIGH

## Summary

Phase 16 creates four new PARAT pillar tables (areas, archives, resources, templates) in master_chronicle with database-level integrity enforcement (immutability triggers, frozen flags, version history), seeds them with initial data, and exposes full CRUD REST endpoints through dpn-api backed by dpn-core query modules. This phase is purely additive -- no existing tables are modified.

The existing codebase provides a clear, repeatable pattern for this work. Each table follows the same cycle: SQL migration -> dpn-core struct + async query functions -> dpn-api handler file -> router registration in main.rs. The `projects` module in both dpn-core and dpn-api serves as the exact template to follow. The only novel elements are the three database triggers (archives immutability, resources frozen enforcement, templates version history) and a new `Conflict` variant in `ApiError`.

**Primary recommendation:** Follow the projects module pattern exactly for all four tables. Write a single SQL migration script that creates all four tables, their triggers, and seed data. Then add dpn-core modules and dpn-api handlers one table at a time.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Seed 5 area domains matching ROADMAP success criteria: EM Corp, Orbis, Living Room Music, N8K99/Personal, Infrastructure/Systems
- **D-02:** Area owners mapped by executive domain routing: Nova (Infrastructure/Systems), Kathryn (EM Corp), Sylvia (Orbis), LRM (Living Room Music), Nathan (N8K99/Personal)
- **D-03:** Database trigger prevents UPDATE on content fields -- not application-level enforcement. Consistent with DB-is-the-OS philosophy. Any client (Rust, Python, Lisp) gets the same guarantee.
- **D-04:** POST creates records; PATCH allowed on metadata fields (tags, topic) but blocked on content/body fields by the trigger.
- **D-05:** Database trigger enforces frozen=true rows -- UPDATE blocked at DB level when frozen flag is set. API returns 409 Conflict for frozen resource updates.
- **D-06:** Resources are a curated index referencing documents/media via source_type/source_id. No data duplication -- resources point to existing rows in documents, media, etc.
- **D-07:** Separate `templates_history` table (not JSONB array) -- proper FK to templates, queryable, matches REQUIREMENTS SCHEMA-04 spec.
- **D-08:** On body field change in templates, a trigger (or application logic) inserts the previous version into templates_history with a version number and timestamp.
- **D-09:** Templates store .dpn expressions as inert text -- no evaluation in v1.3. Innate interpreter is a separate project.

### Claude's Discretion
- Column types, index choices, and migration script structure
- Handler response format (follow existing `Json(serde_json::json!({ "tablename": data }))` pattern)
- Test strategy for triggers (SQL-level tests in migration scripts)

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SCHEMA-01 | Areas table with id, name, slug, description, owner (agent FK), status, created_at, updated_at | SQL DDL from ARCHITECTURE.md verified against agents table (VARCHAR(64) FK). Seed data agent IDs confirmed: nova, kathryn, sylvia, lrm, nathan |
| SCHEMA-02 | Archives table with immutability enforcement (trigger prevents content UPDATE), source_type/source_id tracking, period_start/period_end dates, full-text search | Trigger pattern modeled on existing `update_updated_at_column` function. PostgreSQL 16 supports BEFORE UPDATE triggers with RAISE EXCEPTION |
| SCHEMA-03 | Resources table as curated index with source_type/source_id, type categorization, frozen flag | Frozen enforcement via BEFORE UPDATE trigger. API returns 409 via new `ApiError::Conflict` variant |
| SCHEMA-04 | Templates table with .dpn body, JSONB parameters, category, version tracking via templates_history | Separate history table with FK, trigger or application-level version insertion on body change |
| API-01 | CRUD endpoints for areas (GET list, GET by id, POST create, PATCH update) | Exact pattern from projects handler: `list_areas`, `get_area`, `create_area`, `update_area` |
| API-02 | CRUD endpoints for archives (GET list, GET by id, POST create -- no UPDATE on content) | POST creates; PATCH restricted to metadata. DB trigger is the real guard; API can also validate |
| API-03 | CRUD endpoints for resources (GET list, GET by id, POST create, PATCH update, frozen enforcement) | PATCH checks frozen status. DB trigger catches it too; API should return 409 proactively |
| API-04 | CRUD endpoints for templates (GET list, GET by id, POST create, PATCH update) with version history on body changes | Application logic in dpn-core update function: detect body change -> insert into templates_history before updating |
</phase_requirements>

## Standard Stack

### Core (already in project -- no new dependencies)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| sqlx | 0.8 | PostgreSQL async queries | Already used throughout dpn-core |
| axum | 0.7 | HTTP framework | Already used in dpn-api |
| serde + serde_json | 1 | Serialization | Already used everywhere |
| anyhow | 1 | Error handling | Already used in dpn-core |
| chrono | 0.4 | Timestamps | Already used for date columns |
| tokio | 1 | Async runtime | Already used |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| uuid | 1 | Unique IDs | Only if slug generation needs it (probably not -- SERIAL ids) |

### Alternatives Considered

None. This phase uses the exact existing stack. No new dependencies required.

**Installation:** No new packages needed. Both `dpn-core/Cargo.toml` and `/opt/dpn-api/Cargo.toml` already have all required dependencies.

## Architecture Patterns

### Recommended Project Structure Changes

```
dpn-core/src/db/
  mod.rs              # ADD: pub mod areas; pub mod archives; pub mod resources; pub mod templates;
  areas.rs            # NEW: Area struct + CRUD functions
  archives.rs         # NEW: Archive struct + CRUD functions
  resources.rs        # NEW: Resource struct + CRUD functions
  templates.rs        # NEW: Template, TemplateHistory structs + CRUD functions

dpn-core/src/lib.rs   # ADD: re-exports for all new types and functions

/opt/dpn-api/src/handlers/
  mod.rs              # ADD: pub mod areas; pub mod archives; pub mod resources; pub mod templates;
  areas.rs            # NEW: list, get, create, update handlers
  archives.rs         # NEW: list, get, create handlers (no content update)
  resources.rs        # NEW: list, get, create, update handlers (frozen enforcement)
  templates.rs        # NEW: list, get, create, update handlers (version history)

/opt/dpn-api/src/error.rs  # ADD: Conflict(String) variant -> 409 status
/opt/dpn-api/src/main.rs   # ADD: route registrations for all four tables
```

### Pattern 1: dpn-core Module Pattern (from projects.rs)
**What:** One file per table in `src/db/`, containing struct definition + async CRUD functions.
**When to use:** Every new table.
**Example:**
```rust
// Source: /root/dpn-core/src/db/projects.rs (existing pattern)
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use super::DbPool;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Area {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub owner: Option<String>,       // FK to agents.id (VARCHAR(64))
    pub status: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn list_areas(pool: &DbPool) -> Result<Vec<Area>> {
    let areas = sqlx::query_as::<_, Area>(
        "SELECT id, name, slug, description, owner, status, created_at, updated_at
         FROM areas ORDER BY name"
    )
    .fetch_all(pool)
    .await?;
    Ok(areas)
}
```

### Pattern 2: dpn-api Handler Pattern (from projects.rs)
**What:** One handler file per domain in `src/handlers/`, with request structs and handler functions.
**When to use:** Every new endpoint group.
**Example:**
```rust
// Source: /opt/dpn-api/src/handlers/projects.rs (existing pattern)
pub async fn list_areas(
    State(pool): State<DbPool>,
) -> Result<Json<Value>, ApiError> {
    let areas = dpn_core::list_areas(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;
    Ok(Json(serde_json::json!({ "areas": areas })))
}
```

### Pattern 3: Dynamic Update Query Pattern (from projects.rs)
**What:** Build UPDATE query dynamically from optional fields to handle partial PATCH.
**When to use:** All PATCH handlers where only some fields may be provided.
**Example:** See `/root/dpn-core/src/db/projects.rs` lines 158-221 for the exact pattern with dynamic param binding.

### Pattern 4: ApiError::Conflict for Business Rule Violations
**What:** Add a `Conflict(String)` variant to ApiError returning HTTP 409.
**When to use:** Frozen resource updates, archive content updates.
**Example:**
```rust
// In /opt/dpn-api/src/error.rs
pub enum ApiError {
    // ... existing variants ...
    Conflict(String),  // NEW: 409 Conflict for business rule violations
}

// In IntoResponse impl:
ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
```

### Anti-Patterns to Avoid
- **Application-only enforcement:** Do NOT rely solely on Rust code to enforce immutability/frozen rules. The DB trigger is the source of truth (D-03, D-05). The API check is a courtesy for better error messages.
- **Compile-time sqlx macros:** Use runtime `sqlx::query()` and `sqlx::query_as()`, not `sqlx::query!()` macro. The existing codebase already uses runtime queries consistently.
- **JSONB for version history:** Do NOT store template versions in a JSONB array (D-07). Use a proper `templates_history` table.
- **Evaluating .dpn expressions:** Templates store .dpn as inert text (D-09). No parsing, no evaluation.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Immutability enforcement | Application-level field blocking | PostgreSQL BEFORE UPDATE trigger with RAISE EXCEPTION | DB-is-the-OS philosophy; all clients get same guarantee |
| Frozen row enforcement | Per-handler frozen checks only | PostgreSQL BEFORE UPDATE trigger + API 409 check | Trigger catches any client (Lisp, Python, direct SQL) |
| Version history | Custom diff/patch system | Simple insert-previous-row-on-change pattern | History table with FK is standard, queryable, simple |
| Dynamic PATCH queries | One query per field combination | Dynamic query builder (see projects.rs pattern) | Already proven in codebase |

## Common Pitfalls

### Pitfall 1: Agent FK is VARCHAR, Not Integer
**What goes wrong:** Assuming agents.id is an integer and using `i32` for the owner FK.
**Why it happens:** Most tables use SERIAL integer PKs. The agents table uses `VARCHAR(64)` for id (e.g., "nova", "kathryn").
**How to avoid:** Use `Option<String>` for owner fields referencing agents. FK constraint: `REFERENCES agents(id)`.
**Warning signs:** Compilation succeeds but INSERT fails with type mismatch.

### Pitfall 2: Trigger Must RETURN NEW or RETURN NULL
**What goes wrong:** Trigger function raises exception but doesn't have a RETURN statement for the non-exception path.
**Why it happens:** PostgreSQL BEFORE UPDATE triggers must return NEW to allow the operation, or NULL to silently cancel. Missing RETURN causes unexpected behavior.
**How to avoid:** Always end trigger functions with `RETURN NEW;` after the guard clause.
**Warning signs:** All updates silently failing.

### Pitfall 3: API Check vs Trigger Race Condition
**What goes wrong:** API checks frozen/immutability in Rust, but between the check and the UPDATE, another client modifies the row.
**Why it happens:** The API check and UPDATE are not atomic.
**How to avoid:** The DB trigger is the real enforcement. The API check is for user-friendly error messages. If the trigger fires, catch the database error and map it to a 409.
**Warning signs:** Intermittent 500 errors on frozen resource updates.

### Pitfall 4: Forgetting updated_at Trigger on New Tables
**What goes wrong:** New tables have `updated_at` column but it never auto-updates.
**Why it happens:** The existing `update_updated_at_column()` trigger function exists but must be explicitly attached to each new table.
**How to avoid:** Add `CREATE TRIGGER update_{table}_updated_at BEFORE UPDATE ON {table} FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();` for each new table.
**Warning signs:** updated_at always equals created_at.

### Pitfall 5: Missing Handler Import in main.rs
**What goes wrong:** New handler module exists but endpoints return 404.
**Why it happens:** Forgot to add `use handlers::areas;` in main.rs or forgot to register routes.
**How to avoid:** After adding handler file: (1) add to handlers/mod.rs, (2) add use statement in main.rs, (3) add .route() calls.
**Warning signs:** Compilation succeeds, curl returns 404.

### Pitfall 6: Catching Trigger Errors in Rust
**What goes wrong:** When the DB trigger raises an exception (e.g., "Archives are immutable"), sqlx returns a generic database error. The handler maps it to 500 instead of 409.
**Why it happens:** sqlx surfaces PostgreSQL RAISE EXCEPTION as `sqlx::Error::Database`. Need to inspect the error message to distinguish business rule violations from real DB errors.
**How to avoid:** In the handler, check if the database error message contains the known trigger message (e.g., "immutable", "frozen") and map to `ApiError::Conflict`. Alternatively, do the check in Rust first (preferred for clean error messages) and let the trigger be the safety net.
**Warning signs:** 500 errors when updating frozen/immutable records instead of 409.

## Code Examples

### SQL Migration: Complete Schema (verified against PostgreSQL 16 and existing patterns)

```sql
-- Source: Derived from ARCHITECTURE.md schemas + REQUIREMENTS.md + CONTEXT.md decisions

-- ============================================================
-- AREAS TABLE
-- ============================================================
CREATE TABLE areas (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL UNIQUE,
    slug VARCHAR(256) NOT NULL UNIQUE,
    description TEXT,
    owner VARCHAR(64) REFERENCES agents(id),
    status VARCHAR(32) DEFAULT 'active'
        CHECK (status IN ('active', 'inactive', 'archived')),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_areas_owner ON areas(owner);
CREATE INDEX idx_areas_status ON areas(status);

CREATE TRIGGER update_areas_updated_at
    BEFORE UPDATE ON areas
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Seed data (D-01, D-02)
INSERT INTO areas (name, slug, description, owner, status) VALUES
    ('EM Corp', 'em-corp', 'Eckenrode Muziekopname corporate operations', 'kathryn', 'active'),
    ('Orbis', 'orbis', 'Orbis worldbuilding and TTRPG content', 'sylvia', 'active'),
    ('Living Room Music', 'living-room-music', 'Music production and composition', 'lrm', 'active'),
    ('N8K99/Personal', 'n8k99-personal', 'Personal projects and content', 'nathan', 'active'),
    ('Infrastructure/Systems', 'infrastructure-systems', 'Technical infrastructure and platform systems', 'nova', 'active');

-- ============================================================
-- ARCHIVES TABLE
-- ============================================================
CREATE TABLE archives (
    id SERIAL PRIMARY KEY,
    title VARCHAR(512),
    content TEXT,
    source_type VARCHAR(128) NOT NULL,
    source_id INTEGER,
    original_path TEXT,
    period_start DATE,
    period_end DATE,
    topic VARCHAR(256),
    tags JSONB DEFAULT '[]',
    metadata JSONB DEFAULT '{}',
    embedding VECTOR(768),
    created_at TIMESTAMPTZ DEFAULT NOW()
    -- No updated_at: archives are immutable
);

CREATE INDEX idx_archives_source_type ON archives(source_type);
CREATE INDEX idx_archives_source ON archives(source_type, source_id);
CREATE INDEX idx_archives_period ON archives(period_start, period_end);
CREATE INDEX idx_archives_topic ON archives(topic);
CREATE INDEX idx_archives_embedding ON archives USING hnsw (embedding vector_cosine_ops);

-- Full-text search (SCHEMA-02)
ALTER TABLE archives ADD COLUMN tsv tsvector
    GENERATED ALWAYS AS (
        to_tsvector('english', coalesce(title, '') || ' ' || coalesce(content, ''))
    ) STORED;
CREATE INDEX idx_archives_fts ON archives USING gin(tsv);

-- Immutability trigger (D-03, D-04)
CREATE OR REPLACE FUNCTION enforce_archive_immutability()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.content IS DISTINCT FROM NEW.content
       OR OLD.title IS DISTINCT FROM NEW.title
       OR OLD.source_type IS DISTINCT FROM NEW.source_type
       OR OLD.source_id IS DISTINCT FROM NEW.source_id
       OR OLD.original_path IS DISTINCT FROM NEW.original_path
       OR OLD.period_start IS DISTINCT FROM NEW.period_start
       OR OLD.period_end IS DISTINCT FROM NEW.period_end THEN
        RAISE EXCEPTION 'Archives are immutable: content fields cannot be updated';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_archive_immutability
    BEFORE UPDATE ON archives
    FOR EACH ROW EXECUTE FUNCTION enforce_archive_immutability();

-- ============================================================
-- RESOURCES TABLE
-- ============================================================
CREATE TABLE resources (
    id SERIAL PRIMARY KEY,
    name VARCHAR(512) NOT NULL,
    slug VARCHAR(512) NOT NULL UNIQUE,
    resource_type VARCHAR(64) NOT NULL,
    source_type VARCHAR(64) NOT NULL,
    source_id INTEGER NOT NULL,
    description TEXT,
    tags JSONB DEFAULT '[]',
    frozen BOOLEAN DEFAULT FALSE,
    area_id INTEGER REFERENCES areas(id),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_resources_type ON resources(resource_type);
CREATE INDEX idx_resources_source ON resources(source_type, source_id);
CREATE INDEX idx_resources_area ON resources(area_id);
CREATE INDEX idx_resources_frozen ON resources(frozen) WHERE frozen = true;

CREATE TRIGGER update_resources_updated_at
    BEFORE UPDATE ON resources
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Frozen enforcement trigger (D-05)
CREATE OR REPLACE FUNCTION enforce_resource_frozen()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.frozen = true THEN
        RAISE EXCEPTION 'Resource is frozen and cannot be updated';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_resource_frozen
    BEFORE UPDATE ON resources
    FOR EACH ROW EXECUTE FUNCTION enforce_resource_frozen();

-- ============================================================
-- TEMPLATES TABLE
-- ============================================================
CREATE TABLE templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    slug VARCHAR(256) NOT NULL UNIQUE,
    category VARCHAR(128),
    description TEXT,
    body TEXT NOT NULL,
    parameters JSONB DEFAULT '[]',
    metadata JSONB DEFAULT '{}',
    version INTEGER DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_templates_category ON templates(category);
CREATE INDEX idx_templates_slug ON templates(slug);

CREATE TRIGGER update_templates_updated_at
    BEFORE UPDATE ON templates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================
-- TEMPLATES HISTORY TABLE (D-07)
-- ============================================================
CREATE TABLE templates_history (
    id SERIAL PRIMARY KEY,
    template_id INTEGER NOT NULL REFERENCES templates(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    body TEXT NOT NULL,
    parameters JSONB DEFAULT '[]',
    changed_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_templates_history_template ON templates_history(template_id);
CREATE INDEX idx_templates_history_version ON templates_history(template_id, version);

-- Version history trigger (D-08)
CREATE OR REPLACE FUNCTION track_template_version()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.body IS DISTINCT FROM NEW.body THEN
        INSERT INTO templates_history (template_id, version, body, parameters, changed_at)
        VALUES (OLD.id, OLD.version, OLD.body, OLD.parameters, NOW());
        NEW.version := OLD.version + 1;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_template_version_history
    BEFORE UPDATE ON templates
    FOR EACH ROW EXECUTE FUNCTION track_template_version();
```

### Rust: ApiError Conflict Variant

```rust
// Source: /opt/dpn-api/src/error.rs (add to existing enum)
pub enum ApiError {
    Database(String),
    NotFound(String),
    BadRequest(String),
    Internal(String),
    Unauthorized(String),
    Conflict(String),  // NEW
}

// In IntoResponse:
ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
```

### Rust: dpn-core Template Update with Version History Detection

```rust
// Application-level approach (recommended over trigger-only for cleaner error messages)
pub async fn update_template(
    pool: &DbPool,
    id: i32,
    name: Option<&str>,
    category: Option<&str>,
    description: Option<&str>,
    body: Option<&str>,
    parameters: Option<&serde_json::Value>,
) -> Result<()> {
    // The DB trigger handles version history automatically on body change
    // Just build the dynamic UPDATE like projects.rs pattern
    let mut query = String::from("UPDATE templates SET ");
    let mut updates = vec![];
    let mut param_idx = 1;
    // ... same dynamic builder pattern as projects.rs ...
}
```

### Rust: dpn-api Handler with Frozen Check

```rust
// Handler pattern for resources PATCH with proactive frozen check
pub async fn update_resource(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateResourceRequest>,
) -> Result<Json<Value>, ApiError> {
    let resource = dpn_core::get_resource_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Resource {} not found", id)))?;

    if resource.frozen {
        return Err(ApiError::Conflict("Resource is frozen and cannot be updated".to_string()));
    }

    dpn_core::update_resource(&pool, id, /* fields */)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    // Fetch and return updated resource
    let updated = dpn_core::get_resource_by_id(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Resource {} not found", id)))?;

    Ok(Json(serde_json::json!(updated)))
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| File-based state | DB-is-the-OS | v1.0 | All state in master_chronicle |
| Manual SQL migrations | Still manual SQL (no ORM framework) | Ongoing | Keep using raw SQL scripts |
| sqlx query!() macros | sqlx runtime queries (query/query_as) | v1.1 | Avoids compile-time DB dependency |

## Open Questions

1. **Generated tsvector column vs manual update**
   - What we know: PostgreSQL 12+ supports `GENERATED ALWAYS AS ... STORED` for tsvector. PG 16 is installed.
   - What's unclear: Whether sqlx handles GENERATED columns gracefully in INSERT/SELECT (the column cannot be in INSERT column lists).
   - Recommendation: Use GENERATED STORED column but exclude `tsv` from the Rust `Archive` struct. Query full-text search with `WHERE tsv @@ to_tsquery(...)` in a separate search function.

2. **Templates version history: trigger vs application logic**
   - What we know: D-08 says "trigger (or application logic)". Both work.
   - What's unclear: Whether to rely solely on the DB trigger or also have explicit application logic in dpn-core.
   - Recommendation: Use the DB trigger (consistent with D-03/D-05 philosophy). The dpn-core update function just does a normal UPDATE; the trigger handles versioning transparently.

## Project Constraints (from CLAUDE.md)

- **Stack**: Rust (dpn-api, dpn-core), PostgreSQL -- no new languages
- **DB is the OS**: All state in master_chronicle. No file-based state.
- **UTF-8 Rule**: Never mix character positions with byte indices in Rust code
- **Naming**: Rust uses snake_case functions, PascalCase structs, `#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]` standard combo
- **Error handling**: `anyhow::Result` in dpn-core, `ApiError` enum in dpn-api
- **Workspace portability**: All tools must use relative paths
- **GSD Workflow**: Use GSD commands for work execution

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (Rust built-in) + raw SQL verification |
| Config file | Cargo.toml (both dpn-core and dpn-api) |
| Quick run command | `cd /root/dpn-core && cargo test` |
| Full suite command | `cd /root/dpn-core && cargo test && cd /opt/dpn-api && cargo build` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| SCHEMA-01 | Areas table exists with correct columns | SQL smoke | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -c "\d areas"` | N/A (SQL) |
| SCHEMA-02 | Archives immutability trigger works | SQL smoke | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -c "UPDATE archives SET content='x' WHERE id=1"` (expect error) | N/A (SQL) |
| SCHEMA-03 | Resources frozen trigger works | SQL smoke | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -c "UPDATE resources SET name='x' WHERE frozen=true"` (expect error) | N/A (SQL) |
| SCHEMA-04 | Templates version history on body change | SQL smoke | Insert template, update body, check templates_history has row | N/A (SQL) |
| API-01 | Areas CRUD endpoints respond correctly | integration/curl | `curl -s http://localhost:8080/api/areas -H "Authorization: Bearer $TOKEN"` | No |
| API-02 | Archives POST works, content UPDATE blocked | integration/curl | `curl -s -X POST http://localhost:8080/api/archives ...` | No |
| API-03 | Resources PATCH returns 409 when frozen | integration/curl | `curl -s -X PATCH http://localhost:8080/api/resources/1 ...` | No |
| API-04 | Templates PATCH triggers version history | integration/curl | Update template body via API, GET templates/:id/history | No |

### Sampling Rate
- **Per task commit:** `cd /root/dpn-core && cargo build && cargo test`
- **Per wave merge:** Full build + SQL trigger verification + curl smoke tests
- **Phase gate:** dpn-api starts cleanly (`pm2 restart dpn-api && curl localhost:8080/health`) + all trigger tests pass

### Wave 0 Gaps
- None for unit tests -- existing cargo test infrastructure covers Rust compilation
- SQL trigger tests will be embedded in the migration script as DO blocks or run manually post-migration
- Integration tests are curl-based (consistent with existing project pattern -- no test harness)

## Sources

### Primary (HIGH confidence)
- Live database schema inspection: `\d agents`, `\d projects`, `\d documents`, `\d media` (2026-03-28)
- dpn-core source: `/root/dpn-core/src/db/projects.rs` (pattern template)
- dpn-api source: `/opt/dpn-api/src/handlers/projects.rs`, `/opt/dpn-api/src/main.rs`, `/opt/dpn-api/src/error.rs`
- PostgreSQL 16 documentation (trigger functions, generated columns)
- `.planning/research/ARCHITECTURE.md` (schema designs)
- `.planning/research/PITFALLS.md` (known risks)

### Secondary (MEDIUM confidence)
- `PARAT-NoosphereSchema.docx` (design intent document)

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - exact same stack as existing codebase, no new dependencies
- Architecture: HIGH - follows established projects module pattern exactly, all schemas verified against live DB
- Pitfalls: HIGH - trigger patterns verified against existing `update_updated_at_column()` function, agent FK type confirmed as VARCHAR(64)

**Research date:** 2026-03-28
**Valid until:** 2026-04-28 (stable domain, no external dependencies changing)
