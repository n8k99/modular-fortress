# Requirements: Modular Fortress

**Defined:** 2026-04-04
**Core Value:** The database is the source of truth. Every piece of personal data lives in master_chronicle and is accessible through one unified interface.

## v1 Requirements

### Migration (MIGR)

- [ ] **MIGR-01**: Go API server builds and runs on localhost with health endpoint
- [ ] **MIGR-02**: All existing Rust API endpoints replicated in Go with identical behavior
- [ ] **MIGR-03**: PostgreSQL connection pool configured with pgx/v5
- [ ] **MIGR-04**: Nine Tables schema created alongside existing 83 tables
- [ ] **MIGR-05**: Data migrated from 83 tables → Nine Tables with validation
- [ ] **MIGR-06**: Rust codebase archived, Go is sole production API
- [ ] **MIGR-07**: All secrets moved to `.env` (database credentials, API keys)

### Notes & Knowledge (NOTE)

- [ ] **NOTE-01**: User can create markdown note with title and body
- [ ] **NOTE-02**: User can edit existing notes
- [ ] **NOTE-03**: User can delete notes
- [ ] **NOTE-04**: User can view list of all notes sorted by modification date
- [ ] **NOTE-05**: User can create daily note (auto-generated from date)
- [ ] **NOTE-06**: User can insert wikilink to another note `[[Note Title]]`
- [ ] **NOTE-07**: Wikilinks autocomplete from existing notes
- [ ] **NOTE-08**: Clicking wikilink navigates to linked note
- [ ] **NOTE-09**: User can view backlinks (which notes link here)
- [ ] **NOTE-10**: Full-text search across all notes returns ranked results

### Calendar (CALE)

- [ ] **CALE-01**: User can view calendar in month/week/day views
- [ ] **CALE-02**: User can create event with title, date, time, duration
- [ ] **CALE-03**: User can create all-day event
- [ ] **CALE-04**: User can edit existing events
- [ ] **CALE-05**: User can delete events
- [ ] **CALE-06**: Events display on calendar grid at correct date/time
- [ ] **CALE-07**: User can create recurring events (daily, weekly, monthly)

### Tasks & Goals (TASK)

- [ ] **TASK-01**: User can create task with title and optional description
- [ ] **TASK-02**: User can mark task as complete/incomplete
- [ ] **TASK-03**: User can set due date on task
- [ ] **TASK-04**: User can assign task to project
- [ ] **TASK-05**: User can view all tasks filtered by status (active, complete)
- [ ] **TASK-06**: User can create goal with success criteria
- [ ] **TASK-07**: User can link tasks to goals
- [ ] **TASK-08**: Tasks blocked by other tasks cannot be started until blocker completes

### RSS Reader (RSS)

- [ ] **RSS-01**: User can add RSS feed by URL
- [ ] **RSS-02**: System fetches and stores new articles from subscribed feeds
- [ ] **RSS-03**: User can view list of unread articles
- [ ] **RSS-04**: User can mark article as read
- [ ] **RSS-05**: User can star/favorite articles for later
- [ ] **RSS-06**: Clicking article opens in reading view
- [ ] **RSS-07**: User can search across all RSS articles

### Wikilinks (WIKI)

- [ ] **WIKI-01**: Wikilinks work across all domains (notes, tasks, events, RSS)
- [ ] **WIKI-02**: User can link to task with `[[TASK-123]]` syntax
- [ ] **WIKI-03**: User can link to calendar event with `[[2026-04-15 Meeting]]` syntax
- [ ] **WIKI-04**: User can link to RSS article with `[[Article Title]]` syntax
- [ ] **WIKI-05**: Backlinks show all references regardless of source domain
- [ ] **WIKI-06**: Graph view visualizes wikilink connections across domains

### Ghost Management (GHST)

- [ ] **GHST-01**: User can view list of all ghosts with status (active, idle, dormant)
- [ ] **GHST-02**: User can view ghost profile (name, role, tier, energy, trust)
- [ ] **GHST-03**: User can send message to ghost
- [ ] **GHST-04**: Ghost messages appear in unified inbox
- [ ] **GHST-05**: User can view ghost tick history
- [ ] **GHST-06**: User can view ghost memory state
- [ ] **GHST-07**: User can pause/resume individual ghost

### Authentication (AUTH)

- [ ] **AUTH-01**: User can register with passkey (WebAuthn)
- [ ] **AUTH-02**: User can sign in with passkey
- [ ] **AUTH-03**: Session persists across browser restarts
- [ ] **AUTH-04**: User can sign out
- [ ] **AUTH-05**: No OAuth to external providers (Google, GitHub, etc.)

### Interface (UI)

- [ ] **UI-01**: Foundry VTT-style scene-based navigation (bridge views, scenes)
- [ ] **UI-02**: Left sidebar with 6 fixed items (Forge, Orbis, Journal, Reader, Scenes, Settings)
- [ ] **UI-03**: Scene layer (persistent backdrop) always visible behind panels
- [ ] **UI-04**: Panel layer (overlay windows) for tools and editors
- [ ] **UI-05**: User can switch between scenes without losing panel state
- [ ] **UI-06**: Dark theme with terminal aesthetics

## v2 Requirements

### Advanced Features

- **WIKI-07**: Graph view with force-directed layout and clustering
- **NOTE-11**: Template system with InnateScript variables
- **GHST-08**: Ghost can create notes via InnateScript
- **GHST-09**: Autonomous ghost pipelines execute on tick schedule
- **CALE-08**: Natural language event creation ("tomorrow at 3pm")
- **TASK-09**: AI-assisted task breakdown (goal → subtasks)

### Integrations

- **EXTN-01**: Hot-pluggable droplet extensions via webhook interface
- **EXTN-02**: Extension marketplace (private, Nathan-curated)
- **EXTN-03**: XMPP messaging integration (replace WhatsApp)
- **EXTN-04**: Matrix messaging integration (optional)

### Polish

- **UI-07**: Keyboard shortcuts for all major actions
- **UI-08**: Command palette (Cmd+K) for quick navigation
- **NOTE-12**: Live markdown preview
- **TASK-10**: Drag-and-drop task reordering

## Out of Scope

| Feature | Reason |
|---------|--------|
| Public release / install script | Private system for Nathan only, not building for distribution |
| Multi-user support | Single-user system simplifies auth and data model |
| Mobile apps (v1) | Desktop-first, mobile deferred until core proven |
| Cloud sync | Self-hosted only, no external dependencies |
| WYSIWYG editor | Markdown keeps format transparent and Git-compatible |
| Email client integration | Apple Mail is sufficient, keep it |
| OAuth to Google/GitHub | Passkey-only auth, no external provider dependencies |
| Real-time collaboration | Single-user system, not needed |
| AI-powered features (v1) | Ghost agents provide AI, but user-facing AI deferred to v2 |
| Plugin marketplace | Extensions yes, but curated by Nathan, not open marketplace |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| MIGR-01 | Phase 1 | Pending |
| MIGR-02 | Phase 1 | Pending |
| MIGR-03 | Phase 1 | Pending |
| MIGR-07 | Phase 1 | Pending |
| NOTE-01 | Phase 2 | Pending |
| NOTE-02 | Phase 2 | Pending |
| NOTE-03 | Phase 2 | Pending |
| NOTE-04 | Phase 2 | Pending |
| NOTE-05 | Phase 2 | Pending |
| NOTE-06 | Phase 2 | Pending |
| NOTE-07 | Phase 2 | Pending |
| NOTE-08 | Phase 2 | Pending |
| NOTE-09 | Phase 2 | Pending |
| NOTE-10 | Phase 2 | Pending |
| TASK-01 | Phase 2 | Pending |
| TASK-02 | Phase 2 | Pending |
| TASK-03 | Phase 2 | Pending |
| TASK-04 | Phase 2 | Pending |
| TASK-05 | Phase 2 | Pending |
| TASK-06 | Phase 2 | Pending |
| TASK-07 | Phase 2 | Pending |
| TASK-08 | Phase 2 | Pending |
| CALE-01 | Phase 2 | Pending |
| CALE-02 | Phase 2 | Pending |
| CALE-03 | Phase 2 | Pending |
| CALE-04 | Phase 2 | Pending |
| CALE-05 | Phase 2 | Pending |
| CALE-06 | Phase 2 | Pending |
| CALE-07 | Phase 2 | Pending |
| RSS-01 | Phase 3 | Pending |
| RSS-02 | Phase 3 | Pending |
| RSS-03 | Phase 3 | Pending |
| RSS-04 | Phase 3 | Pending |
| RSS-05 | Phase 3 | Pending |
| RSS-06 | Phase 3 | Pending |
| RSS-07 | Phase 3 | Pending |
| WIKI-01 | Phase 3 | Pending |
| WIKI-02 | Phase 3 | Pending |
| WIKI-03 | Phase 3 | Pending |
| WIKI-04 | Phase 3 | Pending |
| WIKI-05 | Phase 3 | Pending |
| WIKI-06 | Phase 3 | Pending |
| GHST-01 | Phase 4 | Pending |
| GHST-02 | Phase 4 | Pending |
| GHST-03 | Phase 4 | Pending |
| GHST-04 | Phase 4 | Pending |
| GHST-05 | Phase 4 | Pending |
| GHST-06 | Phase 4 | Pending |
| GHST-07 | Phase 4 | Pending |
| AUTH-01 | Phase 4 | Pending |
| AUTH-02 | Phase 4 | Pending |
| AUTH-03 | Phase 4 | Pending |
| AUTH-04 | Phase 4 | Pending |
| AUTH-05 | Phase 4 | Pending |
| MIGR-06 | Phase 5 | Pending |
| UI-01 | Phase 5 | Pending |
| UI-02 | Phase 5 | Pending |
| UI-03 | Phase 5 | Pending |
| UI-04 | Phase 5 | Pending |
| UI-05 | Phase 5 | Pending |
| UI-06 | Phase 5 | Pending |
| MIGR-04 | Phase 6 | Pending |
| MIGR-05 | Phase 6-7 | Pending |

**Coverage:**
- v1 requirements: 54 total
- Mapped to phases: 54 ✓
- Unmapped: 0 ✓

**Note:** MIGR-05 spans Phases 6-7 because migration script creation (Phase 6) and validation (Phase 7) are sequential deliverables for the same requirement.

---
*Requirements defined: 2026-04-04 after research synthesis*
*Last updated: 2026-04-04 after roadmap creation*
