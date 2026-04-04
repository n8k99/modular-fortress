# Technology Stack

**Analysis Date:** 2026-04-04

## Languages

**Primary:**
- Rust 1.70+ (edition 2021) - Unified noosphere web server consolidating dpn-api, dpn-core, dpn-mcp into single codebase (`noosphere/`)
- Common Lisp (SBCL 2.x) - AF64 agent runtime (`project-noosphere-ghosts/lisp/`) and InnateScipt interpreter (`innatescript/`)

**Secondary:**
- Python 3.x - Database export utilities (`export_db_to_markdown.py`) and agent memory synthesis (`project-noosphere-ghosts/tools/`)
- SQL (PostgreSQL dialect) - Database schema definitions (`noosphere-schema/schema/`)
- JavaScript/HTML/CSS - Frontend dashboard UI (`noosphere/static/noosphere-ops.html`, mockups)

## Runtime

**Environment:**
- Rust async runtime: tokio 1.x with full features (powers all I/O operations in noosphere server)
- Common Lisp: SBCL (Steel Bank Common Lisp) 2.x
- Python 3.x interpreter (standard library + psycopg2)

**Package Manager:**
- Rust: Cargo
  - `noosphere/Cargo.toml` - Unified server
  - `dpn-api/Cargo.toml` - Legacy API (being consolidated)
  - `dpn-core/Cargo.toml` - Shared library (being consolidated)
  - Lockfiles: `Cargo.lock` present in all three projects
- Common Lisp: ASDF (system definitions, zero external dependencies per AF64 convention)
  - `project-noosphere-ghosts/lisp/af64.asd` - AF64 agent runtime
  - `innatescript/innatescript.asd` - InnateScipt interpreter
- Python: No requirements.txt (uses psycopg2, requests from system packages)

## Frameworks

**Core:**
- Axum 0.7 with macros feature - Rust web framework (`noosphere/src/main.rs`, `dpn-api/`)
- Tower 0.4 - Service middleware abstraction
- Tower-HTTP 0.5 - CORS (`cors`), request tracing (`trace`), static file serving (`fs`)
- SQLx 0.8 - Async PostgreSQL client with compile-time query verification
  - Features: `["postgres", "runtime-tokio-native-tls", "json", "chrono", "uuid"]`
  - Enables type-safe database queries checked at compile time

**Testing:**
- tokio-test 0.4 - Async runtime test utilities for Rust
- Hand-rolled test harness - InnateScipt uses custom macros (`deftest`, `check`, `combine-results`)
- No external test frameworks per AF64 zero-dependency convention

**Build/Dev:**
- Cargo - Rust compilation and dependency management
- ASDF 3.3+ - Common Lisp system definition (bundled with SBCL)
- rlwrap - REPL line editing wrapper (system package, not Lisp dependency)

## Key Dependencies

**Critical (Rust - noosphere/Cargo.toml):**
- `sqlx = "0.8"` with `["postgres", "runtime-tokio-native-tls", "json", "chrono", "uuid"]` - Database access layer
- `tokio = "1"` with `["full"]` - Async runtime powering all I/O
- `axum = "0.7"` with `["macros"]` - HTTP routing and request handling
- `serde = "1"` with `["derive"]` - Serialization/deserialization
- `serde_json = "1"` - JSON encoding/decoding for API
- `chrono = "0.4"` with `["serde"]` - Date/time handling
- `tower-http = "0.5"` with `["cors", "trace", "fs"]` - CORS middleware, request tracing, static file serving

**Infrastructure (Rust):**
- `reqwest = "0.12"` with `["json"]` - HTTP client for external APIs, RSS fetching, embedding services
- `feed-rs = "2.1"` - RSS/Atom feed parsing
- `scraper = "0.21"` - HTML parsing for feed auto-discovery
- `url = "2.5"` - URL manipulation and validation
- `ical = "0.11"` - ICS calendar file parsing
- `rusqlite = "0.31"` with `["bundled"]` - Embedded SQLite for local caching at `~/.dpn/cache.db`
- `tracing = "0.1"` - Structured logging
- `tracing-subscriber = "0.3"` with `["env-filter"]` - Logging configuration and filtering
- `anyhow = "1"` - Ergonomic error handling
- `thiserror = "1"` - Custom error type derivation
- `dotenvy = "0.15"` - `.env` file loading
- `once_cell = "1.21.3"` - Lazy static initialization
- `dirs = "5"` - Platform-specific directory paths
- `regex = "1"` - Pattern matching
- `rand = "0.8"` - Random number generation
- `uuid = "1"` with `["v4", "serde"]` - UUID generation for conversation threads
- `jsonwebtoken = "9"` - JWT authentication (future multi-user support)
- `async-trait = "0.1"` - Trait support for async methods

**Python Dependencies:**
- `psycopg2` - PostgreSQL adapter for `export_db_to_markdown.py`
- `requests` - HTTP client for Ollama API in `nightly-memory-synthesis.py`

**Common Lisp Dependencies:**
- None - All code hand-rolled per AF64 convention
- Uses SBCL built-ins: `uiop` (file I/O, process spawning), `asdf` (build system)
- System dependencies called via `uiop:run-program`: curl (HTTP), libpq (PostgreSQL)

## Configuration

**Environment:**
- `.env` file support via dotenvy (Rust)
- `DATABASE_URL` - PostgreSQL connection string (default: `postgresql://nebulab_user:nebulab_dev_password@localhost:5432/master_chronicle`)
- `RUST_LOG` - Logging level (default: `noosphere=debug,tower_http=debug`)
- `HOST` - Server bind address (default: `0.0.0.0`)
- `PORT` - Server port (default: `8888`)
- `OPENAI_API_KEY` - OpenAI embeddings API key (optional)

**Centralized Configuration:**
- `config.json` at repository root - API keys, service URLs, database credentials
  - AI service keys: Anthropic, OpenAI, Perplexity, Ollama
  - Integration keys: Discord, GitHub, Ghost CMS, n8n, Figma, Printful
  - Database configurations: master_chronicle, orbis_narratives
  - Service endpoints: Obsidian API, core server, Ollama
  - Warning: Contains live secrets, tracked in git

**Build:**
- `noosphere/Cargo.toml` - Unified server configuration
- `dpn-core/Cargo.toml` - Shared library (being consolidated into noosphere)
- `dpn-api/Cargo.toml` - Legacy API (being consolidated into noosphere)
- `project-noosphere-ghosts/lisp/af64.asd` - AF64 runtime system definition
- `innatescript/innatescript.asd` - InnateScipt interpreter system definition

## Platform Requirements

**Development:**
- Rust 1.70+ with Cargo
- SBCL 2.x (Steel Bank Common Lisp)
- Python 3.x
- PostgreSQL 16+ client tools (pg_restore version 16.13 confirmed via Homebrew)
- rlwrap (optional, for REPL line editing)
- libpq.so.5 (PostgreSQL C client library for AF64 Common Lisp FFI)
- curl (system binary for AF64 HTTP requests)

**Production:**
- PostgreSQL 16.9 server (confirmed from dump header and QUICKSTART.md)
- Database: `master_chronicle` (83 tables, 464MB dump file, 2,554 tasks, 9,846 conversations)
- Database users: `nebulab_user`, `chronicle`, `executive`
- Connection pool size: 10 (configured in `config.json`)
- Server port: 8888 (noosphere web server)
- Optional: Local Ollama at `http://localhost:11434` (llama3.1:8b model for AI embeddings and memory synthesis)

**Deployment Target:**
- DigitalOcean droplet at `144.126.251.126`
- Remote database: `db.eckenrodemuziekopname.com:5432`

## Database Systems

**Primary:**
- PostgreSQL 16.9 - `master_chronicle` database
  - Size: 464MB (dump file)
  - Tables: 83 tables
  - Records: 2,554 tasks, 9,846 conversations (per QUICKSTART.md)
  - Extensions: `vector` (for embeddings), `pg_trgm` (for text search)
  - Configuration: `pg_trgm.similarity_threshold = 0.1`, `idle_in_transaction_session_timeout = 10s`
  - Accessed via:
    - SQLx (Rust async client in noosphere)
    - psycopg2 (Python export script)
    - libpq (Common Lisp FFI in AF64)

**Secondary:**
- SQLite 3 - Local cache at `~/.dpn/cache.db`
  - Client: rusqlite 0.31 with bundled SQLite (no system dependency)
  - Purpose: Offline-first storage, sync queue for master_chronicle

**Schema Management:**
- `noosphere-schema/schema/` - SQL files defining table structure
- `master_chronicle.dump` - Full database backup (pg_dump format, 464MB, downloaded 2026-04-03)

## External HTTP Dependencies

**AI Providers (from config.json):**
- Anthropic Claude API - `https://api.anthropic.com/v1/messages`
  - API key: `sk-ant-api03-...` (config.json)
- OpenAI API - `https://api.openai.com/v1/chat/completions`
  - API key: `sk-proj-EmUb...` (config.json)
  - Embeddings: `text-embedding-3-small` model
- Perplexity API - Research/search augmentation
  - API key: `pplx-WY26...` (config.json)
- Ollama (local) - `http://localhost:11434`
  - Models: llama3.1:8b (default), nomic-embed-text (embeddings)
  - Used by: Burgs pipeline, memory synthesis, local AI fallback

**Infrastructure APIs:**
- Ghost CMS - `https://eckenrodemuziekopname.com` (blog publishing)
- Discord API - Bot token + 20+ webhooks (config.json)
- GitHub API - GraphQL + REST (token: `ghp_4z1jN...`)
- n8n workflows - Local (`http://localhost:5678`) + droplet (`https://n8n.eckenrodemuziekopname.com`)
- Obsidian Local REST API - `http://localhost:27123`

## Build Process

**Rust (Unified Server):**
```bash
cd noosphere
cargo build --release
cargo run  # Starts on http://localhost:8888
```

**Common Lisp (AF64 Runtime):**
```bash
cd project-noosphere-ghosts/lisp
sbcl --load af64.asd
sbcl --eval '(asdf:load-system "af64")' --eval '(af64:run-tick)'
```

**Common Lisp (InnateScipt Interpreter):**
```bash
cd innatescript
./run-tests.sh  # Runs test suite
./run-repl.sh   # Starts REPL
```

**Python (Database Export):**
```bash
python3 export_db_to_markdown.py  # Exports all tables to markdown/
```

## Deployment

**Current State:**
- Noosphere server: `http://localhost:8888` (confirmed running in QUICKSTART.md)
- API endpoints: `/api/health`, `/api/system/stats`, `/api/ghosts`, `/api/tasks`, `/api/conversations`, `/api/pipelines`
- Dashboard: `/static/noosphere-ops.html` (mock data, needs wiring to live API)
- Database: Connected to master_chronicle (83 tables, 2,554 tasks, 9,846 conversations)

**Deployment Process:**
- Build release binary: `cargo build --release`
- Target: DigitalOcean droplet (144.126.251.126)
- Database restore: `pg_restore -d master_chronicle master_chronicle.dump`
- Service management: Manual restart

---

*Stack analysis: 2026-04-04*
