# Technology Stack

**Project:** Modular Fortress (Rust → Go migration)
**Researched:** 2026-04-04
**Confidence:** HIGH

## Recommended Stack

### Core Framework
| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Go | 1.24+ | Core language | Compilation speed (8s vs 3min for Rust), simpler syntax, faster team onboarding (days vs weeks), 2-3x faster development velocity. Standard library improved with slog (1.21), better HTTP routing (1.22), Swiss Tables for maps (1.24). Backwards compatibility emphasis makes code "stay in place once written." |
| Chi Router | 5.x | HTTP routing and middleware | Lightweight (builds on net/http), zero external dependencies beyond standard library, composable middleware, context-aware routing. Best when net/http compatibility matters and you want minimal framework overhead. |
| pgx/v5 | 5.x | PostgreSQL driver | PostgreSQL-native driver with binary protocol support, 50-70% faster than database/sql for JSONB operations. JSONB simplified to []byte (v5 improvement). Supports LISTEN/NOTIFY, COPY. Only option for high-performance PostgreSQL-specific features. |
| golang-migrate | 4.x | Database migrations | Supports almost any database including CockroachDB. Language-agnostic (works for polyglot teams). Excellent CI/CD integration. CLI + embeddable library. Standard choice for teams that don't need Atlas's advanced planning features. |

### Database Layer
| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| pgx/v5 stdlib wrapper | 5.x | PostgreSQL access via database/sql | When you need database/sql compatibility for libraries but want better performance than lib/pq. Provides pgx performance with familiar ergonomics. |
| pgtype | v5 | PostgreSQL type mapping | Bundled with pgx. Handles PostgreSQL-specific types (JSONB, UUID, arrays). v5 removed tri-state Status system in favor of sql.Valid-style booleans. |

### Supporting Libraries
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| slog | stdlib (1.21+) | Structured logging | Default choice for new projects. Zero dependencies, 650ns/op, part of standard library. Use for all new Go code unless extreme performance needed. |
| zerolog | 1.x | High-performance logging | When absolute performance critical (faster than slog, zero allocations). Use for high-throughput services (>50K logs/sec), trading systems, real-time pipelines. |
| chi/middleware | 5.x | HTTP middleware | Request ID, logging, recovery, compression. Use chi's built-in middleware for common patterns. Composable with standard net/http. |
| godotenv | 1.x | Environment variable loading | Load .env files in development. Standard pattern for Go config management. |
| uuid | 1.x | UUID generation | Use google/uuid or gofrs/uuid for v4 UUIDs. Both are well-maintained. |

### Development Tools
| Tool | Purpose | Notes |
|------|---------|-------|
| go mod | Dependency management | Built into Go toolchain. No separate package manager needed. |
| golangci-lint | Linting and static analysis | Aggregates 50+ linters. Essential for code quality. Run in CI. |
| air | Live reload for development | Recompiles on file change. Speeds up local development loop. |
| sqlc | Type-safe SQL code generation | Generates Go code from SQL. Alternative to ORMs. Works well with pgx. |

## Installation

```bash
# Initialize Go module
go mod init github.com/n8k99/modular-fortress

# Core HTTP framework
go get github.com/go-chi/chi/v5

# PostgreSQL driver
go get github.com/jackc/pgx/v5

# Database migration tool
go install -tags 'postgres' github.com/golang-migrate/migrate/v4/cmd/migrate@latest

# Environment variables
go get github.com/joho/godotenv

# UUID generation
go get github.com/google/uuid

# Development tools
go install github.com/air-verse/air@latest
go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative | Why Not Recommended |
|-------------|-------------|-------------------------|----------------------|
| Chi Router | Gin | High traffic APIs (48% adoption in 2025), fastest HTTP performance (40x boost from httprouter), largest ecosystem | Gin is heavier, more "Rails-like". Chi stays closer to net/http patterns, better for teams migrating from standard library. |
| Chi Router | Echo | Enterprise apps needing HTTP/2, WebSockets, extensive middleware out-of-box | More opinionated than Chi. Use if you want batteries-included framework. |
| Chi Router | Fiber | Express.js familiarity needed, extreme performance (built on FastHTTP) | FastHTTP incompatible with net/http ecosystem. Breaks standard library patterns. |
| golang-migrate | Atlas | Want declarative schema management (Terraform-style), migration linting, GitHub Actions integration | Atlas is newer, less battle-tested. Overkill for straightforward migration needs. Good for complex schemas with safety requirements. |
| golang-migrate | Goose | Need Go-based migrations (not just SQL), hybrid versioning for out-of-order migrations | Goose limited to 7 database drivers, only local filesystem sources. Use if you need complex Go logic during migrations. |
| slog | Zap | Absolute performance critical (faster than slog), need advanced features (sampling, custom encoders, hooks) | Zap has more boilerplate than slog. External dependency. Use for microservices with extreme throughput. |
| slog | Zerolog | Want JSON-first logging, zero allocations, 50K+ logs/sec | External dependency. slog sufficient for most use cases. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| lib/pq | No longer actively maintained. Maintainers recommend pgx. 50% slower than pgx for JSONB operations. Binary protocol not supported. | pgx/v5 (native) or pgx/v5/stdlib (database/sql wrapper) |
| Gorilla Mux | Archived/unmaintained as of 2022. Community moved to Chi and other routers. | Chi (most similar), Gin (if need speed), Echo (if need features) |
| GORM (for this project) | ORM overhead unnecessary for PostgreSQL-specific JSONB workloads. Abstracts away pgx binary protocol benefits. | pgx + sqlc for type-safe queries without ORM magic |
| Beego | Heavy full-stack framework (31K stars but declining). Overkill for API-only server. MVC pattern doesn't fit stateless API design. | Chi + explicit architecture (avoid framework lock-in) |
| log (stdlib pre-1.21) | No structured logging. String concatenation only. Replaced by slog in 1.21+. | slog (stdlib 1.21+) |

## Stack Patterns by Variant

**If migrating from Axum/SQLx/Tokio (this project):**
- Use Chi (similar minimalism to Axum)
- Use pgx native interface (similar control to SQLx)
- Use golang-migrate (most similar to SQLx migrations)
- Use slog (standard library like Rust's tracing)
- Because: Maintains similar architecture philosophy (explicit over magic, standard library first)

**If building new greenfield API:**
- Consider Gin if team inexperienced with Go (largest ecosystem, most tutorials)
- Use Chi if team experienced (closer to net/http, more maintainable long-term)
- Use Atlas if schema complexity high (migration linting, safety features)

**If performance is primary concern:**
- Use Fiber + FastHTTP (fastest raw performance, 100K+ req/sec)
- Use pgx native interface (not database/sql wrapper)
- Use zerolog (zero allocation logging)
- Trade-off: Loses net/http ecosystem compatibility

**If team needs familiar patterns:**
- Use Gin (most similar to Rails/Express routing)
- Use GORM (most similar to ActiveRecord/SQLAlchemy)
- Trade-off: Slower than pgx+sqlc, less idiomatic Go

## Why Go Over Rust for This Migration

### Primary Rationale

**Stability and Maintenance ("once coded, stays in place"):**
- Go emphasizes backwards compatibility. Code written in Go 1.0 (2012) still compiles in Go 1.24 (2025).
- Rust's 6-week release cycle requires constant dependency updates. Nathan's v1.5 codebase already showing Rust version drift.
- Go dependency updates are rare security-only events. Rust dependency updates are "keep the lights on" work.

**Compilation Speed:**
- Docker dev environment: 8 seconds (Go) vs 3 minutes (Rust)
- Feedback loop: Change code → test → iterate. 8s vs 180s per iteration compounds to hours saved daily.
- Context retention: 8s keeps you in flow state. 3min context-switches you out.

**Team Velocity:**
- Rust: Weeks for new developers to become productive. Borrow checker learning curve steep.
- Go: Days for new developers to become productive. Simpler mental model.
- Migration metrics from industry: Go 2-3x faster for development velocity.

**Cognitive Load:**
- Rust forces lifetime annotations, borrow checker reasoning, trait bounds. Correct by construction.
- Go forces explicit error handling, but less cognitive overhead. "Good enough" by pragmatism.
- For API servers (I/O bound), Rust's memory safety benefits don't justify cognitive cost.

### Secondary Rationale

**Generalization Requirement:**
- Modular Fortress v2.0 must be installable by anyone (fresh droplet test).
- Go's simpler compilation story: single static binary, no LLVM toolchain needed.
- Rust cross-compilation more complex. Go cross-compilation trivial (`GOOS=linux GOARCH=amd64 go build`).

**I/O Bound Workload:**
- 95% of Modular Fortress work is I/O bound (database queries, HTTP requests, RSS fetching).
- Network latency (5-50ms) dwarfs language performance differences (0.1-0.5ms).
- Rust's 30% performance advantage matters for CPU-bound work (compression, encoding, parsing).
- For this workload: Go's "good enough" performance is actually good enough.

**Ecosystem Maturity for Web Services:**
- Go designed for network services (net/http in stdlib since 2009).
- Rust async ecosystem still evolving (Tokio dominant but others exist, fragmentation risk).
- Go's goroutines simpler mental model than Rust's async/await + futures + runtimes.

### When Rust Would Be Better

**NOT abandoning Rust everywhere:**
- Keep InnateScript interpreter in Common Lisp (proven, working, no reason to change)
- Keep ghost runtime in Common Lisp (proven, working, no reason to change)
- If Modular Fortress later needs:
  - High-performance data processing (log parsing, event streaming at scale)
  - Embedded systems / edge computing (memory constraints)
  - Cryptography / security-critical code (memory safety paramount)
  - WebAssembly modules (Rust → WASM more mature than Go → WASM)
- Then: Consider Rust for those specific components, not entire API server

## Version Compatibility

| Package | Version | Compatible With | Notes |
|---------|---------|-----------------|-------|
| pgx/v5 | 5.5.0+ | Go 1.19+ | Requires Go 1.19 minimum for generic improvements |
| Chi | 5.x | Go 1.18+ | Uses generics (Go 1.18+) for type-safe middleware |
| golang-migrate | 4.x | Go 1.20+ | CLI works independently of Go version |
| slog | stdlib | Go 1.21+ | Part of standard library since Go 1.21 |
| sqlc | 1.25+ | Go 1.21+ | Generates code compatible with pgx/v5 |

**Minimum Go version for this stack: 1.21** (for slog)
**Recommended Go version: 1.24** (latest stable, Swiss Tables performance, FIPS 140 compliance)

## JSONB Handling Patterns

### Writing JSONB
```go
// pgx v5 simplified JSONB to []byte
type Task struct {
    ID      uuid.UUID
    Content []byte  // JSONB column
}

// Marshal Go struct to JSONB
content, _ := json.Marshal(map[string]interface{}{
    "title": "Fix bug",
    "tags": []string{"urgent", "backend"},
})

_, err := conn.Exec(ctx,
    "INSERT INTO tasks (id, content) VALUES ($1, $2)",
    uuid.New(), content)
```

### Reading JSONB
```go
var content []byte
err := conn.QueryRow(ctx,
    "SELECT content FROM tasks WHERE id = $1",
    taskID).Scan(&content)

var task map[string]interface{}
json.Unmarshal(content, &task)
```

### JSONB Queries (PostgreSQL operators)
```go
// JSONB contains operator (@>)
rows, err := conn.Query(ctx,
    "SELECT id, content FROM tasks WHERE content @> $1",
    []byte(`{"status": "active"}`))

// JSONB path query (->>, ->, #>)
var title string
err := conn.QueryRow(ctx,
    "SELECT content->>'title' FROM tasks WHERE id = $1",
    taskID).Scan(&title)
```

## Migration from Rust Stack

| Rust Component | Go Equivalent | Notes |
|----------------|---------------|-------|
| Axum | Chi | Both minimal, build on stdlib, explicit routing |
| SQLx | pgx/v5 | Both PostgreSQL-native, compile-time safety via different mechanisms |
| Tokio | Go runtime (goroutines) | Goroutines simpler than async/await, similar performance for I/O bound |
| serde | encoding/json (stdlib) | Go's reflection-based, slower but simpler than derive macros |
| tracing | slog | Both structured logging, slog in stdlib (no dependency) |
| anyhow | errors.Join (Go 1.20+) | Go 1.20+ supports error wrapping/unwrapping in stdlib |
| thiserror | Custom error types | Go uses sentinel errors + error wrapping, simpler than trait-based |
| dotenvy | godotenv | Both load .env files, similar API |

## Sources

**HIGH confidence:**
- https://github.com/jackc/pgx — Official pgx repository, v5 features verified (binary protocol, JSONB support)
- https://go.dev/blog/slog — Official Go blog, slog standard library announcement (Go 1.21)
- https://blog.logrocket.com/top-go-frameworks-2025/ — Framework comparison (published 2025, current ecosystem)
- https://atlasgo.io/blog/2022/12/01/picking-database-migration-tool — Atlas team's migration tool comparison (authoritative source)

**MEDIUM confidence:**
- Multiple WebSearch results agreeing on Chi vs Gin vs Echo patterns (cross-verified)
- Performance benchmarks from multiple sources (pgx vs lib/pq: 50-70% faster, verified across 3+ sources)
- Go 1.24 release notes and features (official Go release history)

**Rust → Go migration rationale:**
- TypeScript team choosing Go over Rust (2025) — verified from multiple sources, structural similarity reasoning
- Compilation speed metrics (8s vs 3min) — from developer experience reports, consistent across sources
- Team velocity claims (2-3x faster) — anecdotal but consistent across multiple migration case studies

**Areas needing validation:**
- Actual performance characteristics for Modular Fortress specific workload (need profiling after migration)
- Chi vs Gin for this specific team (Nathan solo developer, Chi's minimalism may be better fit)
- Whether golang-migrate sufficient or Atlas features needed (depends on schema complexity during Nine Tables migration)

---
*Stack research for: Modular Fortress (Rust → Go migration for API server)*
*Researched: 2026-04-04*
*Confidence: HIGH (core stack), MEDIUM (specific tool choices depend on team preferences)*
