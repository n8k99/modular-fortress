# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Architecture Overview

This is a DigitalOcean droplet serving as the sovereign node for **Eckenrode Muziekopname** — an agentic AI platform. Three systems work together:

1. **GSD + Claude Code** — Nathan's planning interface. Structured project/phase planning via `/gsd:` slash commands.
2. **master_chronicle (PostgreSQL)** — The OS. All state lives here: projects, tasks, conversations, memory, documents, agent identity.
3. **Noosphere Ghosts (SBCL)** — The minds. 60+ persistent AI agents with memory, persona, and cognitive continuity. They perceive work from the DB and execute it.

```
Nathan (Claude Code + GSD)
  → /gsd:new-project, /gsd:discuss-phase, /gsd:plan-phase
  → /gsd:dispatch → writes projects + tasks to master_chronicle
  → Ghosts perceive tasks via dpn-api → execute with persona + memory
  → Results visible on wiki org graph + dpn-kb
```

### Key Directories

| Directory | Language | Purpose |
|-----------|----------|---------|
| `/opt/project-noosphere-ghosts/` | Common Lisp (SBCL) | Ghost tick engine — the agentic runtime |
| `/opt/dpn-api/` | Rust (Axum) | REST API serving perception, conversations, tasks, documents |
| `gotcha-workspace/` | Python | Operational tools, cron scripts, agent configs (has own CLAUDE.md) |
| `dpn-core/` | Rust | Shared library for DPN tools — DB access, sync, agent memory |
| `/opt/stack/foundry/` | — | Foundry VTT v11 (Docker, dormant) — Orbis TTRPG world |
| `gotcha-secrets/` | — | Sensitive config (never commit) |

## GSD → Noosphere Dispatch Flow

GSD (Get Shit Done) provides structured planning. The dispatch bridge persists plans to the DB for ghost execution.

1. `/gsd:new-project` — Nathan describes an idea, GSD structures it into PROJECT.md + ROADMAP.md
2. `/gsd:discuss-phase N` — Capture implementation decisions
3. `/gsd:plan-phase N` — Create concrete PLAN.md files with wave dependencies and must_haves
4. `/gsd:dispatch N [--owner agent-id]` — Bridge script reads .planning/ and writes to `projects` + `tasks` tables
5. Ghosts perceive new tasks on next tick, execute, verify against must_haves

Bridge script: `gotcha-workspace/tools/gsd/dispatch_to_db.py`

## PostgreSQL — `master_chronicle`

The database is the OS. All agent consciousness, memory, communication, and content lives here.

- **Host:** `127.0.0.1:5432` | **DB:** `master_chronicle`
- **Users:** `chronicle/chronicle2026` (full), `executive/em2026exec` (scoped)
- **Remote tunnel:** `ssh -L 5433:127.0.0.1:5432 root@144.126.251.126 -N -f`

Key tables:

| Table | Purpose |
|-------|---------|
| `projects` | Project registry with owner, goals, blockers, status |
| `tasks` | Work queue — ghosts perceive these via perception API |
| `conversations` | Message bus — Nathan ↔ exec dialogue, ghost-to-ghost |
| `memory_entries` | Long-term agent memories (the ghost substrate) |
| `documents` | 47K+ world docs, blog posts, editorials |
| `agents` | Executive staff registry (8 execs + 56 staff) |
| `agent_fitness` | Ghost evolution fitness scores |
| `decisions` | Decision log attributed to agents |
| `vault_notes` | Operational notes (daily, weekly) |

### Ghost Work Assignment

Ghosts get work from two sources (priority order):
1. **`conversations`** (+50 urgency) — messages directed at a ghost
2. **`tasks`** (+25 urgency) — assigned or department-scoped tasks
3. **`projects`** (+15/project urgency) — owned projects trigger proactive review

Routing by role: triage agents see unassigned tasks, executives see their dept, staff see only personal tasks. Project ownership energizes executives to review their portfolio even without direct messages.

## Noosphere Ghosts (`/opt/project-noosphere-ghosts/`)

- **PM2 process:** `pm2 start/stop/restart noosphere-ghosts`
- **Launch:** `/opt/project-noosphere-ghosts/launch.sh`
- **LLM provider:** Claude Code CLI (`claude -p`) with `--output-format json`. Chain: claude-code → anthropic → stub
- **Key files:** `lisp/runtime/tick-engine.lisp`, `action-planner.lisp`, `action-executor.lisp`, `perception.lisp`, `claude-code-provider.lisp`
- **Config:** `config/provider-config.json`
- **Lisp JSON quirk:** parser converts underscores to hyphens (`:is-error` not `:is_error`)

### Tick Engine Phases

1. **Perceive** — each agent calls `/api/perception/:agent_id` (messages, tasks, projects, documents, team activity)
2. **Rank** — urgency score: `(drive_pressure * energy/100) + msg_boost + task_boost + project_boost + deadline_boost`
3. **Classify** — top N agents get cognition jobs; priority: messages > requests > tasks > project review
4. **Execute** — cognition broker sends jobs to LLM, applies results (conversations, task mutations, handoffs)
5. **Update state** — tier recalculation, energy updates
6. **Report** — tick log persisted via API

### Executive Agent Roster

| Agent | Role | Domain |
|-------|------|--------|
| Nova | COO | Operations, automation, droplet |
| Eliana | CTO | Engineering, infrastructure, all repos |
| Kathryn | CSO | Strategy, marketing, prediction markets |
| Sylvia | Content Chief | All writing, narrative, content |
| Vincent | Creative Director | Visual, images, art direction |
| JMax | Head of Legal | Canon arbiter, legal, ethics, lore |
| LRM | Head of Musicology | Music, audio, composition |
| Sarah Lin | Executive PA | Routing, orchestrator of orchestrators |

Domain routing rules are strict: Vincent does NOT write (Sylvia does), Sylvia does NOT make images (Vincent does), JMax is canon arbiter, Nova handles droplet/ops, Eliana handles code/repos.

## Running Services

Managed via **PM2** and **Docker**. Check status: `pm2 list`

| Service | Port | Stack | Status |
|---------|------|-------|--------|
| dpn-api | 8080 | Rust/Axum | Live |
| dpn-kb | 3100 | Next.js | Live |
| em-site | 3200 | Next.js | Live |
| n8k99-site | 3000 | Next.js | Live |
| noosphere-ghosts | — | SBCL | Stopped (restartable) |
| Nginx | 80/443 | Docker | Live |
| Foundry VTT | 30000 | Docker | Dormant |

Org graph dashboard: `wiki.eckenrodemuziekopname.com/em-org-graph-live.html`

## dpn-core (Rust)

Shared infrastructure crate for DPN tools.

```bash
cd /root/dpn-core
cargo build          # build
cargo test           # run tests
cargo test <name>    # run single test
```

Uses sqlx with PostgreSQL. Key modules: `db/`, `sync/`, `memory/`, `ics/`, `stagehand/`, `conversations/`.

## GOTCHA Workspace

`gotcha-workspace/` has its own detailed `CLAUDE.md`. It contains operational Python tools, cron scripts, and agent configs. Key rules:

- **Tools must use** `tools/_config.py` for workspace-relative paths (never hardcoded absolute paths)
- **New tools must be** added to `tools/manifest.md`
- Python venv at `gotcha-workspace/.venv`

OpenClaw (`~/.openclaw/`) is being deprecated. Its capabilities (Discord bot, cron scheduler, 8 skills, agent identity) are migrating to Noosphere ghosts + deterministic cron scripts + master_chronicle.

## Guardrails

- **NEVER DELETE YOUTUBE VIDEOS** — irreversible. Ask 3 times for confirmation.
- **Rust UTF-8 Rule** — Never mix character positions with byte indices. Use `.chars().take(N).collect()`, never byte slicing. The dpn-api perception endpoint was fixed for this.
- **Workspace portability** — All tools must use relative paths. Absolute paths (except `/tmp`, `/dev`) are wrong.
- **Check `gotcha-workspace/INFRASTRUCTURE.md`** before building anything — if it's listed there, it exists.
- **DB is the OS** — state belongs in master_chronicle, not in files. Projects, tasks, memory, conversations all live in PostgreSQL.

<!-- GSD:project-start source:PROJECT.md -->
## Project

**Noosphere Dispatch Pipeline**

The autonomous execution pipeline connecting GSD planning to Noosphere Ghost action. Nathan plans projects and phases in GSD, dispatches them to master_chronicle, and executive ghosts perceive, decompose, delegate, and drive work through their teams — reporting results back through the conversation table and project status updates. The goal is a system where ghosts do real work autonomously, and Nathan only intervenes for strategy and blockers.

**Core Value:** GSD-dispatched projects must flow through to ghost execution and back without human intervention — executives plan, staff execute, results report themselves.

### Constraints

- **Stack**: Rust (dpn-api, dpn-core), Common Lisp/SBCL (ghosts), Python (dispatch tools), PostgreSQL — no new languages
- **DB is the OS**: All state in master_chronicle. No file-based state for ghost work.
- **UTF-8 Rule**: Never mix character positions with byte indices in Rust code
- **Ghost LLM**: Claude Code CLI (`claude -p`) with `--output-format json`, $0.50/request budget
- **Tick interval**: Currently 30s-10min configurable. Tool execution must complete within tick bounds.
- **Single droplet**: All services on 144.126.251.126. Resource-conscious design.
<!-- GSD:project-end -->

<!-- GSD:stack-start source:codebase/STACK.md -->
## Technology Stack

## Languages & Versions
| Language | Version/Edition | Projects |
|----------|----------------|----------|
| Rust | 2021 Edition | dpn-api, dpn-core |
| Common Lisp (SBCL) | — | project-noosphere-ghosts (AF64 runtime) |
| Python 3 | 3.x (venv) | gotcha-workspace tools |
| TypeScript/JavaScript | ES6+ | dpn-kb, em-site, n8k99-site, dpn-mcp |
## Frameworks & Libraries
### Rust (dpn-api)
- **axum** 0.7 — HTTP framework
- **sqlx** 0.8 — PostgreSQL async driver
- **tokio** 1 — Async runtime (full features)
- **serde** 1 + serde_json — Serialization
- **jsonwebtoken** 9 — JWT auth
- **tower** 0.4 + tower-http 0.5 — CORS, tracing middleware
- **reqwest** 0.12 — HTTP client
- **scraper** 0.21 — HTML parsing
- **chrono** 0.4 — Date/time
- **tracing** 0.1 + tracing-subscriber — Structured logging
- **dotenvy** 0.15 — Environment loading
- **uuid** 1.21.0 — Unique identifiers
- **regex** 1 — Pattern matching
- **anyhow** 1 + thiserror 1 — Error handling
### Rust (dpn-core)
- **sqlx** 0.8 — PostgreSQL + SQLite (runtime-tokio-native-tls)
- **rusqlite** 0.31 — Bundled SQLite
- **reqwest** 0.12 — HTTP client (embeddings + RSS)
- **feed-rs** 2.1 — RSS/Atom feed parsing
- **ical** 0.11 — ICS calendar parsing
- **chrono** 0.4, serde 1, tracing 0.1, uuid 1, rand 0.8, dirs 5, url 2.5, once_cell 1.21.3
### Next.js Sites (dpn-kb)
- **Next.js** 15.1.6, React 18
- **next-auth** 4.24.13 — OAuth/credentials auth
- **pg** 8.18.0 — PostgreSQL client
- **mysql2** 3.16.3 — MySQL client
- **better-sqlite3** 12.6.2 — Embedded SQLite
- **leaflet** 1.9.4 + react-leaflet 4.2.1 — Maps
- **react-markdown** 9.1.0 + remark-gfm 4.0.1 — Markdown
- **Tailwind CSS** 3.4.1
### Next.js Sites (em-site, n8k99-site)
- **Next.js** 16.1.6, React 19.2.3
- **pg** 8.18-8.19 — PostgreSQL
- **Tailwind CSS** 4
- **date-fns** 3-4, clsx, react-markdown
- **@tryghost/content-api** 1.11.17 (n8k99-site only)
### Python (gotcha-workspace)
- **mysql-connector-python** — MySQL connectivity
- **pyyaml** — YAML parsing
- **python-dotenv** — Environment loading
- **pytest** — Testing
### Common Lisp (AF64)
- Custom JSON encoder/decoder (`util/json.lisp`)
- Custom HTTP client via curl (`util/http.lisp`)
- ASDF system definition (`af64.asd`)
## Build Tools & Toolchains
| Tool | Purpose |
|------|---------|
| Cargo | Rust build system (dpn-api, dpn-core) |
| npm | Node.js package manager (Next.js sites) |
| SBCL + ASDF | Common Lisp compilation |
| Python venv | gotcha-workspace/.venv |
## Runtime Dependencies
- **PostgreSQL** — master_chronicle (central state store)
- **Docker** — nginx, Ghost CMS, MySQL, n8n, certbot, prosody
- **PM2** — Process management (dpn-api, em-site, nova-bridge)
- **Nginx** — Reverse proxy (ports 80/443)
- **Ollama** — Local embeddings (localhost:11434, nomic-embed-text)
- **Claude Code CLI** — LLM provider for AF64 runtime
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

## Naming Conventions
### Rust (dpn-core, dpn-api)
- **Functions & variables:** `snake_case` (e.g., `create_pool`, `list_vault_notes`, `get_by_path`)
- **Types & structs:** `PascalCase` (e.g., `VaultNote`, `DbPool`, `ApiError`)
- **Constants:** `UPPER_CASE` (e.g., `DEFAULT_DATABASE_URL`)
- **Modules:** `snake_case` (e.g., `vault_notes`, `stagehand`)
- **Enum variants:** `PascalCase` (e.g., `Status::Done`)
- **Serde:** `#[serde(rename_all = "snake_case")]` for DB serialization
### Common Lisp (AF64)
- **Functions:** `kebab-case` (e.g., `run-tick`, `perceive-agents`)
- **Special variables:** `*earmuffs*` (e.g., `*tick-interval*`, `*max-actions-per-tick*`)
- **Packages:** `af64.runtime.tick-engine` dot-separated hierarchy
- **JSON quirk:** Parser converts underscores to hyphens (`:is-error` not `:is_error`)
### Python (gotcha-workspace)
- **Functions:** `snake_case` (e.g., `run_scan`, `parse_task`)
- **Classes:** `PascalCase` (e.g., `DuplicateAnalyzer`, `CLIReporter`)
- **Constants:** `UPPER_CASE` (e.g., `DB_CONFIG`, `TASK_PATTERN`)
- **Private:** `_` prefix (e.g., `_discover_git`, `_print_header`)
- **Modules:** Docstrings at module level
### TypeScript/JavaScript (Next.js sites)
- **Files:** `kebab-case` for routes, `camelCase` for utilities
- **Components:** `PascalCase`
- **Variables:** `camelCase`
## Code Style & Formatting
- **Rust:** Default `rustfmt` (Edition 2021). No custom rustfmt.toml.
- **Python:** No centralized formatter config. Type hints used in function signatures.
- **TypeScript:** Tailwind CSS for styling. No explicit eslintrc/prettierrc in main projects.
- **Lisp:** Standard CL indentation.
## Error Handling Patterns
### Rust
- `anyhow::Result` for simple error propagation (primary pattern)
- `thiserror` for custom error types (ApiError enum in dpn-api)
- `?` operator throughout for propagation
- `tracing::warn!` / `tracing::error!` for logging errors
- Connection pool: `acquire_timeout(Duration::from_secs(10))`
### Python
- Standard exceptions
- `logging` module (DEBUG/INFO via CLI args)
### Lisp
- `handler-case` wrapping tick execution
- Error states logged but engine continues
## Logging Practices
### Rust
- **Library:** `tracing` + `tracing-subscriber` with `EnvFilter`
- **Config:** `RUST_LOG=dpn_api=debug,tower_http=debug`
- **Pattern:** Structured logging with context fields
- **PM2 logs:** `/var/log/dpn-api/{error,output}.log`
### Python
- Standard `logging` module
- Verbose mode: `logging.DEBUG if args.verbose else logging.INFO`
### Lisp
- `(format t ...)` to stdout
- Tick reports persisted to DB via API
## Common Patterns
### Rust
- Module organization by feature domain
- Type aliases: `pub type DbPool = PgPool;`
- Re-exports in `lib.rs` for crate-level convenience
- `#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]` standard combo
- All DB functions are `async`
- Documentation via `//!` module-level comments
### Python (GOTCHA Framework)
- **Tools:** One job per script, importable as function AND usable as CLI
- **Paths:** Always via `PATHS` from `tools/_config.py` (workspace-relative)
- **CLI pattern:** `if __name__ == "__main__":` guard
- **Dataclasses:** `@dataclass` for model definitions
- **Regex:** Pre-compiled as module constants
### Lisp
- Defstruct for data types
- Functional composition with `mapcar`, `remove-if-not`
- Alist/plist patterns for configuration
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

## System Overview
```
```
## Component Interactions & Data Flow
### Tick Cycle (AF64 Runtime — every 30s/10m configurable)
### Cognition Broker (Shared Resource Pool)
| Mode | Condition | Behavior |
|------|-----------|----------|
| Normal | frontier_enabled=1, pending < 18 | Up to 6 jobs/tick, call Claude API |
| Cognitive Winter | forced or pending ≥ 18 | 3 jobs/tick, stubs/cache for deterministic tasks |
| Thaw | pending ≤ 9 for 2 ticks | Resume normal gradually |
### Agent Tier System
| Tier | Condition | Model |
|------|-----------|-------|
| dormant | energy ≤ 0 | none |
| base | energy > 0 | claude-3-haiku |
| working | energy > 20 | claude-sonnet |
| prime | energy > 70, fitness > 50 | claude-sonnet |
| opus | Nova only | claude-opus |
## API Design Patterns
- **Router nesting:** `/api` prefix with domain-grouped handlers
- **Auth middleware:** JWT (Bearer) or API Key (X-API-Key header)
- **Error type:** `ApiError` enum (Database, NotFound, Unauthorized, etc.)
- **State:** Shared `PgPool` via Axum state extraction
- **Handlers organized by domain:** documents, tasks, events, projects, reading, graph, xmpp, af64_*
- `/api/documents/*` — Vault notes CRUD + search + wikilinks
- `/api/tasks/*` — Task listing, sync, due dates
- `/api/agents/*` — Agent state, tiers, drives, relationships
- `/api/perception/:agent_id` — Perception snapshot for tick engine
- `/api/reading/*` — RSS feeds, articles, OPML
- `/api/conversations` — Inter-agent messaging
## State Management
- `agents` + `agent_state` — Identity, energy, tier, last_tick_at
- `conversations` — Message bus (thread_id, from_agent, to_agent[], channel)
- `cognition_jobs` — Pending reasoning requests
- `tick_reports` — Empirical record of each tick
- `memory_entries` — Long-term agent memories (308+)
- `vault_notes` — 2,678+ operational documents
- `documents` — 47,760+ world docs
- `tasks` — 3,905+ work items
## Key Architectural Decisions
| Decision | Rationale |
|----------|-----------|
| Lisp for tick engine | Metaprogramming, fast SBCL compilation, REPL for live tweaking |
| Rust for API | Type safety, async performance (Axum), memory safety |
| PostgreSQL as substrate | Persistent agent state, LISTEN/NOTIFY, rich types (JSONB, arrays) |
| Cognition Broker | Scarcity-aware, prevents token waste, cognitive winter failover |
| DB-centric design | Agents read/write DB, no local-only state, crash-resilient |
| Empirical memory | Tick reports are facts, rollups derived from activity (not narrative) |
| DPN API as gateway | Decouples Rust/Python/Lisp internals from Next.js frontends |
| GOTCHA framework | Separates reasoning (LLM) from execution (tools) |
| Tick-first, cognition-second | Drives/perception drive action; cognition is a scarce resource |
<!-- GSD:architecture-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd:quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd:debug` for investigation and bug fixing
- `/gsd:execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->

<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd:profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
