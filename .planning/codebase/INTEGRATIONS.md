# External Integrations

**Analysis Date:** 2026-04-04

## APIs & External Services

**AI & Language Models:**
- Anthropic Claude API - Primary LLM provider
  - SDK/Client: `reqwest` HTTP client in Rust (`noosphere/src/core/embeddings/generator.rs`)
  - Auth: `config.json` `api_keys.anthropic.api_key` (`sk-ant-api03-l7cV6PCb...`)
  - Endpoint: `https://api.anthropic.com/v1/messages`
  - Used by: Agent cognition, conversation generation, document analysis
  - Also used by AF64 runtime: Hand-rolled HTTP client via `af64.utils.http` (curl wrapper)

- OpenAI API - Secondary LLM provider + embeddings
  - SDK/Client: `reqwest` HTTP client in Rust
  - Auth: `config.json` `api_keys.openai.api_key` (`sk-proj-EmUbhQMk...`)
  - Endpoint: `https://api.openai.com/v1/chat/completions`
  - Embeddings endpoint: `https://api.openai.com/v1/embeddings`
  - Model: `text-embedding-3-small` (configured in `noosphere/src/core/embeddings/generator.rs`)
  - Used by: Embedding generation, provider adapter chain

- Perplexity API - Research and search augmentation
  - Auth: `config.json` `ai_service.perplexity.api_key` (`pplx-WY26ereFm...`)
  - Enabled: `ai_service.perplexity.enabled: true`
  - Used by: Knowledge retrieval, fact-checking workflows

- Ollama (Local) - Offline LLM inference
  - URL: `http://localhost:11434` (configured in noosphere and config.json)
  - Default model: `llama3.1:8b` (config.json)
  - Embedding model: `nomic-embed-text` (noosphere/src/core/embeddings/generator.rs)
  - Vision model: `llava:13b` (config.json burgs_pipeline)
  - Endpoint: `http://127.0.0.1:11434/api/generate`
  - Used by: Local embeddings, burgs pipeline, nightly memory synthesis (`nightly-memory-synthesis.py`)

- Google Gemini - Image generation
  - Auth: `config.json` `api_keys.google.gemini.api_key` (`AIzaSyAC9bpuh8jN...`)
  - Model: `imagen-3.0-generate-001`
  - Settings: Default size 1024x1024, aspect ratio 16:9
  - Used by: Visual content generation

**Content Management:**
- Ghost CMS - Blog publishing platform
  - URL: `https://eckenrodemuziekopname.com`
  - Admin API key: `config.json` `ghost.admin_api_key` (`697d938a5ea490000132...`)
  - Content API key: `config.json` `ghost.content_api_key` (`e2a4818be6b71ec4c6...`)
  - Used by: Executive blog pipeline, morning pages publishing
  - Tier IDs: 7 membership tiers (free, red-tier, white-tier, blue-tier, green-tier, yellow-tier, black-tier)
  - Functions: Post creation, tag management, member access control

**Communication:**
- Discord API - Multi-bot integration with extensive webhook network
  - Primary Bot:
    - Token: `config.json` `api_keys.discord.bot_token` (`MTMwMjc5OTQzNDc0NDIwNTM5NQ...`)
    - Client ID: 1302799434744205395
    - OAuth URL: `https://discord.com/oauth2/authorize?client_id=1302799434744205395&permissions=2833846198455543...`
  - Eliana Bot (CTO persona):
    - Token: `config.json` `eliana_bot.bot_token` (`MTMyMjM2MDM2Mzk4NzI0MzA3MQ...`)
    - Bot ID: 1322360363987243071
  - Webhooks (20+ channels in config.json):
    - `tech_dev_office.*` - 9 persona webhooks (ElianaRiviera, SamirKhanna, DevinPark, CaseyHan, SanjayPatel, DanielleGreen, MorganFields, IsaacMiller, ElisePark)
    - `personas.*` - LRMorgenstern, FionaCarter, TaraBennett, MarcelloRuiz, EvelynWoods, KathrynLyonne, LenaMorris, SarahLin
    - `combat`, `musicology_channel`, `common`, `cif-13τ07`, `guinea_pigs`
  - Tasks channel ID: 1421565389745295441
  - Command prefix: `!`
  - Max message length: 2000
  - Used by: Agent messaging, command processing, notification system, team coordination

- Telegram API - Messaging integration
  - API ID: 23766963 (config.json)
  - API hash: `config.json` `api_keys.google.telegram.api_hash` (`e559971dd246cbfec6...`)
  - App title: T.A.S.K.S.-N8N Automation
  - App short name: tasksn8n

**Version Control:**
- GitHub API - Repository management
  - Token: `config.json` `api_keys.github.token` (`ghp_4z1jNOTnlybhCfz...`)
  - Used by: Task sync, issue tracking, project descriptions (AF64 `af64.utils.github`)
  - Functions: `sync-all`, `sync-github-issues`, `push-tasks-to-github`
  - Endpoints: GraphQL + REST API

**Workflow Automation:**
- n8n Workflows - Automation platform (dual deployment)
  - Local instance:
    - URL: `http://localhost:5678`
    - API key: `config.json` `n8n.local.api_key` (`eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...`)
  - Droplet instance:
    - URL: `https://n8n.eckenrodemuziekopname.com`
    - API key: `config.json` `n8n.droplet.api_key` (separate token, expiry 2025-12-31)
  - Default: local
  - Used by: Workflow orchestration, external API chaining, automation triggers

**Design Tools:**
- Figma API - Design file access
  - Client ID: `7j1f6DjVUQt5ZUoxeBSFEe` (config.json)
  - Client secret: `a9faRy7iUhMsnvI1dDvaWzHmYwk6qt` (config.json)
  - Personal access token: `config.json` `figma.personal_access_token` (`figd_M6IaN0yVRzNPx4V...`)
  - Used by: Design asset retrieval, mockup generation

## Data Storage

**Databases:**
- PostgreSQL master_chronicle (primary)
  - Connection: `config.json` `database.master_chronicle`
  - Local: `postgresql://nebulab_user:nebulab_dev_password@localhost:5432/master_chronicle`
  - Droplet: `db.eckenrodemuziekopname.com:5432` (password: PLACEHOLDER_PASSWORD_DROPLET)
  - Client libraries:
    - SQLx 0.8 (Rust async client in noosphere)
    - psycopg2 (Python export script)
    - libpq via FFI (Common Lisp AF64 runtime via `af64.utils.pg`)
  - Connection pool: 10 connections
  - Size: 464MB dump, 83 tables, 2,554 tasks, 9,846 conversations
  - Extensions: `vector` (embeddings), `pg_trgm` (text search)

- PostgreSQL orbis_narratives (secondary)
  - Connection: `config.json` `database.orbis_narratives`
  - Host: `localhost:5432`
  - User: `nebulab_user`
  - Purpose: Orbis world data (lore, plots, locations hierarchy, population, items)
  - Client: SQLx (Rust)

- SQLite cache.db (local)
  - Location: `~/.dpn/cache.db`
  - Client: rusqlite 0.31 (bundled, no system dependency)
  - Purpose: Offline-first caching, sync queue for master_chronicle synchronization

**File Storage:**
- Local filesystem only
  - Vault path: `/Volumes/Elements/Nebulab` (Obsidian vault)
  - Assets path: `/Volumes/Elements/Nebulab/Assets`
  - Memory logs: `/Volumes/Elements/tasks_memory`
  - Daily notes: `/Volumes/Elements/tasks_memory/logs/Daily Notes`
  - Executive blog posts: `/Volumes/Elements/Nebulab/02 Areas/06 Executive/03 Blog/ExecutiveDailyPosts`
  - Printful fulfillment: `/Volumes/Elements/Nebulab/02 Areas/10 Success/Printful Fulfillment`
  - Discord logs: `/Volumes/Elements/Nebulab/scripts/.tasks_discord.log`
  - Core server logs: `/Volumes/Elements/Nebulab/scripts/.tasks_core.log`

**Caching:**
- SQLite local cache (dpn-core `cache` module via rusqlite)
- Cognition broker state (AF64 broker-state-path)
- Pipeline definition cache (AF64 `*pipeline-cache*`)
- Tool definition cache (AF64 `*tool-definition-cache*`)

## Authentication & Identity

**Auth Provider:**
- Custom JWT-based authentication
  - Implementation: `dpn-api/src/auth.rs` + `noosphere/src/` (jsonwebtoken 9 crate)
  - Token generation: API `/auth` endpoints
  - API key: `DPN_API_KEY` environment variable
  - Future: Multi-user support planned

**Service Accounts:**
- Discord bots: Primary bot, Eliana bot (CTO persona)
- Ghost CMS: Admin + content API keys
- GitHub: Personal access token
- OpenClaw gateway: Referenced in AF64 runtime (`af64.runtime.openclaw-gateway`)

## Monitoring & Observability

**Error Tracking:**
- None (no Sentry, Rollbar, or third-party error tracking)

**Logs:**
- File-based logging:
  - Core server: `/Volumes/Elements/Nebulab/scripts/.tasks_core.log`
  - Discord bot: `/Volumes/Elements/Nebulab/scripts/.tasks_discord.log`
- Rust tracing:
  - Framework: `tracing` 0.1 + `tracing-subscriber` 0.3
  - Configuration: `RUST_LOG` environment variable (default: `noosphere=debug,tower_http=debug`)
  - Features: Structured logging with env-filter
- Log level: INFO (configurable via `config.json` `logging.level`)
- Format: `%(asctime)s - %(name)s - %(levelname)s - %(message)s`

**Telemetry:**
- Broker telemetry: AF64 cognition broker writes to broker-telemetry-path
- Tick reports: AF64 tick reporting writes to tick-reports-path
- Empirical rollups: Daily/weekly/monthly/quarterly/yearly aggregations
- System stats: `/api/system/stats` endpoint exposes active_ghosts, idle_ghosts, dormant_ghosts, total_tasks, total_conversations, tick_number, uptime_days

## CI/CD & Deployment

**Hosting:**
- DigitalOcean droplet
  - IP: 144.126.251.126
  - API token: `config.json` `api_keys.digitalocean.api_key` (`dop_v1_6e48bf00a...`)
  - Services: dpn-api, master_chronicle database, n8n workflows

**CI Pipeline:**
- None (manual deployment)
- dpn-core synced from droplet at 00:07 (rsync)
- master_chronicle.dump downloaded at 22:54 (464MB)

**Deployment Process:**
- Build: `cargo build --release` (noosphere)
- Transfer: rsync to droplet server
- Database: `pg_restore -d master_chronicle master_chronicle.dump`
- Service: Manual restart

## Environment Configuration

**Required env vars:**
- `DATABASE_URL` - PostgreSQL connection string (default: `postgresql://nebulab_user:nebulab_dev_password@localhost:5432/master_chronicle`)
- `DPN_API_URL` - Base URL for dpn-api (e.g., `http://localhost:8080`)
- `DPN_API_KEY` - Authentication token for dpn-api
- `RUST_LOG` - Logging level (e.g., `noosphere=debug,tower_http=debug`)
- `OPENAI_API_KEY` - OpenAI API key for embeddings (optional, can use config.json)

**Secrets location:**
- `config.json` at repository root - Contains all API keys, tokens, passwords
  - Warning: Tracked in git with live secrets
  - No `.env` file present in repository root

**Local Services:**
- Obsidian REST API: `http://localhost:27123`
  - Vault: `/Volumes/Elements/Nebulab`
  - API key: `config.json` `services.mcp.vault_mcp_server.env.OBSIDIAN_API_KEY` (`80a3e285ffb968c0c6...`)
- Ollama: `http://localhost:11434`
- Noosphere server: `http://localhost:8888` (replaces core_server on port 8888)
- n8n: `http://localhost:5678`

## Webhooks & Callbacks

**Incoming:**
- None detected in noosphere API handlers

**Outgoing:**
- Discord webhooks (20+ endpoints in `config.json` `discord.webhooks`):
  - Combat notifications: `https://discord.com/api/webhooks/1429868525144510466/...`
  - Musicology channel: `https://discord.com/api/webhooks/1336178754136838237/...`
  - Tech dev office personas: 9 separate webhook URLs
  - Office of CEO: Sarah Lin webhook
  - Common channel: `https://discord.com/api/webhooks/1430319624838184981/...`
  - Guinea pigs testing: `https://discord.com/api/webhooks/1430322217509589063/...`
  - CIF-13τ07: `https://discord.com/api/webhooks/1430322214581960817/...`

## E-commerce

**Printful API:**
- API key: `config.json` `printful.api_key` (`JYJaFC08jGVkyZj3N0dH...`)
- Store ID: 13913738
- Products with variant IDs:
  - Poster: 18x24 (4012), 24x36 (4013), 12x16 (4011) - $25.00 base
  - Shirt: XS-3XL (1000-1006) - $20.00 base
  - Hoodie: XS-2XL (2000-2005) - $40.00 base
  - Mug: 11oz (3000), 15oz (3001) - $15.00 base
  - Tote bag: Standard (5000) - $12.00 base
- Product name: "Enhanced Matte Paper Poster (in)"
- Default size: 18x24
- Mockup enabled: true for all products
- Fulfillment tracking: `/Volumes/Elements/Nebulab/02 Areas/10 Success/Printful Fulfillment`

## RSS & News

**News API:**
- API key: `config.json` `api_keys.news_api` (`5cf576ab0d4441cb93cba...`)
- Used by: News aggregation workflows

**Feed Parsing:**
- Rust crates in noosphere:
  - `feed-rs = "2.1"` - RSS/Atom parsing (`noosphere/Cargo.toml`)
  - `scraper = "0.21"` - HTML parsing for feed auto-discovery
  - `url = "2.5"` - URL manipulation and normalization
- Implementation: `noosphere/src/core/reading/` module
- Types: Feed, Article, FeedType (RSS/Atom)

## Calendar Integration

**Google Calendar:**
- ICS URL: `config.json` `calendars.call_stewart.ics_url` (Google Calendar private ICS link)
- Calendar name: "Call Stewart Union Jobs"
- Type: work
- Sync enabled: true
- Parser: `ical = "0.11"` crate in noosphere (`noosphere/Cargo.toml`)
- Implementation: `noosphere/src/core/ics/parser.rs`

## Knowledge Management

**Obsidian Integration:**
- Local REST API: `http://localhost:27123`
- Vault path: `/Volumes/Elements/Nebulab`
- MCP server: Python subprocess via `vault_mcp_server.py`
  - Command: `python3`
  - Args: `["/Volumes/Elements/Nebulab/scripts/vault_mcp_server.py"]`
  - API key: `config.json` `services.mcp.vault_mcp_server.env.OBSIDIAN_API_KEY` (`80a3e285ffb968...`)
- TimeNotes structure:
  - Base: `/Volumes/Elements/Nebulab/02 Areas/01 TimeNotes`
  - Daily: `01 Daily Notes`
  - Weekly: `02 Week Notes`
  - Monthly: `03 Monthly Notes`
  - Quarterly: `04 Quarter Notes`
  - Yearly: `05 Yearly Notes`
- Templates: `/Volumes/Elements/Nebulab/Templates`

**Notion (Configured but unused):**
- Token: `YOUR_NOTION_TOKEN_HERE` (placeholder in config.json)

## Document Processing

**Wikilink Resolution:**
- Implementation: `noosphere/src/core/wikilinks/mod.rs`
- Pattern: `[[Note Title]]` parsing
- Database: PostgreSQL integration via SQLx
- Functions: `parse_wikilinks`, `resolve_wikilink`, `build_link_graph`

**Graph Visualization:**
- Implementation: `noosphere/src/core/graph/mod.rs`
- Database: PostgreSQL integration via SQLx
- Data structures: Node/edge relationships for document graph
- Functions: `build_graph`, `get_hub_documents`, `get_orphan_documents`

## InnateScipt Integration

**Resolver Protocol:**
- Interface: `innate.eval.resolver` - Generic resolver protocol
- Implementation: `af64.runtime.noosphere-resolver` - Noosphere-specific resolver
- Resolves: `@` references, searches, commissions, wikilinks, bundles
- Data sources: PostgreSQL master_chronicle, area_content table

**Template System:**
- Implementation: `af64.runtime.innate-builder`
- Storage: PostgreSQL master_chronicle
- Functions: `validate-innate-expression`, `db-insert-template`, `db-find-template-by-name`
- Validation: InnateScipt syntax checking before storage

## Ghost Capabilities

**YAML Configuration:**
- Implementation: `af64.runtime.ghost-capabilities`
- Parser: `af64.utils.yaml` (hand-rolled, zero external dependencies)
- Validates: InnateScipt expressions in capability declarations
- Functions: `load-ghost-capabilities`, `format-capabilities-for-prompt`
- Storage: YAML files loaded per-ghost at runtime

## Team Configuration

**Burgs Pipeline Team:**
- 14 team members configured in `config.json` `burgs_team.team_members`
- Departments: Technical (5), Art (4), Content (1), Marketing (4)
- Notification system:
  - Default webhook: null (disabled)
  - Project channel: `#burgs-pipeline`
  - Status emojis: 🚀 (started), ⚙️ (in_progress), ✅ (completed), ❌ (failed), ⚠️ (warning), 👀 (review)
  - Department colors: Technical (3447003), Art (15844367), Content (10181046), Marketing (15105570)

**Ghost Voices:**
- 7 executive personas (config.json `ghost_voices.voices`):
  - 0: Nathan Eckenrode (CEO, Monday)
  - 1: Vincent Janssen (CCO, Tuesday)
  - 2: J. Maxwell Charbourne (CSO, Wednesday)
  - 3: Eliana Riviera (CTO, Thursday)
  - 4: L.R. Morgenstern (CAO, Friday)
  - 5: Kathryn Lyonne (COO, Saturday)
  - 6: Sylvia Inkweaver (Chief of Content, Sunday)
- Each voice has: role, slug, bio, writing_style, voice_characteristics, tags
- Used by: Morning pages generation, executive blog posting

---

*Integration audit: 2026-04-04*
