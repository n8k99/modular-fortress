# Requirements

This file is the explicit capability and coverage contract for the project.

## Active

### R001 — Go membrane API server replaces Rust noosphere/dpn-api as the user-facing HTTP layer (Dragonpunk refactor)
- Class: functional
- Status: active
- Description: Go membrane API server replaces Rust noosphere/dpn-api as the user-facing HTTP layer (Dragonpunk refactor)
- Why it matters: v2.0 architecture mandates Go for all user-facing I/O; Rust is legacy v1.x
- Source: Modular Fortress.md — Three-Pillar Architecture
- Primary owning slice: Go Membrane
- Validation: Go binary serves /api/health, connects to master_chronicle, returns system stats

### R002 — Nine Tables schema (9 domain + 3 infrastructure) installable on a fresh PostgreSQL instance
- Class: functional
- Status: active
- Description: Nine Tables schema (9 domain + 3 infrastructure) installable on a fresh PostgreSQL instance
- Why it matters: Fresh-droplet acceptance test requires schema creation from scratch with no pre-existing data
- Source: Modular Fortress.md — The Generalization Requirement
- Primary owning slice: Schema
- Validation: Schema SQL scripts run cleanly on empty PostgreSQL 16+ database; all 12 tables created

### R004 — T.A.S.K.S. — permanent onboard COO ghost, first ghost in every noosphere, cannot be deleted
- Class: functional
- Status: active
- Description: T.A.S.K.S. — permanent onboard COO ghost, first ghost in every noosphere, cannot be deleted
- Why it matters: User's lifeline when stuck; permanent help desk with full system documentation in context
- Source: Modular Fortress.md — T.A.S.K.S.
- Primary owning slice: Ghost Runtime
- Validation: Fresh install includes T.A.S.K.S. ghost; deletion attempt is rejected; rename allowed

### R005 — Control surface UI — command table with Scene Layer (persistent backdrop) and Panel Layer (floating overlay windows)
- Class: functional
- Status: active
- Description: Control surface UI — command table with Scene Layer (persistent backdrop) and Panel Layer (floating overlay windows)
- Why it matters: Not a dashboard — a full CRUD command table inspired by Foundry VTT's GM interface
- Source: Modular Fortress.md — Control Surface
- Primary owning slice: UI
- Validation: TypeScript UI renders scene layer + panel layer; left sidebar with 6 fixed items functional

### R007 — Lisp ghost runtime (AF64) operates via tick engine with perception → cognition → action → reporting cycle
- Class: functional
- Status: active
- Description: Lisp ghost runtime (AF64) operates via tick engine with perception → cognition → action → reporting cycle
- Why it matters: Core artificial life architecture — ghosts must tick autonomously, not just respond to requests
- Source: ARCHITECTURE.md — Ghost Runtime; Modular Fortress.md — Ghost Runtime
- Primary owning slice: Ghost Runtime
- Validation: AF64 tick executes successfully against fresh Nine Tables schema; at least T.A.S.K.S. ghost runs a full cycle

### R009 — Dragonpunk CRUD+Edit+Move — full content lifecycle including rich editing of stubs and cross-table relocation
- Class: functional
- Status: active
- Description: Dragonpunk CRUD+Edit+Move — full content lifecycle including rich editing of stubs and cross-table relocation
- Why it matters: Migration placed some entries in wrong tables. Rather than auditing now, the API should let Nathan fix misplacements as he encounters them during normal use.
- Source: Post-migration observation — Nathan, 2026-04-04
- Primary owning slice: Dragonpunk
- Validation: API endpoint can move a row from e.g. the_commons to the_work, preserving data and assigning correct kind
- Notes: Many migrated entries (especially worldbuilding in the_realms, the_chronicles) are established stubs needing expansion. Edit workflow is a first-class operation, not just field-level update. Move handles cross-table relocation with kind reassignment.

## Deferred

### R008 — All code produced by the system is AGPL-licensed — ghosts, tools, modules
- Class: non-functional
- Status: deferred
- Description: All code produced by the system is AGPL-licensed — ghosts, tools, modules
- Why it matters: The AGPL Covenant — Four Freedoms apply to AI assistants same as compilers
- Source: Modular Fortress.md — The AGPL Covenant
- Primary owning slice: Go Membrane
- Validation: AGPL license headers present; generated code includes license; LICENSE file at repo root
- Notes: License is GPL per CLAUDE.md constraints, but AGPL public covenant is moot for a private project. License headers are nice-to-have, not blocking.

## Out of Scope

### R003 — Fresh-droplet install script: single command installs PostgreSQL, schema, Go membrane, ghost runtime, and InnateScipt on a clean machine
- Class: functional
- Status: out-of-scope
- Description: Fresh-droplet install script: single command installs PostgreSQL, schema, Go membrane, ghost runtime, and InnateScipt on a clean machine
- Why it matters: Core acceptance test for v2.0 — if install fails on a clean machine, v2.0 is not ready
- Source: Modular Fortress.md — Install Experience
- Primary owning slice: Install/Deploy
- Validation: Fresh Ubuntu droplet + install script = working noosphere with browser onboarding link
- Notes: Project is no longer public. No fresh-droplet install script needed — Nathan's existing infrastructure is the target.

### R006 — Zero hardcoded Nathan-specific infrastructure — all paths, credentials, and service URLs from config or .env
- Class: non-functional
- Status: out-of-scope
- Description: Zero hardcoded Nathan-specific infrastructure — all paths, credentials, and service URLs from config or .env
- Why it matters: Generalization requirement: binary must know nothing about Nathan's specific setup
- Source: Modular Fortress.md — The Generalization Requirement
- Primary owning slice: Go Membrane
- Supporting slices: Ghost Runtime, InnateScipt
- Validation: Grep codebase for hardcoded IPs, paths, usernames; zero Nathan-specific assumptions remain
- Notes: Generalization requirement dropped — project is private, single-user, built for Nathan's infrastructure. Hardcoded paths are fine if they're in config/.env.

## Traceability

| ID | Class | Status | Primary owner | Supporting | Proof |
|---|---|---|---|---|---|
| R001 | functional | active | Go Membrane | none | Go binary serves /api/health, connects to master_chronicle, returns system stats |
| R002 | functional | active | Schema | none | Schema SQL scripts run cleanly on empty PostgreSQL 16+ database; all 12 tables created |
| R003 | functional | out-of-scope | Install/Deploy | none | Fresh Ubuntu droplet + install script = working noosphere with browser onboarding link |
| R004 | functional | active | Ghost Runtime | none | Fresh install includes T.A.S.K.S. ghost; deletion attempt is rejected; rename allowed |
| R005 | functional | active | UI | none | TypeScript UI renders scene layer + panel layer; left sidebar with 6 fixed items functional |
| R006 | non-functional | out-of-scope | Go Membrane | Ghost Runtime, InnateScipt | Grep codebase for hardcoded IPs, paths, usernames; zero Nathan-specific assumptions remain |
| R007 | functional | active | Ghost Runtime | none | AF64 tick executes successfully against fresh Nine Tables schema; at least T.A.S.K.S. ghost runs a full cycle |
| R008 | non-functional | deferred | Go Membrane | none | AGPL license headers present; generated code includes license; LICENSE file at repo root |
| R009 | functional | active | Dragonpunk | none | API endpoint can move a row from e.g. the_commons to the_work, preserving data and assigning correct kind |

## Coverage Summary

- Active requirements: 6
- Mapped to slices: 6
- Validated: 0
- Unmapped active requirements: 0
