# Roadmap: Modular Fortress

**Project:** Modular Fortress v2.0 (Rust → Go Migration + Nine Tables Schema)
**Created:** 2026-04-04
**Granularity:** Coarse (7 phases, 1-3 plans each)
**Strategy:** Sequential migration (Go first, schema second)

## Overview

This is a **dual sequential migration**: rewrite Rust API in Go using existing 83-table schema, then migrate to Nine Tables schema once Go is stable. Sequential approach prevents blame ambiguity—if bugs appear during Go migration, it's the language; if during schema migration, it's the schema.

**Phase Group 1 (Phases 1-5):** Go rewrite with existing 83-table schema
**Phase Group 2 (Phases 6-7):** Nine Tables schema migration after Go proven

## Phases

- [ ] **Phase 1: Foundation & Go API Core** - Go server builds, connects to PostgreSQL, health endpoint, basic CRUD patterns established
- [ ] **Phase 2: PIM Features (Notes, Tasks, Calendar)** - Core personal information management features migrated to Go
- [ ] **Phase 3: RSS & Wikilinks** - RSS reader and cross-domain wikilink graph implemented in Go
- [ ] **Phase 4: Ghost Integration & Auth** - Ghost management interface, passkey authentication, real-time layer
- [ ] **Phase 5: UI & Cutover** - Scene-based interface, Rust retirement, Go becomes sole production API
- [ ] **Phase 6: Nine Tables Schema** - Schema design, migration scripts, shadow-write validation
- [ ] **Phase 7: Schema Cutover & Validation** - Gradual cutover to Nine Tables, 83-table retirement

## Phase Details

### Phase 1: Foundation & Go API Core
**Goal**: Go API server operational with database connectivity and basic CRUD patterns established
**Depends on**: Nothing (first phase)
**Requirements**: MIGR-01, MIGR-02, MIGR-03, MIGR-07
**Success Criteria** (what must be TRUE):
  1. Developer can build and run Go API server on localhost with health endpoint responding
  2. Go server connects to master_chronicle PostgreSQL using pgx/v5 connection pool
  3. All database credentials and API keys load from `.env` file (nothing hardcoded)
  4. Nginx routes traffic between Go (8080) and Rust (8888) for strangler fig migration
**Plans**: TBD

### Phase 2: PIM Features (Notes, Tasks, Calendar)
**Goal**: Users can manage notes, tasks, and calendar events through Go API
**Depends on**: Phase 1
**Requirements**: NOTE-01, NOTE-02, NOTE-03, NOTE-04, NOTE-05, NOTE-06, NOTE-07, NOTE-08, NOTE-09, NOTE-10, TASK-01, TASK-02, TASK-03, TASK-04, TASK-05, TASK-06, TASK-07, TASK-08, CALE-01, CALE-02, CALE-03, CALE-04, CALE-05, CALE-06, CALE-07
**Success Criteria** (what must be TRUE):
  1. User can create, edit, delete, and search notes with wikilinks and backlinks working
  2. User can create tasks with due dates, projects, completion status, and dependency chains
  3. User can view calendar in day/week/month views and create/edit/delete events including recurring events
  4. All CRUD operations match Rust API behavior exactly (automated parity tests pass)
**Plans**: TBD
**UI hint**: yes

### Phase 3: RSS & Wikilinks
**Goal**: RSS reader functional and wikilinks work across all domains
**Depends on**: Phase 2
**Requirements**: RSS-01, RSS-02, RSS-03, RSS-04, RSS-05, RSS-06, RSS-07, WIKI-01, WIKI-02, WIKI-03, WIKI-04, WIKI-05, WIKI-06
**Success Criteria** (what must be TRUE):
  1. User can subscribe to RSS feeds and view unread articles with read/starred states
  2. User can search across all RSS articles and open articles in reading view
  3. Wikilinks connect notes, tasks, calendar events, and RSS articles using unified syntax
  4. Graph view visualizes wikilink connections across all domains
**Plans**: TBD
**UI hint**: yes

### Phase 4: Ghost Integration & Auth
**Goal**: Ghost management interface operational with secure passkey authentication
**Depends on**: Phase 3
**Requirements**: GHST-01, GHST-02, GHST-03, GHST-04, GHST-05, GHST-06, GHST-07, AUTH-01, AUTH-02, AUTH-03, AUTH-04, AUTH-05
**Success Criteria** (what must be TRUE):
  1. User can view ghost list, profiles, tick history, and memory state through Go API
  2. User can send messages to ghosts and receive ghost messages in unified inbox
  3. User can register and sign in with passkey (WebAuthn) with no OAuth dependencies
  4. Real-time updates flow from Lisp ghost ticks → PostgreSQL LISTEN/NOTIFY → Go WebSocket → UI
**Plans**: TBD
**UI hint**: yes

### Phase 5: UI & Cutover
**Goal**: Scene-based UI complete and Rust codebase retired
**Depends on**: Phase 4
**Requirements**: MIGR-06, UI-01, UI-02, UI-03, UI-04, UI-05, UI-06
**Success Criteria** (what must be TRUE):
  1. User navigates Foundry VTT-style scene-based interface with 6-item sidebar and persistent panels
  2. All features accessible through scene/panel layers with dark terminal theme
  3. Nginx routes 100% traffic to Go (Rust server stopped)
  4. Rust codebase archived with git tag `rust-legacy-final`
**Plans**: TBD
**UI hint**: yes

### Phase 6: Nine Tables Schema
**Goal**: Nine Tables schema designed, tested, and ready for migration
**Depends on**: Phase 5 (Go must be stable before schema migration)
**Requirements**: MIGR-04, MIGR-05
**Success Criteria** (what must be TRUE):
  1. Nine Tables schema created alongside existing 83 tables with hybrid design (indexed columns + JSONB meta)
  2. Bidirectional migration scripts tested (83→9 and 9→83 rollback verified)
  3. Shadow-write implementation ready (Go writes to both schemas simultaneously)
  4. Reconciliation tests validate row counts and checksums for every table mapping
**Plans**: TBD

### Phase 7: Schema Cutover & Validation
**Goal**: Nine Tables schema is sole production database, 83 tables retired
**Depends on**: Phase 6
**Requirements**: MIGR-05 (validation aspect)
**Success Criteria** (what must be TRUE):
  1. Shadow-write period completes with zero reconciliation failures
  2. Application-level referential integrity validation passes (polymorphic associations verified)
  3. Go API reads/writes exclusively from Nine Tables schema
  4. 83-table schema archived (data preserved, tables marked deprecated)
**Plans**: TBD

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Foundation & Go API Core | 0/0 | Not started | - |
| 2. PIM Features | 0/0 | Not started | - |
| 3. RSS & Wikilinks | 0/0 | Not started | - |
| 4. Ghost Integration & Auth | 0/0 | Not started | - |
| 5. UI & Cutover | 0/0 | Not started | - |
| 6. Nine Tables Schema | 0/0 | Not started | - |
| 7. Schema Cutover | 0/0 | Not started | - |

## Coverage Validation

**Requirements mapped:** 54/54 ✓
**Orphaned requirements:** 0 ✓

### Phase 1 (4 requirements)
- MIGR-01, MIGR-02, MIGR-03, MIGR-07

### Phase 2 (25 requirements)
- NOTE-01 through NOTE-10 (10 requirements)
- TASK-01 through TASK-08 (8 requirements)
- CALE-01 through CALE-07 (7 requirements)

### Phase 3 (13 requirements)
- RSS-01 through RSS-07 (7 requirements)
- WIKI-01 through WIKI-06 (6 requirements)

### Phase 4 (12 requirements)
- GHST-01 through GHST-07 (7 requirements)
- AUTH-01 through AUTH-05 (5 requirements)

### Phase 5 (7 requirements)
- MIGR-06
- UI-01 through UI-06 (6 requirements)

### Phase 6 (2 requirements)
- MIGR-04, MIGR-05 (migration script creation)

### Phase 7 (1 requirement)
- MIGR-05 (validation aspect)

**Note:** MIGR-05 spans Phases 6-7 because migration script creation (Phase 6) and validation (Phase 7) are sequential deliverables for the same requirement.

## Key Decisions

| Decision | Phase | Rationale |
|----------|-------|-----------|
| Sequential migration (Go first, schema second) | All | Prevents blame ambiguity—bugs during Go migration are language issues, bugs during schema migration are schema issues. No compound debugging. |
| Strangler fig pattern with nginx | 1-5 | Route-by-route migration enables per-route rollback, learning Go patterns under low pressure. No big-bang rewrite. |
| Coarse granularity (7 phases vs 9 from research) | All | Config specifies "coarse" preference. Compressed research's 9 phases into 7 by combining related work (Foundation+API, PIM bundle, UI+Cutover). |
| Phase Group boundary at Phase 5/6 | 5-6 | Hard gate: schema migration cannot start until Go is stable. Prevents dual migration failures. |
| Hybrid schema design (indexed + JSONB) | 6 | Prevents JSONB performance cliff (2000x query slowdown). Hot query fields indexed, truly variable data in JSONB. |

## Research Flags

Phases likely needing deeper research during planning:
- **Phase 4:** LISTEN/NOTIFY latency under load (100+ concurrent WebSocket clients) not fully validated. Prototype testing recommended.
- **Phase 6:** Production data benchmarking with 9,846 conversations + 2,554 tasks required before schema finalization.

---
*Last updated: 2026-04-04 after roadmap creation*
