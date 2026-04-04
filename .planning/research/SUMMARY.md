# Project Research Summary

**Project:** Modular Fortress v2.0 (Rust → Go Migration + 83→9 Table Consolidation)
**Domain:** Sovereign digital workspace with autonomous AI agents (PIM + agent runtime)
**Researched:** 2026-04-04
**Confidence:** HIGH

## Executive Summary

Modular Fortress is a dual simultaneous migration: rewriting a working Rust API in Go while collapsing 83 PostgreSQL tables into 9 polymorphic tables. This is inherently high-risk—83% of database migrations fail, and complete rewrites are called "the single worst strategic mistake any software company can make." However, the migration is justified by concrete gains: Go's 8-second compilation (vs 3-minute Rust), 2-3x development velocity, backwards compatibility that keeps code stable once written, and simpler team onboarding. The existing Common Lisp ghost runtime stays unchanged—it's proven and working.

The recommended approach is **sequential migration with strangler fig pattern**: build Go API against the existing 83-table schema first, validate parity route-by-route with nginx routing, then migrate the schema as a separate phase after the Go rewrite is complete. This isolates blame (language vs schema), enables per-route rollback, and prevents compounding failures. The architecture uses PostgreSQL as shared state substrate between Go (HTTP/API layer) and Lisp (ghost tick engine), communicating via LISTEN/NOTIFY rather than FFI. Chi router + pgx/v5 + slog provides a minimalist Go stack similar to Rust's Axum philosophy.

Key risks center on the Netscape Trap (throwing away 15 years of accumulated bugfixes), second-system syndrome (feature creep during rewrite), and JSONB performance cliffs (2000x query slowdown without hybrid schema design). Prevention requires: (1) behavioral audit documenting all Rust quirks before rewriting, (2) strict parity gates rejecting improvements during migration, (3) hybrid schema using indexed columns for hot queries with JSONB only for truly variable data, (4) shadow-write period proving data integrity before cutover. The generalization requirement—producing a binary that works on fresh droplets—demands early dependency auditing and fresh-install testing in Phase 0.

## Key Findings

### Recommended Stack

Go 1.24+ with Chi router, pgx/v5, and golang-migrate provides the best match for migrating from Rust's Axum/SQLx/Tokio. This stack maintains similar minimalism (explicit over magic, standard library first) while delivering 22x faster compilation (8s vs 3min), 2-3x development velocity, and backwards compatibility that prevents the constant dependency churn plaguing the Rust v1.5 codebase. The Lisp runtime stays unchanged—SBCL + Postmodern + InnateScript interpreter are proven components with zero rewrite needed.

**Core technologies:**
- **Go 1.24+**: Core language — compilation speed (8s vs 3min), simpler syntax, faster onboarding (days vs weeks), backwards compatibility means code "stays in place once written"
- **Chi Router 5.x**: HTTP routing — lightweight, zero dependencies beyond stdlib, builds on net/http patterns (similar to Axum's minimalism)
- **pgx/v5**: PostgreSQL driver — 50-70% faster than lib/pq, native binary protocol, JSONB simplified to []byte, LISTEN/NOTIFY support for IPC
- **golang-migrate 4.x**: Schema migrations — language-agnostic, excellent CI/CD integration, standard choice unless Atlas's advanced planning needed
- **slog (stdlib 1.21+)**: Structured logging — zero dependencies, part of stdlib, 650ns/op, default choice for new Go projects
- **SBCL + Postmodern**: Lisp runtime (no changes) — ghost tick engine, cognition broker, InnateScript interpreter all stay as-is

**Critical decision:** Use pgx native interface (not database/sql wrapper) for JSONB operations. The v5 simplification treating JSONB as []byte eliminates the tri-state Status system, making PostgreSQL-specific features accessible without ORM overhead.

**What NOT to use:** lib/pq (unmaintained, 50% slower), Gorilla Mux (archived 2022), GORM (ORM overhead unnecessary for JSONB workloads), Beego (heavy framework overkill), Fiber + FastHTTP (breaks net/http ecosystem).

### Expected Features

Research reveals a KDE-PIM style sovereign workspace where table stakes (expected by all users) must be distinguished from differentiators (unique to this product). The Nine Tables polymorphic schema is itself a technical differentiator, not just an implementation detail—it enables cross-domain wikilinks and unified querying impossible with 83 separate tables.

**Must have (table stakes):**
- Note creation/editing with wikilinks [[note]] — standard in Obsidian/Logseq/Notion, users expect bidirectional linking
- Backlinks panel — makes wikilinks valuable, shows "what links here"
- Daily notes — journaling workflow popularized by Logseq, now expected
- Full-text search — PostgreSQL native capability, prevents 30% time waste (G2 reports)
- Task/todo management — TODO/DOING/DONE states, due dates, priorities
- Calendar view — day/week/month views minimum, core PIM feature
- Data export — sovereignty requirement, Markdown/iCal/JSON formats
- Tags/labels — universal organization feature across all tools
- RSS feed reading — The Commons domain requirement, table stakes for self-hosted

**Should have (differentiators):**
- Autonomous ghost agents — unique Lisp ghosts living in workspace, not bolted-on AI
- Nine Tables polymorphism — technical elegance enabling cross-domain queries
- Wikilink graph across domains — unprecedented cross-linking (notes → tasks → calendar → ghosts)
- No OAuth/passkey-only auth — true sovereignty, zero external auth dependencies
- Ghost-to-human conversations — ghosts can message you (The Post domain), not just respond
- Task dependency chains — blocking/waiting relationships with auto-status updates
- InnateScript templating — ghost-executable notes bridging Lisp and workspace

**Defer (v2+):**
- Scene-based UI (Foundry VTT style) — high visual appeal but requires full feature set first
- Hot-pluggable droplet services — extension architecture for n8n workflows, after core proven
- Temporal compression notes — import historical data, compress for ghost context (Phase 3+)
- Multi-user collaboration — adds 10x complexity, not target use case (single-user with ghosts)
- Mobile apps — desktop-first, document mobile workflow using native iOS camera sync
- Cloud sync service — contradicts sovereignty, provide export/import instead

### Architecture Approach

The database-centric pattern uses PostgreSQL as single source of truth with independent language runtimes (Go for I/O-bound API work, Lisp for cognition-heavy agent work). This avoids FFI complexity between Go and Lisp, leverages ACID guarantees neither language needs to implement, and enables independent scaling (API horizontal, ghosts per-user). LISTEN/NOTIFY provides event-driven coordination without polling—when a ghost completes a tick, PostgreSQL notifies the Go API which pushes WebSocket updates to the UI. This creates clean separation: Go owns HTTP/CRUD/auth, Lisp owns ghost ticks/cognition/actions.

**Major components:**
1. **Go API Server** — HTTP interface, auth (JWT/passkey), CRUD operations, WebSocket/SSE real-time updates. Uses pgx connection pool with context timeouts, Chi router for net/http compatibility.
2. **Lisp Ghost Runtime** — Autonomous agent tick cycle (perception → drive evaluation → action planning → execution → reporting), cognition brokering with LLM APIs, InnateScript interpreter. Proven in v1.5, needs only generalization pass.
3. **PostgreSQL** — All persistent state, Nine Tables schema with polymorphic `kind` + JSONB `meta`, LISTEN/NOTIFY for IPC, triggers/constraints for consistency. Dual role as database and message bus.
4. **TypeScript UI** — Scene-based interface (Foundry VTT patterns), real-time visualization via WebSocket, CRUD panels over spatial canvas.

**Key pattern: Strangler Fig migration** — Nginx routes traffic between Rust (legacy) and Go (new) during incremental rewrite. Migrate route-by-route over 3-6 months: health check → read-only endpoints → write endpoints → complex logic → real-time layer → Rust retirement. Each route independently testable and rollbackable. No big-bang rewrite, no downtime.

**Key pattern: Hexagonal Architecture** — Go API uses ports (repository interfaces) and adapters (pgx implementations) to isolate domain logic from external dependencies. Enables testing without database, swapping implementations, clear separation of concerns.

**Key pattern: LISTEN/NOTIFY Event Bus** — PostgreSQL channels (`ghost_tick_complete`, `task_status_changed`, `memory_created`) propagate events from Lisp to Go to WebSocket clients. Sub-second latency, no polling, no Redis dependency. Go uses `github.com/jackc/pgxlisten`, Lisp uses Postmodern's `cl-postgres-listen`.

### Critical Pitfalls

The research identified 12 critical pitfalls specific to dual simultaneous migration, each with concrete prevention strategies and phase mapping for when to address them.

1. **The Netscape Trap (Complete Rewrite Paralysis)** — Throwing away 15 years of accumulated bugfixes and edge cases. Prevention: Feature freeze Rust API, extract implicit business logic, write characterization tests capturing Rust behavior before any Go code. Phase 0 requirement: Full Rust behavioral audit.

2. **Second-System Syndrome (Feature Creep During Rewrite)** — Go becomes "the version we always wanted" with improvements packed in, doubling scope. Prevention: Strict parity requirement (Go must match Rust endpoint-for-endpoint), "Improvements Later" backlog for post-parity work, automated comparison testing. Every phase: explicit parity checks.

3. **Running Two Systems Forever (Parallel Operation Hell)** — Planned "temporary" parallel Rust/Go operation becomes permanent, doubling maintenance cost. Prevention: Hard cutover date set at project start (non-negotiable), automated cutover criteria (error rates, latency, parity tests), phased rollout (1% → 10% → 50% → 100% traffic). Phase -1: Define cutover date before development starts.

4. **JSONB Performance Cliff (Query Optimizer Blindness)** — Queries 2000x slower because PostgreSQL can't maintain statistics on JSONB values, one team reported 0.3s → 584s degradation. Prevention: Hybrid schema (indexed columns for hot queries, JSONB for truly variable data), denormalize critical fields (agent.name, task.status), query budget testing against production-size data, keep JSONB under 2KB to avoid TOAST overhead. Phase 1: Schema design with benchmarks.

5. **Losing Foreign Key Integrity (Polymorphic Association Trap)** — 83-table schema had database-enforced foreign keys, Nine Tables polymorphic schema can't enforce them (database can't validate polymorphic references). Prevention: Application-level validation before every insert, batch integrity audits via cron, separate junction tables for critical relationships. Phase 1: Define which relationships need real foreign keys vs application validation.

6. **Data Loss During 83→9 Consolidation** — 83% of database migrations fail. Prevention: Pre-migration snapshot with checksums, row count reconciliation for EVERY table mapping, checksum validation of critical columns, dry-run migrations on production snapshot, tested rollback procedure. Phase 1: Define reconciliation tests BEFORE writing migration scripts.

7. **Migration Without Rollback (One-Way Door)** — Discovering critical issues in production with no way to roll back. Prevention: Bidirectional migration scripts (both 83→9 and 9→83 tested), shadow-write period (write to both schemas, verify parity), gradual cutover (read-only new schema → writes → retire old), documented rollback procedure timed under 1 hour. Phase 2: Write and test reverse migration BEFORE running forward migration.

8. **Blame Ambiguity (Which Migration Broke It?)** — Production breaks, debugging becomes exponential because every bug could be language, schema, or interaction. Prevention: Sequential migration (Go rewrite with old schema FIRST, schema migration separately), compatibility layer (Go reads 83-table schema initially), isolated testing (Go vs old schema, new schema vs Rust). Phase -1: Decide migration sequence before development begins.

9. **Generalization Without Understanding Current State** — Removing Nathan-specific assumptions without documenting them, fresh droplet install fails due to 20 implicit dependencies (SSH tunnels, pm2 setup, hardcoded paths, `chronicle:chronicle2026` credentials). Prevention: Dependency audit mapping every environment variable/path/service, fresh droplet test early (document every failure), configuration extraction to `.env` file. Phase 0: Dependency audit before generalization code written.

10. **Technology Stack Shift Without Experience** — Simultaneous Rust→Go transition plus new HTTP frameworks/ORMs/logging without production Go experience. Goroutine leaks, channel deadlocks, unchecked errors (Go's explicit returns vs Rust's Result). Prevention: Minimize technology changes, Go concurrency training before production code, code review gates enforcing sync.WaitGroup/errgroup, Rust→Go translation guide documenting pattern mappings. Phase 0: Go best practices training.

## Implications for Roadmap

Based on research, the migration must be sequential (Go-first, schema-second) with strangler fig pattern. The roadmap should have explicit separation between language migration phases and schema migration phases, with hard gates preventing simultaneous deployment. Research suggests 5 phases for Go migration + 2 phases for schema migration + 2 phases for generalization.

### Phase 0: Foundation & Knowledge Extraction
**Rationale:** Prevent Netscape Trap and institutional knowledge loss by documenting everything BEFORE rewriting
**Delivers:**
- Rust behavioral audit with characterization tests
- Nathan-specific dependency map (SSH, pm2, paths, credentials)
- Go concurrency training completed
- Rust→Go pattern translation guide
- Project structure (Hexagonal Architecture)
- pgx connection pool + health check endpoint
**Addresses:** Table stakes foundation (auth, database access)
**Avoids:** Pitfall #1 (Netscape Trap), #5 (Throwing Away Knowledge), #9 (Generalization Without Understanding), #4 (Technology Stack Shift)
**Research needed:** None—this IS the research phase for implementation
**Duration:** 2 weeks

### Phase 1: Read-Only API Migration (Strangler Fig Start)
**Rationale:** Simple GET endpoints build Go patterns without data mutation risk
**Delivers:**
- Nginx routing layer (Go :8080, Rust :8888)
- GET /api/v2/tasks (list, single)
- GET /api/v2/conversations
- GET /api/v2/ghosts
- Automated endpoint comparison tests (Go === Rust)
**Uses:** Chi router, pgx/v5, slog for structured logging
**Implements:** HTTP adapter in Hexagonal Architecture
**Avoids:** Pitfall #2 (Second-System Syndrome) via strict parity gates
**Research needed:** None—CRUD patterns well-documented
**Duration:** 3-4 weeks

### Phase 2: Write API Migration (CRUD Completion)
**Rationale:** POST/PUT/DELETE establish transaction patterns, validation, error handling
**Delivers:**
- POST /api/v2/tasks (create)
- PUT /api/v2/tasks/:id (update)
- DELETE /api/v2/tasks/:id (soft delete)
- Repeat for conversations, ghosts
- Transaction handling patterns
**Implements:** Repository pattern with domain models
**Avoids:** Pitfall #2 via automated parity tests, no feature additions
**Research needed:** None—standard CRUD patterns
**Duration:** 3-4 weeks

### Phase 3: Complex Business Logic Migration
**Rationale:** Requires deep domain understanding, risks institutional knowledge loss
**Delivers:**
- Pipeline progression (POST /api/v2/pipelines/:id/advance)
- Memory creation from perception (POST /api/v2/ghosts/:id/memories)
- Wikilink graph queries (GET /api/v2/wikilinks/:slug)
- Task dependency chain logic
**Implements:** Use cases layer (CreateMemory, AdvancePipeline)
**Avoids:** Pitfall #5 (Throwing Away Knowledge) via behavioral tests
**Research needed:** Possible for complex domain logic if Rust code lacks documentation
**Duration:** 3-4 weeks

### Phase 4: Real-Time Layer (WebSocket + LISTEN/NOTIFY)
**Rationale:** Most complex networking code, requires all data operations migrated first
**Delivers:**
- WebSocket endpoint (/ws)
- Connection hub managing subscriptions
- pgxlisten integration (ghost_tick_complete, task_status_changed)
- Event fan-out to connected clients
**Implements:** LISTEN/NOTIFY Event Bus pattern from ARCHITECTURE.md
**Avoids:** Pitfall #4 (Technology Stack Shift) via goroutine management patterns
**Research needed:** Possible—LISTEN/NOTIFY at scale needs validation under load
**Duration:** 2 weeks

### Phase 5: Rust Retirement & Cutover
**Rationale:** Only after 100% functionality verified in Go
**Delivers:**
- Nginx config updated (all traffic → Go)
- Rust server stopped
- Archive Rust codebase (tag `rust-legacy-final`)
- Hard cutover date met
**Avoids:** Pitfall #3 (Running Two Systems Forever) via hard deadline
**Research needed:** None—operational phase
**Duration:** 1 week

### Phase 6: Schema Design & Benchmarking (Nine Tables)
**Rationale:** JSONB performance cliff prevention requires upfront hybrid schema design
**Delivers:**
- Nine Tables schema designed with hybrid columns (indexed fields + JSONB meta)
- Query budget established (max latency thresholds)
- GIN index strategy for JSONB paths
- Reconciliation test suite (row counts, checksums)
- Bidirectional migration scripts (83→9, 9→83)
**Addresses:** Differentiator (Nine Tables polymorphism enables cross-domain features)
**Avoids:** Pitfall #7 (JSONB Performance Cliff), #6 (Data Loss), #9 (Migration Without Rollback)
**Research needed:** YES—needs production data benchmarking and load testing
**Duration:** 2 weeks

### Phase 7: Schema Migration & Shadow-Write Validation
**Rationale:** Shadow-write period proves data integrity before cutover
**Delivers:**
- Shadow-write implementation (Go writes to both 83-table and 9-table schemas)
- Row count reconciliation reports
- Checksum validation passing
- Application-level referential integrity validation
- Batch integrity audit cron job
- Gradual cutover (read-only 9-table → writes → retire 83-table)
**Implements:** Polymorphic Table Querying pattern from ARCHITECTURE.md
**Avoids:** Pitfall #8 (Losing Foreign Key Integrity), #10 (Production Data Scale)
**Research needed:** None—migration patterns well-documented
**Duration:** 3-4 weeks

### Phase 8: Generalization & Fresh Droplet Validation
**Rationale:** Sovereignty requirement—installable by anyone, not just Nathan
**Delivers:**
- All hardcoded paths → environment variables
- SSH tunnel assumptions removed
- pm2 process manager configuration extracted
- Database credentials in .env (not hardcoded)
- Fresh droplet test passing (DigitalOcean Ubuntu droplet)
- Installation documentation
**Addresses:** Generalization requirement from spec
**Avoids:** Pitfall #12 (Generalization Without Understanding) via Phase 0 dependency audit
**Research needed:** None—configuration management standard
**Duration:** 2 weeks

### Phase 9: Lisp Runtime Generalization & NOTIFY Integration
**Rationale:** Minimal changes to proven Lisp runtime, only generalization and IPC
**Delivers:**
- Configuration file (config.lisp or .env) for database URL, LLM provider
- NOTIFY statements after tick completion, memory creation
- Connection pooling (`:pooled-p t` consistently used)
- Generalization pass removing Nathan-specific assumptions
**Implements:** No changes to core ghost tick engine, cognition broker, InnateScript
**Avoids:** Rewrite of working Lisp code (not broken, don't fix)
**Research needed:** None—Lisp runtime proven in v1.5
**Duration:** 1 week

### Phase Ordering Rationale

**Sequential not simultaneous:** Go migration (Phases 0-5) completes entirely before schema migration (Phases 6-7) to prevent blame ambiguity (Pitfall #11). If a bug appears during Go migration, it's definitely the language rewrite. If a bug appears during schema migration, it's definitely the schema. No compound debugging.

**Strangler fig incremental:** Phases 1-4 migrate route-by-route with nginx routing, enabling per-route rollback and learning Go patterns under low pressure. No big-bang rewrite (Pitfall #1 prevention).

**Foundation before features:** Phase 0 extracts knowledge and establishes patterns before any migration code, preventing institutional knowledge loss (Pitfall #5) and second-system syndrome (Pitfall #2).

**Schema research before migration:** Phase 6 benchmarks and designs hybrid schema before Phase 7 implements migration, preventing JSONB performance cliff (Pitfall #7) discovered too late.

**Generalization after stability:** Phases 8-9 generalize after core functionality works, preventing premature abstraction and ensuring fresh droplet tests validate real working system.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 4 (Real-Time Layer):** LISTEN/NOTIFY latency under load (100+ concurrent WebSocket clients) not fully validated. Recommend prototype testing before committing.
- **Phase 6 (Schema Design):** Production data benchmarking with 9,846 conversations + 2,554 tasks required. JSONB query patterns need profiling against realistic dataset.

Phases with standard patterns (skip research-phase):
- **Phase 1-2 (CRUD Migration):** HTTP routing and database queries well-documented, Chi + pgx patterns established
- **Phase 7 (Schema Migration):** Migration strategies well-documented (shadow-write, reconciliation testing, rollback procedures)
- **Phase 8-9 (Generalization):** Configuration management and environment variable extraction standard practice

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Go + pgx/v5 + Chi verified from official docs, multiple 2025 sources agreeing. Rust→Go migration rationale validated across industry reports. Lisp runtime proven in v1.5. |
| Features | HIGH | Table stakes verified from KDE-PIM/Obsidian/Notion official docs. Differentiators align with existing codebase (Nine Tables already conceptualized). MVP recommendations grounded in complexity analysis. |
| Architecture | MEDIUM-HIGH | PostgreSQL as shared state substrate validated from official docs. Hexagonal Architecture and Strangler Fig patterns well-established. Go+Lisp polyglot coordination extrapolated from microservices principles (no direct examples found). LISTEN/NOTIFY IPC verified but not battle-tested at scale. |
| Pitfalls | HIGH | Based on authoritative sources (Joel Spolsky, Fred Brooks, GitLab engineering docs, PostgreSQL experts). Dual migration risks validated across multiple 2025 rewrite postmortems. 83% data migration failure rate from industry reports. |

**Overall confidence:** HIGH

Research grounded in official documentation (PostgreSQL, Go stdlib, pgx, Chi), established architectural patterns (Hexagonal, Strangler Fig), and authoritative sources on rewrite risks (Spolsky's Netscape analysis, Brooks' Second-System Syndrome). Medium confidence on Go+Lisp coordination stems from lack of direct examples, but approach extrapolates cleanly from microservices separation principles.

### Gaps to Address

**LISTEN/NOTIFY scalability:** No production case studies found for PostgreSQL LISTEN/NOTIFY with 100+ concurrent WebSocket clients. Recommendation: Prototype in Phase 4 with load testing (simulate 100 clients, measure latency, verify no message loss). Fallback: Redis Pub/Sub if LISTEN/NOTIFY can't handle fan-out at scale.

**JSONB query performance:** Production benchmarks needed with actual data volume (9,846 conversations + 2,554 tasks). Recommendation: Phase 6 loads production snapshot, profiles common queries, establishes latency budget before schema migration. Hybrid schema design (indexed columns + JSONB) is insurance, but real-world validation required.

**InnateScript generalization:** Lisp runtime review needed to identify Nathan-specific assumptions in InnateScript interpreter and ghost routines. Recommendation: Phase 0 includes InnateScript codebase archaeology, document hardcoded paths/credentials/assumptions before generalization pass.

**Fresh droplet dependencies:** Current system has invisible infrastructure (SSH tunnels, pm2, specific file paths, database credentials). Recommendation: Phase 0 dependency audit explicitly documents every external dependency, Phase 8 fresh droplet test validates removal.

**Nine Tables foreign key integrity:** Application-level validation patterns need concrete implementation strategy. Recommendation: Phase 6 defines validation rules and audit procedures, Phase 7 implements cron job verifying referential integrity daily.

## Sources

### Primary (HIGH confidence)
- PostgreSQL 16 Documentation — LISTEN/NOTIFY, JSONB, GIN indexes, full-text search (https://www.postgresql.org/docs/current/)
- Go Official Blog — slog introduction (Go 1.21), standard library evolution (https://go.dev/blog/)
- pgx v5 Repository — binary protocol, JSONB handling, connection pooling (https://github.com/jackc/pgx)
- Chi Router Documentation — middleware patterns, net/http compatibility (https://github.com/go-chi/chi)
- KDE Kontact Suite — PIM feature expectations (https://kontact.kde.org/)
- Obsidian Help — wikilinks, backlinks, daily notes (https://help.obsidian.md/)
- WebAuthn W3C Spec — passkey authentication (https://www.w3.org/TR/webauthn-3/)

### Secondary (MEDIUM confidence)
- LogRocket: "Top Go Frameworks 2025" — Chi vs Gin vs Echo comparison
- Heap.io: "When To Avoid JSONB In A PostgreSQL Schema" — performance cliff analysis
- pganalyze: "Postgres performance cliffs with large JSONB values and TOAST" (2025)
- GitLab Engineering Docs: "Polymorphic Associations" — foreign key integrity warnings
- AWS Prescriptive Guidance: Strangler Fig migration pattern
- Joel Spolsky: "Things You Should Never Do, Part I" (2000) — Netscape rewrite analysis
- Fred Brooks: "The Mythical Man-Month" — Second-System Syndrome
- Multiple Medium posts: Rust→Go migration experiences (2025)
- Industry reports: 83% data migration failure rate (BrowserStack, DataGaps)

### Tertiary (LOW confidence, flagged for validation)
- Catsy blog: 85% PIM data quality issues (vendor claim, needs independent verification)
- Nextcloud press release: Sovereign workspace momentum (vendor claim, aligns with EU regulations)
- Go community consensus: 2-3x development velocity vs Rust (anecdotal but consistent across sources)

### Codebase Context
- `/Volumes/Elements/Modular Fortress/.planning/codebase/CONCERNS.md` — 692 lines of existing technical debt
- `/Volumes/Elements/Modular Fortress/.planning/PROJECT.md` — dual migration context
- `/Volumes/Elements/Modular Fortress/Modular Fortress.md` — generalization requirement specification
- `config.json` — current system configuration revealing dependencies (Ollama, pm2, database)

---
*Research completed: 2026-04-04*
*Ready for roadmap: yes*
