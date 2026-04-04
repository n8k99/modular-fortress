---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T01: Generic Create + Update + Delete query functions

Add to db/query.go: Create (INSERT with dynamic columns from JSON body, return new row), Update (UPDATE SET for provided fields + updated_at=now(), return updated row), Delete (DELETE WHERE id=$1, return boolean). All validate table against whitelist. Column names from JSON keys validated against actual table columns via information_schema query cached at startup.

## Inputs

- `M002 query infrastructure`

## Expected Output

- `dragonpunk/internal/db/columns.go`
- `Updated query.go`

## Verification

go build compiles
