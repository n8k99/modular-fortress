# S02: Slug lookup + kind listing

**Goal:** Slug-based lookup and kind enumeration for each table
**Demo:** After this: curl /api/the_realms/slug/orbis returns the Orbis realm; curl /api/the_work/kinds returns list of distinct kind values

## Tasks
- [x] **T01: Slug lookup endpoint — GET /api/{table}/slug/{slug} returns full row** — Add GetBySlug to db/query.go (SELECT * WHERE slug=$1). Add GET /api/{table}/slug/{slug} handler. 404 if not found. Must not conflict with GET /api/{table}/{id} route — slug route uses explicit /slug/ prefix.
  - Estimate: 10min
  - Files: dragonpunk/internal/db/query.go, dragonpunk/internal/api/tables.go, dragonpunk/internal/api/router.go
  - Verify: curl /api/the_realms/slug/some-known-slug returns row
- [x] **T02: Kinds endpoint — GET /api/{table}/kinds returns distinct kind values with counts** — Add ListKinds to db/query.go (SELECT kind, count(*) GROUP BY kind ORDER BY count DESC). Add GET /api/{table}/kinds handler. Returns JSON array of {kind, count}. Tables without kind column (infrastructure) return 404 or empty.
  - Estimate: 10min
  - Files: dragonpunk/internal/db/query.go, dragonpunk/internal/api/tables.go, dragonpunk/internal/api/router.go
  - Verify: curl /api/the_work/kinds returns kind list with task count=2554
