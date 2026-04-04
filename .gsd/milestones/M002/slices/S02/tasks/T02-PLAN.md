---
estimated_steps: 1
estimated_files: 3
skills_used: []
---

# T02: Kinds endpoint — distinct kind values with counts

Add ListKinds to db/query.go (SELECT kind, count(*) GROUP BY kind ORDER BY count DESC). Add GET /api/{table}/kinds handler. Returns JSON array of {kind, count}. Tables without kind column (infrastructure) return 404 or empty.

## Inputs

- `S01 whitelist`

## Expected Output

- `Updated query.go and tables.go`

## Verification

curl /api/the_work/kinds returns kind list with task count=2554
