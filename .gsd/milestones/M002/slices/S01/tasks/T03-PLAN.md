---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T03: Get-by-ID endpoint returning full row

Add GET /api/{table}/{id} handler. Returns full row as JSON (SELECT * WHERE id=$1). 404 if not found. All columns returned including NULLs (omitted from JSON). Uses the generic map[string]any scanning from T01.

## Inputs

- `T01 GetByID function`
- `T02 router wiring`

## Expected Output

- `Updated dragonpunk/internal/api/tables.go`

## Verification

curl /api/the_work/1 returns full row JSON; curl /api/the_work/999999 returns 404
