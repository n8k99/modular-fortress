# External Integrations

**Analysis Date:** 2026-04-04

## APIs & External Services

**AI & Language Models:**
- Anthropic Claude API - Primary LLM provider
  - SDK/Client: Hand-rolled HTTP client via `af64.utils.http` (curl wrapper)
  - Auth: `config.json` `api_keys.anthropic.api_key` (`sk-ant-api03-...`)
  - Models: claude-sonnet-4-20250514 (prime, working), claude-3-haiku-20240307 (base)
  - Used by: AF64 cognition broker (`af64.runtime.cognition-broker`)

- OpenAI API - Secondary LLM provider
  - SDK/Client: Hand-rolled HTTP client
  - Auth: `config.json` `api_keys.openai.api_key` (`sk-proj-EmUb...`)
  - Endpoint: `https://api.openai.com/v1/chat/completions`
  - Used by: Provider adapter chain (`af64.runtime.provider-adapters`)

- Perplexity API - Research/search augmentation
  - Auth: `config.json` `ai_service.perplexity.api_key` (`pplx-WY26...`)
  - Enabled: `ai_service.perplexity.enabled: true`

- Ollama (Local) - Offline LLM inference
  - URL: `http://localhost:11434`
  - Model: llama3.1:8b (also llava:13b for vision)
  - Used by: Burgs pipeline, local AI fallback

- Google Gemini - Image generation
  - Auth: `config.json` `api_keys.google.gemini.api_key` (`AIzaSyAC9bpuh...`)
  - Model: imagen-3.0-generate-001
  - Default size: 1024x1024, aspect ratio 16:9

**Content Management:**
- Ghost CMS - Blog publishing platform
  - URL: `https://eckenrodemuziekopname.com`
  - Admin API: `config.json` `ghost.admin_api_key` (`697d938a5ea...`)
  - Content API: `config.json` `ghost.content_api_key` (`e2a4818be6b...`)
  - Used by: Executive blog pipeline, morning pages publishing
  - Tier IDs: 7 tiers (free, red, white, blue, green, yellow, black)

**Communication:**
- Discord API - Multi-bot integration
  - Primary Bot Token: `config.json` `api_keys.discord.bot_token` (`MTMwMjc5OTQzNDc0NDIwNTM5NQ...`)
  - Client ID: 1302799434744205395
  - Webhooks: 20+ channels including:
    - `tech_dev_office.*` - 9 persona webhooks (Eliana, Samir, Devin, Casey, Sanjay, Danielle, Morgan, Isaac, Elise)
    - `personas.*` - LRMorgenstern, KathrynLyonne, LenaMorris, SarahLin
    - `combat`, `musicology_channel`, `common`, `cif-13τ07`, `guinea_pigs`
  - Tasks channel: 1421565389745295441
  - Used by: Agent messaging, command processing, notification system

- Telegram API - Messaging integration
  - API ID: 23766963
  - Auth: `config.json` `api_keys.google.telegram.api_hash` (`e559971dd246cbf...`)
  - App: T.A.S.K.S.-N8N Automation

**Version Control:**
- GitHub API - Repository management
  - Token: `config.json` `api_keys.github.token` (`ghp_4z1jNOTnlybhCfz...`)
  - Client: `af64.utils.github` (GraphQL + REST)
  - Used by: Task sync, issue tracking, project descriptions
  - Functions: `sync-all`, `sync-github-issues`, `push-tasks-to-github`

**Workflow Automation:**
- n8n Workflows - Automation platform
  - Local: `http://localhost:5678` (API key: `eyJhbGciOiJIUzI1NiIs...`)
  - Droplet: `https://n8n.eckenrodemuziekopname.com` (separate API key)
  - Default: local
  - Used by: Workflow orchestration, external API chaining

**Design Tools:**
- Figma API - Design file access
  - Client ID: 7j1f6DjVUQt5ZUoxeBSFEe
  - Client Secret: a9faRy7iUhMsnvI1dDvaWzHmYwk6qt
  - Personal Access Token: `config.json` `figma.personal_access_token` (`figd_M6IaN0yVRz...`)

## Data Storage

**Databases:**
- PostgreSQL master_chronicle
  - Connection: `config.json` `database.master_chronicle`
  - Host: localhost:5432 (local), db.eckenrodemuziekopname.com:5432 (droplet)
  - User: nebulab_user
  - Password: nebulab_dev_password (local), PLACEHOLDER_PASSWORD_DROPLET (remote)
  - Client: sqlx (Rust), libpq via FFI (Common Lisp)
  - Pool size: 10 connections
  - Tables: 50+ (agents, tasks, conversations, documents, agent_drives, agent_fitness, etc.)

- PostgreSQL orbis_narratives
  - Connection: `config.json` `database.orbis_narratives`
  - Host: localhost:5432
  - Purpose: Orbis narrative data (lore, plots, locations hierarchy)
  - Client: sqlx (Rust)

- SQLite cache.db
  - Location: `~/.dpn/cache.db`
  - Client: rusqlite (bundled, no system dependency)
  - Purpose: Offline-first caching, sync queue for pending changes

**File Storage:**
- Local filesystem only
  - Vault path: `/Volumes/Elements/Nebulab` (Obsidian vault)
  - Assets path: `/Volumes/Elements/Nebulab/Assets`
  - Memory logs: `/Volumes/Elements/tasks_memory`
  - Daily notes: `/Volumes/Elements/tasks_memory/logs/Daily Notes`
  - Fulfillment tracking: `/Volumes/Elements/Nebulab/02 Areas/10 Success/Printful Fulfillment`

**Caching:**
- Local SQLite (dpn-core `cache` module)
- Cognition broker state cache (AF64 broker-state-path)
- Pipeline definition cache (`af64.runtime.pipeline-definitions` `*pipeline-cache*`)
- Tool definition cache (`af64.runtime.tool-definitions` `*tool-definition-cache*`)

## Authentication & Identity

**Auth Provider:**
- Custom JWT-based authentication
  - Implementation: `dpn-api/src/auth.rs` (jsonwebtoken 9)
  - Token generation: dpn-api `/auth` endpoints
  - API key required: `DPN_API_KEY` environment variable

**Service Accounts:**
- Multiple bot/service identities in Discord (Eliana bot, primary bot)
- OpenClaw gateway (referenced in `af64.runtime.openclaw-gateway`)

## Monitoring & Observability

**Error Tracking:**
- None (no Sentry, Rollbar, etc.)

**Logs:**
- File-based logging:
  - `config.json` `logging.files.core_server`: `/Volumes/Elements/Nebulab/scripts/.tasks_core.log`
  - `config.json` `logging.files.discord_bot`: `/Volumes/Elements/Nebulab/scripts/.tasks_discord.log`
- tracing-subscriber (Rust) - Structured logging with env-filter
- Level: INFO (configurable via `logging.level`)

**Telemetry:**
- Broker telemetry: `af64.runtime.cognition-broker` writes to broker-telemetry-path
- Tick reports: `af64.runtime.tick-reporting` writes to tick-reports-path
- Empirical rollups: Daily/weekly/monthly/quarterly/yearly aggregations

## CI/CD & Deployment

**Hosting:**
- DigitalOcean droplet at 144.126.251.126
- API token: `config.json` `api_keys.digitalocean.api_key` (`dop_v1_6e48bf00a...`)

**CI Pipeline:**
- None (manual deployment via rsync)
- dpn-core synced from droplet at 00:07
- master_chronicle.dump downloaded at 22:54

**Deployment Process:**
- rsync to droplet server
- Database restore from `master_chronicle.dump`
- Service restart (manual)

## Environment Configuration

**Required env vars:**
- `DPN_API_URL` - Base URL for dpn-api (e.g., `http://localhost:8080`)
- `DPN_API_KEY` - Authentication token for dpn-api
- Database credentials in `config.json` (not env vars)

**Secrets location:**
- `config.json` - API keys, tokens, passwords (NOT .env files)
- Note: config.json is tracked in repository (contains live secrets)

**Local Services:**
- Obsidian REST API: `http://localhost:27123`
- Ollama: `http://localhost:11434`
- Core server: `http://localhost:8888`
- n8n: `http://localhost:5678`

## Webhooks & Callbacks

**Incoming:**
- None detected in dpn-api handlers

**Outgoing:**
- Discord webhooks (20+ endpoints in `config.json` `discord.webhooks`)
  - Combat notifications
  - Musicology channel updates
  - Tech dev office persona messages
  - Common channel alerts
  - Guinea pigs testing channel

## E-commerce

**Printful API:**
- API key: `config.json` `printful.api_key` (`JYJaFC08jGVky...`)
- Store ID: 13913738
- Products: poster, shirt, hoodie, mug, tote-bag (with variant IDs)
- Base prices in cents (2500, 2000, 4000, 1500, 1200)
- Fulfillment tracking integrated with Obsidian vault

## RSS & News

**News API:**
- API key: `config.json` `api_keys.news_api` (`5cf576ab0d4441cb...`)

**Feed Parsing:**
- feed-rs 2.1 (Rust crate in dpn-core)
- scraper 0.21 for HTML auto-discovery
- URL normalization via url 2.5 crate

## Calendar Integration

**Google Calendar:**
- ICS URL: `config.json` `calendars.call_stewart.ics_url`
- Type: work
- Sync enabled: true
- Parser: ical 0.11 crate in dpn-core

## Knowledge Management

**Obsidian Integration:**
- Local REST API: `http://localhost:27123`
- Vault path: `/Volumes/Elements/Nebulab`
- MCP server: Python subprocess (`vault_mcp_server.py`)
- API key: `config.json` `services.mcp.vault_mcp_server.env.OBSIDIAN_API_KEY` (`80a3e285ffb968...`)
- TimeNotes structure: Daily/Week/Monthly/Quarter/Yearly notes
- Templates: `/Volumes/Elements/Nebulab/Templates`

**Notion (Configured but unused):**
- Token placeholder: `YOUR_NOTION_TOKEN_HERE`

## Document Processing

**Wikilink Resolution:**
- dpn-core `wikilinks` module
- Pattern: `[[Note Title]]` parsing
- Functions: `parse_wikilinks`, `resolve_wikilink`, `build_link_graph`

**Graph Visualization:**
- dpn-core `graph` module
- Node/edge data for document relationships
- Functions: `build_graph`, `get_hub_documents`, `get_orphan_documents`

## InnateScipt Integration

**Resolver Protocol:**
- `innate.eval.resolver` - Generic resolver interface
- `af64.runtime.noosphere-resolver` - Noosphere-specific implementation
- Resolves: references (`@`), searches, commissions, wikilinks, bundles
- Data sources: PostgreSQL master_chronicle, area_content table

**Template System:**
- `af64.runtime.innate-builder` - Template validation and CRUD
- Storage: PostgreSQL (referenced in packages.lisp)
- Functions: `validate-innate-expression`, `db-insert-template`, `db-find-template-by-name`

## Ghost Capabilities

**YAML Configuration:**
- `af64.runtime.ghost-capabilities` - Per-ghost capability loader
- `af64.utils.yaml` - Simple YAML parser (hand-rolled)
- Validates InnateScipt expressions in capability declarations
- Functions: `load-ghost-capabilities`, `format-capabilities-for-prompt`

---

*Integration audit: 2026-04-04*
