---
estimated_steps: 1
estimated_files: 3
skills_used: []
---

# T01: GetBySlug query + slug endpoint

Add GetBySlug to db/query.go (SELECT * WHERE slug=$1). Add GET /api/{table}/slug/{slug} handler. 404 if not found. Must not conflict with GET /api/{table}/{id} route — slug route uses explicit /slug/ prefix.

## Inputs

- `S01 query infrastructure`

## Expected Output

- `Updated query.go and tables.go`

## Verification

curl /api/the_realms/slug/some-known-slug returns row
