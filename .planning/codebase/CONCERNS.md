# Codebase Concerns

**Analysis Date:** 2026-04-04

## Critical Security Issues

**Unencrypted database dump in repository root:**
- Issue: `master_chronicle.dump` (443 MB) exists untracked in repo root
- Files: `/Volumes/Elements/Modular Fortress/master_chronicle.dump`
- Impact: Contains complete ghost memories, conversations, personal data in plaintext
- Risk: High - single file leak exposes entire noosphere substrate
- Trigger: Synced from droplet server at 2026-04-04 00:07
- Current state: File is untracked but NOT gitignored
- Fix approach: Add `*.dump` to `.gitignore`, delete local dump after verification, implement encrypted backup protocol

**API keys and credentials committed to repository:**
- Issue: `config.json` contains production API keys in plaintext
- Files: `/Volumes/Elements/Modular Fortress/config.json`
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

**Unencrypted .env file in dpn-api:**
- Issue: `.env` file exists with credentials
- Files: `dpn-api/.env` (215 bytes, modified Mar 16)
- Impact: Database connection strings, API keys for REST API
- Current state: File exists but gitignore status unclear
- Risk: Medium - contains operational credentials
- Fix approach: Verify gitignore coverage, rotate credentials if exposed

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
- Impact: Account takeover, unauthorized posting, reputation damage
- Risk: Medium - external platform credentials
- Fix approach: Use OAuth where possible, rotate passwords, store in secrets manager

**Discord webhook URLs exposed:**
- Issue: 20+ Discord webhook URLs in `config.json` lines 245-275
- Impact: Unauthorized message posting to production Discord channels
- Risk: Medium - social engineering, misinformation campaigns
- Fix approach: Regenerate all webhooks, move to environment variables

**No .gitignore for sensitive patterns:**
- Issue: `.gitignore` file doesn't exist or doesn't cover critical patterns
- Files checked: `config.json`, `master_chronicle.dump`, `.env`, `gotcha-secrets/`
- Impact: Easy to accidentally commit secrets
- Risk: High - systematic exposure vulnerability
- Fix approach: Create comprehensive `.gitignore` with patterns: `config.json`, `*.dump`, `.env*`, `*secret*`, `*credential*`, `*.key`, `*.pem`

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

---

## Performance Bottlenecks

**Database connection pool hardcoded to 2 connections:**
- Issue: PG connection pool size fixed at 2 in Common Lisp FFI
- Files: `project-noosphere-ghosts/lisp/util/pg.lisp` line 78
- Problem: `(make-array 2 :initial-element nil)` limits parallelism
- Impact: Tick engine blocks waiting for connection under concurrent load
- Scaling limit: Max 2 simultaneous database operations across all ghosts
- Risk: High - throughput bottleneck for multi-ghost ticks
- Current workaround: Sequential processing, low ghost population
- Fix approach: Make pool size configurable, increase to 10-20 connections, implement dynamic sizing

**No connection pool in dpn-api/dpn-core:**
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

**No health check endpoint in dpn-api:**
- Issue: No `/health` or `/ready` endpoint visible
- Files: `dpn-api/src/handlers/health.rs` exists but deployment unclear
- Problem: Load balancers/orchestrators cannot determine service health
- Impact: Traffic routed to unhealthy instances, cascading failures
- Risk: Medium - operational visibility gap
- Fix approach: Expose Kubernetes-ready health checks (liveness, readiness)

**Database migrations not automated:**
- Issue: SQL migration files exist but no migration runner
- Files: `project-noosphere-ghosts/migrations/`, `noosphere-schema/schema/`
- Problem: Manual execution required, no version tracking
- Impact: Human error during deployments, schema drift between environments
- Risk: Medium - deployment reliability issue
- Fix approach: Integrate Flyway/Liquibase or use SQLx migrations in Rust

**No log aggregation:**
- Issue: Logs only written to stdout/stderr
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
- Files: Implied by `config.json` database section with localhost references
- Problem: Production operations dependent on SSH connectivity
- Impact: Network issues = production outage, complex deployment setup
- Risk: High - fragile operational model
- Fix approach: Use connection pooling proxy (PgBouncer), VPN mesh (Tailscale), or managed database service

---

## Data Integrity Risks

**No foreign key constraints visible:**
- Issue: Schema files in `noosphere-schema/schema/` don't show comprehensive FK coverage
- Files: `noosphere-schema/schema/02_the_chronicles.sql` through `15_the_ledger.sql`
- Problem: Orphaned records possible (tasks referencing deleted ghosts, etc.)
- Impact: Data corruption, cascade delete failures, invalid references
- Risk: Medium - data quality degrades over time
- Fix approach: Audit schema for missing FK constraints, add with ON DELETE CASCADE/SET NULL

**No backup verification process:**
- Issue: `master_chronicle.dump` exists but no verification it's valid
- Files: `master_chronicle.dump`
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

---

## Missing Critical Features

**No authentication in dpn-api:**
- Issue: REST API endpoints appear unauthenticated
- Files: `dpn-api/src/auth.rs` exists but usage unclear
- Problem: Anyone with network access can read/write ghost data
- Impact: Unauthorized data access, ghost manipulation, DOS attacks
- Risk: Critical - complete security bypass
- Fix approach: Implement JWT authentication, API key validation, rate limiting

**No rate limiting:**
- Issue: No rate limiter visible in dpn-api
- Files: API handler modules in `dpn-api/src/handlers/`
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

---

## Test Coverage Gaps

**No integration tests for AF64 runtime:**
- Issue: `project-noosphere-ghosts/` has no visible test directory
- Files: No `tests/` or `spec/` directory in Common Lisp codebase
- Problem: Tick engine, cognition broker, database access untested
- Impact: Regressions go undetected until production
- Risk: High - critical path untested
- Fix approach: Add RT/FiveAM test suite, mock database/API for unit tests, E2E tick tests

**No Rust tests in dpn-core:**
- Issue: Tests exist but coverage unknown
- Files: `dpn-core/src/db/tests.rs` has 37 unwrap/expect calls (test quality concern)
- Problem: Test suite may not cover critical paths
- Impact: Regressions in cache, graph, publish modules
- Risk: Medium - unknown coverage is risky coverage
- Fix approach: Run `cargo tarpaulin`, target 80%+ coverage, add property tests

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
- Files: `dpn-core/Cargo.toml`, `dpn-api/Cargo.toml`
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

## Immediate Actions Required

### Security (URGENT - within 24 hours):

1. **Delete database dump:** Remove `master_chronicle.dump`, add `*.dump` to `.gitignore`
2. **Rotate all API keys:** Every key in `config.json` must be regenerated
3. **Move config.json out of repo:** Add to `.gitignore`, use environment variables
4. **Audit git history:** Check if secrets were ever committed, force-push cleaned history if needed
5. **Move gotcha-secrets:** Migrate to separate encrypted vault outside main repo

### Operational (within 1 week):

1. **Add comprehensive .gitignore:** Cover `config.json`, `*.dump`, `.env*`, `*secret*`, `*.key`
2. **Implement health checks:** Add `/health` and `/ready` endpoints to dpn-api
3. **Set up log aggregation:** Ship logs to centralized system
4. **Add monitoring:** Expose Prometheus metrics for tick engine and API

### Technical Debt (within 1 month):

1. **Sync af64.asd with launch.sh:** Make ASDF system definition accurate
2. **Complete Phase 31 migration:** Remove TODO, finish pipeline definitions
3. **Increase PG pool size:** Make configurable, default to 10 connections
4. **Add graceful shutdown:** Handle SIGTERM in tick engine

---

*Concerns audit: 2026-04-04*
*Focus: Security vulnerabilities, technical debt, operational risks*
*Codebase synced from droplet: 2026-04-04 00:07*
