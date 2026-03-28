# Architecture Patterns: PARAT Noosphere Schema Integration

**Domain:** PostgreSQL schema restructuring for agentic AI platform
**Researched:** 2026-03-28
**Confidence:** HIGH (based on direct schema inspection of live 77-table master_chronicle)

## Current State: 77-Table master_chronicle

The existing database has 77 tables serving multiple concerns without organizational structure. The PARAT integration reorganizes these into five pillars without breaking the live system.

### Table-to-Pillar Mapping

Every existing table maps to a PARAT pillar or remains as infrastructure:

| Pillar | Existing Tables (migrate/modify) | New Tables |
|--------|----------------------------------|------------|
| **@Projects** | `projects` (add lifestage), `goals` (add FK), `tasks` (already linked) | none |
| **@Areas** | `agents` (add area_id) | `areas` |
| **@Resources** | `documents`, `media`, `feeds`, `articles`, `fetched_articles` | `resources` (organizational overlay) |
| **@Archives** | documents with `Archive/` paths (~2179 Nexus imports) | `archives` |
| **@Templates** | none currently | `templates` |
| **Memories** | `vault_notes` (rename to `memories`), `memory_entries`, `agent_daily_memory` | none (rename + view) |
| **Infrastructure** (unchanged) | `conversations`, `agent_state`, `agent_drives`, `agent_fitness`, `tick_reports`, `tick_log`, `decisions`, `persona_mutations`, `metamorphosis_log`, `codebase_scans`, `security_lint_*`, forex/market tables, `users`, `events`, etc. | none |

### Tables That Stay Unchanged (40+)

These are operational/infrastructure tables that don't map to PARAT pillars. They must NOT be touched:

- **Agent runtime:** `agent_state`, `agent_drives`, `agent_fitness`, `agent_requests`, `persona_mutations`, `metamorphosis_log`, `tick_log`, `tick_reports`
- **Communication:** `conversations`, `noosphere_feedback`, `noosphere_replies`, `xmpp_messages`, `discord_messages`
- **Financial:** `forex_*` (4 tables), `kalshi_*` (2 tables), `market_*` (4 tables), `positions`, `position_events`, `orders`, `trade_*` (2 tables), `probability_scores`, `sentiment_scores`
- **Music:** `lrm_corpus`, `music_*` (3 tables), `audio_*` (2 tables), `episodes`
- **Infrastructure:** `users`, `linked_accounts`, `contacts`, `locations`, `events`, `inbox`, `issues`, `daily_logs`, `executive_sessions`, `wave_calendar`, `weekly_executive_summary`, `stagehand_notes`
- **Document meta:** `document_edges`, `document_categories`, `document_registry`, `document_versions`, `annotations`, `comments`, `collected_comments`, `folders`

## Recommended Architecture

### Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| `areas` table | Ongoing domains (EM Corp, Orbis, etc.) | `ghosts.area_id`, `projects.area_id` |
| `projects` table (modified) | Active work with Lifestage arc | `goals`, `tasks`, `areas` |
| `goals` table (modified) | Project objectives with proper FK | `projects` via integer FK |
| `memories` table (renamed from vault_notes) | Temporal notes with ghost memory columns | Ghost perception, temporal compression |
| `ghosts` view (over agents) | Agent registry with org structure | `areas`, `agent_state`, all agent_* tables |
| `resources` table | Read-optimized organizational overlay | Indexes into `media`, `documents`, `feeds` |
| `archives` table | Immutable historical content | Temporal compression terminus |
| `templates` table | Live .dpn expressions | Future Innate interpreter |
| `temporal_compression_log` | Tracks compression runs | `memories`, `archives` |

### Data Flow

```
GSD dispatch --> projects (with lifestage + area_id)
                  --> goals (proper FK to project_id)
                  --> tasks (already has project_id FK)

Tick engine --> perception reads:
                 memories (was vault_notes) -- ghost_memories columns
                 agent_daily_memory -- daily logs
                 memory_entries -- long-term facts
                 projects/tasks -- assigned work

Standing orders --> ghost writes to memories columns
                --> temporal compression rolls daily->weekly->monthly->quarterly->yearly

Nexus Chat Import --> archives table (immutable source)
                   --> temporal compression
                   --> ghost memory injection into memories columns
```

## Schema Changes (Detailed)

### Wave 1: Foundation Tables (no existing code breakage)

#### New: `areas` table
```sql
CREATE TABLE areas (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL UNIQUE,
    slug VARCHAR(256) NOT NULL UNIQUE,
    description TEXT,
    domain VARCHAR(128),
    owner VARCHAR(64),
    status VARCHAR(32) DEFAULT 'active',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

Initial seed: EM Corp, Orbis, Living Room Music, N8K99/Personal, Infrastructure/Systems.

#### New: `templates` table
```sql
CREATE TABLE templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    slug VARCHAR(256) NOT NULL UNIQUE,
    category VARCHAR(128),
    body TEXT NOT NULL,
    description TEXT,
    variables JSONB DEFAULT '[]',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### New: `archives` table
```sql
CREATE TABLE archives (
    id SERIAL PRIMARY KEY,
    source VARCHAR(128) NOT NULL,
    original_path TEXT,
    original_date DATE,
    title VARCHAR(512),
    content TEXT,
    metadata JSONB DEFAULT '{}',
    embedding VECTOR(768),
    archived_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_archives_source ON archives(source);
CREATE INDEX idx_archives_date ON archives(original_date);
CREATE INDEX idx_archives_embedding ON archives USING hnsw (embedding vector_cosine_ops);
```

#### New: `resources` table
```sql
CREATE TABLE resources (
    id SERIAL PRIMARY KEY,
    name VARCHAR(512) NOT NULL,
    resource_type VARCHAR(64) NOT NULL,
    source_table VARCHAR(64),
    source_id INTEGER,
    path TEXT,
    description TEXT,
    tags JSONB DEFAULT '[]',
    metadata JSONB DEFAULT '{}',
    area_id INTEGER REFERENCES areas(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_resources_type ON resources(resource_type);
CREATE INDEX idx_resources_area ON resources(area_id);
```

**Design decision:** `resources` is an organizational overlay, not a replacement. `media`, `documents`, `feeds` continue to exist. Resources provides a unified PARAT-aware index into them. This avoids migrating 48K+ documents.

#### New: `temporal_compression_log` table
```sql
CREATE TABLE temporal_compression_log (
    id SERIAL PRIMARY KEY,
    source_type VARCHAR(32) NOT NULL,
    target_type VARCHAR(32) NOT NULL,
    source_date_start DATE NOT NULL,
    source_date_end DATE NOT NULL,
    target_date DATE NOT NULL,
    ghost_id VARCHAR(64),
    source_count INTEGER,
    compressed_content TEXT,
    status VARCHAR(32) DEFAULT 'completed',
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Wave 2: Project/Goals Modification (low risk)

#### `projects` table additions
```sql
ALTER TABLE projects ADD COLUMN lifestage VARCHAR(32) DEFAULT 'seed'
    CHECK (lifestage IN ('seed', 'sapling', 'tree', 'harvest'));
ALTER TABLE projects ADD COLUMN area_id INTEGER REFERENCES areas(id);
CREATE INDEX idx_projects_area ON projects(area_id);
CREATE INDEX idx_projects_lifestage ON projects(lifestage);
```

#### `goals` table: Add proper FK to projects
```sql
ALTER TABLE goals ADD COLUMN project_id INTEGER REFERENCES projects(id);
-- Backfill from text column:
-- UPDATE goals SET project_id = p.id FROM projects p WHERE goals.project = p.slug;
CREATE INDEX idx_goals_project_id ON goals(project_id);
-- Keep goals.project TEXT column for backward compat during transition
```

### Wave 3: vault_notes Rename (medium risk, view-mitigated)

**Strategy: RENAME + VIEW for backward compatibility**

```sql
-- Step 1: Rename the actual table
ALTER TABLE vault_notes RENAME TO memories;

-- Step 2: Rename indexes/constraints to match
ALTER INDEX vault_notes_pkey RENAME TO memories_pkey;
ALTER INDEX vault_notes_path_key RENAME TO memories_path_key;
ALTER SEQUENCE vault_notes_id_seq RENAME TO memories_id_seq;

-- Step 3: Create backward-compatible view
CREATE VIEW vault_notes AS SELECT * FROM memories;

-- Step 4: Make view insertable (for existing INSERT queries)
CREATE RULE vault_notes_insert AS ON INSERT TO vault_notes
    DO INSTEAD INSERT INTO memories VALUES (NEW.*);
CREATE RULE vault_notes_update AS ON UPDATE TO vault_notes
    DO INSTEAD UPDATE memories SET
        path = NEW.path, title = NEW.title, content = NEW.content,
        frontmatter = NEW.frontmatter, size_bytes = NEW.size_bytes,
        note_type = NEW.note_type, note_date = NEW.note_date,
        embedding = NEW.embedding, modified_at = NEW.modified_at
    WHERE id = OLD.id;
```

Code migration path:
1. dpn-core: rename `vault_notes.rs` to `memories.rs`, update table name in all queries
2. dpn-api: update `documents.rs` handler comments and query strings
3. dpn-api: update `af64_perception.rs` memory column queries
4. Lisp: update `write_vault_memory.py` tool
5. Trigger: recreate `trg_sync_task_checkbox` on `memories` table

**Memory columns stay as-is.** The 64 `{agent_name}_memories` columns are the ghost memory substrate. They work. Don't normalize them.

### Wave 4: Agents Org Structure (low risk, additive)

```sql
ALTER TABLE agents ADD COLUMN area_id INTEGER REFERENCES areas(id);
ALTER TABLE agents ADD COLUMN team VARCHAR(128);
ALTER TABLE agents ADD COLUMN blog_enabled BOOLEAN DEFAULT FALSE;

-- Create ghosts view for PARAT-native code
CREATE VIEW ghosts AS SELECT * FROM agents;
```

**Why not rename agents to ghosts:** The `agents` table has 8 FK references (`agent_state`, `agent_drives`, `agent_fitness`, `agent_daily_memory`, `tick_log`, `metamorphosis_log`, `persona_mutations`, `agent_document_links`). Renaming requires cascading all FK constraint names. A view is safer.

### Wave 5: Nexus Import + Temporal Compression

1. Migration script copies documents with `Archive/%Nexus AI Chat Imports%` paths into `archives` table
2. Temporal compression tool reads archive content, summarizes per-period
3. Compressed summaries injected into `memories` ghost columns (nova_memories primarily)
4. Daily/weekly notes get "Nexus Import Links" section referencing archive IDs

## Patterns to Follow

### Pattern 1: View-Based Rename for Live Migration
**What:** Rename tables by creating views with old names
**When:** Any table rename where existing code references the old name
**Example:** vault_notes -> memories + CREATE VIEW vault_notes

### Pattern 2: Organizational Overlay (not Replacement)
**What:** `resources` table indexes into existing tables rather than replacing them
**When:** Existing tables have complex FK relationships or large row counts (48K documents)

### Pattern 3: JSONB metadata for Extensibility
**What:** `metadata JSONB` column on all new PARAT tables
**When:** Any new table that might need schema evolution

### Pattern 4: Backfill-Then-Drop for Column Migrations
**What:** Add new column, backfill from old, verify, optionally drop old
**When:** Changing column type (goals.project TEXT -> goals.project_id INTEGER FK)

## Anti-Patterns to Avoid

### Anti-Pattern 1: Big Bang Table Rename
**What:** `ALTER TABLE agents RENAME TO ghosts` then fix all code
**Why bad:** 8+ FK references, Lisp tick engine, dpn-api handlers, Python tools all break simultaneously
**Instead:** View-based rename

### Anti-Pattern 2: Migrating 48K Documents to New Table
**What:** Moving all `documents` rows to a `resources` table
**Why bad:** Massive data migration, all document_id FKs break, embedding indexes must be rebuilt
**Instead:** `resources` as organizational overlay

### Anti-Pattern 3: Normalizing Ghost Memory Columns
**What:** Replacing per-agent columns (nova_memories, eliana_memories) with a join table
**Why bad:** These columns work, perception queries are optimized for them, tick engine writes to them via Python helper. Wide-table is intentional (fast per-agent reads without JOINs).
**Instead:** Keep columns as-is.

### Anti-Pattern 4: Modifying Perception Endpoint Mid-Migration
**What:** Changing perception SQL before all table renames are stable
**Why bad:** Ghosts are live. Broken perception = no ghost activity.
**Instead:** Perception changes go last, after all tables/views verified.

## dpn-api Endpoint Changes

### New Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/areas` | GET, POST | List/create areas |
| `/api/areas/:id` | GET, PATCH | Get/update area |
| `/api/areas/:id/ghosts` | GET | Ghosts assigned to area |
| `/api/areas/:id/projects` | GET | Projects in area |
| `/api/templates` | GET, POST | List/create templates |
| `/api/templates/:id` | GET, PUT | Get/update template |
| `/api/archives` | GET | List archives (paginated) |
| `/api/archives/:id` | GET | Get archive entry |
| `/api/archives/import` | POST | Bulk import (Nexus) |
| `/api/resources` | GET | List resources |
| `/api/resources/:id` | GET | Get resource |
| `/api/temporal-compression/run` | POST | Trigger compression |
| `/api/temporal-compression/log` | GET | View compression history |

### Modified Endpoints

| Endpoint | Change |
|----------|--------|
| `/api/projects` | Add `lifestage`, `area_id` to response/input |
| `/api/projects/:id` | Add `lifestage`, `area_id` |
| `/api/perception/:agent_id` | After rename: query `memories` table (view handles backward compat during transition) |

### dpn-core Module Changes

| Module | Change |
|--------|--------|
| `src/db/vault_notes.rs` | Rename to `memories.rs`, update table name in queries |
| `src/db/projects.rs` | Add lifestage, area_id to struct and queries |
| `src/db/mod.rs` | Add: `areas.rs`, `templates.rs`, `archives.rs`, `resources.rs` |

## Nexus Chat AI Import Data Flow

2,179 documents at `Archive/Retired Nebulab/04 Archives/01 Nexus AI Chat Imports/` paths. Duplicates exist in `Archive/backup-Nebulab/` (need dedup).

```
1. documents table (source, 2179 rows)
   | migration script (copy, don't move)
   v
2. archives table (immutable, source='nexus_chat_import', dates from paths)
   | temporal compression (scheduled process)
   v
3. Compressed summaries --> memories ghost columns (nova_memories primarily)
   | link injection
   v
4. Daily/weekly notes get "Nexus Import Links" section
```

Documents table rows are NOT deleted. Archives gets copies. Preserves all existing document_id FKs and search indexes.

## Temporal Compression Architecture

```
daily memories (2199 rows, note_type='daily')
  | weekly rollup (7 days -> 1 summary)
  v
weekly memories (314 rows, note_type='weekly')
  | monthly rollup (4-5 weeks -> 1 summary)
  v
monthly memories (68 rows, note_type='monthly')
  | quarterly rollup (3 months -> 1 summary)
  v
quarterly memories (25 rows, note_type='quarterly')
  | yearly rollup (4 quarters -> 1 summary)
  v
yearly memories (9 rows, note_type='yearly')
```

The compression process:
1. Scheduled via standing order (Nova/T.A.S.K.S. cron)
2. Reads source-level notes from `memories` table
3. LLM summarizes (or deterministic merge) into target-level note
4. Writes compressed note to `memories` with target note_type
5. Logs to `temporal_compression_log`
6. Does NOT delete source notes

Ghost memory columns compress too: daily ghost memories roll up into weekly summary in the weekly note's ghost column.

## Sources

- Live database schema inspection (master_chronicle, 77 tables, 2026-03-28)
- dpn-api source: `/opt/dpn-api/src/handlers/` (perception, documents, memory)
- dpn-core source: `/root/dpn-core/src/db/vault_notes.rs`
- Lisp tick engine: `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp`
- PARAT schema spec: `.claude/projects/-root/memory/project_parat_schema.md`
- PROJECT.md v1.3 milestone definition
