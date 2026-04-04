---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T04: Error handling + integration verification

Add structured JSON error responses: 404 for unknown table or missing row, 400 for invalid params (bad limit, non-numeric id). Test all 14 tables return data. Verify pagination works on the_links (370K rows) without timeout.

## Inputs

- `T02+T03 handlers`

## Expected Output

- `dragonpunk/internal/api/errors.go`
- `All 14 tables verified via curl`

## Verification

curl /api/invalid returns 404 JSON; curl /api/the_links?limit=10 returns in <1s; all 14 tables return rows via curl loop
