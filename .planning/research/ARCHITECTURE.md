# Architecture Patterns

**Domain:** Polyglot AI Agent Platform (Go API + Common Lisp Runtime)
**Researched:** 2026-04-04

## Recommended Architecture

### The Database-Centric Pattern

**Pattern:** PostgreSQL as shared state substrate with independent language runtimes

```
┌─────────────────────────────────────────────────────────────┐
│                      PostgreSQL                             │
│              (Single Source of Truth)                       │
│                                                             │
│  Nine Tables Schema + Ticks + Wikilinks + Config           │
│  - Polymorphic tables (kind + JSONB meta)                  │
│  - LISTEN/NOTIFY for IPC                                   │
│  - Triggers for consistency                                │
└──────────────┬──────────────────────────┬───────────────────┘
               │                          │
               │                          │
     ┌─────────▼──────────┐     ┌────────▼──────────────┐
     │   Go API Server    │     │  Lisp Ghost Runtime   │
     │  (dpn → Modular    │     │   (AF64 + Innate)     │
     │    Fortress)       │     │                       │
     ├────────────────────┤     ├───────────────────────┤
     │ • HTTP/JSON API    │     │ • Tick engine         │
     │ • Auth (JWT/API)   │     │ • Cognition broker    │
     │ • CRUD operations  │     │ • Action executor     │
     │ • WebSocket/SSE    │     │ • Postmodern (FFI)    │
     │ • pgx connection   │     │ • Hand-rolled JSON    │
     │   pool             │     │ • SBCL runtime        │
     └─────────┬──────────┘     └───────────┬───────────┘
               │                            │
               │                            │
     ┌─────────▼────────────────────────────▼───────────┐
     │           TypeScript UI (Foundry-style)          │
     │  • Scene-based interface                         │
     │  • Real-time updates (WebSocket)                 │
     │  • CRUD panels over scenes                       │
     └──────────────────────────────────────────────────┘
```

**Why this pattern:**
- PostgreSQL provides ACID guarantees that neither language needs to implement
- Each runtime optimized for its role (Go for I/O, Lisp for cognition)
- No FFI complexity between Go and Lisp (clean separation)
- LISTEN/NOTIFY enables event-driven coordination without polling
- Independent scaling: API can scale horizontally, ghosts can run on dedicated hardware

### Component Boundaries

| Component | Responsibility | Technology | Why |
|-----------|---------------|------------|-----|
| **Go API Server** | HTTP interface, auth, CRUD operations, WebSocket/SSE real-time updates | Go 1.23+, pgx/v5, Gin or Echo framework | Fast I/O, excellent concurrency, mature HTTP ecosystem, strong static typing |
| **Lisp Ghost Runtime** | Autonomous agent tick cycle, cognition brokering, action execution | SBCL, Postmodern, zero Quicklisp dependencies | Macros for DSL, REPL-driven development, hot reload without restart, proven in v1.5 |
| **InnateScript Interpreter** | Domain-specific scripting language for ghost routines | SBCL, hand-rolled parser | Part of Lisp runtime, loaded as ASDF system |
| **PostgreSQL** | All persistent state, IPC via LISTEN/NOTIFY, constraints/triggers | PostgreSQL 16+ | MVCC for concurrent access, JSONB for polymorphism, row-level security |
| **TypeScript UI** | User interface, real-time visualization | TypeScript, Foundry VTT patterns, WebSocket | Type safety, reactive UI patterns, scene-based navigation |

### Data Flow

#### HTTP Request Cycle (User → Database)

1. **Client** → HTTP request → **Go API Server**
2. **Gin/Echo Router** → Auth middleware (JWT or API key)
3. **Handler** → Business logic layer
4. **pgx Connection Pool** → SQL query to **PostgreSQL**
5. **PostgreSQL** → Result set
6. **Handler** → JSON serialization
7. **Client** ← HTTP response

**Characteristics:**
- Synchronous request/response
- Connection pooling (10-50 connections typical)
- Context with timeouts (prevent hanging queries)
- CORS middleware for web clients

#### Ghost Tick Cycle (Autonomous Agent Loop)

1. **Cron/Scheduler** triggers `sbcl --eval '(af64:run-tick TICK-NUM)'`
2. **Lisp Tick Engine** → Perception phase
   - Query PostgreSQL via Postmodern (`:pooled-p t`)
   - Tier-aware scans (prime/working/base)
3. **Drive Evaluation** → Energy/pressure calculations
4. **Action Planning** → Build cognition jobs
5. **Cognition Broker** → LLM API calls (cached, winter/thaw)
6. **Action Executor** → Database writes, tool calls
7. **Reporting** → Write tick logs to `the_ledger`
8. **PostgreSQL NOTIFY** → `ghost_tick_complete` channel
9. **Go API Server** (LISTEN subscriber) → Push WebSocket update to UI

**Characteristics:**
- Asynchronous event-driven
- Single-threaded Lisp (one agent at a time)
- Database as coordination layer
- LISTEN/NOTIFY for real-time UI updates

#### IPC Pattern: PostgreSQL as Message Bus

```sql
-- Lisp runtime after completing tick
NOTIFY ghost_tick_complete, '{"agent_id": 42, "tick": 12345}';

-- Go API server listening
LISTEN ghost_tick_complete;
-- Receives payload, pushes WebSocket message to connected clients
```

**Why LISTEN/NOTIFY:**
- Built into PostgreSQL, no Redis dependency
- Lightweight (payload up to 8KB)
- Async notification delivery
- Perfect for event-driven updates (tick completion, task state changes, memory creation)

**Go implementation:** `github.com/jackc/pgxlisten` package
**Lisp implementation:** Postmodern's `cl-postgres-listen` or hand-rolled via `libpq` FFI

## Patterns to Follow

### Pattern 1: Hexagonal Architecture (Ports and Adapters)

**What:** Core domain logic isolated from external dependencies, with adapters for I/O

**When:** Building the Go API server

**Structure:**
```
modular-fortress/
├── internal/
│   ├── domain/          # Business logic, domain models, interfaces
│   │   ├── entities/    # Core entities (Task, Conversation, Ghost)
│   │   ├── ports/       # Interfaces for external dependencies
│   │   └── services/    # Domain services
│   ├── application/     # Use cases, orchestration
│   │   └── usecases/    # CreateTask, UpdateGhost, etc.
│   └── adapter/         # External interface implementations
│       ├── http/        # Gin/Echo handlers, middleware
│       ├── postgres/    # pgx repository implementations
│       └── websocket/   # Real-time push notifications
├── cmd/
│   └── server/          # Main entry point
└── pkg/                 # Public libraries (if extractable)
```

**Why:** Testable without external dependencies, swap implementations (mock DB for tests), clear separation of concerns

**Example:**
```go
// internal/domain/ports/repository.go
type TaskRepository interface {
    Create(ctx context.Context, task *domain.Task) error
    FindByID(ctx context.Context, id int64) (*domain.Task, error)
}

// internal/adapter/postgres/task_repo.go
type pgTaskRepo struct {
    pool *pgxpool.Pool
}

func (r *pgTaskRepo) Create(ctx context.Context, task *domain.Task) error {
    query := `INSERT INTO the_work (slug, kind, title, body, meta, status)
              VALUES ($1, $2, $3, $4, $5, $6) RETURNING id`
    return r.pool.QueryRow(ctx, query, task.Slug, "task", task.Title,
                           task.Body, task.Meta, task.Status).Scan(&task.ID)
}
```

### Pattern 2: Connection Pooling with Context Timeouts

**What:** Reuse database connections with lifecycle management

**When:** Both Go and Lisp database access

**Go implementation:**
```go
// Initialize once at startup
config, _ := pgxpool.ParseConfig(os.Getenv("DATABASE_URL"))
config.MaxConns = 25
config.MinConns = 5
config.MaxConnLifetime = time.Hour
config.MaxConnIdleTime = 30 * time.Minute
pool, _ := pgxpool.NewWithConfig(context.Background(), config)

// Use with timeout
func (r *repo) GetTask(id int64) (*Task, error) {
    ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
    defer cancel()

    var task Task
    err := r.pool.QueryRow(ctx, "SELECT * FROM the_work WHERE id=$1", id).
           Scan(&task.ID, &task.Title, &task.Body)
    return &task, err
}
```

**Lisp implementation (Postmodern):**
```lisp
;; Set connection pool parameters
(setf postmodern:*max-pool-size* 20)

;; Use pooled connections
(postmodern:with-connection
    '("master_chronicle" "nebulab_user" "password" "localhost" :pooled-p t)
  (postmodern:query "SELECT * FROM the_work WHERE id=$1" task-id :single))
```

**Why:** PostgreSQL connections consume ~10MB each; pooling prevents resource exhaustion. SCRAM-SHA-256 auth is expensive; connection reuse amortizes cost.

### Pattern 3: Strangler Fig Migration (Incremental Rewrite)

**What:** Gradually replace Rust API with Go, routing requests via proxy

**When:** Migrating from current Rust codebase to Go

**Implementation:**
```
┌─────────────────────────────────────────────────────────┐
│                    Nginx Reverse Proxy                  │
│                    (Routing Layer)                      │
└───────────────┬──────────────────────┬──────────────────┘
                │                      │
     ┌──────────▼─────────┐  ┌─────────▼────────────┐
     │   Go API Server    │  │  Rust API (Legacy)   │
     │   (New routes)     │  │  (Remaining routes)  │
     │  :8080             │  │  :8888               │
     └────────────────────┘  └──────────────────────┘
```

**Nginx configuration:**
```nginx
upstream go_api {
    server localhost:8080;
}

upstream rust_api {
    server localhost:8888;
}

server {
    listen 80;

    # Migrated routes → Go
    location /api/v2/tasks {
        proxy_pass http://go_api;
    }

    location /api/v2/conversations {
        proxy_pass http://go_api;
    }

    # Legacy routes → Rust
    location / {
        proxy_pass http://rust_api;
    }
}
```

**Migration sequence:**
1. Deploy Go API on port 8080 with zero routes (health check only)
2. Implement `/api/v2/tasks` in Go
3. Update nginx to route `/api/v2/tasks` → Go
4. Verify, monitor, rollback if needed
5. Repeat for next endpoint
6. When all routes migrated, retire Rust server

**Why:** Zero downtime, incremental risk, easy rollback per route, allows learning Go patterns before committing fully

### Pattern 4: LISTEN/NOTIFY Event Bus

**What:** PostgreSQL as lightweight message bus for inter-process events

**When:** Coordinating Go API and Lisp runtime without polling

**Go listener setup:**
```go
import "github.com/jackc/pgxlisten"

conn, _ := pgx.Connect(context.Background(), os.Getenv("DATABASE_URL"))
listener := pgxlisten.NewListener(conn)

err := listener.Listen(context.Background(), "ghost_tick_complete")
for notification := range listener.Notify {
    var payload struct {
        AgentID int64 `json:"agent_id"`
        TickNum int64 `json:"tick"`
    }
    json.Unmarshal([]byte(notification.Payload), &payload)

    // Push WebSocket update to UI
    wsHub.Broadcast(payload)
}
```

**Lisp notifier:**
```lisp
(postmodern:execute
  (format nil "NOTIFY ghost_tick_complete, '~A'"
          (jonathan:to-json
            (list :agent-id agent-id :tick tick-num))))
```

**Events to propagate:**
- `ghost_tick_complete` → UI updates ghost status
- `task_status_changed` → UI updates task board
- `memory_created` → UI shows notification
- `pipeline_advanced` → UI updates pipeline view

**Why:** No polling, sub-second latency, leverages existing PostgreSQL connection, no Redis dependency

### Pattern 5: Polymorphic Table Querying

**What:** Query Nine Tables schema with `kind` discriminator and JSONB meta

**When:** Accessing domain tables in both Go and Lisp

**Schema pattern:**
```sql
CREATE TABLE the_work (
    id BIGSERIAL PRIMARY KEY,
    slug TEXT UNIQUE NOT NULL,
    kind TEXT NOT NULL, -- 'task', 'goal', 'decision', 'routine', 'issue'
    title TEXT NOT NULL,
    body TEXT,
    meta JSONB, -- Kind-specific fields
    status TEXT DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_work_kind ON the_work(kind);
CREATE INDEX idx_work_meta ON the_work USING GIN(meta);
```

**Go querying:**
```go
type Task struct {
    ID        int64
    Slug      string
    Title     string
    Body      string
    Meta      map[string]interface{} // Unmarshal JSONB
    Status    string
    CreatedAt time.Time
}

func (r *repo) FindTasks(ctx context.Context) ([]*Task, error) {
    query := `SELECT id, slug, title, body, meta, status, created_at
              FROM the_work WHERE kind = 'task' ORDER BY created_at DESC`
    rows, _ := r.pool.Query(ctx, query)
    defer rows.Close()

    var tasks []*Task
    for rows.Next() {
        var task Task
        var metaJSON []byte
        rows.Scan(&task.ID, &task.Slug, &task.Title, &task.Body,
                  &metaJSON, &task.Status, &task.CreatedAt)
        json.Unmarshal(metaJSON, &task.Meta)
        tasks = append(tasks, &task)
    }
    return tasks, rows.Err()
}
```

**Lisp querying:**
```lisp
(defun find-tasks ()
  (postmodern:query
    (:select :id :slug :title :body :meta :status :created-at
     :from 'the-work
     :where (:= :kind "task")
     :order-by (:desc :created-at))
    :plists))
```

**Why:** Reduces 83 tables to 9, simplifies queries, JSONB allows schema evolution without migrations, GIN indexes support JSONB queries

## Anti-Patterns to Avoid

### Anti-Pattern 1: Direct FFI Between Go and Lisp

**What:** Using CGO to call Lisp code directly from Go

**Why bad:**
- CGO disables Go's best concurrency features
- Complex memory management (Go GC vs SBCL GC)
- Debugging nightmares across runtimes
- Deployment complexity (single binary impossible)
- Performance overhead at FFI boundary

**Instead:** Use PostgreSQL as shared state + LISTEN/NOTIFY for coordination. Clean process separation.

### Anti-Pattern 2: Shared In-Memory State

**What:** Redis or shared memory segments for state coordination

**Why bad:**
- PostgreSQL already provides ACID guarantees
- Adds operational complexity (one more service to run)
- Cache invalidation problems
- Violates "database as single source of truth" principle

**Instead:** PostgreSQL with proper indexing (GIN for JSONB, B-tree for IDs). Use connection pooling to reduce query latency. LISTEN/NOTIFY for real-time updates.

### Anti-Pattern 3: Big Bang Rewrite

**What:** Replace entire Rust codebase in one release

**Why bad:**
- High risk, no rollback path
- Requires code freeze during migration
- Learning Go patterns under pressure
- Testing entire surface area before release

**Instead:** Strangler Fig pattern. Migrate route by route. Learn incrementally. Each migration is independently deployable.

### Anti-Pattern 4: Nano-Services (Over-Decomposition)

**What:** Separate Go service per domain table

**Why bad:**
- Network latency between services
- Distributed transaction complexity
- Operational overhead (9+ services to deploy)
- Doesn't match team size (single developer)

**Instead:** Monolithic Go API server with Hexagonal Architecture. Use internal packages for domain boundaries. Scale horizontally if needed (stateless API).

### Anti-Pattern 5: REST for Real-Time Updates

**What:** Client polling `/api/ghosts/status` every second

**Why bad:**
- Unnecessary database load
- Network inefficiency
- UI lag (polling interval vs event arrival)
- Scales poorly (N clients = N × requests/sec)

**Instead:** WebSocket or Server-Sent Events (SSE) for real-time push. LISTEN/NOTIFY triggers push to connected clients. Client receives updates within milliseconds of database change.

## Scalability Considerations

| Concern | At 1 User (Now) | At 10 Users | At 100 Users |
|---------|----------------|-------------|--------------|
| **Database** | Single PostgreSQL instance, 25 connections | Same (PostgreSQL handles this easily) | Consider read replicas, connection pooling via PgBouncer |
| **Go API** | Single server, localhost | Single server with proper connection pooling | Horizontal scaling (2-3 instances behind load balancer) |
| **Lisp Runtime** | Single process, sequential ticks | Same (ghosts are per-user) | One Lisp process per user, scheduled independently |
| **Real-time Updates** | WebSocket, in-process hub | Same | Consider Redis Pub/Sub for WebSocket fan-out across Go instances |
| **Storage** | PostgreSQL on same machine | Same | Consider separate DB server, SSD storage |
| **Authentication** | JWT + API keys | Same | Consider session storage in PostgreSQL or Redis |

**Key insight:** Architecture naturally scales to 10-100 users without major changes. Stateless Go API scales horizontally. Lisp runtime scales by running independent processes per user.

## Rewrite Strategy: Strangler Fig (Incremental)

### Why Incremental Over Big Bang

**Incremental advantages:**
- **Risk mitigation:** Each route migration is independently testable and rollbackable
- **Learning curve:** Practice Go patterns on simple endpoints before complex ones
- **No downtime:** Nginx routes traffic seamlessly during migration
- **Continuous delivery:** Ship improvements while migration progresses
- **Cost containment:** Spread effort over time, no code freeze

**Big bang disadvantages:**
- **All-or-nothing risk:** Single bug breaks entire application
- **Development freeze:** No feature work during migration window
- **Pressure testing:** Must validate entire surface area before release
- **Rollback complexity:** Reverting requires full deployment

**Decision:** Use Strangler Fig pattern with nginx routing layer. Migrate route by route over 3-6 months.

### Migration Phases

#### Phase 0: Foundation (Week 1-2)

**Goal:** Go project structure, database connection, health check endpoint

**Deliverables:**
- Go project initialized with Hexagonal Architecture
- pgx connection pool configured
- `/health` endpoint returning 200 OK
- Nginx routing `/api/v2/health` → Go, everything else → Rust
- CI/CD pipeline for Go (build, test, deploy)

**Why first:** Validates deployment pipeline before adding complexity. Establishes patterns for subsequent routes.

#### Phase 1: Read-Only Endpoints (Week 3-6)

**Goal:** Migrate simple GET endpoints (tasks, conversations, ghosts)

**Suggested order:**
1. `GET /api/v2/tasks` → List tasks from `the_work` table
2. `GET /api/v2/tasks/:id` → Single task by ID
3. `GET /api/v2/conversations` → List conversations from `the_post`
4. `GET /api/v2/ghosts` → List ghosts from `the_forge`

**Why first:** No data mutation, easy to validate correctness, builds confidence with database queries.

#### Phase 2: Write Endpoints (Week 7-10)

**Goal:** Migrate POST/PUT/DELETE endpoints

**Suggested order:**
1. `POST /api/v2/tasks` → Create task
2. `PUT /api/v2/tasks/:id` → Update task
3. `DELETE /api/v2/tasks/:id` → Soft delete (set status='archived')
4. Repeat for conversations, ghosts

**Why second:** Requires transaction handling, validation, error handling patterns. Builds on Phase 1 knowledge.

#### Phase 3: Complex Endpoints (Week 11-14)

**Goal:** Migrate endpoints with business logic

**Suggested order:**
1. Pipeline progression (`POST /api/v2/pipelines/:id/advance`)
2. Memory creation from perception (`POST /api/v2/ghosts/:id/memories`)
3. Wikilink graph queries (`GET /api/v2/wikilinks/:slug`)

**Why third:** Requires deep understanding of domain logic. May need refactoring from Rust patterns to Go idioms.

#### Phase 4: Real-Time Layer (Week 15-16)

**Goal:** WebSocket server with LISTEN/NOTIFY integration

**Deliverables:**
- WebSocket endpoint (`/ws`)
- Connection hub managing client subscriptions
- pgxlisten integration subscribing to PostgreSQL events
- Event fan-out to connected clients

**Why last:** Requires all data operations migrated. Most complex networking code.

#### Phase 5: Rust Retirement (Week 17)

**Goal:** Remove Rust codebase, update nginx to route all traffic to Go

**Deliverables:**
- Nginx config updated (remove Rust upstream)
- Rust server stopped
- Archive Rust codebase (tag as `rust-legacy-final`)
- Update documentation

**Why last:** Only when 100% of functionality verified in Go.

### Migration Validation Strategy

**Per-route validation:**
1. **Functional testing:** Postman/curl scripts comparing Rust vs Go responses
2. **Load testing:** `wrk` or `hey` confirming Go performance ≥ Rust
3. **Monitoring:** Prometheus metrics comparing error rates, latency
4. **Canary deployment:** Route 10% traffic to Go, monitor for 24 hours, then 100%

**Rollback plan:**
- Nginx config revert (point route back to Rust)
- No data migration needed (PostgreSQL is source of truth for both)
- 1-minute rollback window

## Build Order Recommendations

### For Go API Server

**Order of implementation:**

1. **Project structure** (Hexagonal Architecture directories)
2. **Database connection pool** (pgx with context timeouts)
3. **Health check endpoint** (validates deployment)
4. **Auth middleware** (JWT validation, API key checking)
5. **Domain models** (Task, Conversation, Ghost structs)
6. **Repository interfaces** (ports)
7. **PostgreSQL repository implementations** (adapters)
8. **Use cases** (CreateTask, UpdateTask, etc.)
9. **HTTP handlers** (Gin/Echo routes)
10. **WebSocket hub** (connection management)
11. **LISTEN/NOTIFY integration** (pgxlisten)
12. **WebSocket event fan-out** (push notifications)

**Rationale:** Bottom-up approach. Database layer first (validates schema understanding), then domain logic, then HTTP layer, finally real-time.

### For Lisp Runtime (No Rewrite Needed)

**Current Lisp codebase stays.** Only changes needed:

1. **Generalization pass** (remove Nathan-specific hardcoded paths)
2. **Configuration file** (`.env` or `config.lisp` for database URL, LLM provider)
3. **NOTIFY statements** (add after tick completion, memory creation)
4. **Connection pooling** (ensure `:pooled-p t` used consistently)

**Why minimal changes:** Lisp runtime proven in v1.5. Ghost tick engine, cognition broker, InnateScript interpreter all stable. Focus Go effort on API layer.

### Cross-Cutting Concerns

**Implement early:**
- **Logging:** Structured logging (zerolog or zap in Go, format statements in Lisp)
- **Error handling:** Go error wrapping with context, Lisp condition system
- **Configuration:** Environment variables (`.env` via godotenv)
- **Secrets management:** All credentials in `.env`, never committed

**Implement later:**
- **Metrics:** Prometheus endpoints (after core functionality stable)
- **Distributed tracing:** OpenTelemetry (only if debugging cross-language issues)
- **Rate limiting:** Nginx or Go middleware (only when scaling to multi-user)

## Sources

**Confidence Assessment:**

| Topic | Confidence | Sources |
|-------|------------|---------|
| Go + Lisp polyglot patterns | MEDIUM | WebSearch (no authoritative Go+Lisp guides), extrapolated from microservices patterns |
| PostgreSQL as shared state | HIGH | PostgreSQL documentation (LISTEN/NOTIFY), Go/Lisp library docs |
| Strangler Fig migration | HIGH | AWS Prescriptive Guidance, Microsoft Azure Architecture Center (2025) |
| Go project structure | HIGH | Go community best practices (2025), Hexagonal Architecture articles |
| Connection pooling | HIGH | pgx documentation, Postmodern documentation, performance benchmarks |

**Key sources:**
- PostgreSQL LISTEN/NOTIFY: https://www.postgresql.org/docs/current/sql-notify.html
- Go pgx best practices: https://dev.to/mx_tech/go-with-postgresql-best-practices-for-performance-and-safety-47d7
- Postmodern connection pooling: https://github.com/marijnh/Postmodern
- Strangler Fig pattern: https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-design-patterns/strangler-fig.html
- Hexagonal Architecture in Go: https://www.glukhov.org/post/2025/12/go-project-structure/

**Gaps identified:**
- No direct Go + Common Lisp integration examples found (relied on microservices separation principles)
- Limited production case studies of PostgreSQL LISTEN/NOTIFY at scale (validated approach but not battle-tested at 1000+ users)
- InnateScript generalization requirements not fully explored (Lisp codebase review needed)

---

*Architecture analysis: 2026-04-04*
*Confidence: MEDIUM-HIGH (strong on Go and PostgreSQL patterns, extrapolated on Go+Lisp coordination)*
*Recommendation: Validate LISTEN/NOTIFY latency under load before committing. Consider prototyping WebSocket fan-out with 100 simulated clients.*
