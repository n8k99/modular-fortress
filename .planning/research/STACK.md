# Technology Stack

**Project:** Noosphere Dispatch Pipeline
**Researched:** 2026-03-26

## Current Stack (No Changes Needed)

The existing stack is well-chosen for this project. The dispatch pipeline is a plumbing project connecting existing components, not a greenfield build. The recommendation is to deepen investment in what already works, not add new technologies.

### Why Not Change Anything

The system already has Rust/Axum for the API, Common Lisp/SBCL for the tick engine, Python for dispatch tooling, and PostgreSQL as the central state store. Each technology is where it should be. The pipeline's problems are broken connections between components, not wrong technology choices.

## Recommended Stack (Existing + Extensions)

### Rust API Layer (dpn-api + dpn-core)

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| axum | 0.7.9 (current) | HTTP framework | Already deployed, working. Upgrade to 0.8 is available but NOT recommended during this project -- 0.8 changes path syntax from `/:param` to `/{param}` which touches every route. Do that separately. |
| sqlx | 0.8.x | PostgreSQL async driver | Already in use. Has `PgListener` for LISTEN/NOTIFY. No version change needed. |
| tokio | 1.x | Async runtime | Already in use with full features. |
| serde / serde_json | 1.x | Serialization | Already in use. No changes. |
| chrono | 0.4.x | Timestamps | Already in use. Needed for deadline calculations. |
| uuid | 1.x | Identifiers | Already in use. Use for GSD dispatch correlation IDs. |

**Confidence:** HIGH -- these are already running in production.

### New Rust Dependencies to Add

| Library | Version | Purpose | Why This One |
|---------|---------|---------|--------------|
| (none) | -- | -- | sqlx 0.8 already includes `PgListener` for LISTEN/NOTIFY. No new crates needed. |

**Confidence:** HIGH -- PgListener is a documented, stable part of sqlx 0.8.

**What NOT to add to Rust:**
- **axum 0.8** -- Breaking path syntax change (`/:id` to `/{id}`) affects every route. Not worth the churn during a pipeline project. Schedule separately.
- **Redis/RabbitMQ** -- PostgreSQL is already the message bus via conversations table. Adding a separate queue system contradicts the "DB is the OS" architecture.
- **tower-sessions** -- Ghost agents authenticate via API key, not sessions.
- **async-graphql** -- REST is fine. The perception endpoint is a single JSON blob.

### PostgreSQL Schema Extensions

| Change | Purpose | Why |
|--------|---------|-----|
| `ALTER TABLE tasks ADD COLUMN source VARCHAR(50)` | Track task origin ('gsd', 'ghost', 'manual') | dispatch_to_db.py tries to write this but it doesn't exist |
| `ALTER TABLE tasks ADD COLUMN context JSONB` | Store GSD metadata (wave, must_haves, depends_on) | dispatch_to_db.py tries to write this but it doesn't exist |
| Verify `project_id` FK on tasks | Link tasks to projects | af64_tasks.rs already queries it -- confirm the column + constraint exist |
| `CREATE INDEX idx_tasks_status_project ON tasks (status, project_id)` | Fast perception queries | Perception queries open tasks per project every tick for every agent |
| `CREATE INDEX idx_tasks_assignee_status ON tasks (assignee, status)` | Fast per-agent task lookup | Perception filters by assignee + status constantly |

**Confidence:** HIGH -- standard PostgreSQL patterns.

**What NOT to change in PostgreSQL:**
- **Don't add a separate job queue table** -- The `tasks` table IS the job queue. `FOR UPDATE SKIP LOCKED` is for competing workers, but task assignment flows through the tick engine's rank/classify cycle, not competing workers.
- **Don't partition the tasks table** -- 3,905 rows is tiny. Partitioning adds complexity for zero benefit below 100K rows.
- **Don't use LISTEN/NOTIFY as primary dispatch** -- Notifications are not persisted. If no listener is connected when NOTIFY fires, the message is lost. The tick engine's polling via perception is the reliable primary path. LISTEN/NOTIFY is only an optional latency optimization.

### Python Dispatch Layer (dispatch_to_db.py)

| Library | Version | Purpose | Why |
|---------|---------|---------|-----|
| psycopg2-binary | 2.9.x | PostgreSQL driver | Already in use. Synchronous CLI tool -- no benefit from async. |
| pyyaml | 6.x | YAML frontmatter parsing | The hand-rolled `parse_frontmatter()` is fragile. pyyaml handles edge cases properly. Already available in gotcha-workspace. |

**Confidence:** HIGH -- psycopg2-binary is battle-tested.

**What NOT to change in Python:**
- **Don't switch to psycopg3** -- Script is synchronous CLI, runs once per dispatch. Zero async benefit.
- **Don't add SQLAlchemy** -- Direct SQL with psycopg2 is simpler for a 300-line script.
- **Don't add Click/Typer** -- argparse already works.

### Common Lisp / SBCL (Noosphere Ghosts)

| Component | Purpose | Why |
|-----------|---------|-----|
| `uiop:run-program` | Subprocess execution for tool calls | Already available in SBCL. Used for Claude Code CLI calls. Extend for all tool types. |
| Custom JSON (util/json.lisp) | JSON encoding/decoding | Already in use. Critical: underscore-to-hyphen conversion (`:project-id` not `:project_id`). |
| Custom HTTP (util/http.lisp) | API calls to dpn-api | Already in use via `api-get`, `api-post`. |

**Confidence:** HIGH -- all components already running in tick engine.

**What NOT to change in Lisp:**
- **Don't add cl-json or jonathan** -- Custom JSON parser handles the underscore-to-hyphen conversion the whole system depends on.
- **Don't add Dexador/Drakma** -- Custom curl-based HTTP client works and is integrated.
- **Don't rewrite the tick engine** -- Project constraint: "extend it, don't rewrite it."

### Tool Execution Framework (New, in Lisp)

The action-executor already has pipeline advancement and tool validation. Extend with a tool dispatch table:

| Tool Type | Execution Strategy | Implementation |
|-----------|-------------------|----------------|
| `code` | `claude -p "prompt" --output-format json` via `uiop:run-program` | Already exists for cognition. Reuse Claude Code CLI provider. |
| `db-query` | API call to dpn-api read endpoints | Use existing `api-get`. |
| `db-mutate` | API call to dpn-api mutation endpoints | Use existing `api-post` / `api-patch`. |
| `api-external` | HTTP call via custom HTTP client | Use existing curl-based client. Add timeout enforcement. |
| `file-read` | `uiop:run-program` calling reader | New but trivial. Scope to specific directories. |
| `file-write` | `uiop:run-program` calling sandboxed writer | New. Must validate paths against allowlist. |

**Confidence:** MEDIUM -- execution strategy is sound but file operation sandboxing needs phase-specific design.

### Progress Reporting

| Mechanism | Technology | Purpose |
|-----------|-----------|---------|
| Task status mutations | PATCH `/api/af64/tasks/:id` | Primary: ghost updates task status |
| Conversation posts | POST `/api/conversations` | Secondary: notable events, blockers |
| Project status updates | (new endpoint needed) | Tertiary: executive updates project-level status |
| `/gsd:progress` query | dispatch_to_db.py `--status` | Nathan's view: aggregate counts |

**Confidence:** HIGH -- most endpoints already exist.

**What NOT to build:**
- **Don't build SSE/WebSocket streaming** -- Out of scope per PROJECT.md. Tick interval is 60s+, polling is fine.
- **Don't build a dashboard** -- Frontend changes are out of scope.

## Migration Path

No technology migration. The work is:

1. **ALTER TABLE tasks** -- Add `source`, `context` JSONB, verify `project_id` FK
2. **Add indexes** -- `(status, project_id)` and `(assignee, status)` on tasks
3. **Fix dispatch_to_db.py** -- Match column names to actual schema
4. **Extend action-executor.lisp** -- Add tool dispatch table
5. **Perception is already comprehensive** -- af64_perception.rs returns messages, tasks, projects, team activity, relationships, proactive eligibility

## Installation

No new packages. Existing dependencies cover everything:

```bash
# Rust -- no Cargo.toml changes
cd /opt/dpn-api && cargo build --release
cd /root/dpn-core && cargo build

# Python -- pyyaml may need install
cd /root/gotcha-workspace && source .venv/bin/activate
pip install pyyaml  # if not already present

# Lisp -- no ASDF changes
```

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| Task queue | PostgreSQL tasks table | Redis + Bull / pgqueuer | "DB is the OS" -- second state store contradicts architecture |
| Message bus | conversations table | RabbitMQ / NATS | Same reason. Two sources of truth is worse than one. |
| Dispatch signal | Tick polling (primary) | LISTEN/NOTIFY (primary) | NOTIFY not persisted. If listener down, message lost. |
| API framework | axum 0.7 (keep) | axum 0.8 (upgrade) | Breaking path syntax. Do as maintenance, not during pipeline work. |
| Python DB driver | psycopg2-binary | psycopg3 | Synchronous CLI script. Zero async benefit. |
| Tool execution | In-process (Lisp) | Separate service | More processes = more failure modes. Keep it in the tick engine. |
| Progress | DB polling | SSE / WebSocket | Explicitly out of scope per PROJECT.md. |

## Sources

- [sqlx PgListener docs](https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgListener.html) -- HIGH confidence
- [PostgreSQL LISTEN/NOTIFY](https://www.postgresql.org/docs/current/sql-notify.html) -- HIGH confidence
- [Axum 0.8.0 announcement](https://tokio.rs/blog/2025-01-01-announcing-axum-0-8-0) -- HIGH confidence, verified breaking changes
- [PostgreSQL task queue patterns](https://medium.com/@huimin.hacker/task-queue-design-with-postgres-b57146d741dc) -- MEDIUM confidence
- [Common Lisp OS interfacing](https://lispcookbook.github.io/cl-cookbook/os.html) -- HIGH confidence
- [sqlx on crates.io](https://crates.io/crates/sqlx) -- HIGH confidence, version 0.8.x confirmed
