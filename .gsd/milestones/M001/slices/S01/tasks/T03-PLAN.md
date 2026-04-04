---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T03: PostgreSQL connection pool via pgx

Create db package with NewPool(databaseURL) returning *pgxpool.Pool. Ping on creation to verify connectivity. Connection pool size 10 to match existing config.json convention.

## Inputs

- `T02 config output`
- `master_chronicle on localhost:5432`

## Expected Output

- `dragonpunk/internal/db/db.go`

## Verification

go test ./internal/db/... passes (requires live DB)
