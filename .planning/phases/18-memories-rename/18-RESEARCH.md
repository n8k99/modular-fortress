# Phase 18: Memories Rename - Research

**Researched:** 2026-03-28
**Domain:** PostgreSQL table rename with view bridge, Rust module rename, department normalization
**Confidence:** HIGH

## Summary

Phase 18 renames the `vault_notes` table (2,719 rows, 75 columns including 64 per-agent memory columns) to `memories`, adds compression metadata columns, normalizes departments via lookup table, and migrates all Rust code from vault_notes/VaultNote to memories/Memory. The table has zero inbound foreign keys, which makes the rename straightforward from a schema dependency perspective. The critical risk factor is the blast radius across 4 codebases (dpn-core: 15 files, dpn-api: 4 files, Lisp: 2 files, Python: 1 helper).

All Rust SQL queries use runtime `sqlx::query()` / `sqlx::query_as()` -- never compile-time `sqlx::query!()`. This eliminates the major pitfall identified in PITFALLS.md regarding sqlx compile-time query checking vs. view RULES. The view bridge only needs to support the Lisp tick engine (which calls a Python helper with raw SQL) and the dpn-api perception endpoint's raw SQL (which gets migrated to `memories` directly). The Python helper `write_vault_memory.py` queries `information_schema.columns` for `table_name = 'vault_notes'` -- this works against views since PostgreSQL includes views in information_schema.columns.

**Primary recommendation:** Execute in strict order: (1) add columns to vault_notes, (2) backfill data, (3) rename table + create view + INSTEAD OF triggers, (4) migrate trigger, (5) create departments table + backfill, (6) rename Rust module + all references, (7) build + test, (8) sync to /opt + rebuild dpn-api.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01: ALTER TABLE vault_notes RENAME TO memories. Then CREATE VIEW vault_notes AS SELECT * FROM memories.
- D-02: View must support INSERT/UPDATE/DELETE for Lisp and Python backward compatibility. Use INSTEAD OF triggers (not RULES) on the view.
- D-03: Existing trigger `trg_sync_task_checkbox` must be recreated on the `memories` table. The trigger function does not reference the table name internally.
- D-04: All Rust code migrates to reference `memories` table directly. The vault_notes view is ONLY for Lisp tick engine and Python gotcha-workspace tools.
- D-05: Add `compression_tier` column as VARCHAR with CHECK constraint for values: daily, weekly, monthly, quarterly, yearly. NOT NULL with default 'daily'.
- D-06: Backfill compression_tier from existing note_type mapping (daily->daily, weekly->weekly, monthly->monthly, quarterly->quarterly, yearly->yearly, agent_memory->daily, freeform->daily, zenwriter->daily, NULL->daily).
- D-07: Add `compressed_from INTEGER[]` column (nullable).
- D-08: Create `departments` lookup table with: id SERIAL, name VARCHAR(64) UNIQUE, slug VARCHAR(64) UNIQUE, description TEXT, created_at TIMESTAMPTZ.
- D-09: Canonical department names mapped from 19 existing values to 8 canonical departments.
- D-10: Add `department_id INTEGER REFERENCES departments(id)` to agents table. Backfill from mapping. Keep original `department` text column.
- D-11: Rename dpn-core module: vault_notes.rs -> memories.rs. Rename struct: VaultNote -> Memory. Rename all query functions.
- D-12: Update dpn-core mod.rs and lib.rs re-exports.
- D-13: Rename dpn-api handler internal references from vault_notes to memories.
- D-14: Both dpn-core copies (/root and /opt) must be synced after changes.

### Claude's Discretion
- Index migration strategy (indexes on vault_notes auto-rename with table)
- Whether to rename dpn-api handler file from documents.rs to memories.rs (or just rename internal references)
- Order of migration steps
- Whether to add updated_at trigger to memories table

### Deferred Ideas (OUT OF SCOPE)
None.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| MEM-01 | vault_notes table renamed to memories with VIEW bridge preserving backward compatibility | View bridge + INSTEAD OF triggers pattern documented; trigger function body verified (no table name refs); zero FK dependencies confirmed |
| MEM-02 | memories table has compression_tier enum backfilled from note_type | All 9 note_type values mapped; 2,719 rows with distribution documented; CHECK constraint pattern ready |
| MEM-03 | memories table has compressed_from INTEGER[] column | Simple nullable array column addition; no backfill needed |
| MEM-04 | Departments normalized via lookup table with FK from agents | 19 distinct department values mapped to 8 canonical entries; agents table structure verified; 64 total agents across 19 departments |
| MEM-05 | All dpn-api Rust endpoints updated from vault_notes to memories | 4 files identified: documents.rs (main handler), af64_perception.rs (raw SQL), timeline.rs (comment only), documents.rs.bak (delete) |
| MEM-06 | All dpn-core Rust queries updated from vault_notes to memories | 15 files identified with exact reference locations; module rename + struct rename + re-export updates all scoped |
</phase_requirements>

## Standard Stack

No new libraries needed. This phase uses existing stack exclusively:

### Core (Already Installed)
| Library | Version | Purpose | Role in Phase |
|---------|---------|---------|---------------|
| sqlx | 0.8 | PostgreSQL driver | All DB queries use runtime sqlx::query() -- no compile-time macros |
| PostgreSQL | 14+ | Database | ALTER TABLE, CREATE VIEW, INSTEAD OF triggers |
| psycopg2 | (Python) | Direct DB access | Used by write_vault_memory.py (Lisp helper) |

### Discretion Recommendation: Handler File Rename
Do NOT rename `documents.rs` to `memories.rs` in dpn-api. The handler file serves both vault_notes/memories AND legacy documents table queries. Renaming the file would be misleading since it still handles `documents::list_canonical`. Only rename internal variable names and dpn-core call paths.

### Discretion Recommendation: Index Migration
PostgreSQL automatically renames indexes when the table is renamed. The 5 indexes will become:
- `vault_notes_pkey` stays as-is (PG does NOT auto-rename index names)
- All other indexes keep their original names

Recommendation: Manually rename indexes to match new table name for clarity:
```sql
ALTER INDEX vault_notes_pkey RENAME TO memories_pkey;
ALTER INDEX vault_notes_path_key RENAME TO memories_path_key;
ALTER INDEX idx_vault_notes_date RENAME TO idx_memories_date;
ALTER INDEX idx_vault_notes_type RENAME TO idx_memories_type;
ALTER INDEX idx_vault_notes_embedding RENAME TO idx_memories_embedding;
```

### Discretion Recommendation: Migration Order
Add columns BEFORE rename (simpler -- no view complications during column addition):
1. Add compression_tier + compressed_from columns to vault_notes
2. Backfill compression_tier
3. Rename table to memories
4. Create vault_notes view + INSTEAD OF triggers
5. Migrate trigger
6. Create departments table + backfill
7. Rust code migration
8. Build + sync

### Discretion Recommendation: updated_at Trigger
Skip. Adding an updated_at trigger is a nice-to-have but out of phase scope. The existing modified_at column serves this purpose and is set explicitly in Rust code.

## Architecture Patterns

### Database Migration Pattern
```sql
-- Phase 18 migration must run as postgres user (table owned by postgres)
-- Step 1: Add columns to existing table
ALTER TABLE vault_notes ADD COLUMN compression_tier VARCHAR(32) NOT NULL DEFAULT 'daily'
    CHECK (compression_tier IN ('daily', 'weekly', 'monthly', 'quarterly', 'yearly'));
ALTER TABLE vault_notes ADD COLUMN compressed_from INTEGER[];

-- Step 2: Backfill compression_tier from note_type
UPDATE vault_notes SET compression_tier = CASE
    WHEN note_type = 'daily' THEN 'daily'
    WHEN note_type = 'weekly' THEN 'weekly'
    WHEN note_type = 'monthly' THEN 'monthly'
    WHEN note_type = 'quarterly' THEN 'quarterly'
    WHEN note_type = 'yearly' THEN 'yearly'
    WHEN note_type = 'agent_memory' THEN 'daily'
    WHEN note_type = 'freeform' THEN 'daily'
    WHEN note_type = 'zenwriter' THEN 'daily'
    WHEN note_type IS NULL THEN 'daily'
    ELSE 'daily'
END;

-- Step 3: Rename table
ALTER TABLE vault_notes RENAME TO memories;

-- Step 4: Rename indexes
ALTER INDEX vault_notes_pkey RENAME TO memories_pkey;
ALTER INDEX vault_notes_path_key RENAME TO memories_path_key;
ALTER INDEX idx_vault_notes_date RENAME TO idx_memories_date;
ALTER INDEX idx_vault_notes_type RENAME TO idx_memories_type;
ALTER INDEX idx_vault_notes_embedding RENAME TO idx_memories_embedding;

-- Step 5: Rename sequence
ALTER SEQUENCE vault_notes_id_seq RENAME TO memories_id_seq;

-- Step 6: Migrate trigger (drop old, create on new table)
DROP TRIGGER trg_sync_task_checkbox ON memories;
CREATE TRIGGER trg_sync_task_checkbox
    AFTER UPDATE ON memories
    FOR EACH ROW EXECUTE FUNCTION sync_task_checkbox();

-- Step 7: Create backward-compatible view
CREATE VIEW vault_notes AS SELECT * FROM memories;

-- Step 8: INSTEAD OF triggers for INSERT/UPDATE/DELETE on view
CREATE OR REPLACE FUNCTION vault_notes_insert_fn() RETURNS trigger AS $$
BEGIN
    INSERT INTO memories VALUES (NEW.*);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION vault_notes_update_fn() RETURNS trigger AS $$
BEGIN
    UPDATE memories SET
        path = NEW.path,
        title = NEW.title,
        frontmatter = NEW.frontmatter,
        content = NEW.content,
        size_bytes = NEW.size_bytes,
        note_type = NEW.note_type,
        note_date = NEW.note_date,
        embedding = NEW.embedding,
        created_at = NEW.created_at,
        modified_at = NEW.modified_at,
        compression_tier = NEW.compression_tier,
        compressed_from = NEW.compressed_from
        -- 64 agent memory columns handled by dynamic column list
    WHERE id = OLD.id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION vault_notes_delete_fn() RETURNS trigger AS $$
BEGIN
    DELETE FROM memories WHERE id = OLD.id;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER vault_notes_insert
    INSTEAD OF INSERT ON vault_notes
    FOR EACH ROW EXECUTE FUNCTION vault_notes_insert_fn();

CREATE TRIGGER vault_notes_update
    INSTEAD OF UPDATE ON vault_notes
    FOR EACH ROW EXECUTE FUNCTION vault_notes_update_fn();

CREATE TRIGGER vault_notes_delete
    INSTEAD OF DELETE ON vault_notes
    FOR EACH ROW EXECUTE FUNCTION vault_notes_delete_fn();
```

**CRITICAL: The UPDATE trigger function must list ALL 75 columns explicitly.** The 64 agent memory columns must be included. Generate this dynamically from `information_schema.columns` or write it out completely.

### Rust Module Rename Pattern
```
dpn-core/src/db/
  vault_notes.rs  ->  memories.rs
  mod.rs          ->  update: pub mod memories; (was pub mod vault_notes)

dpn-core/src/lib.rs:
  pub use db::memories::{Memory, MemoryLight};  (was VaultNote, VaultNoteLight)
```

### Struct Renames
```rust
// OLD
pub struct VaultNote { ... }
pub struct VaultNoteLight { ... }

// NEW
pub struct Memory { ... }
pub struct MemoryLight { ... }
```

### Function Renames (dpn-core/src/db/memories.rs)
| Old | New |
|-----|-----|
| get_count | get_count (unchanged) |
| list_light | list_light (unchanged) |
| list_all_light | list_all_light (unchanged) |
| list_with_content | list_with_content (unchanged) |
| get_by_path | get_by_path (unchanged) |
| get_by_id | get_by_id (unchanged) |
| search | search (unchanged) |
| get_daily_note | get_daily_note (unchanged) |
| list_by_type | list_by_type (unchanged) |
| list_by_date_range | list_by_date_range (unchanged) |
| list_modified_since | list_modified_since (unchanged) |
| update_content | update_content (unchanged) |
| create | create (unchanged) |

Function names stay the same -- only the module path changes from `vault_notes::get_by_id` to `memories::get_by_id`. SQL strings inside each function change from `FROM vault_notes` to `FROM memories`.

### Anti-Patterns to Avoid
- **Partial rename:** Renaming the module but leaving some SQL strings as `FROM vault_notes`. Every SQL string in the Rust code must reference `memories` directly.
- **Forgetting the sequence:** The `vault_notes_id_seq` sequence must be renamed to `memories_id_seq`, otherwise the DEFAULT on the id column references the old sequence name (which still works but is confusing).
- **Building dpn-api before syncing dpn-core:** dpn-api depends on `/opt/dpn-core`. If you edit `/root/dpn-core` but build dpn-api before syncing to `/opt/dpn-core`, the build uses the old module names.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| View INSERT/UPDATE/DELETE | Custom application-level routing | INSTEAD OF triggers | PostgreSQL-native, no application code changes for Lisp/Python |
| Column enumeration for UPDATE trigger | Manual 75-column listing | Generate from information_schema | 64 agent columns are error-prone to list manually |
| Department mapping | Lookup hash in app code | SQL CASE statement in migration | One-time backfill, not ongoing logic |

## Runtime State Inventory

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | vault_notes table: 2,719 rows, 75 columns (11 base + 64 agent memory). Sequence: vault_notes_id_seq. No inbound FKs. | Table rename + sequence rename via migration SQL |
| Live service config | dpn-api (PM2): queries vault_notes in af64_perception.rs raw SQL (line 481). Python helper write_vault_memory.py queries vault_notes directly. | Code update for dpn-api; Python helper works via view (verified: information_schema.columns includes views) |
| OS-registered state | None -- no OS-level registrations reference vault_notes |  |
| Secrets/env vars | None -- no env vars reference vault_notes by name |  |
| Build artifacts | dpn-api release binary at /opt/dpn-api/target/release/dpn-api runs old code until rebuilt. /opt/dpn-core must be synced before rebuild. | Sync dpn-core to /opt, rebuild dpn-api release, restart PM2 |

**The canonical question:** After every file in the repo is updated, what runtime systems still have the old string cached, stored, or registered?

1. **dpn-api PM2 process** -- runs compiled binary with old SQL strings. Must rebuild release and `pm2 restart dpn-api`.
2. **write_vault_memory.py** -- references `vault_notes` in SQL. Works via view bridge (verified). No code change needed.
3. **Lisp onboard.lisp** -- default string `"vault_notes"` in prompt. Works via view. Cosmetic only.
4. **SQLite cache** (`~/.dpn/cache.db`) -- has local `vault_notes` table. NOT updated by this phase (cache is offline-first mirror). Acceptable drift.

## Common Pitfalls

### Pitfall 1: INSTEAD OF UPDATE Trigger Must List All 75 Columns
**What goes wrong:** The INSTEAD OF UPDATE trigger on the vault_notes view must explicitly SET every column on the memories table. If any of the 64 agent memory columns are omitted, UPDATEs via the view silently lose data in those columns.
**Why it happens:** PostgreSQL INSTEAD OF triggers replace the default UPDATE behavior entirely. The trigger function must manually map NEW.column to the underlying table for every column.
**How to avoid:** Generate the column list from the database schema. Query: `SELECT column_name FROM information_schema.columns WHERE table_name = 'memories' ORDER BY ordinal_position;` and build the SET clause programmatically. Alternatively, use `INSERT INTO memories SELECT NEW.*` pattern for the INSERT trigger, and for UPDATE use a dynamic approach.
**Warning signs:** Ghost memory columns showing NULL after a write via the view.

### Pitfall 2: Table Owned by postgres, Not chronicle
**What goes wrong:** Running migration as `chronicle` user fails with permission denied on ALTER TABLE.
**Why it happens:** vault_notes table is owned by `postgres` user. The `chronicle` user has DML privileges but not DDL.
**How to avoid:** Run migration SQL as postgres user: `sudo -u postgres psql -d master_chronicle -f migration.sql`.
**Warning signs:** "ERROR: must be owner of table vault_notes"

### Pitfall 3: dpn-api Build Depends on /opt/dpn-core, Not /root/dpn-core
**What goes wrong:** After renaming vault_notes.rs to memories.rs in /root/dpn-core, dpn-api fails to compile because it depends on /opt/dpn-core which still has the old module name.
**Why it happens:** dpn-api's Cargo.toml references a local path to /opt/dpn-core. The two copies have slightly different features (root has conversations module, opt does not).
**How to avoid:** Sync /root/dpn-core to /opt/dpn-core BEFORE attempting dpn-api build. Use the Phase 17 sync pattern.
**Warning signs:** `cargo check` in /opt/dpn-api reports "unresolved import dpn_core::db::memories"

### Pitfall 4: Trigger Migration Timing
**What goes wrong:** If the table rename happens while ghosts are writing memories, the trg_sync_task_checkbox trigger briefly doesn't fire.
**Why it happens:** The trigger follows the table rename automatically in PostgreSQL (it stays attached to the underlying table OID). However, if you DROP and recreate the trigger, there's a window where it's missing.
**How to avoid:** Do NOT drop and recreate the trigger. PostgreSQL keeps triggers attached through table renames. Just verify it's still there after rename: `SELECT tgname FROM pg_trigger WHERE tgrelid = 'memories'::regclass AND NOT tgisinternal;`
**Warning signs:** Task checkbox syncing stops working after migration.

### Pitfall 5: Re-export Cascade in lib.rs
**What goes wrong:** Renaming `pub use db::vault_notes::{VaultNote, VaultNoteLight}` to `pub use db::memories::{Memory, MemoryLight}` breaks every external consumer that imports these types.
**Why it happens:** lib.rs re-exports are the public API. dpn-api imports `dpn_core::VaultNote` and `dpn_core::db::vault_notes::*`.
**How to avoid:** Update ALL import sites in dpn-api simultaneously with the dpn-core rename. Search for `VaultNote`, `VaultNoteLight`, `vault_notes` across all dpn-api source files.
**Warning signs:** Compilation errors listing every file that imports the old names.

### Pitfall 6: af64_perception.rs Raw SQL Not Via dpn-core
**What goes wrong:** The perception handler builds SQL with string interpolation: `FROM vault_notes WHERE...`. This is NOT routed through dpn-core and will be missed if you only rename the dpn-core module.
**Why it happens:** The perception endpoint was written with inline SQL for performance (dynamic column names can't use parameterized queries).
**How to avoid:** Explicitly update af64_perception.rs line 481 from `FROM vault_notes` to `FROM memories`. Search all dpn-api handler files for raw `vault_notes` strings.
**Warning signs:** Ghost perception returns empty memory context after migration.

## Code Examples

### INSTEAD OF INSERT Trigger (Handles All Columns)
```sql
-- Source: PostgreSQL 14 docs on INSTEAD OF triggers
-- Uses INSERT INTO ... SELECT to avoid listing all 75 columns
CREATE OR REPLACE FUNCTION vault_notes_insert_fn() RETURNS trigger AS $$
BEGIN
    INSERT INTO memories
    SELECT NEW.*;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

### INSTEAD OF UPDATE Trigger (Must Be Explicit)
```sql
-- NOTE: UPDATE cannot use SELECT NEW.* pattern
-- Must list all columns explicitly. Generate from:
-- SELECT string_agg(column_name || ' = NEW.' || column_name, ', ')
-- FROM information_schema.columns WHERE table_name = 'vault_notes';
CREATE OR REPLACE FUNCTION vault_notes_update_fn() RETURNS trigger AS $$
BEGIN
    UPDATE memories SET
        path = NEW.path,
        title = NEW.title,
        -- ... all 75 columns ...
    WHERE id = OLD.id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

### Department Normalization SQL
```sql
CREATE TABLE departments (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) UNIQUE NOT NULL,
    slug VARCHAR(64) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

INSERT INTO departments (name, slug) VALUES
    ('Operations', 'operations'),
    ('Engineering', 'engineering'),
    ('Content & Brand', 'content-brand'),
    ('Creative', 'creative'),
    ('Legal', 'legal'),
    ('Music', 'music'),
    ('Strategy', 'strategy'),
    ('Executive', 'executive');

ALTER TABLE agents ADD COLUMN department_id INTEGER REFERENCES departments(id);

UPDATE agents SET department_id = (
    SELECT id FROM departments WHERE slug = CASE agents.department
        WHEN 'Operations' THEN 'operations'
        WHEN 'Systems' THEN 'operations'
        WHEN 'support' THEN 'operations'
        WHEN 'Engineering' THEN 'engineering'
        WHEN 'Content & Brand' THEN 'content-brand'
        WHEN 'content_brand' THEN 'content-brand'
        WHEN 'Creative' THEN 'creative'
        WHEN 'art' THEN 'creative'
        WHEN 'Legal' THEN 'legal'
        WHEN 'legal' THEN 'legal'
        WHEN 'music' THEN 'music'
        WHEN 'Research' THEN 'music'
        WHEN 'strategic_office' THEN 'strategy'
        WHEN 'audience_experience' THEN 'strategy'
        WHEN 'social_impact' THEN 'strategy'
        WHEN 'digital_partnership' THEN 'strategy'
        WHEN 'Executive' THEN 'executive'
        WHEN 'Office of the CEO' THEN 'executive'
        WHEN 'cross_functional' THEN 'executive'
    END
);
```

### Rust Struct Rename (memories.rs)
```rust
// Source: existing vault_notes.rs, renamed
//! memories table access (renamed from vault_notes)

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Memory {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub frontmatter: Option<String>,
    pub size_bytes: Option<i32>,
    pub note_type: Option<String>,
    pub note_date: Option<NaiveDate>,
    pub modified_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    // New fields
    pub compression_tier: String,
    pub compressed_from: Option<Vec<i32>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MemoryLight {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    pub size_bytes: Option<i32>,
    pub note_type: Option<String>,
    pub note_date: Option<NaiveDate>,
    pub modified_at: Option<NaiveDateTime>,
}
```

### dpn-core lib.rs Re-exports
```rust
// OLD
pub use db::vault_notes::{VaultNote, VaultNoteLight};

// NEW
pub use db::memories::{Memory, MemoryLight};
```

## Blast Radius Map

Complete list of files requiring changes:

### dpn-core (/root/dpn-core/src/) -- 15 files
| File | Change Type | References |
|------|-------------|------------|
| db/vault_notes.rs | FILE RENAME to memories.rs + all SQL strings | ~20 SQL refs |
| db/mod.rs | Module declaration | 2 refs |
| lib.rs | Re-exports | 3 refs |
| timeline/mod.rs | VaultNote enum + SQL + string literals | ~8 refs |
| sync/snapshot.rs | Function names + string literals | ~6 refs |
| sync/incremental.rs | Function names + string literals | ~8 refs |
| replay/collector.rs | VaultNote enum + module import + function calls | ~12 refs |
| replay/narrative.rs | VaultNote enum references | ~3 refs |
| replay/templates.rs | VaultNote enum reference | 1 ref |
| pipeline/tasks_due.rs | Module import + function calls + struct refs | ~8 refs |
| embeddings/batch.rs | VaultNotes enum + string literal | ~4 refs |
| context/injection.rs | VaultNote enum + SQL string + function | ~8 refs |
| cache/sqlite.rs | SQL schema + string literals + struct field | ~8 refs |
| cache/hybrid.rs | Module import + function calls + struct refs + SQL | ~20 refs |
| cache/sync_queue.rs | String literal in test | 2 refs |
| db/tests.rs | Module import + function calls | ~10 refs |
| db/documents.rs | Comment only | 2 refs |

### dpn-api (/opt/dpn-api/src/) -- 3 files (+ 1 delete)
| File | Change Type | References |
|------|-------------|------------|
| handlers/documents.rs | dpn_core::db::vault_notes -> dpn_core::db::memories everywhere | ~15 refs |
| handlers/af64_perception.rs | Raw SQL string + dpn_core type ref | ~3 refs |
| handlers/timeline.rs | Comment only | 1 ref |
| handlers/documents.rs.bak | DELETE (stale backup) | - |

### NOT Changed (Protected by View)
| File | Why Safe |
|------|----------|
| /opt/project-noosphere-ghosts/lisp/tools/onboard.lisp | Default string "vault_notes" in prompt -- cosmetic |
| /opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp | Comments only + calls Python helper |
| /opt/project-noosphere-ghosts/tools/write_vault_memory.py | Queries vault_notes view -- works via INSTEAD OF triggers |

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| vault_notes table name | memories table + vault_notes view | Phase 18 | PARAT naming alignment |
| Text department column | departments lookup table + FK | Phase 18 | Data normalization |
| No compression metadata | compression_tier + compressed_from columns | Phase 18 | Enables Phase 20 temporal compression |

## Open Questions

1. **Agent memory column handling in INSTEAD OF UPDATE trigger**
   - What we know: There are 64 agent memory columns (e.g., `nova_memories`, `eliana_memories`). The UPDATE trigger must SET all of them.
   - What's unclear: Whether `UPDATE memories SET (col1, col2, ...) = (NEW.col1, NEW.col2, ...)` works or if individual SET assignments are needed for 75 columns.
   - Recommendation: Generate the trigger function body dynamically from `information_schema.columns` during migration. Test INSERT and UPDATE via the view before proceeding with Rust changes.

2. **Systems department ambiguity**
   - What we know: D-09 maps "Systems" to both Operations and Engineering.
   - What's unclear: Which agents with department='Systems' should go where.
   - Recommendation: Map Systems -> Operations (1 agent). The Engineering department already has 9 agents with explicit "Engineering" label. Verify the specific agent before backfill.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| PostgreSQL | All migrations | Yes | 14+ | -- |
| postgres user (sudo) | DDL on vault_notes | Yes | -- | -- |
| Rust/Cargo | dpn-core + dpn-api rebuild | Yes | 2021 Edition | -- |
| PM2 | dpn-api restart | Yes | -- | -- |

No missing dependencies.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (Rust, built-in) |
| Config file | /root/dpn-core/Cargo.toml |
| Quick run command | `cd /root/dpn-core && cargo test db::tests -- --test-threads=1` |
| Full suite command | `cd /root/dpn-core && cargo test -- --test-threads=1` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| MEM-01 | View bridge SELECT/INSERT/UPDATE/DELETE | smoke | `sudo -u postgres psql -d master_chronicle -c "SELECT count(*) FROM vault_notes; INSERT INTO vault_notes (path,title,content) VALUES ('test/mem01','test','test') RETURNING id;"` then cleanup | N/A (SQL) |
| MEM-02 | compression_tier backfilled correctly | smoke | `sudo -u postgres psql -d master_chronicle -c "SELECT compression_tier, count(*) FROM memories GROUP BY compression_tier;"` | N/A (SQL) |
| MEM-03 | compressed_from column exists | smoke | `sudo -u postgres psql -d master_chronicle -c "SELECT compressed_from FROM memories LIMIT 1;"` | N/A (SQL) |
| MEM-04 | Department FK valid for all agents | smoke | `sudo -u postgres psql -d master_chronicle -c "SELECT count(*) FROM agents WHERE department_id IS NULL;"` (should be 0) | N/A (SQL) |
| MEM-05 | dpn-api compiles and starts | integration | `cd /opt/dpn-api && cargo build --release 2>&1 && pm2 restart dpn-api && sleep 2 && curl -s http://localhost:8080/api/documents?limit=1` | N/A (build) |
| MEM-06 | dpn-core tests pass with new module names | unit | `cd /root/dpn-core && cargo test db::tests -- --test-threads=1` | Yes (db/tests.rs -- needs renaming) |

### Sampling Rate
- **Per task commit:** `cd /root/dpn-core && cargo check`
- **Per wave merge:** `cd /root/dpn-core && cargo test -- --test-threads=1`
- **Phase gate:** Full suite green + dpn-api release build + PM2 restart + curl test

### Wave 0 Gaps
- [ ] Migration SQL script needs to be created (no existing migration framework)
- [ ] db/tests.rs test names reference vault_notes -- must rename with module
- [ ] View bridge INSERT/UPDATE/DELETE needs empirical test before committing Rust changes

## Sources

### Primary (HIGH confidence)
- Live database inspection via psql (table structure, triggers, indexes, constraints, owners, department values, note_type distribution, FK dependencies)
- Source code inspection: /root/dpn-core/src/db/vault_notes.rs (352 lines)
- Source code inspection: /opt/dpn-api/src/handlers/documents.rs (613 lines)
- Source code inspection: /opt/dpn-api/src/handlers/af64_perception.rs (lines 475-498)
- Source code inspection: /opt/project-noosphere-ghosts/tools/write_vault_memory.py (55 lines)
- Source code inspection: /opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp (lines 1171-1234)
- Grep audit across all 4 codebases for vault_notes/VaultNote references

### Secondary (MEDIUM confidence)
- .planning/research/PITFALLS.md -- domain pitfalls from project-level research
- PostgreSQL documentation on INSTEAD OF triggers (well-established feature, stable since PG 9.1)
- PostgreSQL documentation on information_schema.columns including views

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - no new libraries, pure rename + migration
- Architecture: HIGH - all code paths traced, all references enumerated, DB state verified live
- Pitfalls: HIGH - trigger function body inspected, table ownership confirmed, view behavior verified against PostgreSQL docs

**Research date:** 2026-03-28
**Valid until:** 2026-04-28 (stable -- no external dependencies changing)
