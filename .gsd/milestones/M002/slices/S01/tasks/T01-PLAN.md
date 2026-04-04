---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T01: Table whitelist + generic row scanning

Create internal/tables package with ValidTable() whitelist of all 14 Nine Table names. Create internal/db/query.go with generic List and GetByID functions that use pgx.CollectRows with dynamic SELECT * and return []map[string]any. Table name is validated against whitelist before interpolation into SQL (safe because whitelist, not user input). Handles NULL columns gracefully.

## Inputs

- `M001 db package`
- `Schema column inspection from planning`

## Expected Output

- `dragonpunk/internal/tables/tables.go`
- `dragonpunk/internal/db/query.go`

## Verification

go build ./... compiles; unit test confirms whitelist rejects invalid names
