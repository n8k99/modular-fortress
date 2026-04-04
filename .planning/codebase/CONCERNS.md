# Codebase Concerns

**Analysis Date:** 2026-04-03

## Security Concerns

**Secrets in config.json:**
- Issue: `config.json` contains plaintext API keys, tokens, and passwords for multiple services (OpenAI, Anthropic, GitHub, Discord, DigitalOcean, Ghost, Printful, database credentials, etc.)
- Files: `/Volumes/Elements/Modular Fortress/config.json`
- Impact: Committed secrets = immediate security breach. Anyone with repo access has full API access to production services, databases, and third-party integrations.
- Current mitigation: File is in .gitignore (confirmed by git status showing it untracked)
- Recommendations:
  - Move all secrets to environment variables or vault service
  - Use `.env` files that are explicitly ignored
  - Add pre-commit hook to scan for accidental secret commits
  - Rotate all exposed keys immediately if config.json was ever committed
  - Consider using service like doppler.com or AWS Secrets Manager for production

**Authentication bypass in dpn-api:**
- Issue: Login endpoint accepts any non-empty username/password combination
- Files: `/Volumes/Elements/Modular Fortress/dpn-api/src/handlers/auth.rs` (line 30-34)
- Impact: No real authentication — any attacker can generate valid JWT tokens
- Trigger: `POST /api/auth/login` with any credentials
- Workaround: Currently relies on API key authentication as primary security layer
- Fix approach: Implement proper credential validation against `agents` table or separate user store

**Hardcoded database credentials in deployment docs:**
- Issue: Database password `chronicle2026` appears in multiple README files
- Files:
  - `/Volumes/Elements/Modular Fortress/dpn-core/README.md` (line 24)
  - `/Volumes/Elements/Modular Fortress/dpn-api/DEPLOYMENT.md` (line 99)
  - `/Volumes/Elements/Modular Fortress/config.json` (lines 195-196, 215-216)
- Impact: If these docs are public or committed to public repo, database is compromised
- Current mitigation: Requires SSH tunnel to access database (port 5433 forwarded from droplet)
- Recommendations:
  - Replace examples with placeholder values
  - Use strong unique passwords per environment
  - Enable PostgreSQL SSL/TLS required mode
  - Restrict database access by IP whitelist

**Open webhook URLs in config.json:**
- Issue: Discord webhook URLs are bearer tokens that allow posting to channels
- Files: `/Volumes/Elements/Modular Fortress/config.json` (lines 245-275)
- Impact: Anyone with these URLs can impersonate ghost personas and post to Discord channels
- Recommendations:
  - Treat webhooks as secrets
  - Rotate webhooks if config.json was ever exposed
  - Consider using Discord bot with proper OAuth instead of webhooks

**Missing rate limiting:**
- Issue: dpn-api documents rate limiting as "planned but not yet implemented"
- Files: `/Volumes/Elements/Modular Fortress/dpn-api/README.md` (line 132-133)
- Impact: API vulnerable to abuse, DoS, and resource exhaustion
- Current mitigation: None at application layer
- Recommendations:
  - Implement rate limiting middleware (tower-governor crate)
  - Add per-IP and per-API-key limits
  - Use reverse proxy rate limiting (nginx limit_req) as interim solution

## Technical Debt

**af64.asd out of sync with launch.sh:**
- Issue: ASDF system definition (`af64.asd`) uses `:serial t` and different load order than production launcher
- Files:
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/af64.asd` (line 5, serial: t)
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/launch.sh` (lines 9-11, explicit load order)
- Impact: Development loads via ASDF may fail or behave differently than production. New developers can't use `(asdf:load-system :af64)` reliably.
- Cause: Production runtime evolved to inline Innate dependencies and explicit module ordering; ASDF definition not kept in sync
- Fix approach:
  - Update af64.asd to match launch.sh load order
  - Remove `:serial t`, use explicit `:depends-on`
  - Add ASDF load test to CI/pre-commit hook

**Massive runtime modules (77KB action-executor, 69KB action-planner):**
- Issue: Core runtime files exceed 60-70KB, making them difficult to reason about and modify
- Files:
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` (77,987 bytes)
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` (69,413 bytes)
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/db-client.lisp` (36,953 bytes)
- Impact: High cognitive load, merge conflicts, difficult to test in isolation
- Cause: Kitchen-sink modules accumulating multiple responsibilities over ~31 phases of development
- Fix approach:
  - Split action-executor into: execution engine, stage handlers, pipeline dispatch
  - Extract action-planner cognition request builders into separate module
  - Consider phase 32: "Module Decomposition" to systematically refactor

**Tool registry migration incomplete:**
- Issue: Phase 31 migrated tools from JSON to database, but legacy references may remain
- Files:
  - Migration: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/migrations/028_pipeline_definitions.sql`
  - New code: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/tool-definitions.lisp`
  - Old system: `tool-registry.json` (deleted according to git status)
- Impact: Risk of ghost ticks referencing non-existent tool-registry.json, breaking execution
- Test coverage: No evidence of migration rollback testing
- Fix approach:
  - Grep entire codebase for "tool-registry.json" references
  - Add regression test that ticks complete without tool-registry file present
  - Document migration in CHANGELOG or migration guide

**Innate load order hardcoded in launch.sh:**
- Issue: Innate interpreter loaded via manual file list instead of ASDF system
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/launch.sh` (line 9)
- Impact: Changes to Innate structure require updating launch.sh manually; no dependency tracking
- Cause: Innate is in separate repo (`/opt/innatescript`) but loaded directly into AF64 runtime
- Fix approach:
  - Load Innate via ASDF: `(asdf:load-system :innatescript)`
  - Add Innate as proper ASDF dependency in af64.asd
  - Coordinate with Innate repo to ensure ASDF system is stable

**464MB database dump in repo:**
- Issue: `master_chronicle.dump` is 464MB and tracked in git
- Files: `/Volumes/Elements/Modular Fortress/master_chronicle.dump`
- Impact: Bloats git history, slows clones, wastes storage
- Cause: Likely committed for backup/migration purposes
- Fix approach:
  - Add `*.dump` to .gitignore
  - Remove from git history: `git filter-repo --path master_chronicle.dump --invert-paths`
  - Store database backups in S3 or separate backup system

**No test coverage for dpn-api authentication:**
- Issue: Auth handlers have no automated tests
- Files: `/Volumes/Elements/Modular Fortress/dpn-api/src/handlers/auth.rs`
- Impact: Auth bypass bug (TODO on line 30) hasn't been caught by tests; refactoring is risky
- Test coverage: No `tests/` directory found in dpn-api
- Fix approach:
  - Add integration tests using `axum-test` crate
  - Test both JWT and API key flows
  - Add negative tests (invalid tokens, missing keys, expired JWTs)

**Placeholder persona webhooks in config.json:**
- Issue: Multiple Discord persona webhooks set to "PLACEHOLDER_*" values
- Files: `/Volumes/Elements/Modular Fortress/config.json` (lines 250-253)
- Impact: Ghosts assigned to FionaCarter, TaraBennett, MarcelloRuiz, EvelynWoods personas cannot post to Discord
- Cause: Partial persona roster deployment
- Fix approach:
  - Generate missing webhooks or remove personas from active roster
  - Add validation that checks for PLACEHOLDER values at startup
  - Fail fast if ghost attempts to use placeholder webhook

**Droplet database password is placeholder:**
- Issue: Production database credential in config.json is "PLACEHOLDER_PASSWORD_DROPLET"
- Files: `/Volumes/Elements/Modular Fortress/config.json` (line 225)
- Impact: Production deployments will fail to connect to droplet database
- Cause: Environment-specific config not fully populated
- Fix approach:
  - Never commit real production passwords
  - Use environment variable override: `DPN_DB_PASSWORD_DROPLET`
  - Document required env vars in deployment guide

## Performance Bottlenecks

**Cognition broker cache without TTL:**
- Issue: Cognition response cache has no time-based expiration
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/cognition-broker.lisp`
- Problem: Cache grows indefinitely; stale responses may be reused inappropriately
- Cause: Simple hash-table caching without LRU or TTL logic
- Improvement path:
  - Add TTL to cache entries (e.g., 1 hour for haiku tier, 24 hours for opus)
  - Implement LRU eviction when cache exceeds size threshold
  - Add cache metrics to tick reports

**Database connection pool per tick:**
- Issue: Each tick may create new database connections instead of reusing pool
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/util/pg.lisp`
- Problem: Connection overhead adds latency to every tick
- Current capacity: Unknown — no connection pool size visible in config
- Cause: libpq FFI wrapper may not implement pooling
- Improvement path:
  - Implement connection pool with configurable max size
  - Reuse connections across ticks
  - Add connection pool metrics (active, idle, wait time)

**Linear search in stub resolver:**
- Issue: Stub resolver uses `maphash` to search entities, O(n) for each query
- Files: `/Volumes/Elements/Modular Fortress/innatescript/src/eval/stub-resolver.lisp` (line 87)
- Problem: Scales poorly with entity count
- Current impact: Low (stub resolver only used in tests)
- Fix: Add proper indexing when implementing real resolver

**Action executor TODO comment about tool stage migration:**
- Issue: "TODO(Phase 31): Move tool-execution stage list to DB pipeline definitions" still in code
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` (line 221)
- Problem: Phase 31 is complete but TODO remains, suggesting incomplete migration
- Impact: May be using hardcoded pipeline stages instead of database definitions
- Fix: Verify migration completed, remove TODO or create phase 32 task

## Operational Risks

**launch.sh assumes specific directory structure:**
- Issue: Hardcoded paths to `/opt/project-noosphere-ghosts` and `/opt/innatescript`
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/launch.sh` (lines 2, 6, 9)
- Problem: Cannot run from different locations; breaks on dev machines
- Trigger: Running outside /opt/ directory or on macOS (shown as Darwin in env)
- Safe modification: Use `dirname $0` and relative paths, or environment variables
- Workaround: Symlink repos to /opt/ on dev machines

**No graceful shutdown in tick loop:**
- Issue: Infinite tick loop has no signal handling for clean shutdown
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/launch.sh` (lines 20-26)
- Problem: SIGTERM kills process immediately, may leave ticks incomplete or database transactions uncommitted
- Trigger: `systemctl stop`, Ctrl+C, server restart
- Fix approach:
  - Add `(sb-sys:enable-interrupt sb-posix:sigterm #'shutdown-handler)`
  - Set shutdown flag, wait for current tick to complete
  - Flush any pending logs/reports before exit

**SSH tunnel requirement not enforced:**
- Issue: dpn-core assumes SSH tunnel is running but doesn't verify
- Files: `/Volumes/Elements/Modular Fortress/dpn-core/README.md` (lines 47-52)
- Problem: Connection attempts hang or timeout if tunnel is down; error messages unclear
- Trigger: Reboot, SSH session timeout, network change
- Fix approach:
  - Check port 5433 is listening before attempting connection
  - Provide clear error: "SSH tunnel not detected. Run: ssh -L 5433:..."
  - Add health check endpoint that verifies database connectivity

**Config loading without validation:**
- Issue: config.json loaded without schema validation
- Files: `/Volumes/Elements/Modular Fortress/config.json` (770 lines)
- Problem: Typos, missing required fields, or type mismatches cause runtime failures
- Examples of fragility: Placeholder passwords, optional vs required keys unclear
- Fix approach:
  - Add JSON schema definition
  - Validate config at startup using jsonschema crate (Rust) or custom validator (Lisp)
  - Fail fast with actionable error messages

**Environment variable dependencies undocumented:**
- Issue: launch.sh sources `af64.env` but required variables not listed
- Files:
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/launch.sh` (line 2)
  - Expected: `/opt/project-noosphere-ghosts/config/af64.env` (not found in repo)
- Problem: New deployments fail with cryptic errors
- Trigger: Fresh deployment, Docker container, CI environment
- Fix approach:
  - Add `config/af64.env.example` with all required variables
  - Document in README.md or DEPLOYMENT.md
  - Add validation that checks required env vars at startup

## Known Bugs

**GitHub sync warning about missing token:**
- Issue: GitHub integration logs warning but doesn't fail when GITHUB_TOKEN unset
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/util/github.lisp` (line 436)
- Symptoms: Warning message during tick: "[github-sync] WARNING: GITHUB_TOKEN not set, skipping sync."
- Trigger: Any ghost action that uses GitHub integration
- Impact: Silently skips GitHub operations; may appear to succeed but do nothing
- Workaround: Set GITHUB_TOKEN environment variable
- Fix: Either require token at startup or remove GitHub integration from action options

**Ghost capabilities validation errors logged but not surfaced:**
- Issue: Invalid Innate expressions log warnings but evaluation continues
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp` (lines 94, 97)
- Symptoms: Logs show "WARNING: Invalid expression" or "WARNING: Validation error" but tick completes normally
- Trigger: Ghost generates malformed Innate script
- Impact: Silent failures — ghost thinks action succeeded but nothing happened
- Fix: Propagate validation errors to tick reporting, fail tick if critical expression invalid

**Action executor resistance reporting ambiguity:**
- Issue: Resistance severity calculation uses string comparison ("CRITICAL" > "WARNING" > "SUGGESTION")
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` (lines 763, 777-780, 788-800)
- Problem: String comparisons are fragile; adding new severity levels breaks logic
- Trigger: Any action that returns resistance/validation issues
- Fix: Use enum or keyword symbols instead of strings, compare numerically

## Fragile Areas

**Innate-Noosphere integration boundary:**
- Files:
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/noosphere-resolver.lisp` (22KB)
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/innate-builder.lisp` (6KB)
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp` (12KB)
- Why fragile: Three modules coordinate between Innate language and AF64 runtime; changes to Innate syntax break ghost capabilities
- Test coverage: Innate has 4,123 lines of tests but integration with Noosphere not tested end-to-end
- Safe modification:
  - Add integration tests that load both systems and execute real ghost capabilities
  - Version Innate syntax and maintain backwards compatibility
  - Use feature flags to gate new Innate features

**Provider chain fallback logic:**
- Files:
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/provider-adapters.lisp` (12KB)
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/claude-code-provider.lisp` (9KB)
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/cognition-broker.lisp` (19KB)
- Why fragile: 3-provider chain (claude-code -> anthropic -> stub) with budget limits, winter/thaw logic, and caching
- Test coverage: No mocking infrastructure for provider testing
- Safe modification:
  - Mock provider responses in tests
  - Add circuit breaker for failing providers
  - Test budget exhaustion scenarios

**Database migration history:**
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/migrations/` (28 migrations found)
- Why fragile: No migration rollback strategy, schema changes affect both Rust API and Lisp runtime
- Test coverage: No schema validation tests
- Safe modification:
  - Add schema version check at runtime
  - Test migrations against production dump
  - Maintain schema.sql that represents current state

## Scaling Limits

**Single-tick serial execution:**
- Current capacity: One agent action per tick, sequential processing
- Limit: Cannot scale beyond ~30 ghosts with current 120s tick interval
- Scaling path:
  - Parallelize perception and planning phases
  - Use agent priority queue instead of round-robin
  - Consider multi-process architecture with message passing

**Cognition broker queue unbounded:**
- Current capacity: In-memory hash table, no size limit
- Limit: Memory exhaustion if cognition requests queue up faster than LLM can process
- Scaling path:
  - Add max queue size with backpressure
  - Persist queue to database for durability
  - Implement job expiration/TTL

**Config.json loading into memory:**
- Current capacity: 770-line JSON file loaded at startup
- Limit: As more ghosts/personas added, config grows; becomes bottleneck to parse and load
- Scaling path:
  - Move persona definitions to database (already in `em_staff` table)
  - Load config sections on-demand
  - Use config service with caching

**464MB database dump workflow:**
- Current capacity: Full database dump for backup/migration
- Limit: Grows with data, takes minutes to load/restore
- Scaling path:
  - Use incremental backups (pg_dump with --data-only)
  - Implement WAL archiving for point-in-time recovery
  - Use logical replication for read replicas

## Missing Critical Features

**No tick failure recovery:**
- Problem: If a tick fails, next tick starts from scratch — no retry or partial recovery
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/launch.sh` (lines 23-24)
- Blocks: Reliable production operation — transient LLM failures cause dropped ticks
- Priority: High — affects all ghost execution

**No authentication session management:**
- Problem: JWT tokens expire after 24 hours with no refresh mechanism
- Files: `/Volumes/Elements/Modular Fortress/dpn-api/src/handlers/auth.rs` (line 45)
- Blocks: Long-running applications must re-authenticate daily
- Priority: Medium — workaround is re-login

**No metrics or observability:**
- Problem: No Prometheus metrics, no traces, no structured logging
- Files: All — no telemetry infrastructure found
- Blocks: Production monitoring, debugging performance issues, capacity planning
- Priority: High for production deployment

**No database migration tooling:**
- Problem: Migrations are SQL files with numeric prefixes, no migration runner
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/migrations/` (28 files)
- Blocks: Safe schema evolution, rollback capability
- Priority: High — currently applying migrations manually

## Test Coverage Gaps

**No integration tests for dpn-api:**
- What's not tested: Full request/response flows, authentication, error handling
- Files: `/Volumes/Elements/Modular Fortress/dpn-api/` (no tests/ directory found)
- Risk: Auth bypass bug on line 30 hasn't been caught
- Priority: High

**No end-to-end ghost tick tests:**
- What's not tested: Complete tick lifecycle from perception to reporting
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/` (only unit tests inferred)
- Risk: Integration failures between tick-engine, cognition-broker, action-executor not caught
- Priority: High

**No provider failure scenario tests:**
- What's not tested: LLM provider timeout, budget exhaustion, malformed responses
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/cognition-broker.lisp`
- Risk: Production failures due to provider issues
- Priority: Medium

**No Innate-Noosphere integration tests:**
- What's not tested: Ghost capabilities calling Innate resolver with noosphere substrate
- Files:
  - `/Volumes/Elements/Modular Fortress/innatescript/` (4,123 test lines, all unit tests)
  - `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp`
- Risk: Changes to either system break ghost scripting
- Priority: Medium

**No database migration rollback tests:**
- What's not tested: Applying migration, rolling back, data integrity preserved
- Files: `/Volumes/Elements/Modular Fortress/project-noosphere-ghosts/migrations/`
- Risk: Cannot safely revert schema changes
- Priority: Medium

**No SSH tunnel failure handling tests:**
- What's not tested: dpn-core behavior when port 5433 is not forwarded
- Files: `/Volumes/Elements/Modular Fortress/dpn-core/src/db/connection.rs`
- Risk: Unclear error messages, hangs during connection attempts
- Priority: Low

---

*Concerns audit: 2026-04-03*
