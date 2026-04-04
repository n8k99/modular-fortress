# M002: Dragonpunk Read API — Browse the Nine Tables

## Vision
Expose all 14 Nine Tables as browsable JSON endpoints. Generic handler pattern using the shared polymorphic columns (id, slug, kind, title, body, status, created_at). Nathan can list, filter by kind, get by id/slug, and search across any table. Foundation for the full CRUD+Edit+Move in M003.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Generic List + Get endpoints for all Nine Tables | medium — dynamic sql with table name whitelisting | — | ✅ | curl /api/the_work?kind=task&limit=10 returns paginated task list from live data |
| S02 | Slug lookup + kind listing | low | S01 | ✅ | curl /api/the_realms/slug/orbis returns the Orbis realm; curl /api/the_work/kinds returns list of distinct kind values |
