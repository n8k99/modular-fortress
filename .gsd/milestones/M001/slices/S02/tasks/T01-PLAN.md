---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T01: Grant nebulab_user access to Nine Tables and drop legacy tables

Nine Tables existed but were owned by nathaneckenrode — nebulab_user had no access. Grant permissions, drop all 83 legacy tables + firehose view.

## Inputs

- `Existing Nine Tables owned by nathaneckenrode`
- `83 legacy tables from dump restore`

## Expected Output

- `14 Nine Tables with nebulab_user access`
- `0 legacy tables`

## Verification

psql shows 14 tables; curl /api/health returns table_count:14
