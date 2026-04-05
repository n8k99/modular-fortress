---
estimated_steps: 1
estimated_files: 3
skills_used: []
---

# T01: Create DbService with Wails bindings

Create a new Go service struct that wraps the existing dragonpunk database code for Wails binding. The service needs to: (1) load config from .env using existing dragonpunk/internal/config, (2) connect to PostgreSQL using existing dragonpunk/internal/db, (3) expose methods callable from TypeScript: Health() returns db status, ListTables() returns table names with row counts, ListKinds(table) returns kind breakdown for a table. Import the existing packages from dragonpunk/internal/ — do NOT duplicate the database code.

## Inputs

- `dragonpunk/internal/db/db.go`
- `dragonpunk/internal/db/query.go`
- `dragonpunk/internal/config/config.go`

## Expected Output

- `dbservice.go with Health, ListTables, ListKinds methods`
- `Updated main.go registering DbService`
- `go.mod with workspace or replace directive for dragonpunk/internal`

## Verification

go build compiles with no errors, binding generation succeeds
