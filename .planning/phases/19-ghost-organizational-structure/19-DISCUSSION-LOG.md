# Phase 19: Ghost Organizational Structure - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-28
**Phase:** 19-ghost-organizational-structure
**Areas discussed:** Teams, Relationships, Agent Areas, Aliases, EM Staff Enrichment, Routine Ownership
**Mode:** --auto with user expansion input

---

## Scope Expansion (User-Directed)

Nathan explicitly requested folding three additional capabilities into Phase 19:

1. **EM Staff document enrichment** — 64 ghost files in `Areas/Eckenrode Muziekopname/EM Staff/` get YAML frontmatter with agent_id, memory_column, department, team, area
2. **Routine ownership** — New `routines` table formalizing ghost-owned recurring work, backed by the `(agent){action}` Innate pattern from `Templates/Daily Note.md`
3. **Scheduling integration** — Routines carry cron schedules in same JSONB format as projects.schedule

Additionally noted (DEFERRED): "each of those Areas/* deserve their own table" — captured for future phase.

---

## Teams (ORG-01)

**User's choice:** [auto] Teams table with department_id FK, seeded from EM department folder structure
**Notes:** 8 teams matching 8 canonical departments from Phase 18

## Relationships (ORG-02)

**User's choice:** [auto] Typed rows replacing text arrays, keep original columns
**Notes:** Parse existing text values in reports_to, mentor, mentee, collaborators, liaises_with

## Agent Areas (ORG-03)

**User's choice:** [auto] Junction table with role, cross-functional agents get multiple assignments

## Aliases (ORG-04)

**User's choice:** [auto] TEXT[] column, seed Nova with T.A.S.K.S.

## EM Staff Enrichment (EXPANDED)

**User's choice:** User-directed. Add YAML frontmatter to all 64 EM Staff documents with structured links to memories table columns.

## Routine Ownership (EXPANDED)

**User's choice:** User-directed. Routines table as ghost-owned view of standing order schedules. NOT replacing standing orders — layering ownership on top.

---

## Claude's Discretion

- Index choices, API endpoints (add or defer), migration ordering, text parsing strategy

## Deferred Ideas

- Department content tables (Areas/* each get their own table)
- GitHub repo links + issues→tasks sync
- API endpoints for org structure
- Tick engine reading routines directly
