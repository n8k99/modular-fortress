# S01: Generic List + Get endpoints for all Nine Tables

**Goal:** Generic read handlers serving list (paginated, filterable) and get-by-id for all 14 Nine Tables
**Demo:** After this: curl /api/the_work?kind=task&limit=10 returns paginated task list from live data

## Tasks
- [x] **T01: Table whitelist + generic row scanning with per-table column selection** — Create internal/tables package with ValidTable() whitelist of all 14 Nine Table names. Create internal/db/query.go with generic List and GetByID functions that use pgx.CollectRows with dynamic SELECT * and return []map[string]any. Table name is validated against whitelist before interpolation into SQL (safe because whitelist, not user input). Handles NULL columns gracefully.
  - Estimate: 20min
  - Files: dragonpunk/internal/tables/tables.go, dragonpunk/internal/db/query.go
  - Verify: go build ./... compiles; unit test confirms whitelist rejects invalid names
- [x] **T02: List endpoint with pagination, kind filter, and text search across all 14 tables** — Add GET /api/{table} handler. Query params: limit (default 50, max 200), offset (default 0), kind (exact match on kind column), q (ILIKE search on title and body). Returns JSON: {table, total, limit, offset, rows: [...]}. Total count via separate COUNT query for pagination. List query selects core columns only for performance: id, slug, kind, title, status, created_at, updated_at.
  - Estimate: 20min
  - Files: dragonpunk/internal/api/tables.go, dragonpunk/internal/api/router.go
  - Verify: curl /api/the_work?limit=5 returns 5 rows with total count; curl /api/the_work?kind=task filters correctly; curl /api/the_realms?q=orbis finds matching rows
- [x] **T03: Get-by-ID endpoint returning full row as JSON with 404 handling** — Add GET /api/{table}/{id} handler. Returns full row as JSON (SELECT * WHERE id=$1). 404 if not found. All columns returned including NULLs (omitted from JSON). Uses the generic map[string]any scanning from T01.
  - Estimate: 10min
  - Files: dragonpunk/internal/api/tables.go
  - Verify: curl /api/the_work/1 returns full row JSON; curl /api/the_work/999999 returns 404
- [x] **T04: Error handling verified; all 14 tables confirmed working with pagination under 25ms on large tables** — Add structured JSON error responses: 404 for unknown table or missing row, 400 for invalid params (bad limit, non-numeric id). Test all 14 tables return data. Verify pagination works on the_links (370K rows) without timeout.
  - Estimate: 15min
  - Files: dragonpunk/internal/api/errors.go, dragonpunk/internal/api/tables.go
  - Verify: curl /api/invalid returns 404 JSON; curl /api/the_links?limit=10 returns in <1s; all 14 tables return rows via curl loop
