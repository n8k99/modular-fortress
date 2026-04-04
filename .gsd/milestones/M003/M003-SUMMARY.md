---
id: M003
title: "Dragonpunk Write API — Create, Update, Delete, Move"
status: complete
completed_at: 2026-04-04T23:13:19.642Z
key_decisions:
  - Column validation from information_schema at startup
  - Transactional move with SELECT FOR UPDATE
  - Protected columns: id, created_at never writable
key_files:
  - dragonpunk/internal/db/query.go
  - dragonpunk/internal/db/columns.go
  - dragonpunk/internal/api/write.go
lessons_learned:
  - Move was simple enough to include with CUD — don't over-slice
  - Column validation at startup is cheap and prevents runtime SQL errors from bad field names
---

# M003: Dragonpunk Write API — Create, Update, Delete, Move

**Full CRUD+Move for all Nine Tables — generic write layer with column validation and transactional cross-table move**

## What Happened

Built generic write operations on top of M002's read API. Column metadata loaded from information_schema at startup for validation. Dynamic INSERT/UPDATE from arbitrary JSON fields. Transactional move with SELECT FOR UPDATE locking. 9-step lifecycle test passed: Create→Read→Update→Read→Move→Verify source→Verify target→Delete→Verify gone. R009 validated.

## Success Criteria Results

All criteria met — CRUD+Move verified with full lifecycle test against live data.

## Definition of Done Results

- [x] Create endpoint for all domain tables\n- [x] Update with partial field updates + auto updated_at\n- [x] Delete with 404 for missing\n- [x] Move with transactional safety\n- [x] Column validation rejects invalid fields

## Requirement Outcomes

R009: validated — full CRUD+Move lifecycle verified\nR001: advanced — Dragonpunk now has complete data API

## Deviations

None.

## Follow-ups

None.
