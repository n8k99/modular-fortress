# External Integrations

**Analysis Date:** 2026-04-03

## APIs & External Services

**LLM Providers:**
- Anthropic Claude - Primary cognition provider
  - SDK/Client: `claude-code` CLI binary (`/root/.local/bin/claude`) via subprocess, plus direct HTTP API
  - Auth: `ANTHROPIC_API_KEY` in `config.json` and `OPENCLAW_GATEWAY_PASS` env var
  - Models: claude-opus-4-20250514, claude-sonnet-4-20250514, claude-3-haiku-20240307
  - Proxy: `http://127.0.0.1:18789/proxy/anthropic/v1/messages`
  - Budget: $0.50 max per Claude Code request, 180s timeout
  - Implementation: `project-noosphere-ghosts/lisp/runtime/claude-code-provider.lisp`, `cognition-broker.lisp`

- OpenAI - Alternative LLM provider
  - SDK/Client: HTTP via reqwest (Rust) or curl subprocess (Lisp)
  - Auth: `OPENAI_API_KEY` in `config.json`
  - Used by: Burgs pipeline, AI service hybrid mode

- Perplexity AI - Research and synthesis
  - Auth: `PERPLEXITY_API_KEY` in `config.json`
  - Status: Enabled in `ai_service` config

- Google Gemini - Image generation
  - SDK/Client: HTTP REST API
  - Auth: `GOOGLE_GEMINI_API_KEY` in `config.json`
  - Model: `imagen-3.0-generate-001`
  - Default size: 1024x1024, aspect ratio 16:9

- Ollama - Local LLM inference
  - URL: `http://localhost:11434` or `http://127.0.0.1:11434`
  - Models: `llama3.1:8b` (default), `llama3.2:latest`, `llava:13b` (vision)
  - Used by: Burgs pipeline for local AI generation

**Communication:**
- Discord - Bot integration and webhooks
  - SDK/Client: HTTP webhooks via curl/reqwest
  - Bot token: Configured in `config.json` under `discord.bot_token`, `eliana_bot.bot_token`
  - Client IDs: 1302799434744205395 (main), 1322360363987243071 (Eliana)
  - Webhooks: Combat, musicology, personas (LRMorgenstern, KathrynLyonne, LenaMarris, SarahLin), tech dev office team, office of CEO
  - Implementation: Webhook URLs in `config.json` under `discord.webhooks`

**Version Control:**
- GitHub - Code hosting and sync
  - SDK/Client: `gh` CLI tool via subprocess
  - Auth: GitHub PAT in `config.json` under `api_keys.github.token`
  - Implementation: `project-noosphere-ghosts/lisp/util/github.lisp`

**Print-on-Demand:**
- Printful - Merchandise fulfillment
  - SDK/Client: HTTP REST API
  - Auth: `PRINTFUL_API_KEY` in `config.json`
  - Store ID: 13913738
  - Products: Posters (18x24, 24x36, 12x16), shirts, hoodies, mugs, tote bags
  - Fulfillment tracking: `/Volumes/Elements/Nebulab/02 Areas/10 Success/Printful Fulfillment`

**CMS:**
- Ghost CMS - Blog and content publishing
  - URL: `https://eckenrodemuziekopname.com`
  - Admin API key: Configured in `config.json` under `ghost.admin_api_key`
  - Content API key: `e2a4818be6b71ec4c683101b9e` in `config.json`
  - Tier IDs: free, red-tier, white-tier, blue-tier, green-tier, yellow-tier, black-tier
  - Used by: Executive blogs, Ghost Voices rotation (7 personas/weekdays)

**Workflow Automation:**
- n8n - Workflow orchestration
  - Local: `http://localhost:5678` with JWT API key
  - Droplet: `https://n8n.eckenrodemuziekopname.com` with JWT API key
  - Auth: JWT tokens in `config.json` under `n8n.local.api_key`, `n8n.droplet.api_key`
  - Default: Local instance

**Infrastructure:**
- DigitalOcean - Cloud hosting
  - Auth: `DIGITALOCEAN_API_KEY` in `config.json`
  - Droplet: 144.126.251.126 (SSH tunnel for DB access)

**News/Data:**
- NewsAPI - News aggregation
  - Auth: API key `5cf576ab0d4441cb93cba884ba003d15` in `config.json`

**OpenRouter:**
- OpenRouter - Multi-model LLM gateway
  - Auth: `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` (noted at end of `config.json`)

**Design Tools:**
- Figma - Design collaboration
  - Client ID: `7j1f6DjVUQt5ZUoxeBSFEe`
  - Auth: Personal access token in `config.json` under `figma.personal_access_token`

**Legacy/Placeholder:**
- Telegram (API ID/hash configured, no active integration)
- Notion (token placeholder in config)
- Google Sheets (credentials path placeholder)
- OpenCode Zen (custom API key in config)

## Data Storage

**Databases:**
- PostgreSQL 16 - Primary database
  - Connection: `DATABASE_URL` env var, `config.json` database section
  - Host: localhost (or 127.0.0.1 via SSH tunnel on port 5433)
  - Port: 5432 (local), 5433 (SSH tunnel)
  - Database: `master_chronicle` (primary), `orbis_narratives` (narrative data)
  - User: `nebulab_user` (dev), `chronicle` (production)
  - Client: sqlx (Rust), libpq.so.5 via SB-ALIEN FFI (Common Lisp)
  - Schema: `noosphere-schema/schema/` directory
  - Connection pool: Size 10, configurable in `config.json`

- DoltgreSQL - Versioned PostgreSQL
  - Host: 127.0.0.1
  - Port: 5435
  - User: root
  - Config: `noosphere-schema/doltgres-config.yaml`
  - Purpose: Experimental schema evolution with git-like versioning

- SQLite - Embedded database
  - Client: rusqlite 0.31 with bundled library (`dpn-core`)
  - Purpose: Local state, incremental sync tracking (`.sync_state.json`)

**File Storage:**
- Local filesystem primary
  - Vault path: `/Volumes/Elements/Nebulab`
  - TimeNotes: `/Volumes/Elements/Nebulab/02 Areas/01 TimeNotes/`
  - Product tracking: `./products_log`
  - TASKS memory: `/Volumes/Elements/tasks_memory`
  - Runtime artifacts: `/tmp/noosphere_ghosts`

**Caching:**
- In-memory caches in cognition broker (`project-noosphere-ghosts/lisp/runtime/cognition-broker.lisp`)
- No external caching service (Redis, Memcached) detected

## Authentication & Identity

**Auth Provider:**
- Custom JWT implementation
  - Algorithm: HS256
  - Secret: `JWT_SECRET` env var in `dpn-api/.env`
  - Expiry: 24 hours
  - Claims: sub (user ID), name (user name), iat, exp
  - Implementation: `dpn-api/src/` using jsonwebtoken 9

**API Key Authentication:**
- API Keys: Comma-separated list in `API_KEYS` env var (`dpn-api/.env.example`)
- Header: `X-API-Key`
- Multiple services configured in `config.json` under `api_keys`

**External Auth:**
- Discord OAuth (client IDs and secrets configured, URLs available)
- GitHub PAT for version control operations

## Monitoring & Observability

**Error Tracking:**
- None (no Sentry, Rollbar, or similar service detected)

**Logs:**
- Structured logging via tracing/tracing-subscriber (Rust)
- File-based logs:
  - `dpn-api`: systemd journal or `/var/log/dpn-api/` (PM2)
  - Discord bot: `/Volumes/Elements/Nebulab/scripts/.tasks_discord.log`
  - Core server: `/Volumes/Elements/Nebulab/scripts/.tasks_core.log`
- Log level: Configurable via `RUST_LOG` env var

**Telemetry:**
- Tick reports stored in database (`project-noosphere-ghosts` tick engine)
- Empirical rollups: daily/weekly/monthly/quarterly/yearly (`runtime/empirical-rollups.lisp`)

## CI/CD & Deployment

**Hosting:**
- DigitalOcean Droplet (144.126.251.126)
- Local development: macOS (Darwin 25.3.0)

**CI Pipeline:**
- None (no GitHub Actions, GitLab CI, or similar detected)

**Deployment:**
- systemd service: `dpn-api.service` for `dpn-api`
- PM2: `ecosystem.config.js` for process management
- Launch script: `project-noosphere-ghosts/launch.sh` for AF64 runtime
- Manual deployment via SSH and binary copy

## Environment Configuration

**Required env vars:**

dpn-api:
- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Strong secret for JWT signing (32+ chars)
- `API_KEYS` - Comma-separated API keys
- `RUST_LOG` - Logging level (default: `dpn_api=info,tower_http=debug`)
- `PORT` - Server port (default: 8080)

project-noosphere-ghosts (AF64):
- `AF64_RUNTIME_DIR` - Runtime artifact directory (default: `/tmp/noosphere_ghosts`)
- `AF64_PRIMARY_USER_HANDLE` - Primary user wiki handle
- `AF64_PRIMARY_USER_ID` - Primary user database ID
- `AF64_PRIMARY_USER_NAME` - Primary user name
- `AF64_PERSONA_DIR` - Ghost persona files location
- `AF64_PERSONA_MAP_FILE` - Persona mapping file
- `AF64_MEMORY_TABLE` - Memory persistence table
- `AF64_MEMORY_LAYER` - Memory layer (e.g., daily)
- `COGNITION_PROVIDER_CONFIG` - Provider config path (e.g., `@config/provider-config.json`)
- `COGNITION_API_KEY_ENV` - Default API key env var name
- `FRONTIER_COGNITION_ENABLED` - Enable/disable frontier LLM access
- `OPENCLAW_GATEWAY_PASS` - OpenClaw proxy authentication

**Secrets location:**
- `config.json` at repo root (contains API keys, database credentials, service tokens)
- `.env` files in `dpn-api/` (not committed, template in `.env.example`)
- `config/af64.env` in `project-noosphere-ghosts/` (generated by onboarding wizard)

## Webhooks & Callbacks

**Incoming:**
- None detected (no webhook receivers in `dpn-api` handlers)

**Outgoing:**
- Discord webhooks for multiple channels and personas
- URLs configured in `config.json` under `discord.webhooks`
- Used by: Combat system, musicology team, persona voices, tech dev office communications

## Database Schema

**Key tables in master_chronicle:**
- `vault_notes` - Primary note storage (~2,678 records) with embeddings
- `stagehand_notes` - Show/venue notes with semantic search
- `documents` - Legacy document storage (~47K records, light queries only)
- `daily_logs` - Agent daily log entries (47 records) with embeddings
- `memory_entries` - Structured memories (312 records) with tags and embeddings
- `agents` - Agent registry (7 records) with roles and permissions
- `conversations` - Conversation threads with UUIDs
- `tasks` - Task management with status and dependencies
- `events` - Calendar events
- `projects` - Project tracking
- `areas` - Life areas organization
- `archives` - Archived content

**Orbis narratives database:**
- Historical lore, plots, population data
- Items, locations hierarchy
- Generated from Azgaar's Fantasy Map Generator data

---

*Integration audit: 2026-04-03*
