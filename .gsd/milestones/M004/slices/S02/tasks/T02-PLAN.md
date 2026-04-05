---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T02: Wire frontend to display live DB data

Update the TypeScript frontend to call DbService bindings and render the results. On load: call Health() and display connection status. Then call ListTables() and render a table list with row counts. Each table name is clickable — clicking calls ListKinds(table) and shows the kind breakdown below.

## Inputs

- `T01 DbService bindings`

## Expected Output

- `Frontend displaying live Nine Tables data`
- `Interactive kind breakdown on table click`

## Verification

wails3 build succeeds, app shows live table data from master_chronicle
