# Phase 17: Projects & Goals Restructuring - Research

**Researched:** 2026-03-28
**Domain:** PostgreSQL schema migration, Rust struct/handler modification, perception endpoint enrichment
**Confidence:** HIGH

## Summary

This phase adds three columns to existing tables (lifestage on projects, project_id FK on goals, area_id FK on projects), creates one database trigger for forward-only lifestage transitions, migrates 44 goals from text wikilinks to integer FKs, backfills all 15 projects with lifestage and area assignments, and enriches the perception endpoint with two new fields. All modifications target existing tables and code -- no new tables are created.

The schema changes are straightforward ALTER TABLE operations. The main complexity lies in: (1) the forward-only lifestage trigger logic, (2) parsing mixed wikilink formats in goals.project for FK migration, and (3) ensuring both dpn-core copies stay synchronized. Phase 16 established all the patterns needed (triggers, struct additions, handler updates, module sync).

**Primary recommendation:** Execute as a single SQL migration file plus Rust struct/handler updates. The migration should be idempotent (use IF NOT EXISTS / ADD COLUMN IF NOT EXISTS where possible) and include inline verification queries.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Lifestage as VARCHAR with CHECK constraint, values: Seed, Sapling, Tree, Harvest (not PostgreSQL ENUM)
- **D-02:** Backfill mapping: completed->Harvest(id=1), paused->Seed(id=3), long-running active->Tree(id=5,6,7,9,10,12,13,14,16,17,51), recent active->Sapling(id=56,59)
- **D-03:** Forward-only transition enforced by database trigger (not CHECK). Prevents: Harvest->anything, Tree->Seed/Sapling, Sapling->Seed
- **D-04:** Add project_id INTEGER FK to goals (nullable during transition)
- **D-05:** Migration parses [[...]] and {{...}} wikilink formats. DragonPunk maps to id=1. GOTCHA and Puppet Show map to NULL
- **D-06:** Original goals.project text column kept but deprecated (not dropped)
- **D-07:** Add area_id INTEGER FK to projects (nullable for standalone)
- **D-08:** Area assignments by domain (Infrastructure=5, EM Corp=1, Orbis=2, Living Room Music=3, N8K99/Personal=4)
- **D-09:** Perception endpoint adds lifestage and area_name via LEFT JOIN on areas
- **D-10:** No perception endpoint rewrite -- incremental addition only

### Claude's Discretion
- Index choices for new FK columns
- Exact wikilink parsing regex in migration script
- Order of ALTER TABLE statements
- Whether to add a composite index on (area_id, status) for area-based project queries

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SCHEMA-05 | Projects table has lifestage enum column (Seed/Sapling/Tree/Harvest) with forward-only transition constraint and backfilled values for existing 15 projects | SQL migration with VARCHAR + CHECK + trigger; backfill UPDATE statements verified against live project data |
| SCHEMA-06 | Goals table has proper project_id integer FK to projects, migrated from text project field, with all 44 existing goals mapped | SQL migration with ALTER TABLE ADD COLUMN + UPDATE using text parsing; live data shows 4 distinct wikilink values across 44 goals |
| SCHEMA-07 | Projects table has area_id FK to areas table (nullable for standalone projects) | SQL migration with ALTER TABLE ADD COLUMN + FK; areas table confirmed with 5 seeded rows (Phase 16) |
| API-05 | Perception endpoint includes area context and project lifestage in ghost perception responses | Modify af64_perception.rs project query (line 424-449) to LEFT JOIN areas and include lifestage + area_name fields |
</phase_requirements>

## Standard Stack

No new libraries needed. All changes use the existing stack.

### Core (Already Installed)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| sqlx | 0.8 | PostgreSQL queries in Rust | Already used throughout dpn-core and dpn-api |
| serde | 1 | Serialization for struct fields | Already derived on all DB structs |
| axum | 0.7 | HTTP handlers in dpn-api | Already the framework |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| psql | Direct SQL migration execution | Running the 17-*.sql migration file |
| cargo build | Verify Rust compiles after struct changes | After every Rust file edit |

**No installation needed.** This phase modifies existing code only.

## Architecture Patterns

### Migration File Structure
```
migrations/
  16-parat-foundation-tables.sql   (existing, Phase 16)
  17-projects-goals-restructuring.sql  (new, this phase)
```

### Pattern 1: ALTER TABLE + Backfill in Single Transaction
**What:** Wrap all DDL and DML in BEGIN/COMMIT with verification queries
**When to use:** Schema migrations that modify live data
**Example:**
```sql
-- Source: Phase 16 migration pattern
BEGIN;

-- DDL
ALTER TABLE projects ADD COLUMN lifestage VARCHAR(32);

-- CHECK constraint
ALTER TABLE projects ADD CONSTRAINT projects_lifestage_check
    CHECK (lifestage IN ('Seed', 'Sapling', 'Tree', 'Harvest'));

-- Backfill
UPDATE projects SET lifestage = 'Harvest' WHERE id = 1;
UPDATE projects SET lifestage = 'Seed' WHERE id = 3;
-- ... etc

-- Verification
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM projects WHERE lifestage IS NULL) THEN
        RAISE EXCEPTION 'Backfill incomplete: NULL lifestage found';
    END IF;
END $$;

-- After verification, make NOT NULL
ALTER TABLE projects ALTER COLUMN lifestage SET NOT NULL;

COMMIT;
```

### Pattern 2: Forward-Only Trigger (D-03)
**What:** Database trigger that compares OLD and NEW lifestage values using a numeric ordering
**When to use:** Enforcing lifecycle state machines at the DB level
**Example:**
```sql
-- Source: Based on Phase 16 immutability trigger pattern
CREATE OR REPLACE FUNCTION enforce_lifestage_forward_only() RETURNS TRIGGER AS $$
DECLARE
    old_rank INTEGER;
    new_rank INTEGER;
BEGIN
    -- Map lifestage to ordinal rank
    old_rank := CASE OLD.lifestage
        WHEN 'Seed' THEN 1
        WHEN 'Sapling' THEN 2
        WHEN 'Tree' THEN 3
        WHEN 'Harvest' THEN 4
    END;
    new_rank := CASE NEW.lifestage
        WHEN 'Seed' THEN 1
        WHEN 'Sapling' THEN 2
        WHEN 'Tree' THEN 3
        WHEN 'Harvest' THEN 4
    END;

    IF new_rank < old_rank THEN
        RAISE EXCEPTION 'Lifestage transition not allowed: % -> % (forward only)',
            OLD.lifestage, NEW.lifestage;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER enforce_project_lifestage
    BEFORE UPDATE ON projects
    FOR EACH ROW
    WHEN (OLD.lifestage IS DISTINCT FROM NEW.lifestage)
    EXECUTE FUNCTION enforce_lifestage_forward_only();
```

### Pattern 3: Wikilink Text-to-FK Migration (D-05)
**What:** Parse wikilink text and map to integer FK
**When to use:** Migrating denormalized text references to proper FKs
**Example:**
```sql
-- Goals data observed in live DB:
--   {{"Project DragonPunk"}} -> 27 rows -> project_id = 1
--   [[Project DragonPunk]]   ->  5 rows -> project_id = 1
--   [[Project GOTCHA]]       -> 11 rows -> project_id = NULL (deprecated)
--   [[Project Puppet Show]]  ->  1 row  -> project_id = NULL (no match)

ALTER TABLE goals ADD COLUMN project_id INTEGER REFERENCES projects(id);

UPDATE goals SET project_id = 1
    WHERE project IN ('[[Project DragonPunk]]', '{{"Project DragonPunk"}}');
-- GOTCHA and Puppet Show: leave project_id NULL per D-05
```

### Pattern 4: Perception Query Enrichment (D-09)
**What:** Add LEFT JOIN to existing perception query, include new fields in JSON output
**When to use:** Incremental perception endpoint additions
**Example:**
```sql
-- Current query (line 424):
SELECT p.id, p.name, p.status, p.description, ...
FROM projects p
WHERE p.owner = $1 AND p.status = 'active'

-- Modified query:
SELECT p.id, p.name, p.status, p.description, p.lifestage,
       a.name as area_name,
       ...
FROM projects p
LEFT JOIN areas a ON p.area_id = a.id
WHERE p.owner = $1 AND p.status = 'active'
```

### Pattern 5: Rust Struct + Dynamic Update Builder (existing pattern)
**What:** Add optional fields to Project struct, extend dynamic query builder
**When to use:** When adding nullable columns to existing CRUD
**Example:**
```rust
// Source: dpn-core/src/db/projects.rs existing pattern
pub struct Project {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub status: String,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub schedule: Option<serde_json::Value>,
    pub lifestage: Option<String>,   // NEW
    pub area_id: Option<i32>,        // NEW
}
```

### Anti-Patterns to Avoid
- **Using PostgreSQL ENUM type:** Decision D-01 explicitly chose VARCHAR + CHECK for extensibility. Do NOT create a custom ENUM type.
- **Application-level lifestage enforcement:** Decision D-03 explicitly chose DB trigger. Do NOT add Rust-side transition logic as the primary enforcement.
- **Dropping goals.project column:** Decision D-06 explicitly keeps it for backward compatibility.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Lifestage transition logic | Rust-side state machine | PostgreSQL trigger | DB-is-the-OS philosophy; all clients get same guarantee |
| Wikilink parsing | Complex regex in Rust | Simple SQL string matching | Only 4 distinct values exist; exact match is sufficient |
| Updated_at management | Application-level timestamps | Existing `update_updated_at_column()` trigger | Already exists and is reusable |

**Key insight:** The goals migration does NOT need regex. There are exactly 4 distinct text values in the live data. Use exact string matching (`IN (...)`) rather than regex pattern extraction.

## Common Pitfalls

### Pitfall 1: Forgetting to Sync Both dpn-core Copies
**What goes wrong:** Edit `/root/dpn-core/src/db/projects.rs` but forget `/opt/dpn-core/src/db/projects.rs`, causing dpn-api build failure
**Why it happens:** Two copies of dpn-core exist with different dependency versions (STATE.md Phase 16 decision)
**How to avoid:** After every edit to `/root/dpn-core/src/db/projects.rs`, immediately copy to `/opt/dpn-core/src/db/projects.rs`
**Warning signs:** `cargo build` in `/opt/dpn-api` fails with missing field errors

### Pitfall 2: SELECT Column List Mismatch After ALTER TABLE
**What goes wrong:** Rust queries use explicit column lists (e.g., `SELECT id, name, slug, status, description, owner, schedule FROM projects`). After adding lifestage and area_id columns, all queries must be updated or FromRow derivation will fail.
**Why it happens:** Every query in projects.rs uses explicit SELECT (not SELECT *)
**How to avoid:** Grep for all `FROM projects` in dpn-core and dpn-api, update each SELECT list
**Warning signs:** sqlx runtime errors about missing columns

### Pitfall 3: NULL Handling in Perception JSON
**What goes wrong:** Lifestage is NOT NULL after backfill, but area_id is nullable. LEFT JOIN may produce NULL area_name.
**Why it happens:** Not all projects have an area assignment
**How to avoid:** Use `r.get::<Option<String>, _>("area_name")` in the perception handler, not `r.get::<String, _>()`
**Warning signs:** Runtime panic on perception endpoint for projects without area_id

### Pitfall 4: Trigger Firing During Backfill
**What goes wrong:** The lifestage forward-only trigger fires during the backfill UPDATE statements
**Why it happens:** Trigger is created before backfill runs
**How to avoid:** Create the trigger AFTER the backfill is complete, within the same transaction. Or: backfill first (when column has no trigger), then create trigger.
**Warning signs:** Backfill UPDATE fails with "forward only" exception

### Pitfall 5: UpdateProjectRequest Missing New Fields
**What goes wrong:** API handler `UpdateProjectRequest` in `/opt/dpn-api/src/handlers/projects.rs` doesn't include lifestage or area_id, so updates through the API cannot set these fields
**Why it happens:** Handler struct not updated alongside dpn-core struct
**How to avoid:** Add `lifestage: Option<String>` and `area_id: Option<i32>` to `UpdateProjectRequest` and the dynamic query builder
**Warning signs:** PATCH /api/projects/:id silently ignores lifestage/area_id fields

## Code Examples

### Complete Migration SQL Structure
```sql
-- Source: Phase 16 migration pattern + CONTEXT.md decisions
BEGIN;

-- 1. Add lifestage to projects (SCHEMA-05, D-01)
ALTER TABLE projects ADD COLUMN IF NOT EXISTS lifestage VARCHAR(32);
ALTER TABLE projects ADD CONSTRAINT projects_lifestage_check
    CHECK (lifestage IN ('Seed', 'Sapling', 'Tree', 'Harvest'));

-- 2. Backfill lifestage (D-02)
UPDATE projects SET lifestage = 'Harvest' WHERE id = 1;
UPDATE projects SET lifestage = 'Seed' WHERE id = 3;
UPDATE projects SET lifestage = 'Tree' WHERE id IN (5,6,7,9,10,12,13,14,16,17,51);
UPDATE projects SET lifestage = 'Sapling' WHERE id IN (56, 59);

-- Verify all backfilled
DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM projects WHERE lifestage IS NULL) THEN
        RAISE EXCEPTION 'Lifestage backfill incomplete';
    END IF;
END $$;
ALTER TABLE projects ALTER COLUMN lifestage SET NOT NULL;

-- 3. Forward-only trigger (D-03) -- AFTER backfill
CREATE OR REPLACE FUNCTION enforce_lifestage_forward_only() RETURNS TRIGGER AS $$ ... $$;
CREATE TRIGGER enforce_project_lifestage BEFORE UPDATE ON projects
    FOR EACH ROW WHEN (OLD.lifestage IS DISTINCT FROM NEW.lifestage)
    EXECUTE FUNCTION enforce_lifestage_forward_only();

-- 4. Add area_id FK to projects (SCHEMA-07, D-07)
ALTER TABLE projects ADD COLUMN IF NOT EXISTS area_id INTEGER REFERENCES areas(id);
CREATE INDEX IF NOT EXISTS idx_projects_area_id ON projects(area_id);

-- 5. Backfill area assignments (D-08)
UPDATE projects SET area_id = 5 WHERE id IN (3, 51, 13, 5);    -- Infrastructure/Systems
UPDATE projects SET area_id = 1 WHERE id IN (10, 12, 14, 6);   -- EM Corp
UPDATE projects SET area_id = 2 WHERE id IN (16, 17, 9, 1);    -- Orbis
UPDATE projects SET area_id = 3 WHERE id = 7;                   -- Living Room Music
UPDATE projects SET area_id = 4 WHERE id IN (56, 59);           -- N8K99/Personal

-- 6. Add project_id FK to goals (SCHEMA-06, D-04)
ALTER TABLE goals ADD COLUMN IF NOT EXISTS project_id INTEGER REFERENCES projects(id);
CREATE INDEX IF NOT EXISTS idx_goals_project_id ON goals(project_id);

-- 7. Migrate wikilinks to FK (D-05)
UPDATE goals SET project_id = 1
    WHERE project IN ('[[Project DragonPunk]]', '{{"Project DragonPunk"}}');

-- 8. Verify migration (all DragonPunk goals mapped)
DO $$ BEGIN
    IF (SELECT COUNT(*) FROM goals WHERE project LIKE '%DragonPunk%' AND project_id IS NULL) > 0 THEN
        RAISE EXCEPTION 'Goals migration incomplete: DragonPunk goals without project_id';
    END IF;
END $$;

COMMIT;
```

### Perception Endpoint Modification
```rust
// Source: /opt/dpn-api/src/handlers/af64_perception.rs lines 424-449
// Modified query with lifestage and area_name
let project_rows = sqlx::query(
    r#"SELECT p.id, p.name, p.status, p.description, p.goals, p.blockers,
              p.current_context, p.schedule, p.lifestage,
              a.name as area_name,
              (SELECT COUNT(*) FROM tasks t WHERE t.project_id = p.id
               AND t.status IN ('open', 'pending', 'in-progress')) as open_tasks,
              (SELECT COUNT(*) FROM tasks t WHERE t.project_id = p.id
               AND t.status IN ('done', 'completed')) as completed_tasks
       FROM projects p
       LEFT JOIN areas a ON p.area_id = a.id
       WHERE p.owner = $1 AND p.status = 'active'
       ORDER BY p.updated_at DESC"#
)
.bind(&agent_id)
.fetch_all(&pool).await.unwrap_or_default();

// In the JSON builder, add:
"lifestage": r.get::<String, _>("lifestage"),
"area_name": r.get::<Option<String>, _>("area_name"),
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| goals.project as wikilink text | goals.project_id as integer FK | This phase | Proper relational integrity, queryable joins |
| Projects have flat status only | Projects have lifestage + status | This phase | Growth lifecycle tracking for ghost portfolio awareness |
| Projects unlinked to areas | Projects have area_id FK | This phase | Area-based project queries, org domain grouping |

## Open Questions

1. **Should lifestage allow skipping stages (e.g., Seed -> Tree)?**
   - What we know: D-03 says "forward only" which implies Seed->Tree is valid (it's forward)
   - What's unclear: Whether Nathan wants strictly sequential (Seed->Sapling->Tree->Harvest) or just forward (any higher rank)
   - Recommendation: Implement as "any forward" per D-03 wording. Strictly sequential would need different trigger logic.

2. **Should existing trigger `update_updated_at_column` be added to projects?**
   - What we know: Projects table does NOT currently have this trigger (verified). Areas table does have it.
   - What's unclear: Whether this is intentional or an oversight
   - Recommendation: Add it. Consistent with Phase 16 pattern and the projects table has an updated_at column.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | SQL verification queries + cargo build + cargo test |
| Config file | None (SQL run via psql, Rust via cargo) |
| Quick run command | `PGPASSWORD=chronicle2026 psql -U chronicle -h 127.0.0.1 -d master_chronicle -f migrations/17-projects-goals-restructuring.sql` |
| Full suite command | `cd /root/dpn-core && cargo test && cd /opt/dpn-api && cargo build` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| SCHEMA-05 | Lifestage column exists, backfilled, NOT NULL, forward-only trigger works | SQL verification | `psql -c "SELECT id, lifestage FROM projects ORDER BY id"` + trigger violation test | N/A (Wave 0) |
| SCHEMA-06 | project_id FK on goals, all DragonPunk goals mapped | SQL verification | `psql -c "SELECT COUNT(*) FROM goals WHERE project LIKE '%DragonPunk%' AND project_id IS NULL"` (must return 0) | N/A (Wave 0) |
| SCHEMA-07 | area_id FK on projects, backfilled | SQL verification | `psql -c "SELECT id, name, area_id FROM projects ORDER BY id"` | N/A (Wave 0) |
| API-05 | Perception includes lifestage and area_name | smoke | `curl localhost:8080/api/perception/nova \| jq '.projects[0] \| {lifestage, area_name}'` | N/A (Wave 0) |

### Sampling Rate
- **Per task commit:** `cargo build` in both /root/dpn-core and /opt/dpn-api
- **Per wave merge:** Full SQL verification + `cargo test` + perception curl test
- **Phase gate:** All 4 requirement verification queries pass

### Wave 0 Gaps
- [ ] Trigger violation test SQL (attempt backward transition, verify exception)
- [ ] Orphan goal verification query (LEFT JOIN goals on projects returning no NULLs for DragonPunk)
- [ ] Perception endpoint smoke test script

## Sources

### Primary (HIGH confidence)
- Live database inspection via psql -- projects table schema, goals data, areas table, existing triggers
- `/root/dpn-core/src/db/projects.rs` -- current Project struct and CRUD functions
- `/opt/dpn-api/src/handlers/af64_perception.rs` lines 423-509 -- current perception endpoint
- `/opt/dpn-api/src/handlers/projects.rs` -- current project handlers
- `/root/migrations/16-parat-foundation-tables.sql` -- Phase 16 trigger and migration patterns

### Secondary (MEDIUM confidence)
- `/root/.planning/phases/16-foundation-tables-api/16-CONTEXT.md` -- Phase 16 patterns and decisions
- `/root/.planning/STATE.md` -- dpn-core sync decision context

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - no new libraries, verified existing code
- Architecture: HIGH - all patterns observed in Phase 16, live DB schema verified
- Pitfalls: HIGH - identified from direct code inspection (two dpn-core copies, explicit SELECT lists, trigger ordering)

**Research date:** 2026-03-28
**Valid until:** 2026-04-28 (stable -- schema migration, no external dependencies)
