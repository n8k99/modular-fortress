---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T02: CUD HTTP handlers + routes

Add POST /api/{table} (create), PATCH /api/{table}/{id} (update), DELETE /api/{table}/{id} (delete) handlers. Create and Update accept JSON body with field:value pairs. Create returns 201 with new row. Update returns 200 with updated row. Delete returns 200 with {deleted:true, id:N}. Wire routes.

## Inputs

- `T01 query functions`

## Expected Output

- `Updated tables.go and router.go`

## Verification

Full CUD cycle via curl on the_work
