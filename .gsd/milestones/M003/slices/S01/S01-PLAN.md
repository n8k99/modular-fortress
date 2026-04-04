# S01: Create + Update + Delete

**Goal:** Generic create, update, delete handlers for all Nine Tables
**Demo:** After this: curl -X POST /api/the_work with JSON body creates a task; PATCH updates it; DELETE removes it

## Tasks
- [x] **T01: Generic Create, Update, Delete, Move query functions with column validation and transactional move** — Add to db/query.go: Create (INSERT with dynamic columns from JSON body, return new row), Update (UPDATE SET for provided fields + updated_at=now(), return updated row), Delete (DELETE WHERE id=$1, return boolean). All validate table against whitelist. Column names from JSON keys validated against actual table columns via information_schema query cached at startup.
  - Estimate: 25min
  - Files: dragonpunk/internal/db/query.go, dragonpunk/internal/db/columns.go
  - Verify: go build compiles
- [x] **T02: CUD+Move HTTP handlers wired — POST create, PATCH update, DELETE, POST move** — Add POST /api/{table} (create), PATCH /api/{table}/{id} (update), DELETE /api/{table}/{id} (delete) handlers. Create and Update accept JSON body with field:value pairs. Create returns 201 with new row. Update returns 200 with updated row. Delete returns 200 with {deleted:true, id:N}. Wire routes.
  - Estimate: 15min
  - Files: dragonpunk/internal/api/tables.go, dragonpunk/internal/api/router.go
  - Verify: Full CUD cycle via curl on the_work
- [x] **T03: Full CRUD+Move lifecycle verified: Create→Read→Update→Read→Move→Verify→Delete→Verify** — Create a test entry in the_work (kind=task, title=test), read it back, update the title, read again, delete it, confirm 404. All via curl against running server.
  - Estimate: 10min
  - Verify: Create → Read → Update → Read → Delete → 404 cycle passes
