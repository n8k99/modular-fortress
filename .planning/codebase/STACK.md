# Technology Stack

**Analysis Date:** 2026-04-04

## Languages

**Primary:**
- Common Lisp (SBCL) - AF64 tick engine runtime, InnateScipt interpreter
- Rust 2021 edition - Database infrastructure (dpn-core), REST API (dpn-api)

**Secondary:**
- SQL (PostgreSQL dialect) - Schema definitions in `noosphere-schema/schema/`
- JSON - Configuration (`config.json`), API payloads
- YAML - Ghost capability definitions (referenced in AF64 runtime)

## Runtime

**Environment:**
- SBCL (Steel Bank Common Lisp) - AF64 tick engine and InnateScipt
- Tokio async runtime (Rust) - dpn-core and dpn-api

**Package Manager:**
- ASDF (Common Lisp) - Build system for `af64.asd`, `innatescript.asd`
- Cargo (Rust) - Dependency management for dpn-core and dpn-api
- Lockfiles: `Cargo.lock` present in both Rust projects

## Frameworks

**Core:**
- Axum 0.7 - REST API framework in `dpn-api/src/main.rs`
- ASDF 3.x (bundled with SBCL) - Common Lisp build system

**Testing:**
- Hand-rolled test harness - InnateScipt uses custom `test-framework.lisp`
- Tokio-test 0.4 - Rust async testing in dpn-core

**Build/Dev:**
- Cargo build system - Rust compilation
- ASDF system definitions - Common Lisp compilation

## Key Dependencies

**Critical (Rust):**
- sqlx 0.8 - PostgreSQL async client with compile-time query verification (`dpn-core/Cargo.toml`)
- tokio 1.x - Async runtime for all Rust services (`features = ["full"]`)
- serde 1.x + serde_json 1.x - JSON serialization/deserialization
- axum 0.7 - Web framework for dpn-api REST endpoints
- tower-http 0.5 - CORS, tracing middleware for dpn-api

**Infrastructure (Rust):**
- reqwest 0.12 - HTTP client for external APIs (embedding services, RSS)
- feed-rs 2.1 + scraper 0.21 - RSS/Atom parsing with HTML scraping
- anyhow 1.x + thiserror 1.x - Error handling
- tracing 0.1 + tracing-subscriber 0.3 - Structured logging
- chrono 0.4 - Date/time handling with serde support
- uuid 1.x - UUID generation (`v4` feature for conversation threads)
- rusqlite 0.31 - Local SQLite cache (`~/.dpn/cache.db`, bundled)
- dotenvy 0.15 - Environment variable loading

**Critical (Common Lisp):**
- uiop (ASDF utility) - Process spawning, file I/O, cross-platform paths
- cl-json (hand-rolled in `af64.utils.json`) - JSON parsing/encoding
- curl (system dependency) - HTTP requests via `uiop:run-program` in `af64.utils.http`

**AF64 Runtime Modules:**
- `af64.utils.pg` - PostgreSQL connection pooling via libpq system library
- `af64.runtime.api` - REST API client calling dpn-api endpoints
- `af64.runtime.provider-adapters` - AI provider abstraction layer
- `af64.runtime.noosphere-resolver` - InnateScipt resolver for Noosphere data
- `af64.runtime.innate-builder` - Template validation and CRUD
- `af64.runtime.ghost-capabilities` - YAML capability loader

## Configuration

**Environment:**
- `config.json` - Centralized configuration (API keys, database credentials, service URLs)
- Environment variables:
  - `DPN_API_URL` - dpn-api base URL (e.g., `http://localhost:8080`)
  - `DPN_API_KEY` - Authentication for dpn-api
  - Database credentials sourced from `config.json` `database.master_chronicle`
- `master_chronicle.dump` - PostgreSQL database dump (443 MB, downloaded 2026-04-03)

**Build:**
- `dpn-core/Cargo.toml` - Rust library manifest
- `dpn-api/Cargo.toml` - Rust binary manifest
- `project-noosphere-ghosts/lisp/af64.asd` - AF64 system definition
- `innatescript/innatescript.asd` - InnateScipt system definition

## Platform Requirements

**Development:**
- SBCL (Common Lisp implementation) - Not in PATH during analysis, must be installed
- Rust toolchain 2021 edition - For dpn-core and dpn-api compilation
- PostgreSQL 14+ - master_chronicle database (local or via SSH tunnel)
- libpq.so.5 - PostgreSQL C client library (required by AF64 `af64.utils.pg`)
- curl - System binary for HTTP requests in Common Lisp code
- rlwrap (optional) - Line editing for Lisp REPL

**Production:**
- DigitalOcean droplet at `144.126.251.126` - Primary deployment target
- SSH tunnel to PostgreSQL - Connection via `db.eckenrodemuziekopname.com:5432`
- Local PostgreSQL database - `master_chronicle` on `localhost:5432`

## Database Systems

**Primary:**
- PostgreSQL 14+ - `master_chronicle` database
  - Tables: 50+ (agents, tasks, conversations, documents, decisions, etc.)
  - Connection pooling: 10 connections (configured in `config.json`)
  - Accessed via:
    - sqlx (Rust async client in dpn-core)
    - libpq (C library via FFI in AF64 `af64.utils.pg`)

**Secondary:**
- SQLite 3 - Local cache at `~/.dpn/cache.db` via rusqlite (dpn-core)
  - Offline-first storage for pending changes
  - Sync queue for master_chronicle synchronization

**Schema Management:**
- `noosphere-schema/schema/` - 15 SQL files defining table structure
  - 00_extensions.sql - PostgreSQL extensions
  - 01-15_*.sql - Modular table definitions ("The Chronicles", "The Forge", etc.)
  - 14_triggers.sql - Database triggers
- `master_chronicle.dump` - Full database backup (443 MB, pg_dump format)

## External HTTP Dependencies

**AI Providers (config.json api_keys):**
- Anthropic Claude API - `https://api.anthropic.com/v1/messages`
- OpenAI API - `https://api.openai.com/v1/chat/completions`
- Perplexity API - Enabled in `ai_service.perplexity`
- Ollama (local) - `http://localhost:11434` (llama3.1:8b model)

**Infrastructure APIs:**
- Ghost CMS - `https://eckenrodemuziekopname.com` (admin + content API)
- Discord - Bot token + webhooks for 20+ channels (tech_dev_office, personas, etc.)
- GitHub - GraphQL API + REST API (token: `ghp_4z1jN...`)
- n8n workflows - Local (`http://localhost:5678`) + droplet (`https://n8n.eckenrodemuziekopname.com`)

**Data Services:**
- Obsidian Local REST API - `http://localhost:27123` (vault at `/Volumes/Elements/Nebulab`)
- News API - API key `5cf576ab...`
- Google APIs - Sheets, Gemini image generation, Telegram integration
- Figma API - Personal access token for design file access
- Printful API - Product fulfillment (store_id: 13913738)

## Build Process

**Rust:**
```bash
cd dpn-core && cargo build --release
cd dpn-api && cargo build --release
```

**Common Lisp:**
```bash
sbcl --load project-noosphere-ghosts/lisp/af64.asd
sbcl --eval '(asdf:load-system "af64")' --eval '(af64:run-tick)'
```

**InnateScipt:**
```bash
sbcl --load innatescript/innatescript.asd
sbcl --eval '(asdf:load-system "innatescript")'
```

## Deployment

**Target:**
- DigitalOcean droplet (144.126.251.126)
- Synced via rsync at 00:07 (dpn-core freshly synced)

**Database:**
- PostgreSQL dump: `master_chronicle.dump` (464 MB)
- Restore: `pg_restore -d master_chronicle master_chronicle.dump`

---

*Stack analysis: 2026-04-04*
