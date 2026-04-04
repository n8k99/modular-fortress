# Modular Fortress

## What This Is

Modular Fortress is your sovereign digital workspace — a self-hosted KDE-PIM-style suite built on the Nine Tables database architecture. It replaces Big Tech dependencies (Google Calendar, Apple Notes, Obsidian Sync, WhatsApp) with one unified application where autonomous Lisp ghost agents live alongside your daily work. All data flows through master_chronicle PostgreSQL. All secrets stay in `.env`. Your desk. Your ghosts. Your data.

## Core Value

**The database is the source of truth.** Every piece of personal data—notes, tasks, conversations, calendar events, RSS feeds—lives in master_chronicle and is accessible through one unified interface. No vendor lock-in. No cloud sync. No external dependencies.

## Requirements

### Validated

- ✓ PostgreSQL database operational (83 tables, 2,554 tasks, 9,846 conversations) — existing
- ✓ Rust API server running (noosphere/) on port 8888 — existing
- ✓ Lisp ghost runtime exists (project-noosphere-ghosts/) — existing
- ✓ InnateScript DSL interpreter exists (innatescript/) — existing
- ✓ Codebase mapped across 7 dimensions — existing

### Active

- [ ] Migrate 83-table schema → Nine Tables (polymorphic with `kind` + JSONB)
- [ ] Rewrite Rust API servers → unified Go codebase (dpn-api, dpn-core, dpn-mcp, noosphere)
- [ ] Database-backed note system (Obsidian replacement)
- [ ] Wikilink graph across all nine domains
- [ ] RSS reader interface (The Commons domain)
- [ ] Calendar view (Wave Calendar / Executive Sessions)
- [ ] Task/goal management (The Work domain)
- [ ] Ghost management interface (The Forge domain)
- [ ] Foundry VTT-style scene-based UI
- [ ] Passkey authentication (Pocket ID, no OAuth)
- [ ] Hot-pluggable extension system for droplet services
- [ ] All secrets in `.env` (never committed)

### Out of Scope

- Public release / install script for others — Private GPL repo, Nathan only
- AGPL licensing / open distribution — GPL but not shared publicly
- Generalization for strangers — Built for Nathan's specific workflow
- Multi-user support — Single user system
- Mobile apps — Desktop-first, maybe iOS later
- Replacing djay Pro — Best-in-class DJ software, keep it
- Replacing iOS Camera sync — Hardware integration too tight

## Context

**Current state:**
- Four separate Rust projects (dpn-api, dpn-core, dpn-mcp, noosphere) to be rewritten as unified Go codebase
- 83-table PostgreSQL schema that's difficult to query coherently (agents across 8 tables, markets across 11, etc.)
- Lisp codebase (project-noosphere-ghosts + innatescript) works but has Nathan-specific assumptions
- 9,000+ markdown files exported from database (security concern, delete after migration)
- Working API endpoints but UI has mock JavaScript data, not wired to live data

**The Nine Tables design:**
1. **The Chronicles** — Narrative canon, archives, story arcs (Sovereign)
2. **The Realms** — Orbis worldbuilding, geography, ghost movement (Sovereign)
3. **The Press** — Editorials, publishing, Thought Police (Sovereign)
4. **The Markets** — Trading, investments, Complete Success commerce (Sovereign)
5. **The Music** — Corpus, episodes, wave calendar, audio metadata (Sovereign)
6. **The Forge** — Agents, state, memories, tick reports, vault notes (Sovereign)
7. **The Commons** — RSS feeds, articles, images, research (Substrate)
8. **The Work** — Tasks, goals, decisions, routines, issues (Substrate)
9. **The Post** — Conversations, inbox, ghost-to-ghost messages (Substrate)

Plus 3 infrastructure tables: Ticks (ghost heartbeat), Wikilinks (backlink graph), Config (feature flags).

**Digital Sovereignty goals absorbed from Project Digital Sovereignty:**
- Zero Google account dependencies for daily workflow
- Zero Apple account dependencies (except hardware)
- All personal data exportable in open formats
- Self-hosted auth via passkeys (no OAuth to external providers)
- Federated messaging (XMPP/Matrix) replacing proprietary chat

## Constraints

- **Tech Stack**: Rust (API server), Common Lisp (ghost runtime + InnateScript), PostgreSQL (Nine Tables), TypeScript (UI) — Already committed, not changing
- **Database**: master_chronicle PostgreSQL on localhost:5432 — Existing instance with live data
- **Architecture**: Three-pillar (Rust API, Lisp runtime, InnateScript) — Established pattern from v1.5
- **Licensing**: GPL (private repo on GitHub n8k99/modular-fortress) — Code is GPL but not publicly distributed
- **Secrets Management**: All credentials in `.env`, never committed to git — Security requirement
- **Single User**: Built for Nathan's workflow only — Not generalizing for others
- **Desktop First**: macOS primary target, Linux secondary — No mobile support initially

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Nine Tables over 83 tables | Current schema is unmaintainable—agents split across 8 tables, markets across 11, impossible to query coherently | — Pending |
| Rust→Go rewrite | "Once Go is coded it stays in place"—22x faster compilation, 2-3x development velocity, backwards compatibility guarantee | — Pending |
| KDE-PIM style unified interface | One application for all personal data management (notes, calendar, tasks, RSS, ghosts) instead of separate tools | — Pending |
| Keep Lisp ghost runtime separate | Ghost cognition belongs in Lisp, not Rust—proven architecture from v1.5 | ✓ Good |
| Private GPL repo | GPL licensing for ethical reasons, but private repo because not building for public distribution | — Pending |
| Database as source of truth | All data in master_chronicle, no external dependencies or cloud sync | ✓ Good |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd:transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd:complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-04 after initialization*
