# M003: Dragonpunk Write API — Create, Update, Delete, Move

## Vision
Complete CRUD for all Nine Tables. Nathan can create new entries, update/expand stubs, delete rows, and move misplaced entries between tables — all through Dragonpunk's API. This is the foundation for both the TypeScript UI and ghost write operations.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Create + Update + Delete | medium — dynamic insert/update sql for polymorphic tables | — | ✅ | curl -X POST /api/the_work with JSON body creates a task; PATCH updates it; DELETE removes it |
| S02 | Move operation | medium — transactional cross-table transfer | S01 | ✅ | POST /api/the_commons/{id}/move with {target_table: the_work, kind: task} transfers the row |
