# Codebase Concerns

**Analysis Date:** 2026-04-04 02:03
**Previous Analysis:** 2026-04-04 00:40 (updated with additional findings)

## Critical Security Issues

**Unencrypted database dump in repository root:**
- Issue: `master_chronicle.dump` (443 MB) exists untracked in repo root
- Files: `/Volumes/Elements/Modular Fortress/master_chronicle.dump`
- Impact: Contains complete ghost memories, conversations, personal data in plaintext
- Risk: Critical - single file leak exposes entire noosphere substrate
- Trigger: Synced from droplet server at 2026-04-04 00:07
- Current state: File is untracked but NOT gitignored
- Fix approach: Add `*.dump` to `.gitignore`, delete local dump after verification, implement encrypted backup protocol

**Markdown export directory contains sensitive data:**
- Issue: `markdown/` directory contains database exports as readable markdown files
- Files: `markdown/agent_areas/`, `markdown/agent_daily_memory/` (9000+ files)
- Content: Agent daily logs, area content, potentially private conversations
- Impact: All ghost memory and operational data in plaintext, searchable format
- Risk: High - easier to leak than binary dump, indexed by search tools
- Current state: Directory is untracked, 9284+ markdown files
- Fix approach: Add `markdown/` to `.gitignore`, delete after archival, never export to working directory

**API keys and credentials committed to repository:**
- Issue: `config.json` contains production API keys in plaintext
- Files: `/Volumes/Elements/Modular Fortress/config.json` (770 lines)
- Impact: Full access to OpenAI ($), Anthropic ($), Discord bot, GitHub, DigitalOcean, Printful, n8n, Ghost CMS, Perplexity, news APIs
- Exposed keys include:
  - OpenAI: `sk-proj-EmUb...NKEA` (project key with payment access)
  - Anthropic: `sk-ant-api03-l7cV...NEQ` (full API access)
  - GitHub: `ghp_4z1j...vwZ` (personal access token)
  - Discord bot: `MTMwMjc5OTQzNDc0NDIwNTM5NQ.G8-5nq...YshI` (multiple bots)
  - DigitalOcean: `dop_v1_6e48...993b8` (infrastructure control)
  - Obsidian API key: `80a3e285ff...d98c2a`
  - Perplexity: `pplx-WY26...hjw`
  - Printful: `JYJaFC08...5txt` (e-commerce fulfillment)
  - Ghost CMS admin: `697d938a5e...c8e2994`
  - n8n: JWT tokens for local and droplet instances
  - News API: `5cf576ab0d...003d15`
  - Figma personal access token: `figd_M6IaN0...qse56`
  - Open Router: `sk-or-v1-5b1722...534a20` (in plain text at bottom of file)
- Current state: File is untracked but NOT gitignored
- Risk: Critical - any repository exposure = complete system compromise and financial loss
- Fix approach: Rotate ALL keys immediately, move to environment variables, add `config.json` to `.gitignore`, implement secrets management (Vault/1Password CLI)

**Hardcoded database credentials in Common Lisp FFI:**
- Issue: Default connection string hardcoded in `pg.lisp`
- Files: `project-noosphere-ghosts/lisp/util/pg.lisp` line 69-73
- Credentials: `user=chronicle password=chronicle2026`
- Impact: Production database credentials in source code
- Risk: High - anyone with codebase access can connect to production database
- Fix approach: Read from environment variables, no defaults in source

**Private keys in gotcha-secrets directory:**
- Issue: `gotcha-secrets/` contains trading platform private keys
- Files: `gotcha-secrets/kalshi/kalshi_private.key`
- Impact: Trading account access with real money
- Current state: Directory appears tracked (has `.git` subdirectory)
- Risk: High - financial account compromise
- Fix approach: Remove from main repo, move to separate encrypted secrets vault

**Email and social platform credentials in config.json:**
- Issue: Plain text passwords for external services
- Credentials exposed:
  - Email: `task.data.harmony@gmail.com` / `QxW9GX)\`6B6h`
  - LiveJournal: `T4SKS` / `%*@EzDjjYm2_b4y`
  - Telegram API: hash `e559971dd246cbfec641f11983ec8ff7`
- Impact: Account takeover, unauthorized posting, reputation damage
- Risk: Medium - external platform credentials
- Fix approach: Use OAuth where possible, rotate passwords, store in secrets manager

**Discord webhook URLs exposed:**
- Issue: 20+ Discord webhook URLs in `config.json` lines 245-275
- Webhooks: Combat, Musicology, Personas (7 different), Common, CIF-13ξ07, Guinea Pigs, Tech Dev Office (9 staff), Office of CEO
- Impact: Unauthorized message posting to production Discord channels
- Risk: Medium - social engineering, misinformation campaigns
- Fix approach: Regenerate all webhooks, move to environment variables

**No .gitignore for sensitive patterns:**
- Issue: `.gitignore` file doesn't exist or doesn't cover critical patterns
- Files checked: `config.json`, `master_chronicle.dump`, `.env`, `gotcha-secrets/`, `markdown/`
- Impact: Easy to accidentally commit secrets
- Risk: Critical - systematic exposure vulnerability
- Fix approach: Create comprehensive `.gitignore` with patterns:
  ```
  config.json
  *.dump
  .env*
  *secret*
  *credential*
  *.key
  *.pem
  markdown/
  gotcha-secrets/
  master_chronicle.dump
  ```

---

## Architectural Fragmentation

**Three-pillar architecture not yet consolidated:**
- Issue: dpn-api, dpn-core, project-noosphere-ghosts exist as separate codebases
- Files: Three root directories with independent build systems
- Problem: Modular Fortress spec describes unified "install script pulls all three" but integration unclear
- Impact: Complex deployment, version skew risk between components
- Dependencies:
  - dpn-api depends on dpn-core (Cargo.toml)
  - project-noosphere-ghosts calls dpn-api via HTTP
  - No clear versioning contract between them
- Risk: High - integration failures after independent updates
- Fix approach: Either (1) monorepo with workspace Cargo.toml, or (2) strict version locking + integration tests

**"noosphere" Rust project purpose unclear:**
- Issue: Fourth Rust project exists at repo root, not mentioned in architecture docs
- Files: `noosphere/Cargo.toml`, `noosphere/target/` (350MB build artifacts)
- Problem: Relationship to dpn-api/dpn-core unknown, appears to be separate binary
- Contains: Full target/ directory with release + debug builds, ring crypto, libsqlite
- Impact: Potential duplicate functionality, confusing entry point for developers
- Risk: Medium - architectural confusion, wasted compute
- Current state: Built (has target/ artifacts), not documented
- Fix approach: Document purpose, determine if obsolete, remove if redundant

**dpn-core 20+ modules copied but not integrated:**
- Issue: Fresh sync from dpn-core repo but dpn-api integration status unknown
- Files: `dpn-core/src/` has 24 module directories (16,177 lines Rust)
- Modules: cache, context, conversations, db, dedup, embeddings, events, graph, ics, memory, notify, patterns, pipeline, publish, reading, replay, stagehand, sync, tasks, timeline, wikilinks
- Problem: dpn-api last touched Mar 28, dpn-core synced Apr 4 - potential version mismatch
- Impact: API may serve stale data structures, runtime type errors
- Risk: High - integration failure after sync
- Fix approach: Version lock dpn-core in dpn-api Cargo.toml, run integration test suite

**UI mockup has no implementation:**
- Issue: Two UI design specs exist but no actual frontend code found
- Files: `noosphere-schema/prompts/operations-mockup.md` (6KB), `operations-mockup-v2.md` (8KB)
- Spec: Foundry VTT-inspired interface, scene-based navigation, Forge Workshop, Pipeline Workbench, Ghost Bridges
- Problem: No HTML/CSS/JS, no React/Vue/Svelte components, no build system for frontend
- Impact: UI design work complete but not implemented = wasted design effort, no user interface
- Risk: High - major feature gap, unusable without interface
- Fix approach: Choose frontend stack (Tauri + Svelte?), implement mockup, wire to dpn-api

**Nine Tables schema exists but migration to live database unclear:**
- Issue: Comprehensive schema in `noosphere-schema/schema/*.sql` (1008 lines) but adoption status unknown
- Files: 15 SQL files defining the Nine Tables architecture
- Tables: the_chronicles, the_realms, the_press, the_markets, the_music, the_forge, the_commons, the_work, the_post, plus infrastructure
- Problem: Current production still uses 83-table schema from master_chronicle dump
- Impact: Documented architecture doesn't match running system
- Risk: High - schema migration incomplete, documentation misleading
- Fix approach: Verify if schema applied, run migration, or update docs to reflect reality

**doltgres-data directory purpose unclear:**
- Issue: New directory not mentioned in documentation
- Files: `doltgres-data/` with test_connection.py
- Problem: Doltgres is a PostgreSQL-compatible version-controlled database, but role in architecture unknown
- Impact: Potential experimental feature or abandoned POC cluttering codebase
- Risk: Low - likely exploratory work
- Fix approach: Document purpose or remove if obsolete

---

## Technical Debt

**af64.asd out of sync with actual runtime loader:**
- Issue: ASDF system definition missing critical modules that `launch.sh` loads manually
- Files:
  - `project-noosphere-ghosts/lisp/af64.asd` (ASDF definition)
  - `project-noosphere-ghosts/launch.sh` (actual loader)
- Discrepancy: `launch.sh` loads InnateScipt dependencies and 10+ modules not in af64.asd
- Missing from ASDF: `util/yaml`, `runtime/openclaw-gateway`, `runtime/tool-definitions`, `runtime/noosphere-resolver`, `runtime/innate-builder`, `runtime/ghost-capabilities`, `runtime/pipeline-definitions`
- Impact: `(asdf:load-system :af64)` fails or loads incomplete system; documentation misleads developers
- Current workaround: Everyone must use `launch.sh` instead of ASDF
- Risk: Medium - developer confusion, broken tooling integration
- Fix approach: Update `af64.asd` to match `launch.sh` module list exactly, or vice versa

**Phase 31 TODO comment still present:**
- Issue: "TODO(Phase 31): Move tool-execution stage list to DB pipeline definitions" remains in code
- Files: `project-noosphere-ghosts/lisp/runtime/action-executor.lisp` line 221
- Problem: Phase 31 completed but hardcoded stage list still exists
- Impact: Tool execution validation relies on hardcoded list instead of database-driven config
- Stages hardcoded: thesis, research, analysis, compliance, documentation, approval, collection, curation, composition, editing, polish, publish
- Risk: Low - functional but inflexible, requires code changes to add stages
- Fix approach: Complete migration to database-driven pipeline definitions, remove hardcoded list

**Semantic search placeholder in dpn-core:**
- Issue: Vector-based semantic search not implemented, using ILIKE text matching
- Files: `dpn-core/src/stagehand/recall.rs` lines 3, 82
- Problem: `semantic_search()` function uses simple text matching with TODO for embeddings
- Impact: Poor recall quality for stagehand show notes, no true semantic similarity
- Risk: Low - feature incomplete but functional fallback exists
- Fix approach: Implement embeddings pipeline (pgvector extension), replace ILIKE with vector similarity

**Incomplete office phase mappings:**
- Issue: Operations and Content office phase mappings stubbed out
- Files: `dpn-core/src/notify/phases.rs` lines 80, 85
- Problem: Only TechDev office has phase-to-staff mappings, COO and Content offices have empty HashMaps
- Impact: Phase notifications won't route to Kathryn Lyonne (COO) or Sylvia Inkweaver (Content) teams
- Risk: Low - features gated on future capability activation
- Fix approach: Define phase mappings when COO/Content ghost capabilities are activated

**47,760 documents with 65.9% duplication:**
- Issue: Massive document duplication in legacy database
- Files: Analysis documented in `dpn-core/analysis/dedup-report.md`
- Scale: 31,452 duplicate documents, 82.7% in Archive/
- Top duplicate: "NPC" title appears 118 times (actually distinct NPCs with bad titles)
- Impact: Wasted storage, slow queries, confusing search results
- Risk: Medium - system bloat and data quality degradation
- Fix approach: Execute deduplication strategy from report (purge Archive, keep canonical in Areas/Master Chronicle)

**137 unwrap/expect calls in dpn-core:**
- Issue: Potential panic points throughout Rust codebase
- Files: 23 files including `cache/hybrid.rs` (3), `notify/mod.rs` (6), `tasks/parser.rs` (17), `db/tests.rs` (37)
- Impact: Runtime crashes on unexpected None/Err values instead of graceful error handling
- Risk: Medium - production stability issues, difficult debugging
- Pattern: Especially prevalent in cache layer and parser modules
- Fix approach: Audit high-count files, replace with proper Result propagation or descriptive expects

**Large complex modules:**
- Issue: Multiple files exceed 800+ lines indicating high complexity
- Files:
  - `project-noosphere-ghosts/lisp/runtime/action-executor.lisp` (1329 lines)
  - `project-noosphere-ghosts/lisp/runtime/action-planner.lisp` (1087 lines)
  - `dpn-core/src/cache/hybrid.rs` (855 lines, 30KB)
  - `dpn-core/src/publish/db.rs` (591 lines)
- Impact: Difficult to understand, test, maintain; cognitive load for developers
- Risk: Medium - bugs hide in complexity, refactoring is expensive
- Fix approach: Extract responsibilities into smaller modules, improve test coverage for complex logic

**InnateScipt dependencies not modular:**
- Issue: `launch.sh` manually loads 7 InnateScipt files without using ASDF system
- Files: `launch.sh` line 9 loads individual `.lisp` files from `/opt/innatescript/src/`
- Problem: Tight coupling between AF64 and InnateScipt, no version control
- Impact: AF64 cannot load without InnateScipt, breakage risk on updates
- Risk: Medium - system coupling creates deployment complexity
- Fix approach: Load InnateScipt as proper ASDF dependency, version lock in system definition

**Authentication stub in dpn-api:**
- Issue: Login endpoint accepts any credentials
- Files: `dpn-api/src/handlers/auth.rs` line 30
- Code: `// TODO: Validate credentials against database`
- Problem: Auth handler generates valid JWT for ANY username/password
- Impact: Complete authentication bypass in development, easy to forget before production
- Risk: Critical - security vulnerability if deployed without fixing
- Fix approach: Implement actual credential validation against agent table, add integration test

---

## Performance Bottlenecks

**Database connection pool hardcoded to 2 connections:**
- Issue: PG connection pool size fixed at 2 in Common Lisp FFI
- Files: `project-noosphere-ghosts/lisp/util/pg.lisp` line 78
- Problem: `(make-array 2 :initial-element nil)` limits parallelism
- Impact: Tick engine blocks waiting for connection under concurrent load
- Scaling limit: Max 2 simultaneous database operations across all ghosts
- Risk: High - throughput bottleneck for multi-ghost ticks
- Current workaround: Sequential processing, low ghost population (7 agents currently)
- Fix approach: Make pool size configurable, increase to 10-20 connections, implement dynamic sizing

**No connection pool configuration visible in dpn-core:**
- Issue: Database access pattern unclear, potential connection leaks
- Files: `dpn-core/src/db/connection.rs`
- Problem: No visible connection pooling layer (sqlx pool not explicitly configured)
- Impact: Connection exhaustion under load, slow startup times
- Risk: Medium - production stability under traffic spikes
- Fix approach: Verify sqlx pool configuration, tune min/max connections, add monitoring

**Hybrid cache using synchronous file I/O:**
- Issue: `SyncStateFile` uses blocking filesystem operations
- Files: `dpn-core/src/cache/hybrid.rs` lines 30-45
- Problem: `std::fs::read_to_string` and `std::fs::write` block async runtime
- Impact: Event loop stalls during cache sync operations
- Risk: Medium - latency spikes on disk I/O
- Fix approach: Use `tokio::fs` for async file operations, consider memory-backed sync state

**No query result caching:**
- Issue: No evidence of query result caching in dpn-core
- Files: Database query modules in `dpn-core/src/db/`
- Problem: Repeated identical queries hit PostgreSQL every time
- Impact: Unnecessary database load, higher latency
- Risk: Low - database handles load currently but wasteful
- Fix approach: Implement Redis/in-memory cache for frequently accessed data (ghost profiles, pipelines)

**Tick engine sleeps between ticks:**
- Issue: Fixed `sleep` interval regardless of work completion time
- Files: `project-noosphere-ghosts/launch.sh` line 26
- Problem: If tick takes 8s and interval is 10s, system waits full 10s before next tick
- Impact: Wasted idle time, lower effective throughput
- Risk: Low - acceptable for current scale
- Fix approach: Sleep for `(interval - elapsed-time)` to maintain cadence

**350MB build artifacts in noosphere/target/:**
- Issue: Compiled Rust artifacts checked into working directory
- Files: `noosphere/target/debug/` and `noosphere/target/release/`
- Impact: Wasted disk space, slow git operations, unclear what this binary does
- Risk: Low - space wasteful but not breaking
- Fix approach: Add `target/` to `.gitignore`, document binary purpose or remove project

---

## Operational Risks

**Runtime depends on external services without fallback:**
- Issue: AF64 tick engine crashes if PostgreSQL unavailable
- Files: `project-noosphere-ghosts/lisp/runtime/tick-engine.lisp`
- Problem: `(af64.runtime.db:init-db-pool)` failure = runtime abort
- Impact: Complete system downtime on database hiccup
- Risk: High - single point of failure
- Current mitigation: `handler-case` around tick execution but not initialization
- Fix approach: Implement graceful degradation mode with local cache, retry logic for initialization

**Health check endpoint exists but deployment unclear:**
- Issue: `/health` endpoint in code but operational status unknown
- Files: `dpn-api/src/handlers/health.rs` exists, DEPLOYMENT.md references it
- Problem: Deployment guide shows systemd/PM2 setup but health check integration unclear
- Impact: Load balancers/orchestrators may not use it, cannot determine service health
- Risk: Medium - operational visibility gap
- Fix approach: Verify health endpoint accessible, integrate with monitoring (uptime checks)

**Database migrations not automated:**
- Issue: SQL migration files exist but no migration runner
- Files: `project-noosphere-ghosts/migrations/`, `noosphere-schema/schema/`
- Problem: Manual execution required, no version tracking
- Impact: Human error during deployments, schema drift between environments
- Risk: Medium - deployment reliability issue
- Fix approach: Integrate Flyway/Liquibase or use SQLx migrations in Rust

**No log aggregation:**
- Issue: Logs only written to stdout/stderr or local files
- Files: `config.json` specifies log files: `.tasks_core.log`, `.tasks_discord.log`
- Problem: Distributed logs across multiple services, no centralized search
- Impact: Difficult to debug production issues, no alerting
- Risk: Medium - operational blindness
- Fix approach: Ship logs to Loki/ELK stack, implement structured logging (JSON lines)

**Tick engine has no graceful shutdown:**
- Issue: Infinite loop with no signal handling
- Files: `project-noosphere-ghosts/launch.sh` lines 20-26
- Problem: `(loop for tick from 1 ...)` never checks for termination signal
- Impact: Ungraceful kills leave database connections open, incomplete transactions
- Risk: Medium - resource leaks on restarts
- Fix approach: Add signal handlers for SIGTERM/SIGINT, drain in-flight work before exit

**No monitoring/metrics exposed:**
- Issue: No Prometheus metrics, no telemetry export
- Files: Codebase has no `/metrics` endpoint
- Problem: Cannot track ghost activity, tick duration, error rates, queue depth
- Impact: Reactive incident response, no capacity planning data
- Risk: High - operational blindness
- Fix approach: Add Prometheus client, export key metrics (ticks/sec, cognition latency, error rates)

**SSH tunnel dependency for production database:**
- Issue: Database access requires SSH tunnel to droplet
- Files: Implied by `config.json` database section with localhost references, DEPLOYMENT.md line 263
- Connection: `ssh -L 5433:127.0.0.1:5432 root@144.126.251.126`
- Problem: Production operations dependent on SSH connectivity
- Impact: Network issues = production outage, complex deployment setup
- Risk: High - fragile operational model
- Fix approach: Use connection pooling proxy (PgBouncer), VPN mesh (Tailscale), or managed database service

**No deployment rollback procedure:**
- Issue: DEPLOYMENT.md shows binary backup but no tested rollback process
- Files: `dpn-api/DEPLOYMENT.md` line 302
- Problem: Backup mentioned but no verification it works, no database rollback strategy
- Impact: Cannot safely roll back broken deployments
- Risk: Medium - deployment safety issue
- Fix approach: Document and test rollback procedure, include database migration rollback

---

## Data Integrity Risks

**No foreign key constraints visible:**
- Issue: Schema files in `noosphere-schema/schema/` don't show comprehensive FK coverage
- Files: `noosphere-schema/schema/02_the_chronicles.sql` through `15_the_ledger.sql`
- Problem: Orphaned records possible (tasks referencing deleted ghosts, conversations pointing to missing agents)
- Impact: Data corruption, cascade delete failures, invalid references
- Risk: Medium - data quality degrades over time
- Fix approach: Audit schema for missing FK constraints, add with ON DELETE CASCADE/SET NULL

**No backup verification process:**
- Issue: `master_chronicle.dump` exists but no verification it's valid
- Files: `master_chronicle.dump` (443 MB)
- Problem: Backup could be corrupted, incomplete, or outdated
- Impact: Cannot restore if production database fails
- Risk: Critical - disaster recovery failure
- Fix approach: Automated pg_restore dry-run, backup integrity checks, retention policy

**Duplicate prevention unclear:**
- Issue: No deduplication logic visible in conversation/task creation
- Files: `dpn-core/src/db/` modules
- Problem: Same conversation/task could be created multiple times
- Impact: Duplicate ghost actions, wasted cognition budget, confused state
- Risk: Medium - system bloat and confusion
- Fix approach: Add unique constraints (conversation + timestamp), upsert patterns

**Document versioning schema created but unused:**
- Issue: Migration 16 added `document_versions` table and columns but no code uses it
- Files: `dpn-core/analysis/schema-migration.md`, migration applied Feb 23
- Tables: `document_versions` table exists, `documents` has `is_canonical`, `canonical_id`, `version_count` columns
- Problem: Schema ready for deduplication but workflow not implemented
- Impact: Duplication problem not solved despite schema support
- Risk: Low - groundwork complete but inactive
- Fix approach: Implement deduplication workflow that populates version relationships

---

## Missing Critical Features

**No authentication enforcement in dpn-api:**
- Issue: Auth middleware exists but coverage unclear
- Files: `dpn-api/src/auth.rs` has JWT + API key validation, but handler usage inconsistent
- Problem: Development TODO allows any credentials through login endpoint
- Impact: Potential security bypass if auth middleware not applied to all routes
- Risk: Critical - complete security bypass possible
- Fix approach: Audit all route handlers, ensure auth middleware applied, remove development stub

**No rate limiting:**
- Issue: No rate limiter visible in dpn-api
- Files: API handler modules in `dpn-api/src/handlers/`, README notes it's planned but not implemented
- Problem: Single client can exhaust API resources
- Impact: DOS attacks, runaway costs from LLM API calls
- Risk: High - financial and operational exposure
- Fix approach: Add tower middleware rate limiter, per-client quotas

**No rollback capability for ghost actions:**
- Issue: Ghost actions are fire-and-forget
- Files: `project-noosphere-ghosts/lisp/runtime/action-executor.lisp`
- Problem: Bad cognition results permanently alter state
- Impact: No recovery from LLM hallucinations or bugs
- Risk: Medium - data quality degradation
- Fix approach: Transaction log of state changes, admin rollback interface

**No schema versioning:**
- Issue: Database schema has no version tracking
- Files: `noosphere-schema/schema/*.sql`
- Problem: Cannot detect schema drift or enforce migration order
- Impact: Production schema diverges from dev, migration conflicts
- Risk: Medium - deployment safety issue
- Fix approach: Add schema_version table, number migrations sequentially

**No install script:**
- Issue: Modular Fortress spec describes "one install script" but none exists
- Files: No `install.sh` or setup automation found
- Problem: Three-pillar architecture requires manual setup of each component
- Impact: Cannot achieve "fresh droplet test" from project spec
- Risk: High - core requirement unmet, project not deployable as designed
- Fix approach: Create install.sh that sets up Postgres, pulls three repos, configures environment

**No onboarding interface:**
- Issue: Modular Fortress spec describes onboarding flow but no implementation
- Spec: Page 1 (identity/git), Page 2 (AI brains), T.A.S.K.S. assistant guides rest
- Problem: No web interface for configuration
- Impact: Manual .env editing required, error-prone setup
- Risk: Medium - poor user experience, deployment barriers
- Fix approach: Implement web-based onboarding after UI framework chosen

---

## Test Coverage Gaps

**No integration tests for AF64 runtime:**
- Issue: `project-noosphere-ghosts/` has no visible test directory
- Files: No `tests/` or `spec/` directory in Common Lisp codebase
- Problem: Tick engine, cognition broker, database access untested
- Impact: Regressions go undetected until production
- Risk: High - critical path untested
- Fix approach: Add RT/FiveAM test suite, mock database/API for unit tests, E2E tick tests

**Rust test quality concerns:**
- Issue: Tests exist but heavy use of unwrap/expect in test code
- Files: `dpn-core/src/db/tests.rs` has 37 unwrap/expect calls
- Problem: Test suite may panic instead of reporting failures cleanly
- Impact: Hard-to-debug test failures, false confidence in coverage
- Risk: Medium - test reliability issue
- Fix approach: Replace unwrap with assert_eq!/assert! + error messages, use Result in tests

**No load testing:**
- Issue: System behavior under high ghost population unknown
- Problem: Tick engine designed for 64 ghosts but scaling characteristics unknown
- Impact: Production outages when system grows
- Risk: High - capacity planning blindness
- Fix approach: Load test with 32/64/128 ghosts, measure tick duration, identify bottlenecks

**No chaos testing:**
- Issue: Failure modes not exercised
- Problem: Unknown behavior when PostgreSQL hangs, API rate limits hit, disk fills
- Impact: Cascading failures in production
- Risk: Medium - resilience untested
- Fix approach: Inject failures (network partition, database timeout), verify graceful degradation

**No UI tests:**
- Issue: UI mockup exists but no way to verify implementation when built
- Problem: When frontend gets built, no automated testing strategy defined
- Impact: Manual testing burden, regression risk
- Risk: Low - UI not built yet
- Fix approach: Plan for Playwright/Cypress E2E tests when UI implemented

---

## Dependency Risks

**Zero-dependency Common Lisp approach:**
- Issue: AF64 reimplements JSON parser, HTTP client, PostgreSQL driver
- Files: `project-noosphere-ghosts/lisp/util/json.lisp`, `util/http.lisp`, `util/pg.lisp`
- Trade-off: No Quicklisp dependencies = maintainability burden
- Impact: Security patches in libpq won't auto-apply, HTTP/JSON bugs must be fixed in-house
- Risk: Medium - security and correctness burden
- Current state: Intentional design decision per README
- Fix approach: Weigh benefits vs. costs, consider minimal Quicklisp subset (Dexador, Shasht, Postmodern)

**libpq.so.5 version lock:**
- Issue: Hard dependency on specific libpq version
- Files: `project-noosphere-ghosts/lisp/util/pg.lisp` line 7
- Problem: Breaks on systems with libpq.so.6 or different PostgreSQL versions
- Impact: Deployment failures on non-Ubuntu systems, version skew
- Risk: Medium - portability issue
- Fix approach: Dynamic library loading with version fallback, or use Postmodern for portability

**Rust dependency audit needed:**
- Issue: No `cargo audit` output visible
- Files: `dpn-core/Cargo.toml`, `dpn-api/Cargo.toml`, `noosphere/Cargo.toml`
- Problem: Unknown if dependencies have security vulnerabilities
- Risk: Medium - supply chain security
- Fix approach: Run `cargo audit`, update vulnerable dependencies, add to CI pipeline

**dpn-core and dpn-api sync unclear:**
- Issue: Relationship between freshly synced dpn-core and dpn-api unknown
- Files: `dpn-core/` synced 2026-04-04 00:07, `dpn-api/` last touched Mar 28
- Problem: API may be using old dpn-core version, schema mismatch risk
- Impact: Runtime errors from type mismatches, broken endpoints
- Risk: High - integration failure after sync
- Fix approach: Version lock dpn-core in dpn-api Cargo.toml, enforce matching versions

---

## Documentation Gaps

**Modular Fortress spec vs. reality mismatch:**
- Issue: `Modular Fortress.md` describes v2.0 target state, not current implementation
- Files: `Modular Fortress.md` line 114 "Acceptance Test: The Fresh Droplet"
- Spec items not implemented:
  - Install script
  - Onboarding interface
  - T.A.S.K.S. permanent COO ghost
  - Control surface UI
  - Three-pillar unified deployment
  - Nine Tables schema (or unclear if applied)
- Problem: README reads like current state but describes aspirational future
- Impact: New developers confused about what exists vs. what's planned
- Risk: Medium - onboarding friction, misplaced effort
- Fix approach: Split into two docs: CURRENT.md (as-is) and ROADMAP.md (future)

**No ARCHITECTURE.md at project root:**
- Issue: Architecture documentation only exists in `.planning/codebase/`
- Files: `.planning/codebase/ARCHITECTURE.md` (226 lines)
- Problem: Hidden from casual repo browsing, not linked from README
- Impact: Developers don't understand system design
- Risk: Low - docs exist but poorly discoverable
- Fix approach: Symlink or copy to root, link from README

**QUICKSTART.md missing:**
- Issue: Project spec mentions "QUICKSTART.md" with agent schema issues but file not found
- Problem: No quick start guide visible for developers
- Impact: High barrier to contribution, slow onboarding
- Risk: Medium - contributor friction
- Fix approach: Write QUICKSTART.md covering: clone → configure → build → test → run

**dpn-core README incomplete:**
- Issue: README lists modules but doesn't explain integration with dpn-api
- Files: `dpn-core/README.md`
- Problem: Developers don't know how dpn-core is consumed
- Impact: Changes to dpn-core may break dpn-api
- Risk: Low - internal integration
- Fix approach: Add "Integration" section explaining dpn-api dependency

**project-noosphere-ghosts README focuses on AF64 but doesn't explain dpn integration:**
- Issue: README describes standalone operation but not dpn-api integration
- Files: `project-noosphere-ghosts/README.md`
- Problem: Unclear how AF64 tick engine calls dpn-api endpoints
- Impact: Cannot understand full system flow
- Risk: Low - integration works but undocumented
- Fix approach: Add architecture diagram showing HTTP calls to dpn-api

---

## Immediate Actions Required

### Security (URGENT - within 24 hours):

1. **Create comprehensive .gitignore:**
   ```
   config.json
   master_chronicle.dump
   *.dump
   .env*
   markdown/
   gotcha-secrets/
   *secret*
   *credential*
   *.key
   *.pem
   */target/
   ```

2. **Delete sensitive files from working directory:**
   - `master_chronicle.dump` (after backup verification)
   - `markdown/` directory (9000+ files)
   - Verify `gotcha-secrets/` separation

3. **Rotate all API keys:** Every key in `config.json` must be regenerated:
   - OpenAI, Anthropic (financial exposure)
   - GitHub PAT (code access)
   - Discord bots (15+ tokens)
   - DigitalOcean (infrastructure)
   - All other services (20+ keys total)

4. **Move config.json out of repo:**
   - Create `config.example.json` with placeholder values
   - Document environment variable alternatives
   - Update documentation to reference environment config

5. **Audit git history:**
   - Check if secrets were ever committed: `git log --all --full-history --source -- config.json`
   - If found, consider force-push cleaned history or repository rotation

6. **Harden database credentials:**
   - Remove hardcoded `chronicle:chronicle2026` from pg.lisp
   - Use environment variables exclusively
   - Rotate production password

### Operational (within 1 week):

1. **Verify health check availability:** Test `curl http://localhost:8080/health` on all deployments

2. **Set up log aggregation:** Ship logs to centralized system (Loki/CloudWatch/ELK)

3. **Add monitoring:** Expose Prometheus metrics for tick engine and API

4. **Document and test rollback:** Create and verify deployment rollback procedure

5. **Fix authentication stub:** Remove TODO in auth.rs, implement real credential validation

### Architectural (within 2 weeks):

1. **Clarify three-pillar integration:**
   - Document version dependencies
   - Add integration tests
   - Choose monorepo vs. multi-repo strategy

2. **Determine noosphere/ project fate:**
   - Document purpose or remove if obsolete
   - Delete target/ build artifacts

3. **UI implementation decision:**
   - Choose frontend framework
   - Implement operations-mockup-v2.md
   - Wire to dpn-api

4. **Nine Tables schema status:**
   - Verify if applied to production
   - Document migration plan if pending

### Technical Debt (within 1 month):

1. **Sync af64.asd with launch.sh:** Make ASDF system definition accurate

2. **Complete Phase 31 migration:** Remove TODO, finish pipeline definitions

3. **Increase PG pool size:** Make configurable, default to 10 connections

4. **Add graceful shutdown:** Handle SIGTERM in tick engine

5. **Execute deduplication:** Apply strategy from dedup-report.md to eliminate 31K duplicate docs

### Documentation (within 1 month):

1. **Split project documentation:**
   - CURRENT.md (as-is state)
   - ROADMAP.md (Modular Fortress v2.0 goals)

2. **Create QUICKSTART.md:** Developer onboarding guide

3. **Write install script:** Implement "fresh droplet test" automation

---

*Concerns audit: 2026-04-04 02:03*
*Focus: Security vulnerabilities, architectural fragmentation, technical debt, operational risks*
*Codebase synced from droplet: 2026-04-04 00:07*
*This update incorporates findings from deeper exploration including UI mockups, markdown exports, noosphere project, and integration gaps*
