---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T02: List endpoint with pagination, kind filter, text search

Add GET /api/{table} handler. Query params: limit (default 50, max 200), offset (default 0), kind (exact match on kind column), q (ILIKE search on title and body). Returns JSON: {table, total, limit, offset, rows: [...]}. Total count via separate COUNT query for pagination. List query selects core columns only for performance: id, slug, kind, title, status, created_at, updated_at.

## Inputs

- `T01 query functions`

## Expected Output

- `dragonpunk/internal/api/tables.go`

## Verification

curl /api/the_work?limit=5 returns 5 rows with total count; curl /api/the_work?kind=task filters correctly; curl /api/the_realms?q=orbis finds matching rows
