# S02: Nine Tables Schema Validation + Migration Runner

**Goal:** Validate Nine Tables schema is clean — 14 tables only, legacy dropped, permissions correct
**Demo:** After this: Schema SQL scripts run on fresh empty PostgreSQL database; Go binary confirms all 12 tables exist

## Tasks
- [x] **T01: Granted nebulab_user access to all 14 Nine Tables and dropped 83 legacy tables** — Nine Tables existed but were owned by nathaneckenrode — nebulab_user had no access. Grant permissions, drop all 83 legacy tables + firehose view.
  - Estimate: 10min
  - Verify: psql shows 14 tables; curl /api/health returns table_count:14
