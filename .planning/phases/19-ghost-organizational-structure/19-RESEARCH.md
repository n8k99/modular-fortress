# Phase 19: Ghost Organizational Structure - Research

**Researched:** 2026-03-28
**Domain:** PostgreSQL schema design, data migration, document enrichment
**Confidence:** HIGH

## Summary

Phase 19 formalizes ghost organizational structure across four new tables (teams, team_members, ghost_relationships, agent_areas), one new column (agents.aliases), one new table (routines), and enriches 64 EM Staff documents with YAML frontmatter. The schema dependencies (departments from Phase 18, areas from Phase 16) are fully in place. The existing data is clean: all relationship columns contain valid agent IDs (no display name parsing needed), all 64 agents have document_id FKs to their EM Staff docs, and team names are extractable directly from document content.

This is a pure PostgreSQL migration + document UPDATE phase with no Rust/Lisp code changes required (API endpoints deferred per CONTEXT.md). The main complexity is the data volume: 64 EM Staff documents need YAML frontmatter injection, ~300+ relationship rows need migrating from text columns, and 11 standing order schedules need routines table seeding.

**Primary recommendation:** Execute as 3 waves -- (1) DDL for all new tables + aliases column, (2) data migration for relationships + teams + areas + routines seeding, (3) EM Staff document enrichment with YAML frontmatter and document_path backfill.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01: `teams` table: id SERIAL, name VARCHAR(128), department_id INTEGER REFERENCES departments(id), lead_id VARCHAR(64) REFERENCES agents(id), area_id INTEGER REFERENCES areas(id), description TEXT, created_at TIMESTAMPTZ
- D-02: `team_members` junction: team_id REFERENCES teams(id), agent_id REFERENCES agents(id), role_in_team VARCHAR(64), joined_at TIMESTAMPTZ, PRIMARY KEY (team_id, agent_id)
- D-03: Seed teams from Areas/Eckenrode Muziekopname/ department structure mapped to departments table
- D-04: `ghost_relationships` table: id SERIAL, from_agent REFERENCES agents(id), to_agent REFERENCES agents(id), relationship_type VARCHAR(32), created_at TIMESTAMPTZ. CHECK on type: reports_to, mentor, mentee, collaborators, liaises_with
- D-05: Migrate agents.reports_to, mentor, mentee, collaborators, liaises_with to ghost_relationships rows
- D-06: Keep original text columns (not dropped this phase)
- D-07: `agent_areas` junction: agent_id REFERENCES agents(id), area_id REFERENCES areas(id), role VARCHAR(64), PRIMARY KEY (agent_id, area_id)
- D-08: Backfill from department-to-area mapping. Cross-functional agents (Nova, Sarah Lin) get multiple areas
- D-09: Add aliases TEXT[] to agents. Backfill Nova with {'T.A.S.K.S.'}
- D-10: Seed other aliases from EM Staff files
- D-11: Update 64 EM Staff documents with YAML frontmatter (agent_id, memory_column, department, team, area)
- D-12: Verify all 64 agents have document_id linked. Backfill missing.
- D-13: Verify document_path points to correct EM Staff path. Backfill missing.
- D-14: `routines` table: id SERIAL, name VARCHAR(256), owner_agent REFERENCES agents(id), department_id REFERENCES departments(id), project_id REFERENCES projects(id), schedule JSONB, description TEXT, status VARCHAR(32) DEFAULT 'active', tool_label VARCHAR(128), created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ
- D-15: Seed routines from existing standing order labels on projects.schedule
- D-16: Routines are NOT a replacement for standing orders -- they add ghost ownership layer
- D-17: `(agent){action}` Innate expressions are the reference pattern; routines table is database backing

### Claude's Discretion
- Index choices for new junction tables
- Exact parsing strategy for text relationship columns
- Whether to add API endpoints for teams/relationships/routines in this phase or defer
- Migration script structure and ordering

### Deferred Ideas (OUT OF SCOPE)
- Department content tables (Areas/* folders each deserve their own table -- future phase)
- GitHub repo links on projects + issues-to-tasks sync
- API endpoints for org structure (GET /api/teams, GET /api/routines etc.)
- Routine evaluation by tick engine (routines are registry only this phase)
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ORG-01 | Teams table with name, department, lead_id, area_id, plus team_members junction with role_in_team | 13 teams identified from EM Staff docs mapped to 8 departments; DDL per D-01/D-02 |
| ORG-02 | ghost_relationships table formalizing reports_to, mentor, mentee, collaborators, liaises_with | All values are valid agent IDs (verified); ~300+ rows to migrate from 5 text columns across 64 agents |
| ORG-03 | agent_areas junction for multi-area ghost assignment | 5 areas exist; department-to-area mapping needed; Nova/Sarah Lin cross-functional |
| ORG-04 | Agents aliases TEXT[] column for dual identities | Nova IS T.A.S.K.S.; column addition + backfill |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| PostgreSQL | 14+ (installed) | All schema changes, data migration | DB is the OS per CLAUDE.md |

### Supporting
No additional libraries needed. This is a pure SQL migration phase.

**Installation:** N/A -- all tools already available.

## Architecture Patterns

### Migration Script Pattern (from Phase 18)
Phase 18 used `/tmp/18-01-migration.sql` executed via `psql`. Same pattern applies here. All DDL and DML in SQL scripts, executed as the `postgres` user (projects table ownership precedent from Phase 17).

### Recommended Wave Structure
```
Wave 1: DDL (all new tables + column)
  - teams, team_members, ghost_relationships, agent_areas, routines tables
  - agents.aliases TEXT[] column
  - All indexes, constraints, CHECK constraints

Wave 2: Data Migration (relationships + teams + areas + routines)
  - Seed 13 teams from EM Staff document analysis
  - Populate team_members from agents.department_id + document Team field
  - Migrate relationship columns to ghost_relationships rows
  - Seed agent_areas from department-to-area mapping
  - Seed routines from projects.schedule standing orders
  - Backfill agents.aliases

Wave 3: Document Enrichment
  - Backfill agents.document_path for all 64 agents
  - Update 64 EM Staff documents with YAML frontmatter
```

### Team Seeding Data (from live DB analysis)

13 teams identified from EM Staff document `**Team:**` fields, mapped to departments:

| Team Name | Department | Agent Count | Likely Lead |
|-----------|-----------|-------------|-------------|
| Technical Development Office | Engineering (2) | 8 | samir or devin |
| Content and Brand Office | Content & Brand (3) | 9 | sylvia (exec) |
| Creative Development Team | Creative (4) | 7 | vincent (exec, dept head in doc) |
| Legal and Ethics Team | Legal (5) | 8 | jmax (exec) |
| Musicology Research Office | Music (6) | 4 | lrm (exec) |
| Audience Experience Office | Strategy (7) | 5 | kathryn-adjacent |
| Strategic Office | Strategy (7) | 5 | kathryn-adjacent |
| Digital Partnership and Innovation | Strategy (7) | 2 | kathryn-adjacent |
| Social Impact Team | Strategy (7) | 1 | kathryn-adjacent |
| Office of the CEO | Operations (1) / Executive (8) | 4 | nathan/sarah |
| Marketing and Communications | Operations (1) | 1 | nova-adjacent |
| Cross-Functional Team | Executive (8) | 2 | sarah |
| Executive | Executive (8) | 5 | nathan |

Note: Some executives appear in "Executive" team in their doc but belong to departmental departments (e.g., Vincent is Creative dept but doc says Team: Executive). The `team_members` junction handles this -- an exec can be in the Executive team AND lead their departmental team.

### Relationship Migration Data (from live DB analysis)

| Column | Type | Agents With Data | Total Values | Format |
|--------|------|-----------------|--------------|--------|
| reports_to | text[] | 63 (all except nathan) | ~80 values | Valid agent IDs |
| mentor | text (scalar) | 54 | 54 values | Valid agent IDs |
| mentee | text[] | 0 | 0 | Empty (mentor is inverse) |
| collaborators | text[] | 60 | ~300+ values | Valid agent IDs |
| liaises_with | text[] | 52 | ~100+ values | Valid agent IDs |

**Key finding:** All relationship values are already agent IDs matching `agents.id`. No display name parsing needed. The `mentee` column is empty -- mentee relationships are derivable from the inverse of `mentor`.

### Relationship Migration SQL Pattern
```sql
-- reports_to: array column -> multiple rows
INSERT INTO ghost_relationships (from_agent, to_agent, relationship_type, created_at)
SELECT a.id, unnest(a.reports_to), 'reports_to', now()
FROM agents a
WHERE a.reports_to IS NOT NULL;

-- mentor: scalar text -> single row
INSERT INTO ghost_relationships (from_agent, to_agent, relationship_type, created_at)
SELECT a.id, a.mentor, 'mentor', now()
FROM agents a
WHERE a.mentor IS NOT NULL;

-- mentee: derive from inverse of mentor
INSERT INTO ghost_relationships (from_agent, to_agent, relationship_type, created_at)
SELECT a.mentor, a.id, 'mentee', now()
FROM agents a
WHERE a.mentor IS NOT NULL;

-- collaborators: array -> rows (bidirectional, but store from perspective of source)
INSERT INTO ghost_relationships (from_agent, to_agent, relationship_type, created_at)
SELECT a.id, unnest(a.collaborators), 'collaborators', now()
FROM agents a
WHERE a.collaborators IS NOT NULL;

-- liaises_with: array -> rows
INSERT INTO ghost_relationships (from_agent, to_agent, relationship_type, created_at)
SELECT a.id, unnest(a.liaises_with), 'liaises_with', now()
FROM agents a
WHERE a.liaises_with IS NOT NULL;
```

### Department-to-Area Mapping for agent_areas

| Department | Primary Area | Rationale |
|-----------|-------------|-----------|
| Operations (1) | Infrastructure/Systems (5) | Nova manages droplet/ops |
| Engineering (2) | Infrastructure/Systems (5) | Code/infra domain |
| Content & Brand (3) | EM Corp (1) | Brand/content is EM Corp |
| Creative (4) | EM Corp (1) | Art direction is EM Corp |
| Legal (5) | EM Corp (1) | Legal/ethics is EM Corp |
| Music (6) | Living Room Music (3) | Musicology domain |
| Strategy (7) | EM Corp (1) | Strategy serves EM Corp |
| Executive (8) | EM Corp (1) | Default for execs |

Cross-functional agents:
- **Nova**: Operations (Infra/Systems) + EM Corp (cross-functional ops oversight)
- **Sarah Lin**: Executive (EM Corp) + Infra/Systems (scheduling/routing)
- **Kathryn**: Strategy (EM Corp) + N8K99/Personal (prediction markets)

### EM Staff YAML Frontmatter Pattern

Current documents have NO YAML frontmatter. Content starts with `![[Image.png]]` then `# Name`. Enrichment prepends:

```markdown
---
agent_id: alex_torres
memory_column: alex_torres_memories
department: Success
team: Audience Experience Office
area: EM Corp
---

![[AlexTorres.png]]
... (existing content unchanged)
```

### Memory Column Mapping

63 per-agent columns exist on `memories` table (nathan has none). Pattern: `{agent_slug}_memories`. The slug matches the agent ID with underscores (e.g., `sarah` -> `sarah_memories`, `alex_torres` -> `alex_torres_memories`).

### Routines Seeding Data

From existing `projects.schedule` standing orders:

| Routine Name | Owner | Department | Project | Schedule | Tool Label |
|-------------|-------|-----------|---------|----------|------------|
| Daily Health Check | nova | Operations (1) | 14 | 0 13 * * * | daily-health-check |
| Nightly Synthesis | nova | Operations (1) | 14 | 5 4 * * * | nightly-synthesis |
| Daily Note Population | nova | Operations (1) | 14 | 50 3 * * * | daily-note-population |
| Weekly Finalization | nova | Operations (1) | 14 | 30 4 * * 6 | weekly-finalization |
| Monthly Finalization | nova | Operations (1) | 14 | 0 5 1 * * | monthly-finalization |
| Podcast Watch | nova | Operations (1) | 14 | 10 23 * * * | podcast-watch |
| Nightly Editorial | sylvia | Content & Brand (3) | 12 | 0 1 * * * | nightly-editorial |
| Tokyo Session | kathryn | Strategy (7) | 10 | 0 22 * * 0-4 | tokyo-session |
| London Session | kathryn | Strategy (7) | 10 | 0 6 * * 1-5 | london-session |
| NYC Session | kathryn | Strategy (7) | 10 | 0 12 * * 1-5 | nyc-session |
| Calendar Sync | kathryn | Strategy (7) | 10 | 0 10 * * * | calendar-sync |

11 routines total from 3 projects.

### Anti-Patterns to Avoid
- **Dropping relationship columns prematurely:** D-06 explicitly says keep them. Same pattern as Phase 17 goals.project.
- **Making routines replace standing orders:** D-16 says they are an ownership layer, not a replacement. Tick engine still reads projects.schedule.
- **Adding API endpoints:** Deferred per CONTEXT.md. This is schema + data only.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Team membership extraction | Manual team list | SQL query against EM Staff doc content regex | 13 teams already identifiable from `**Team:**` field |
| Relationship parsing | Complex name-matching logic | Direct INSERT from arrays (all values are valid agent IDs) | Verified: zero display names in relationship columns |
| Document frontmatter injection | String concatenation in app code | SQL UPDATE with concat('---\n...', content) | All 64 docs follow same structure |
| Memory column name derivation | Lookup table | Pattern: `{agent_id}_memories` | Consistent naming, verified against all 63 columns |

## Common Pitfalls

### Pitfall 1: Duplicate Relationship Rows
**What goes wrong:** Collaborators/liaises_with may have A->B and B->A stored separately, leading to apparent duplicates in ghost_relationships.
**Why it happens:** Text array columns stored relationships from each agent's perspective independently.
**How to avoid:** Store as-is from source data. The CHECK constraint allows duplicates (A collaborates_with B AND B collaborates_with A are two valid rows). Deduplication is a future concern if needed.
**Warning signs:** Row count significantly higher than expected.

### Pitfall 2: Nathan's Missing Memory Column
**What goes wrong:** Agent `nathan` has no `_memories` column (63 columns for 64 agents).
**Why it happens:** Nathan is the human CEO, not a ghost -- no memory column needed.
**How to avoid:** YAML frontmatter for nathan should set `memory_column: null` or omit it. Nathan does have a document_id (if an EM Staff doc exists for him).
**Warning signs:** Error on frontmatter generation for nathan.

### Pitfall 3: Executive Team vs Department Team Confusion
**What goes wrong:** Executives (Vincent, Eliana, etc.) have `**Team:** Executive` in their EM Staff doc but belong to departmental departments.
**Why it happens:** Executives serve on the Executive team AND lead department teams.
**How to avoid:** An executive can be a member of the Executive team AND the lead_id of their departmental team. Use team_members junction for Executive team membership separately.

### Pitfall 4: Document Path vs Document Content
**What goes wrong:** `agents.document_path` is almost entirely empty (only Nova has one, pointing to old location). Confusing document_path with the `documents.path` column.
**Why it happens:** document_path was added but never backfilled systematically.
**How to avoid:** Backfill from `documents.path WHERE id = agents.document_id`. The correct paths are `Areas/Eckenrode Muziekopname/EM Staff/{Name}.md` (verified: ids 60271-60334).

### Pitfall 5: YAML Frontmatter Injection Breaking Content
**What goes wrong:** Prepending YAML frontmatter to document content that already has markdown structure.
**Why it happens:** Content starts with `![[Image.png]]` -- frontmatter must go before this.
**How to avoid:** Simple SQL: `UPDATE documents SET content = '---\n...\n---\n\n' || content WHERE id = X`. Verify no existing `---` delimiters at content start.

### Pitfall 6: postgres User Required for Migration
**What goes wrong:** Migration fails with permission error.
**Why it happens:** Some tables are owned by postgres user (Phase 17 precedent).
**How to avoid:** Run migration as postgres user: `sudo -u postgres psql -d master_chronicle -f migration.sql`. Or use chronicle user and verify permissions first.

## Code Examples

### DDL for teams table
```sql
-- Source: CONTEXT.md D-01
CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL UNIQUE,
    department_id INTEGER REFERENCES departments(id),
    lead_id VARCHAR(64) REFERENCES agents(id),
    area_id INTEGER REFERENCES areas(id),
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_teams_department_id ON teams(department_id);
CREATE INDEX idx_teams_lead_id ON teams(lead_id);
```

### DDL for ghost_relationships table
```sql
-- Source: CONTEXT.md D-04
CREATE TABLE ghost_relationships (
    id SERIAL PRIMARY KEY,
    from_agent VARCHAR(64) NOT NULL REFERENCES agents(id),
    to_agent VARCHAR(64) NOT NULL REFERENCES agents(id),
    relationship_type VARCHAR(32) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    CHECK (relationship_type IN ('reports_to', 'mentor', 'mentee', 'collaborators', 'liaises_with'))
);

CREATE INDEX idx_ghost_rel_from ON ghost_relationships(from_agent);
CREATE INDEX idx_ghost_rel_to ON ghost_relationships(to_agent);
CREATE INDEX idx_ghost_rel_type ON ghost_relationships(relationship_type);
```

### DDL for routines table
```sql
-- Source: CONTEXT.md D-14
CREATE TABLE routines (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    owner_agent VARCHAR(64) REFERENCES agents(id),
    department_id INTEGER REFERENCES departments(id),
    project_id INTEGER REFERENCES projects(id),
    schedule JSONB,
    description TEXT,
    status VARCHAR(32) DEFAULT 'active',
    tool_label VARCHAR(128),
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_routines_owner ON routines(owner_agent);
CREATE INDEX idx_routines_department ON routines(department_id);
CREATE INDEX idx_routines_project ON routines(project_id);
CREATE INDEX idx_routines_status ON routines(status);
```

### Document frontmatter injection pattern
```sql
-- Backfill document_path from documents.path
UPDATE agents a
SET document_path = d.path
FROM documents d
WHERE a.document_id = d.id
  AND d.path LIKE 'Areas/Eckenrode Muziekopname/EM Staff/%';

-- Inject YAML frontmatter (example for one agent)
UPDATE documents
SET content = E'---\nagent_id: alex_torres\nmemory_column: alex_torres_memories\ndepartment: Success\nteam: Audience Experience Office\narea: EM Corp\n---\n\n' || content
WHERE id = 60271;
```

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | PostgreSQL direct queries (psql) |
| Config file | N/A -- SQL validation queries |
| Quick run command | `PGPASSWORD=chronicle2026 psql -U chronicle -h 127.0.0.1 -d master_chronicle -f /tmp/19-validate.sql` |
| Full suite command | Same as quick run |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| ORG-01 | teams + team_members exist with correct data | SQL query | `psql -c "SELECT COUNT(*) FROM teams; SELECT COUNT(*) FROM team_members;"` | Wave 0 |
| ORG-02 | ghost_relationships populated from text columns | SQL query | `psql -c "SELECT relationship_type, COUNT(*) FROM ghost_relationships GROUP BY 1;"` | Wave 0 |
| ORG-03 | agent_areas populated with multi-area assignments | SQL query | `psql -c "SELECT COUNT(*) FROM agent_areas; SELECT * FROM agent_areas WHERE agent_id = 'nova';"` | Wave 0 |
| ORG-04 | aliases column exists, Nova has T.A.S.K.S. | SQL query | `psql -c "SELECT id, aliases FROM agents WHERE aliases IS NOT NULL;"` | Wave 0 |

### Additional Validation Queries
| Check | Query |
|-------|-------|
| All 64 agents have document_path | `SELECT COUNT(*) FROM agents WHERE document_path IS NOT NULL AND document_path <> ''` = 64 |
| All 64 EM Staff docs have frontmatter | `SELECT COUNT(*) FROM documents WHERE path LIKE 'Areas/Eckenrode Muziekopname/EM Staff/%' AND content LIKE '---\nagent_id:%'` = 64 |
| 11 routines seeded | `SELECT COUNT(*) FROM routines` = 11 |
| 13 teams seeded | `SELECT COUNT(*) FROM teams` = 13 |
| Relationship row count sanity | Reports_to: ~80, Mentor: ~54, Mentee: ~54 (inverse), Collaborators: ~300+, Liaises: ~100+ |

### Sampling Rate
- **Per task commit:** Quick validation queries for affected tables
- **Per wave merge:** Full validation suite
- **Phase gate:** All ORG-01 through ORG-04 verified + expanded scope checks

### Wave 0 Gaps
- [ ] `/tmp/19-validate.sql` -- validation query script (create during Wave 1)

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Text array columns for relationships | Typed junction table (ghost_relationships) | Phase 19 | Queryable, constrained relationships |
| No team formalization | teams + team_members tables | Phase 19 | Org structure in DB |
| Standing orders only on projects | Routines table with ghost ownership | Phase 19 | Who-owns-what-recurring-work is queryable |
| EM Staff docs as plain markdown | YAML frontmatter with agent_id, memory_column | Phase 19 | Documents linked to infrastructure |

## Open Questions

1. **Team lead assignment for Strategy sub-teams**
   - What we know: Strategy has 4 sub-teams (Audience Experience, Strategic Office, Digital Partnership, Social Impact). Kathryn is the Strategy exec.
   - What's unclear: Who leads each sub-team? (lead_id on teams table)
   - Recommendation: Set Kathryn as lead for all 4 Strategy teams initially. Can be refined later.

2. **Nathan's EM Staff document**
   - What we know: Nathan has document_id pointing to an EM Staff doc (id in 60271-60334 range). Nathan has no memory column.
   - What's unclear: Should nathan get YAML frontmatter with `memory_column: null`?
   - Recommendation: Yes, include nathan in frontmatter enrichment with `memory_column: ~` (YAML null). He is CEO with a document.

3. **Collaborator deduplication**
   - What we know: If A has B in collaborators AND B has A in collaborators, both get rows.
   - What's unclear: Is this wasteful or intentional?
   - Recommendation: Store as-is. The source data represents each agent's perspective. Dedup can happen at query time if needed.

## Project Constraints (from CLAUDE.md)

- **DB is the OS**: All state in master_chronicle. No file-based state.
- **UTF-8 Rule**: Not directly applicable (no Rust changes this phase).
- **Stack constraint**: PostgreSQL only for this phase. No new languages.
- **Ghost LLM**: Not applicable (no tick engine changes).
- **Guardrails**: Check INFRASTRUCTURE.md before building. Use postgres user for migration if needed.
- **GSD Workflow**: All work through GSD commands.

## Sources

### Primary (HIGH confidence)
- Live database queries against master_chronicle (agents, departments, areas, documents, projects tables)
- EM Staff document content analysis (64 documents, ids 60271-60334)
- Phase 18 migration pattern (18-01-PLAN.md)
- CONTEXT.md decisions D-01 through D-17

### Secondary (MEDIUM confidence)
- Tool-registry.json (9 tools, routine label mapping)
- Daily Note template (document 11651, Innate expression patterns)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- pure PostgreSQL, no library choices needed
- Architecture: HIGH -- schema fully specified in CONTEXT.md decisions, data verified in live DB
- Pitfalls: HIGH -- all relationship data formats verified, document structure inspected
- Data migration: HIGH -- all values confirmed as valid agent IDs, counts verified

**Research date:** 2026-03-28
**Valid until:** 2026-04-28 (stable -- schema design, no external dependencies)
