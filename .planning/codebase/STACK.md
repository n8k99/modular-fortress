# Technology Stack

**Analysis Date:** 2026-04-03

## Languages

**Primary:**
- Common Lisp (SBCL) - AF64 tick engine (`project-noosphere-ghosts/lisp/`) and Innate DSL interpreter (`innatescript/src/`)
- Rust 2021 edition - Database infrastructure (`dpn-core/`) and REST API (`dpn-api/`)

**Secondary:**
- Python 3.x - Utility scripts (`project-noosphere-ghosts/tools/nightly-memory-synthesis.py`, `write_vault_memory.py`)
- Bash - Deployment and runtime scripts (`project-noosphere-ghosts/launch.sh`, `innatescript/run-repl.sh`)

## Runtime

**Environment:**
- SBCL (Steel Bank Common Lisp) - Native compilation for Common Lisp code
- Tokio async runtime - Rust asynchronous execution

**Package Manager:**
- ASDF 3.3+ (bundled with SBCL) - Common Lisp system definition and build
- Cargo - Rust package management and build system
- Lockfile: `Cargo.lock` present in both `dpn-api/` and `dpn-core/`

**Zero External Dependencies (Common Lisp):**
- No Quicklisp usage in AF64 or Innate projects
- Hand-rolled JSON parser (`project-noosphere-ghosts/lisp/util/json.lisp`)
- Direct libpq FFI via SB-ALIEN (`project-noosphere-ghosts/lisp/util/pg.lisp`)
- HTTP via curl subprocess (`project-noosphere-ghosts/lisp/util/http.lisp`)

## Frameworks

**Core:**
- Axum 0.7 - Rust async web framework with macros for `dpn-api`
- Tower 0.4 / Tower-HTTP 0.5 - Middleware (CORS, tracing) for HTTP stack

**Testing:**
- Hand-rolled test harness - Common Lisp (`innatescript/tests/test-framework.lisp`)
- tokio-test 0.4 - Rust async test utilities (dev-dependency in `dpn-core`)

**Build/Dev:**
- ASDF - Common Lisp system definition (`.asd` files)
- Cargo - Rust build system with workspace structure

## Key Dependencies

**Critical:**
- sqlx 0.8 - Rust PostgreSQL driver with async support, compile-time query verification, JSON/UUID/Chrono features (`dpn-core`, `dpn-api`)
- libpq.so.5 - PostgreSQL C client library loaded via FFI in Common Lisp (`project-noosphere-ghosts/lisp/util/pg.lisp`)
- serde 1.x / serde_json 1.x - Rust serialization for API payloads and config

**Infrastructure:**
- reqwest 0.12 - Rust HTTP client with JSON support (`dpn-core/src/lib.rs`)
- axum 0.7 - Web server framework (`dpn-api/src/main.rs`)
- chrono 0.4 - Date/time handling with serde support (Rust projects)
- jsonwebtoken 9 - JWT authentication (`dpn-api/Cargo.toml`)

**Parsing/Processing:**
- feed-rs 2.1 - RSS/Atom feed parsing (`dpn-core/Cargo.toml`)
- scraper 0.21 - HTML parsing for feed auto-discovery (`dpn-core`, `dpn-api`)
- ical 0.11 - ICS calendar parsing (`dpn-core/Cargo.toml`)

**Error Handling:**
- anyhow 1.x - Flexible error handling (both Rust projects)
- thiserror 1.x - Custom error types (both Rust projects)

**Logging:**
- tracing 0.1 / tracing-subscriber 0.3 - Structured logging (Rust projects)

**Other:**
- dotenvy 0.15 - Environment variable loading from `.env` files
- uuid 1.x - UUID generation and handling (v4, serde features)
- regex 1.x - Pattern matching (both Rust projects)
- rand 0.8 - Random ID generation (`dpn-core`)
- dirs 5 - Cross-platform path resolution (`dpn-core`)
- rusqlite 0.31 - SQLite support with bundled library (`dpn-core`)

## Configuration

**Environment:**
- `.env` files for both `dpn-api` (DATABASE_URL, JWT_SECRET, API_KEYS, RUST_LOG, PORT) and Common Lisp runtime (`config/af64.env`)
- `config.json` at repo root contains extensive service configuration including API keys, database credentials, and service endpoints
- Provider configuration via JSON: `project-noosphere-ghosts/config/provider-config.json` defines LLM provider chain (claude-code, anthropic, stub fallback)
- DoltgreSQL config: `noosphere-schema/doltgres-config.yaml` (port 5435, root user)

**Build:**
- `Cargo.toml` files in `dpn-api/` and `dpn-core/`
- ASDF system definitions: `innatescript/innatescript.asd`, `project-noosphere-ghosts/lisp/af64.asd`

## Platform Requirements

**Development:**
- SBCL 2.x (current version) for Common Lisp runtime
- Rust 1.70+ toolchain
- libpq.so.5 (PostgreSQL client library) available on system
- curl binary for HTTP requests from Common Lisp
- rlwrap (optional) for REPL line editing in Innate

**Production:**
- Linux/macOS server (SBCL optimized for these platforms)
- PostgreSQL 16 database (`master_chronicle`, port 5432 or SSH tunnel to 5433)
- systemd or PM2 for process management (`dpn-api.service`, `ecosystem.config.js`)
- Optional: nginx reverse proxy for SSL/TLS termination
- Runtime directory: `/tmp/noosphere_ghosts` or `/opt/project-noosphere-ghosts`

**Database:**
- PostgreSQL 16 (`master_chronicle` database)
- Connection pool size: 10 (configurable via `config.json`)
- SSH tunnel support for remote database access (port 5433 forwarding)
- DoltgreSQL (versioned PostgreSQL) on port 5435 for experimental schema work

---

*Stack analysis: 2026-04-03*
