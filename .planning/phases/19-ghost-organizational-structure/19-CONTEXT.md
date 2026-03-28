# Phase 19: Ghost Organizational Structure - Context

**Gathered:** 2026-03-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Formalize ghost organizational structure in the noosphere: teams, typed relationships, multi-area assignments, identity aliases, EM Staff document enrichment with YAML frontmatter pointing to memory columns, and routine ownership connecting ghosts to their scheduled pipelines. The `Areas/Eckenrode Muziekopname/` document tree in the noosphere is the starting reference — EM Staff files are the ghost identity documents, department folders represent operational domains.

**Expanded scope beyond original ORG-01 to ORG-04:** Nathan explicitly requested folding EM Staff YAML enrichment and routine ownership into this phase. This connects the ghost identity documents to the living infrastructure (memories table, standing orders, scheduling).

</domain>

<decisions>
## Implementation Decisions

### Teams & Team Members (ORG-01)
- **D-01:** Create `teams` table with: id SERIAL, name VARCHAR(128), department_id INTEGER REFERENCES departments(id), lead_id VARCHAR(64) REFERENCES agents(id), area_id INTEGER REFERENCES areas(id), description TEXT, created_at TIMESTAMPTZ.
- **D-02:** Create `team_members` junction table: team_id INTEGER REFERENCES teams(id), agent_id VARCHAR(64) REFERENCES agents(id), role_in_team VARCHAR(64), joined_at TIMESTAMPTZ DEFAULT now(), PRIMARY KEY (team_id, agent_id).
- **D-03:** Seed teams from existing department structure in `Areas/Eckenrode Muziekopname/`: Engineering, Content & Brand, Creative (Art), Legal, Music (Musicology), Strategy (Success), Executive, Operations. Map to departments table from Phase 18.

### Ghost Relationships (ORG-02)
- **D-04:** Create `ghost_relationships` table: id SERIAL, from_agent VARCHAR(64) REFERENCES agents(id), to_agent VARCHAR(64) REFERENCES agents(id), relationship_type VARCHAR(32), created_at TIMESTAMPTZ. CHECK constraint on relationship_type: reports_to, mentor, mentee, collaborators, liaises_with.
- **D-05:** Migrate existing text columns from agents table: `reports_to`, `mentor`, `mentee`, `collaborators`, `liaises_with` → typed rows in ghost_relationships. Parse text values (some are agent IDs, some are display names) and match to agents.id.
- **D-06:** Keep original text columns for reference (not dropped this phase), same pattern as goals.project in Phase 17.

### Agent Areas (ORG-03)
- **D-07:** Create `agent_areas` junction table: agent_id VARCHAR(64) REFERENCES agents(id), area_id INTEGER REFERENCES areas(id), role VARCHAR(64), PRIMARY KEY (agent_id, area_id).
- **D-08:** Backfill from department→area mapping. Cross-functional agents (Nova, Sarah Lin) get multiple area assignments.

### Identity Aliases (ORG-04)
- **D-09:** Add `aliases TEXT[]` column to agents table. Backfill Nova with `{'T.A.S.K.S.'}`.
- **D-10:** Other known aliases to seed: any dual-identity ghosts documented in EM Staff files.

### EM Staff Document Enrichment (EXPANDED)
- **D-11:** Update the 64 EM Staff documents in `Areas/Eckenrode Muziekopname/EM Staff/` with YAML frontmatter that includes:
  - `agent_id` — FK to agents table
  - `memory_column` — name of their column in the memories table (e.g., `nova_memories`, `eliana_memories`)
  - `department` — canonical department name
  - `team` — team name
  - `area` — primary area assignment
- **D-12:** The document_id FK already exists on agents table (`document_id INTEGER REFERENCES documents(id)`) — verify all 64 agents have their EM Staff document linked. Backfill any missing links.
- **D-13:** The `document_path` column on agents already exists — verify it points to the correct `Areas/Eckenrode Muziekopname/EM Staff/{Name}.md` path.

### Routine Ownership (EXPANDED)
- **D-14:** Create `routines` table: id SERIAL, name VARCHAR(256), owner_agent VARCHAR(64) REFERENCES agents(id), department_id INTEGER REFERENCES departments(id), project_id INTEGER REFERENCES projects(id), schedule JSONB (cron expressions, same format as projects.schedule), description TEXT, status VARCHAR(32) DEFAULT 'active', tool_label VARCHAR(128) (maps to tool-registry entries), created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ.
- **D-15:** Seed routines from existing standing order labels. Current standing orders live on projects.schedule — routines are the ghost-owned view of those same schedules. Example: Nova's "Daily Health Check" from Project #14 becomes a routine owned by nova, department Operations, project_id=14, schedule={"expr":"0 13 * * *","label":"Daily Health Check"}, tool_label="daily-health-check".
- **D-16:** Routines are NOT a replacement for standing orders — they are a formalization. The tick engine still reads schedules from projects.schedule. Routines add ghost ownership, department context, and a queryable registry of who owns what recurring work.
- **D-17:** The `(agent){action}` Innate expressions in `Templates/Daily Note.md` are the reference pattern for how routines are expressed in templates. The routines table is the database backing for these expressions.

### Claude's Discretion
- Index choices for new junction tables
- Exact parsing strategy for text relationship columns
- Whether to add API endpoints for teams/relationships/routines in this phase or defer to a follow-up
- Migration script structure and ordering

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### EM Staff Documents (Ghost Identity Source)
- `Areas/Eckenrode Muziekopname/EM Staff/*.md` — 64 ghost identity documents in documents table (ids 60271-60334+). Read 2-3 to understand current YAML structure.

### Templates (Routine Expression Reference)
- `Templates/Daily Note.md` — document id 11651. Contains `(agent){action}` Innate expressions showing how routines are expressed. This is the target pattern.

### Department Structure (Navigation Reference)
- `Areas/Eckenrode Muziekopname/` — department folders: Engineering, ContentandBrandingOffice, Art, Legal, Musicology, Success, Executive, Discord, Projects, Thought Police

### Schema Dependencies
- `.planning/REQUIREMENTS.md` — ORG-01 through ORG-04
- Phase 18 departments table — 8 canonical entries, agents.department_id FK
- Phase 16 areas table — 5 seeded domains
- Phase 12 standing orders — projects.schedule JSONB with cron expressions + labels

### Existing Code
- `/opt/project-noosphere-ghosts/config/tool-registry.json` — 9 registered tools (routines map to these)
- `/opt/dpn-api/src/handlers/af64_perception.rs` — perception endpoint (may need routine awareness)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `departments` table (Phase 18) — 8 entries, ready for team FK
- `areas` table (Phase 16) — 5 entries, ready for team/agent FK
- `agents.document_id` and `agents.document_path` — already exist for linking to EM Staff docs
- `agents.reports_to`, `mentor`, `mentee`, `collaborators`, `liaises_with` — text columns to migrate
- `projects.schedule` JSONB — existing cron format to reuse for routines
- Standing order infrastructure (Phase 12) — tool-registry.json with 9 tools, label-to-tool mapping

### Current EM Staff Document Format
```yaml
# (from SarahLin.md / ElianaRiviera.md)
Role: [text]
Department: [text]
Team: [text]
City: [text]
Timezone: [text]
Joined: [date]
Email: [text]
```
No structured YAML frontmatter yet — content is markdown with `##` headers. Enrichment adds proper `---` delimited frontmatter with agent_id, memory_column, etc.

### Memory Column Pattern
64 per-agent columns on memories table: `{agent_slug}_memories` (e.g., `nova_memories`, `eliana_memories`, `sarah_lin_memories`). Each ghost's YAML should point to their specific column.

</code_context>

<specifics>
## Specific Ideas

- The `Areas/Eckenrode Muziekopname/` document tree is the organizational "folder" metaphor Nathan uses — department folders map to teams, EM Staff files are ghost identities
- The Daily Note template `(agent){action}` expressions are Innate syntax that will eventually be evaluated — for now, routines table provides the database backing
- Standing orders (project-level schedules) remain the tick engine's scheduling source — routines are the ghost-ownership layer on top
- Nathan explicitly said "each of those [Areas/*] deserve their own table" — this is a DEFERRED idea for a future phase where department-specific tables or views provide structured access to department content. Phase 19 focuses on org structure, not department content tables.

</specifics>

<deferred>
## Deferred Ideas

- **Department content tables** — Nathan noted Areas/* folders each deserve their own table. This is a content organization concern, not an org structure concern. Belongs in a future phase or v1.4.
- **GitHub repo links on projects + issues→tasks sync** — Nathan requested project.repo_url column and GitHub issue import. Noted for backlog.
- **API endpoints for org structure** — GET /api/teams, GET /api/routines etc. Could be added here or in v1.4 frontend phase.
- **Routine evaluation by tick engine** — Currently routines are a registry. Making the tick engine read routines (instead of/alongside projects.schedule) is a future integration.

</deferred>

---

*Phase: 19-ghost-organizational-structure*
*Context gathered: 2026-03-28*
