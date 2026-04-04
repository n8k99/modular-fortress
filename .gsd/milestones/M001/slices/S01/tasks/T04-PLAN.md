---
estimated_steps: 1
estimated_files: 3
skills_used: []
---

# T04: Health endpoint handler + router

Create /api/health handler that queries SELECT count(*) FROM information_schema.tables WHERE table_schema='public' and returns JSON: {status, db_connected, table_count, timestamp}. Wire up chi or stdlib router with structured request logging middleware. Graceful shutdown on SIGINT/SIGTERM.

## Inputs

- `T02 config`
- `T03 db pool`

## Expected Output

- `dragonpunk/internal/api/health.go`
- `dragonpunk/internal/api/router.go`

## Verification

go run ./dragonpunk, then curl localhost:PORT/api/health returns valid JSON with db_connected:true and table_count:85
