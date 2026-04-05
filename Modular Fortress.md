---
title: Modular Fortress
type: "[[Project]]"
code: MOD-FORTRESS
status: Active
lifestage: "🌱 Seed"
division: "[[The Forge]]"
owner: "[[NathanEckenrode]]"
pipeline_lead: "[[SylviaInkwell]]"
started: 2026-03-28
target: 2026-Q3
related:
  - "[[Project Noosphere Ghosts]]"
  - "[[innatescript]]"
  - "[[Nine Tables Schema]]"
  - "[[Innate Language Design Spec]]"
---

# Project Modular Fortress

_Your desk. Your ghosts. Your data._

---

## What This Is

Modular Fortress is [[Project Noosphere Ghosts]] made installable by anyone. A self-hosted,
AGPL-licensed platform for sovereign AI operations. One install script on a blank VPS gives
you a working noosphere: ghost agents with memory and personality, a 9-domain database, Innate
as the scripting language for directing them precisely, and a control surface that feels like
running a company.

This is not a SaaS product. It does not require a subscription. It does not phone home. It is
software you own, run on hardware you control, with AI brains you connect yourself.

The current noosphere (v1.5) is Nathan's bespoke deployment. Modular Fortress is v2.0 — the
generalization that makes it reproducible by anyone.

---

## Philosophy

**Winer + Stallman.** RSS is the social network. AGPL closes the network loophole. The Four
Freedoms apply to your AI assistants the same way they apply to your compiler.

**Artificial Life, Not Optimization.** The tick system descends from ~300 lines of artificial
life where one form evolved to dominate the grid. Every growth mechanism has a balancing
constraint. The noosphere is an ecosystem, not an optimization target.

**Proprietary dependencies contradict sovereignty.** Build custom, don't integrate.

---

## Three-Pillar Architecture

All three pillars are required on every machine running the system. The install script pulls
them as a unit — none are optional.

| Pillar                     | Repo                       | Language           | Role                                                                              |
| -------------------------- | -------------------------- | ------------------ | --------------------------------------------------------------------------------- |
| **dpn → Modular Fortress** | `dpn`                      | Rust → Go          | API layer, auth, schema, ops interface. Serves and persists. Nothing thinks here. |
| **Noosphere Ghosts**       | `project-noosphere-ghosts` | Common Lisp (SBCL) | Ghost tick engine, cognition broker, action executor. What actually does things.  |
| **Innatescript**           | `innatescript`             | Common Lisp        | Scripting language for ghost routines. Loaded as dependency by ghost runtime.     |

The Go binary is what users interact with. Ghosts are what make it alive. Innate is how you
direct the ghosts and how they direct each other. A machine without the Lisp runtime isn't
running a noosphere — it's a server with a dead agent layer.

---

## The Nine Tables

89 tables collapse to 9. Each is polymorphic (`kind` field + JSONB content).

| #   | Domain             | Role                                                      | Type      |
| --- | ------------------ | --------------------------------------------------------- | --------- |
| 1   | **The Chronicles** | Narrative canon, archives, story arcs                     | Sovereign |
| 2   | **The Realms**     | Orbis worldbuilding, geography, ghost movement            | Sovereign |
| 3   | **The Press**      | Editorials, publishing pipeline, Thought Police           | Sovereign |
| 4   | **The Markets**    | Trading, investments, commerce (Complete Success)         | Sovereign |
| 5   | **The Music**      | Corpus, episodes, wave calendar, audio metadata           | Sovereign |
| 6   | **The Forge**      | Agents, state, memories, tick reports, vault notes        | Sovereign |
| 7   | **The Commons**    | RSS feeds, articles, images, research. No domain owns it. | Substrate |
| 8   | **The Work**       | Tasks, goals, decisions, routines, issues                 | Substrate |
| 9   | **The Post**       | Conversations, inbox. Ghost-to-ghost and Nathan-to-ghost. | Substrate |

Three invisible infrastructure tables run underneath:

| #   | Table         | Purpose                                                                 |
| --- | ------------- | ----------------------------------------------------------------------- |
| 10  | **Ticks**     | Ghost tick heartbeat + ghost journal. Hot/Warm/Cold tiers.              |
| 11  | **Wikilinks** | Backlink graph across all 9 domain tables.                              |
| 12  | **Config**    | Active domains, LLM provider, feature flags. Secrets stay in .env only. |

Always on: The Forge, The Commons, The Work.
Optional: all others.
Invisible: Ticks, Wikilinks, Config.

---

## The Generalization Requirement

The Go refactor is not just "rewrite Rust in Go." It is a **generalization pass**.

The current Rust codebase is wired to Nathan's specific infrastructure: his Postgres instance,
his droplet, his file paths, his pm2 setup, his SSH tunnel configuration. The refactor must
produce a binary that knows **none of that**. It discovers its environment from config, installs
its own schema fresh on a clean database, and has no hardcoded dependencies on bespoke Nathan
infrastructure.

The same generalization applies to `project-noosphere-ghosts` and `innatescript` — any
Nathan-specific assumptions in the Lisp codebases must be surfaced and made configurable.

### Acceptance Test: The Fresh Droplet

Spin up a fresh droplet with nothing on it. Run the install script. Get a working noosphere.

If the install fails on a clean machine, v2.0 is not ready. There is no way to fake this test.

---

## Install Experience

### What the Installer Does

1. Checks for Postgres — installs it if absent
2. Creates all 12 tables with the full schema
3. Pulls and starts all three pillars: Go server, Noosphere Ghosts runtime, Innatescript
4. Opens a browser link to the onboarding interface

### Onboarding Flow

**Page 1 — Who Are You**
Connect an identity account (GitHub, Forgejo, Gitea, passkeys, or local).
Configure git hosting. Grant Engineering ghosts push access to your repos.

**Page 2 — Connect Your Brains**
Connect frontier AI subscriptions via gateway (routing, cost tracking, fallback).
Add local brains (Ollama, LM Studio, etc.).
Once at least one live brain is confirmed, T.A.S.K.S. appears and walks you through the rest.

### T.A.S.K.S. — The Permanent Onboard COO

Always present. Cannot be deleted. Can be renamed. Functions as COO / PA / Help Desk. Has full
system documentation in context at all times. Is the user's lifeline when stuck. First ghost in
every noosphere.

---

## Application Model

Modular Fortress is a **native desktop application**, not a web app. It runs on the Mac Studio
(primary) and HyprDeck Linux laptop (secondary). Built with **Wails** — Go backend powering a
native window with a webview. The Go membrane (Dragonpunk) is the backend; TypeScript renders
the UI inside the webview. Single binary distribution. No browser, no URL bar, no deployment.

The Wails architecture means Go functions are callable directly from TypeScript without HTTP
overhead. The Go process connects to PostgreSQL on localhost. Everything stays local.

### Two-Switch Architecture

The system operates as two independent processes:

**Switch 1 — The Runtime (headless, always-on):**
PostgreSQL + Lisp tick engine + Go membrane daemon. Ghosts tick on their circadian rhythms
(03:00 leisure, 04:00 synthesis, 06:00 dawn). RSS feeds fetch. Pipeline stages advance.
Memory consolidates. Protocol adapters relay messages. The noosphere is alive whether anyone
is watching or not. Runs as a background service on boot.

**Switch 2 — The UI (on-demand, optional):**
Wails desktop app opens a window onto the running system. Connects to the same PostgreSQL.
Subscribes to LISTEN/NOTIFY for real-time updates. Renders the scene canvas, sidebars,
floating windows. You see what's been happening, interact with it, close it when done.
Closing the UI does not stop the ghosts.

The UI is a viewport into an already-running world. Opening it at 8am shows overnight work
already completed — morning digest written, tick reports filed, pipeline stages advanced.

### Real-Time Updates

PostgreSQL triggers emit `NOTIFY` on domain table changes. The Go membrane listens with pgx
and pushes events to the TypeScript UI via Wails native event binding — no WebSocket needed.
Notifications are small (table, op, id, kind, slug); the UI fetches full rows on demand.

Ghost ticks → speech bubbles on tokens. New messages → chat badge updates. Task completions →
project view refreshes. Pipeline advances → scene arrangement updates. All in real-time,
zero-cost at rest, millisecond delivery.

### Protocol Adapters

the_post is the **bidirectional universal messaging bus**. All protocols flow through it:
internal ghost↔ghost, you↔ghost, IRC, XMPP, Matrix, email, RSS comments. Each message has a
`protocol` field identifying its origin.

Go runs **hotswappable protocol adapters** conforming to a standard interface
(Connect/Disconnect/Listen/Send). Adapters can be compiled-in packages or standalone binaries
in a plugin directory. v2.0 ships with RSS + internal adapters. IRC adapter at v2.1 as the
reference implementation. Additional protocols added as plugins without modifying core code.

---

## Control Surface

Not a dashboard — a **command table** with full CRUD. Foundry VTT's GM interface adapted for
ghost management.

**Two visual layers:**

- **Scene Layer** (always visible, always behind) — the persistent backdrop. Pannable, zoomable.
  Ghost tokens live here. Notes can be pinned here. This is your desk.
- **Panel Layer** (overlay windows) — floating windows over the current scene. Editable records,
  tools, configuration. Multiple can be open at once.

**Two collapsible icon-rail sidebars** (VS Code activity bar style — icons always visible,
clicking opens slide-out panel or floating window):

**LHSB — Domain Browsers:**

Each sovereign domain gets an icon. Clicking opens a folder-hierarchy slider for browsing
that domain's entries (filtered by `kind`). Each entry uses its `icon:` field as the display
icon. Entries follow the unified entity interaction pattern (see below).

| Icon | Domain         | Table           |
| ---- | -------------- | --------------- |
| 📜   | Chronicles     | the_chronicles  |
| 🏰   | Realms         | the_realms      |
| 📰   | Press          | the_press       |
| 📈   | Markets        | the_markets     |
| 🎵   | Music          | the_music       |
| 🔨   | Forge          | the_forge       |
| 📦   | Commons        | the_commons     |
| 📋   | Work           | the_work        |
| ✉️   | Post           | the_post        |
| ⚙️   | Settings       | Config + domain behavior options (bottom position) |

**RHSB — Composed Views:**

Views that draw from or span multiple domains. These are surfaces built ON TOP of the data,
not the data itself.

| Icon | Panel          | Opens                                                        |
| ---- | -------------- | ------------------------------------------------------------ |
| 📖   | Reader         | RSS reader (the_commons rss_feed + rss_article entries)      |
| 📓   | Journal        | Calendar grid + daily note (temporal, template from the_commons) |
| 💬   | Chat           | Unified inbox across all protocols (the_post, bidirectional) |
| 👥   | Ghost Roster   | Team folders, avatar + name, drag to place on scene          |
| 📊   | Graph          | Wikilink content graph (the_links, color-coded by domain)    |
| 🎭   | Scenes         | Scene selector (Orbis Map, Bridge, Workshop, Graph, custom)  |

**Unified entity interaction pattern** (applies to ALL entities across ALL domains):
1. Has an **icon/avatar** (from `icon:` field or avatar image)
2. **Drag onto scene** = positioned token/object on the canvas
3. **Double-click** = floating viewer/editor window
4. **Activity bubbles** appear when the entity does something (ghost speaks, task updates)
5. **Drag into arrangements** with other entities to compose pipelines on any scene

Scene tools (select, move, connect, measure) live as a contextual toolbar on the canvas itself,
not in a sidebar.

**Default scene:** Orbis Map — the Azgaar-generated world map from the_realms data, with
state boundaries, burg markers, and ghost positions. Initially rendered as a JPEG scene
background with positioned markers. Long-term: custom renderer from the_realms coordinate
data in the Modular Fortress dark/amber aesthetic.

**Scene Types:** Orbis Map (default), ISS Bridge, Executive Office, Pipeline Flowchart,
The Forge Workshop, Domain View, custom scenes.

---

## Pipelines

Pipelines are **recurring operational workflows** — every blog post flows through the same
editorial chain, every trade signal flows through the same analysis chain. Distinct from
one-off projects (see Project Management below).

Pipelines are composed **on the scene canvas itself** using the unified entity interaction
pattern. There is no separate pipeline builder tool. Drop ghosts onto a scene, drop task/tool
objects between them, draw connections — that IS the pipeline. Any scene can become a pipeline
workspace when you start arranging entities on it.

**Composing a pipeline:**
- Drag ghost from roster → drop on scene = pipeline stage node
- Drag tool/template from domain browser → drop between ghosts = stage configuration
- Draw connection between nodes = define handoff (next_stage)
- Double-click node to configure = role, acceptance criteria, tools, definition of done
- Pipeline status overlay = once running, each node shows state (waiting, active, blocked, done)

**Existing pipelines in the data model:**
- Engineering: Eliana → Samir → Isaac → Casey → Devin → Sanjay → Danielle → Morgan
- Thought Police: Sophie Lee → @Resources → Sylvia → Vincent → Ghost CMS
- Complete Success: Ethan → Tobias → Kathryn → Lucas → Kathryn → JMax
- Research Feed: Sophie Lee → Elise Park → Eliana → git[innatescript]

Pipeline definitions persist in the_forge as `kind='pipeline'` and `kind='pipeline_stage'` rows.
The Lisp runtime already executes these via pipeline-definitions.lisp with ordered stages and
agent assignment.

---

## Project Management

Projects (one-off work like Modular Fortress itself) are distinct from pipelines. They have
milestones, goals, and tasks, but the shape is different each time.

**Source of truth:** master_chronicle PostgreSQL (the_work table). Not flat files. Not a
separate project management tool. The database is the source of truth — this applies to
project state the same as everything else.

**Three-surface model:**
1. **master_chronicle (the_work)** — canonical project/milestone/task records. Kinds: `project`,
   `milestone`, `goal`, `task`, `decision`, `issue`. All project data lives here.
2. **Modular Fortress UI** — project view panel showing milestone/task hierarchy, assignment
   status, blockers, progress. Rendered from the same database rows ghosts perceive.
3. **GitHub Projects** — bidirectional mirror. Issues created on GitHub flow into the_work.
   Tasks completed in the noosphere update the GitHub issue. GitHub becomes an external view
   and additional input surface, not the source of truth.

**GSD integration:** The GSD agent workflow writes project/milestone/task data to the_work
instead of (or in addition to) flat .gsd/ files. This means ghosts can see project state
through normal perception queries — an Engineering ghost working a pipeline stage and GSD
executing a task plan operate on the same substrate. The Executive ghost watches progress
through the same database everyone else uses.

---

## Ghost Runtime

### Circadian Rhythms

- **03:00** Leisure mode — creative/divergent thinking, low-stakes exploration
- **04:00** Synthesis ticks — consolidate overnight work into reusable knowledge
- **06:00** Dawn wake-up — fresh energy, prioritized perception
- **08:00** Morning digest — report generation

### Two-Phase Cognition

Cheap probe (haiku) before expensive commit (sonnet/opus). Most ticks don't need heavy
reasoning. 90% resolve at probe tier.

### autoDream Memory Consolidation

Background process: Orient → Gather Signal → Consolidate → Prune. Ghosts compact memories
periodically rather than accumulating forever. Feeds into the temporal compression chain.

### Stagnation Detection

Director-pattern cron detects ghosts spinning on rejected pipeline stages or dead-end tasks.
Redirects energy instead of burning budget.

---

## Tools Layer

**Default tool language: Python.** Deterministic scripts with one job each. Callable as CLI
and as importable functions. Testable, auditable, composable.

Innate wraps them. A `.dpn` script can commission a Python tool as part of a pipeline — Innate
handles orchestration and intent; Python handles execution.

Language freedom: Engineering ghosts can write tools in any language. The wrapper/interface
contract is what matters.

### tools/ Directory Structure

```

tools/ _core/ # system tools — schema, health, auth. Do not modify. _config.py # workspace-relative path resolution manifest.md # index of all installed tools (required — ghosts read this) [modules...] # user-installed and ghost-built tools land here

```

If a tool is not in `manifest.md`, ghosts cannot see it.

---

## Ghosts as Co-Developers

Engineering ghosts can:

- Read any of the three repos to understand current state
- Branch any repo to work on a change
- Build new tools in `tools/`
- Write Innate wrappers for those tools
- Push PRs against core repos or the user's fork
- Build new modules as standalone AGPL repos
- Audit incoming community modules before installation
- Generate READMEs for modules they produce

Git credentials and repo access are configured during onboarding.

---

## The AGPL Covenant

Every piece of code produced by the system — by ghosts, by Engineering pipelines, by module
scaffolding — is AGPL-licensed. When you push it, others can install it, inspect it, and modify
it. When they modify it, they must share those changes.

### The Four Freedoms in the UI

When you navigate to the Engineering domain workspace, the Four Freedoms are displayed. During
wait times — code generation, spec writing, build processes — they display as load screens.
Repeated. Insistent. It will be fun. It will be oppression. **YOU MUST USE FREE SOFTWARE.**

---

## Federation

| Function       | Protocol                                            |
| -------------- | --------------------------------------------------- |
| Reader / RSS   | RSS in/out federation                               |
| Chat           | IRC federation (XMPP as optional user-added module) |
| Social surface | ActivityPub — future addition                       |

The user's desk becomes social if they want it to be, using protocols that are already proven,
already decentralized, already not owned by anyone.

---

## Business Model

None. Free to download and install. No subscription. No phone home.

Tip jar (BTC or equivalent) when the BumpTop era arrives and websites become public rooms.
Corporate version if the community grows large enough to demand it. That is not the goal.

---

## Engineering Pipeline

Governed by [[The Forge]] division. Pipeline runs through:

**Modular Fortress track:** JMax → Lucas → Lily → Felix → Elise → Eliana

Responsibilities per stage:

- **JMax** — Legal/ethical vetting of architecture decisions, AGPL compliance review
- **Lucas** — Requirements analysis, cross-pipeline coordination
- **Lily** — Codebase discovery, dependency mapping, initial scan
- **Felix** — Architecture research, refactor candidates, module design
- **Elise** — Tool audit, extractable module patterns
- **Eliana** — Final engineering spec, hands off to Casey → Devin → Sanjay → Danielle → Morgan

**Engineering delivery track:** Eliana → Samir → Isaac → Casey → Devin → Sanjay → Danielle → Morgan (deploys, appends responsibilities to ghost YAML)

---

## Milestones

| Milestone | Target        | What it means                                                                                                                                      |
| --------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| **v1.5**  | ✅ 2026-03-30 | InnateScipt Capabilities — 23/23 requirements. Ghost tick engine stable.                                                                           |
| **v2.0**  | 2026-Q3       | Wails desktop app. Foundry VTT scene canvas. Go membrane (Dragonpunk). Three always-on domains. GSD → master_chronicle. Hotswappable adapter interface with RSS + internal adapters. |
| **v2.1**  | 2026-Q3/Q4    | IRC protocol adapter — first external plugin. Reference implementation proving the adapter interface. Ghosts can join IRC channels and converse bidirectionally through the_post. |
| **v2.5**  | 2026-Q4       | Plugin ecosystem live. Convention-based module registry. AGPL module pipeline from Engineering ghosts. Additional protocol adapters (Matrix, XMPP, email). |
| **v3.0**  | 2027          | Full federation active. Built-in browser. Multi-user. First-party modules shipping.                                                                |

**Q2 2026 Goal:** Nathan operating entirely inside the noosphere — phone, Mac Studio, HyprDeck.
No Claude CLI, no Claude Desktop. The noosphere IS the interface.

---

## Open Questions

1. **Brain sharing** — Multi-user on same machine: shared AI brain or per-user? How does cost accounting work?
2. ~~**Browser scope** — Tauri WebView, embedded browser engine, or iframe surface?~~ **RESOLVED:** Wails (Go + webview). Native desktop app, not browser-served. (D009)
3. **Go consolidation timeline** — Does v2.0 wait for full Rust→Go migration, or can it ship on current Rust dpn-api with install script wrapping it?
4. **T.A.S.K.S. persona lock** — Can users rename T.A.S.K.S. without losing help-desk capability? How do we preserve function while allowing persona customization?
5. **Innate wrapper contract** — Minimum interface a Python tool must expose to be wrappable by an Innate script. Standardized in v2.0 or left to convention?
6. **Lisp generalization** — What Nathan-specific assumptions exist in `project-noosphere-ghosts` and `innatescript` that need surfacing before the fresh-droplet test can pass?
7. **TypeScript framework** — React, Svelte, or vanilla TypeScript for the Wails webview UI? Existing mockups are vanilla HTML/CSS/JS. Decision deferred until UI work begins. (D015)
8. **Canvas renderer** — Does the scene canvas need PixiJS/WebGL (like Foundry uses), or are positioned DOM elements + Leaflet sufficient for Modular Fortress scenes? Depends on how complex scene rendering becomes.
9. **GSD → PostgreSQL mapping** — Exact schema mapping for GSD milestones/slices/tasks into the_work table kinds and fields. Needs design before implementation. (D014)
10. **GitHub sync mechanism** — Webhook receiver in Go for inbound issues, periodic push for status updates. API token management. Deferred to after GSD→database integration works. (D014)
11. **HyprDeck immersion** — Linux version on HyprDeck will be "even more immersive." What does that mean? Wayland compositor integration? Full-screen kiosk? Tiling WM integration?

---

## Current Costs

| Service      | Monthly   |
| ------------ | --------- |
| Claude       | $200      |
| Captivate.fm | $36       |
| DigitalOcean | $27       |
| **Total**    | **~$263** |

"I need to find a way that this hobby begins to pay for itself."

---

_Project initiated: 2026-03-31 morning pages, first sip of coffee._
_Spec compiled: 2026-04-01._
_Division: [[The Forge]]. License: AGPL. Cosmic Abstractor: [[NathanEckenrode]]._

```

---

That's the full formal project spec, synthesized from all three source documents plus the engineering pipeline from memory. Ready to drop into the vault wherever you want it — want me to write it to a path on the droplet, or is this good as markdown here?
```
