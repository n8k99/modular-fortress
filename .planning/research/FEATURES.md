# Feature Landscape

**Domain:** Sovereign digital workspace (KDE-PIM style)
**Researched:** 2026-04-04

## Table Stakes

Features users expect. Missing = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Note creation/editing** | Core PIM capability, all competitors have it | Low | Rich text + Markdown syntax expected |
| **Wikilinks [[note]]** | Standard in Obsidian/Logseq/Notion, users expect backlinks | Low | Bidirectional linking is non-negotiable for knowledge management |
| **Calendar view** | Core PIM feature (KDE-PIM, Notion, all calendar apps) | Medium | Day/week/month views minimum |
| **Task/todo management** | Universal PIM feature, basic checkbox functionality | Low | States (TODO/DOING/DONE), due dates, priorities |
| **Full-text search** | Users report 30% time waste without it (G2, 2025) | Medium | PostgreSQL full-text search sufficient initially |
| **Backlinks panel** | Obsidian standard, shows "what links here" | Low | Database query on wikilink table |
| **Daily notes** | Logseq popularized, now expected for journaling workflows | Low | Auto-create note for today's date |
| **Data export** | Sovereignty requirement, users demand open formats | Low | Markdown export for notes, iCal for calendar, JSON for tasks |
| **Offline-first operation** | Local-first software expectation (Ink & Switch, 2025) | High | PostgreSQL local means offline by default (✓ already have) |
| **Tags/labels** | Universal organization feature across all tools | Low | Multiple tags per note/task/event |
| **RSS feed reading** | The Commons domain requirement, replace RSS readers | Medium | FreshRSS shows this is table stakes for self-hosted |
| **Contact management** | KDE-PIM has KAddressBook, EssentialPIM has contacts | Medium | Name, email, phone, notes fields minimum |

## Differentiators

Features that set product apart. Not expected, but valued.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Autonomous ghost agents** | Unique: Lisp ghosts living in workspace, not bolted-on AI | High | Already have runtime, integrate with Nine Tables |
| **Nine Tables polymorphism** | Technical elegance: one schema for all data types via `kind` + JSONB | Medium | Eliminates 83-table chaos, queryable coherently |
| **Wikilink graph across domains** | Cross-domain linking (notes → tasks → calendar → ghosts) unprecedented | Medium | Unified backlink table across all nine domains |
| **No OAuth/passkey-only auth** | True sovereignty: zero external auth dependencies | Medium | WebAuthn widespread in 2025, library support mature |
| **Ghost-to-human conversations** | The Post domain: ghosts can message you, not just respond | Medium | Database-backed chat with ghost agents as participants |
| **Scene-based UI (Foundry VTT style)** | Spatial organization vs flat lists, game-like workspace | High | Novel for productivity tools, high visual appeal |
| **Task dependency chains** | Blocking/waiting relationships visualized, auto-status updates | Medium | Jira/Asana have this, rare in personal PIM |
| **Hot-pluggable droplet services** | Deploy n8n workflows to DigitalOcean, integrate via webhook | High | Extension architecture for self-hosted services |
| **InnateScript templating** | Ghost-executable notes: notes that ghosts can interpret/run | High | Unique to this system, bridges Lisp and workspace |
| **All-domain unified inbox** | RSS + ghost messages + task notifications in one feed | Medium | "Unified inbox" trend in 2025 (Spike, Superhuman) |
| **Sovereign credentials** | All secrets in `.env`, never leaves your machine | Low | Differentiates from cloud PIM that syncs to vendor servers |
| **Temporal compression notes** | Import historical data, compress context for ghosts | High | Novel: turn years of data into ghost-readable summaries |

## Anti-Features

Features to explicitly NOT build.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **Cloud sync service** | Contradicts sovereignty, creates vendor dependency | Provide export/import, document self-hosted sync (Syncthing) |
| **Multi-user collaboration** | Adds 10x complexity, not target use case | Single-user system, ghost-to-human is "collaboration" |
| **Mobile apps (initially)** | Desktop-first, iOS hardware integration too tight | Document mobile workflow (iOS camera sync stays native) |
| **Plugin marketplace** | Requires curation, moderation, infrastructure | Document hot-pluggable droplet pattern instead |
| **Rich media editing** | PIMs fail at this (85% incomplete records, Catsy 2025), not core value | Link to external tools (djay Pro for music, external editors) |
| **Multi-language support** | Single-user (Nathan), English-only reduces complexity | Keep UI strings in English, don't generalize |
| **WYSIWYG editor complexity** | Adds megabytes of JS, Markdown+preview is sufficient | Markdown input + live preview (simpler, faster) |
| **Email client** | Requires IMAP/SMTP, spam filters, threading complexity | Document email integration via n8n webhooks to database |
| **Built-in AI models** | Requires GPU/RAM, local models (Ollama) already available | Use existing Ollama service (config.json shows it's running) |
| **Social sharing features** | Not a social tool, sovereignty means private-first | Export individual notes as Markdown to share manually |
| **Automatic backups** | User controls their own backup strategy | Document backup recommendations (pg_dump + git) |

## Feature Dependencies

```
Authentication (passkey) → All features (must auth first)
Wikilinks → Backlinks (backlinks query wikilinks table)
Notes → Daily notes (daily notes are special case of notes)
Notes → Wikilink graph (graph requires notes to link)
Tasks → Task dependencies (dependencies are relationship between tasks)
Calendar → Wave Calendar (wave calendar adds ghost/music layer)
RSS feeds → Unified inbox (inbox aggregates RSS + messages)
Ghost runtime → Ghost conversations (runtime must exist for ghosts to message)
Nine Tables → All domain features (polymorphic schema is foundation)
```

## MVP Recommendation

Prioritize:
1. **Notes with wikilinks** — Core knowledge management, enables all subsequent features
2. **Backlinks panel** — Makes wikilinks valuable, shows relationships
3. **Daily notes** — Journaling workflow, low complexity, high daily use
4. **Full-text search** — Prevents 30% time waste, PostgreSQL native capability
5. **Task list (simple)** — TODO/DONE states, no dependencies yet
6. **Calendar view (basic)** — Day/week views, create events
7. **Passkey auth** — Sovereignty requirement, gate access

Defer:
- **RSS reader** (Phase 2): Table stakes but can use external reader initially
- **Task dependencies** (Phase 2): Differentiator but adds complexity
- **Ghost conversations** (Phase 2): Differentiator requiring runtime integration
- **Scene-based UI** (Phase 3): High visual appeal but requires full feature set first
- **Wikilink graph visualization** (Phase 2): Valuable but search + backlinks sufficient for MVP
- **Unified inbox** (Phase 3): Requires RSS + messages to be implemented first
- **Hot-pluggable droplets** (Phase 4): Extension architecture after core proven

## Complexity Analysis

### Low Complexity (< 1 week)
- Note CRUD operations (PostgreSQL JSONB storage)
- Wikilink detection via regex `\[\[([^\]]+)\]\]`
- Tags (many-to-many table)
- Daily notes (auto-create note with `YYYY-MM-DD` title)
- Basic task list (status field: todo/doing/done)
- Data export (pg_dump for raw, formatters for Markdown/JSON)

### Medium Complexity (1-2 weeks)
- Full-text search (PostgreSQL `tsvector` + GIN index)
- Backlinks panel (query wikilink table, group by target)
- Calendar views (day/week/month grid layout)
- RSS feed reader (parse XML, store articles, mark read/unread)
- Task dependencies (blocked_by array column, visualize relationships)
- Passkey authentication (WebAuthn library, store credential IDs)
- Cross-domain wikilink graph (unified wikilink table across Nine Tables)
- Contact management (form fields + PostgreSQL storage)

### High Complexity (2-4 weeks)
- Offline-first sync (conflict resolution, CRDTs or last-write-wins)
- Scene-based UI (spatial canvas, drag-drop, viewport management)
- Ghost conversation integration (IPC with Lisp runtime, message protocol)
- Hot-pluggable droplets (webhook registry, n8n workflow deployment)
- InnateScript templating (parser, ghost execution sandbox)
- Temporal compression (import historical data, summarize for ghost context)

## Sources

**HIGH CONFIDENCE (official docs, Context7):**
- Kontact Suite overview: https://kontact.kde.org/ (KDE official)
- Obsidian wikilinks: https://help.obsidian.md/linking (Obsidian official)
- Notion databases: https://www.notion.so/help/databases (Notion official)
- WebAuthn spec: https://www.w3.org/TR/webauthn-3/ (W3C standard)
- PostgreSQL full-text search: https://www.postgresql.org/docs/current/textsearch.html (PostgreSQL official)

**MEDIUM CONFIDENCE (verified with multiple sources):**
- Logseq daily journal: https://blog.logseq.com/how-to-get-started-with-networked-thinking-and-logseq/ (official blog, verified by user reviews 2025)
- Trilium hierarchical notes: https://triliumnotes.org/ (official site, verified by XDA review 2026)
- FreshRSS self-hosted RSS: https://freshrss.org/ (official site, verified by XDA 2025)
- Passkey adoption: Microsoft report 1M daily registrations, 350% increase 2024→2025
- Task dependencies: Asana, Jira, SmartSuite all implement blocked/waiting relationships (2025 docs)
- Local-first principles: Ink & Switch "Local-first software" essay (canonical reference)
- Unified inbox trend: Spike, Superhuman, Mailbird all converging on multi-channel inbox (2025)
- Knowledge management: G2 reports 30% time reduction with search (2025)

**LOW CONFIDENCE (WebSearch only, flagged for validation):**
- PIM data quality issues (85% facing quality disasters): Catsy blog 2025 — industry vendor claim, not independent research
- Sovereign workspace momentum: Nextcloud press release (vendor claim, but aligns with EU regulations)
- Offline-first performance claims (Seafile 2-3x faster): LogicWeb comparison — unverified benchmarks

**Research gaps:**
- Scene-based UI for productivity tools: No direct precedent found, Foundry VTT is gaming context
- Ghost-human conversation patterns: No existing PIM has autonomous agents as conversation participants
- Nine Tables polymorphism: Custom architecture, no established pattern in PIM space
