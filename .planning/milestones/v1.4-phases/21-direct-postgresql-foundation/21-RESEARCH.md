# Phase 21: Direct PostgreSQL Foundation - Research

**Researched:** 2026-03-29
**Domain:** SBCL Common Lisp PostgreSQL integration via libpq FFI
**Confidence:** HIGH

## Summary

Phase 21 replaces the AF64 tick engine's HTTP-over-curl calls to dpn-api with direct PostgreSQL queries for perception and agent state updates. The ghost tick engine currently shells out to `curl` for every API call (via `uiop:run-program`), incurring process-spawn overhead per request. After this phase, the perception path and state update path run as SQL queries through libpq's C library, called via SBCL's built-in `SB-ALIEN` FFI.

The research confirms that **libpq FFI is the correct approach** for the zero-deps AF64 convention. It requires no Quicklisp, no vendored CL libraries, and no SCRAM-SHA-256 implementation -- libpq handles all authentication natively. A proof-of-concept connecting SBCL to master_chronicle via libpq, executing queries, and reading results has been verified on the target droplet. The perception endpoint's SQL logic (514 lines in Rust) will be restructured into ~6 separate queries returning Lisp data structures directly, as specified in D-06.

**Primary recommendation:** Use SBCL's `SB-ALIEN` FFI to call `libpq.so.5` functions (PQconnectdb, PQexec, PQescapeLiteral, PQgetvalue, etc.) for a zero-dependency PostgreSQL client that handles SCRAM-SHA-256 authentication automatically.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Claude's discretion on PG client approach -- vendor cl-postgres, write minimal wire protocol client, or use libpq FFI. Must follow AF64 zero-deps convention (no Quicklisp). Must handle: authentication (md5 or scram-sha-256), simple queries, parameterized queries, and result parsing into Lisp data structures.
- **D-02:** Big bang replacement -- replace all HTTP calls in one shot, no dual-path fallback. Once the SQL path is wired in, the old HTTP calls are removed entirely.
- **D-03:** dpn-api is NOT removed or modified. It continues serving Next.js frontends and MCP tools.
- **D-04:** Connection pool with 2-3 connections opened at startup. Connections persist across ticks and reconnect on failure.
- **D-05:** Connection parameters: host=127.0.0.1, port=5432, db=master_chronicle, user=chronicle, password=chronicle2026.
- **D-06:** Restructure queries for Lisp rather than mirroring dpn-api's Rust SQL exactly. Query each data type separately (messages, tasks, projects, documents, team activity) and build the perception hash-table directly from row results.
- **D-07:** The perception data shape (hash-table with :messages, :tasks, :projects, :documents, :team-activity keys) MUST remain identical so downstream tick engine code works unchanged.
- **D-08:** State updates (energy, tier, last_tick_at) are simple UPDATE statements replacing current `api-patch` calls.

### Claude's Discretion
- PG client implementation choice (D-01) -- **RECOMMENDATION: libpq FFI** (see analysis below)
- Exact connection pool size (2 or 3) -- **RECOMMENDATION: 2 connections** (sufficient for serial tick cycle)
- Whether to use prepared statements or simple queries -- **RECOMMENDATION: simple queries with PQescapeLiteral** (simpler FFI, safe for known parameter types)
- Error handling strategy for connection failures mid-tick -- **RECOMMENDATION: reconnect-on-failure with fallback to empty-perception**
- Whether to add a health-check query at startup -- **RECOMMENDATION: yes, `SELECT 1` at startup to fail fast**

### Deferred Ideas (OUT OF SCOPE)
- Conversations and task mutations via SQL -- Phase 22
- Removing dpn-api entirely -- out of scope (serves frontends)
- LISTEN/NOTIFY for real-time perception -- future enhancement
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DB-01 | Perception queries run as SQL from Lisp tick engine, returning same data shape as /api/perception/:agent_id -- messages, tasks, projects, documents, team activity | libpq FFI verified working on target system; perception endpoint SQL fully analyzed (514 lines Rust); 6 separate queries identified for Lisp restructuring |
| DB-02 | Agent state updates (energy, tier, last_tick_at) written directly via SQL from Lisp, bypassing HTTP PATCH | State update SQL fully documented from dpn-api af64_agents.rs; single UPDATE statement with GREATEST/LEAST energy clamping identified |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| libpq | 5.16 (PG 16.9) | PostgreSQL C client library | Already installed, handles SCRAM-SHA-256 auth, battle-tested |
| SB-ALIEN | SBCL 2.4.10 built-in | Foreign Function Interface | SBCL built-in, zero deps, direct C library calls |
| SB-BSD-SOCKETS | SBCL 2.4.10 built-in | TCP sockets (if needed) | Available but NOT needed -- libpq handles connections |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| libpq FFI | Raw PG wire protocol via SB-BSD-SOCKETS | Would require implementing SCRAM-SHA-256 from scratch (SHA-256 not in SBCL stdlib, would need openssl FFI anyway) |
| libpq FFI | Vendor cl-postgres from Postmodern | Requires vendoring 5+ ASDF systems (ironclad, cl-base64, uax-15, split-sequence, md5) -- violates zero-deps convention |
| libpq FFI | Shell out to psql | Same process-spawn overhead as curl approach; defeats the purpose |

**Why libpq FFI wins decisively:**
1. Zero additional dependencies -- libpq.so.5 already installed, SB-ALIEN already in SBCL
2. SCRAM-SHA-256 handled automatically by libpq (verified: PG uses scram-sha-256 for 127.0.0.1 TCP)
3. Proven working: test script connects, queries, reads results on this exact droplet
4. The alternative (raw wire protocol) would require implementing SCRAM-SHA-256 which needs SHA-256 + HMAC + PBKDF2 -- no SBCL built-in for any of these
5. cl-postgres vendoring would bring in ~20,000+ lines of ironclad alone

**Verified on target system:**
```
$ sbcl --noinform --load test-libpq.lisp
Status: 0 (0=OK)
Result status: 2 (2=TUPLES_OK)
Rows: 3, Cols: 2
```

## Architecture Patterns

### Recommended Project Structure
```
/opt/project-noosphere-ghosts/lisp/
  util/
    json.lisp          # existing -- unchanged
    http.lisp          # existing -- still used by action-executor (Phase 22 scope)
    pg.lisp            # NEW: libpq FFI bindings + connection pool
  runtime/
    api-client.lisp    # existing -- still used by non-perception code
    db-client.lisp     # NEW: db-query, db-execute wrappers (mirrors api-client pattern)
    perception.lisp    # MODIFIED: replace api-get with db-query
    energy.lisp        # MODIFIED: replace api-patch with db-execute
    tick-engine.lisp   # MODIFIED: replace api-get/api-patch with db equivalents
    ...
```

### Pattern 1: libpq FFI Binding Layer (pg.lisp)
**What:** Low-level libpq function definitions using SB-ALIEN, plus a connection pool struct and result-to-Lisp conversion.
**When to use:** All PostgreSQL operations flow through this layer.
**Example:**
```lisp
;; Source: Verified on target droplet 2026-03-29
(sb-alien:load-shared-object "libpq.so.5")

(sb-alien:define-alien-routine "PQconnectdb" (* t) (conninfo sb-alien:c-string))
(sb-alien:define-alien-routine "PQstatus" sb-alien:int (conn (* t)))
(sb-alien:define-alien-routine "PQexec" (* t) (conn (* t)) (query sb-alien:c-string))
(sb-alien:define-alien-routine "PQresultStatus" sb-alien:int (res (* t)))
(sb-alien:define-alien-routine "PQntuples" sb-alien:int (res (* t)))
(sb-alien:define-alien-routine "PQnfields" sb-alien:int (res (* t)))
(sb-alien:define-alien-routine "PQgetvalue" sb-alien:c-string
  (res (* t)) (row sb-alien:int) (col sb-alien:int))
(sb-alien:define-alien-routine "PQfname" sb-alien:c-string (res (* t)) (col sb-alien:int))
(sb-alien:define-alien-routine "PQgetisnull" sb-alien:int
  (res (* t)) (row sb-alien:int) (col sb-alien:int))
(sb-alien:define-alien-routine "PQclear" sb-alien:void (res (* t)))
(sb-alien:define-alien-routine "PQfinish" sb-alien:void (conn (* t)))
(sb-alien:define-alien-routine "PQescapeLiteral" sb-alien:c-string
  (conn (* t)) (str sb-alien:c-string) (length sb-alien:int))
(sb-alien:define-alien-routine "PQerrorMessage" sb-alien:c-string (conn (* t)))
```

### Pattern 2: Connection Pool
**What:** A simple struct holding 2 PG connections with acquire/release and reconnect-on-failure.
**When to use:** All db-query/db-execute calls acquire a connection from the pool.
**Example:**
```lisp
(defstruct pg-pool
  (connections (make-array 2 :initial-element nil))
  (in-use (make-array 2 :initial-element nil))
  (conninfo ""))

(defun pool-acquire (pool)
  "Get an available connection, reconnecting if needed."
  (let ((conns (pg-pool-connections pool))
        (used (pg-pool-in-use pool)))
    (dotimes (i (length conns))
      (unless (aref used i)
        (let ((conn (aref conns i)))
          (when (or (null conn) (not (= 0 (pqstatus conn))))
            ;; Reconnect
            (when conn (pqfinish conn))
            (setf conn (pqconnectdb (pg-pool-conninfo pool)))
            (setf (aref conns i) conn))
          (when (= 0 (pqstatus conn))
            (setf (aref used i) t)
            (return-from pool-acquire (values conn i))))))
    (error "No available PG connections in pool")))

(defun pool-release (pool index)
  (setf (aref (pg-pool-in-use pool) index) nil))
```

### Pattern 3: Result-to-Hash-Table Conversion
**What:** Convert PQresult rows into Lisp hash-tables matching the perception data shape.
**When to use:** Every query result needs conversion to match existing data structures.
**Example:**
```lisp
(defun result-to-vectors (res)
  "Convert a PG result into a vector of hash-tables (one per row).
   Column names become keywords via json-keyword (underscore->hyphen)."
  (let* ((nrows (pqntuples res))
         (ncols (pqnfields res))
         (col-names (loop for c below ncols collect (json-keyword (pqfname res c))))
         (rows (make-array nrows)))
    (dotimes (r nrows)
      (let ((ht (make-hash-table :test #'equal)))
        (loop for c below ncols
              for name in col-names
              do (if (= 1 (pqgetisnull res r c))
                     (setf (gethash name ht) :null)
                     (setf (gethash name ht) (pqgetvalue res r c))))
        (setf (aref rows r) ht)))
    rows))
```

### Pattern 4: db-query / db-execute Wrappers
**What:** High-level wrappers mirroring the api-get/api-post pattern but using SQL.
**When to use:** All perception and state update code uses these instead of api-get/api-patch.
**Example:**
```lisp
(defun db-query (pool sql)
  "Execute SQL, return vector of hash-tables."
  (multiple-value-bind (conn idx) (pool-acquire pool)
    (unwind-protect
        (let ((res (pqexec conn sql)))
          (unwind-protect
              (let ((status (pqresultstatus res)))
                (if (= status 2)  ;; PGRES_TUPLES_OK
                    (result-to-vectors res)
                    (error "PG query error (~a): ~a" status (pqerrormessage conn))))
            (pqclear res)))
      (pool-release pool idx))))

(defun db-execute (pool sql)
  "Execute SQL for side effects (UPDATE/INSERT). Returns command tag string."
  (multiple-value-bind (conn idx) (pool-acquire pool)
    (unwind-protect
        (let ((res (pqexec conn sql)))
          (unwind-protect
              (let ((status (pqresultstatus res)))
                (if (= status 1)  ;; PGRES_COMMAND_OK
                    t
                    (error "PG execute error (~a): ~a" status (pqerrormessage conn))))
            (pqclear res)))
      (pool-release pool idx))))
```

### Anti-Patterns to Avoid
- **String-interpolating user input into SQL without escaping:** Use PQescapeLiteral for all string values. Agent IDs are alphanumeric but always escape anyway.
- **Not clearing PQ results:** Every PQexec result MUST be PQclear'd to avoid memory leaks. Use unwind-protect.
- **Holding connections across ticks:** Release connections back to pool ASAP. Don't hold a connection for the entire tick duration.
- **Mixing HTTP and SQL for the same operation:** D-02 says big bang -- don't leave any perception or state update path going through HTTP.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| SCRAM-SHA-256 auth | Custom SHA-256 + HMAC + PBKDF2 in Lisp | libpq handles it | SCRAM needs SHA-256 (not in SBCL stdlib), HMAC, PBKDF2, nonce generation -- 500+ lines minimum |
| Connection pooling | Thread-safe pool with mutexes | Simple array-based pool (single-threaded tick engine) | AF64 tick engine is single-threaded; no need for locking overhead |
| SQL parameter binding | PQexecParams FFI (complex alien struct marshaling) | PQexec + PQescapeLiteral | PQexecParams needs alien array allocation for params; PQescapeLiteral + format string is simpler for known types |
| JSON parsing from PG | Custom JSONB binary parser | Query with `::text` cast, use existing parse-json | AF64 already has a solid JSON parser; cast JSONB columns to text in SQL |
| PostgreSQL array parsing | Custom PG array parser | Query with `array_to_json()` cast, use existing parse-json | PG text arrays like `{a,b,c}` need custom parsing; `array_to_json()` gives JSON arrays that parse-json handles |

**Key insight:** libpq gives us the entire PostgreSQL client stack (auth, connection, query execution, result handling, error reporting) through ~15 FFI function definitions. The alternative approaches require reimplementing substantial portions of this.

## Common Pitfalls

### Pitfall 1: PG Text Array Format vs JSON Arrays
**What goes wrong:** PostgreSQL returns text[] columns as `{value1,value2}` format, not JSON arrays. The existing code expects vectors from parse-json.
**Why it happens:** PQgetvalue returns the text representation of PG types. Arrays use PG's own format.
**How to avoid:** Cast array columns to JSON in SQL: `array_to_json(to_agent) as to_agent` or use `SELECT to_json(column)`. Then parse with existing parse-json.
**Warning signs:** Hash-table values that are strings starting with `{` instead of Lisp vectors.

### Pitfall 2: NULL Handling
**What goes wrong:** PQgetvalue returns empty string for NULL columns. Code treats empty string as valid data.
**Why it happens:** libpq convention -- PQgetisnull must be checked separately before PQgetvalue.
**How to avoid:** Always check `(= 1 (pqgetisnull res row col))` before reading value. Map NULLs to `:null` or `nil` as appropriate.
**Warning signs:** Empty strings where perception code expects nil/null.

### Pitfall 3: Type Conversion
**What goes wrong:** PQgetvalue always returns strings (in text mode). Integers, floats, booleans, timestamps all come as strings.
**Why it happens:** libpq text format returns everything as C strings.
**How to avoid:** Type-convert in the result mapping: `(parse-integer value)` for ints, `(read-from-string value)` for floats, string comparison for booleans. Or cast to JSON in SQL for complex types.
**Warning signs:** String comparisons failing because values are "42" not 42.

### Pitfall 4: JSONB Column Handling
**What goes wrong:** JSONB columns (metadata, stage_notes, schedule) return as PG's JSON text representation. Need to parse with existing json parser.
**Why it happens:** PQgetvalue returns text representation of JSONB.
**How to avoid:** Use existing `parse-json` on JSONB column values. They're valid JSON strings.
**Warning signs:** Metadata comparisons failing because the value is a string not a hash-table.

### Pitfall 5: Memory Leak from PQresult
**What goes wrong:** Every PQexec allocates a PGresult that must be freed with PQclear. Forgetting causes growing memory.
**Why it happens:** C memory management -- libpq allocates, caller must free.
**How to avoid:** ALWAYS use unwind-protect around PQexec/PQclear pairs. Never return from a function without clearing.
**Warning signs:** Growing RSS memory over many ticks.

### Pitfall 6: Connection Staleness
**What goes wrong:** TCP connection to PG goes stale (server restart, timeout) but pool still holds the dead handle.
**Why it happens:** Persistent connections across ticks; PG might restart between ticks.
**How to avoid:** Check PQstatus before each use. If not CONNECTION_OK (0), reconnect.
**Warning signs:** Sudden errors on all queries after a PG restart.

### Pitfall 7: Underscore-to-Hyphen Key Convention
**What goes wrong:** Perception data uses hyphenated keywords (:from-agent, :to-agent, :team-activity) because the JSON parser converts underscores. SQL column names use underscores.
**Why it happens:** AF64's json-keyword function does `(substitute #\- #\_ string)`.
**How to avoid:** Use the same `json-keyword` function when converting PG column names to Lisp keywords. This is already in the result-to-vectors pattern above.
**Warning signs:** Missing keys in hash-tables because code looks for :from-agent but the key is :from_agent.

## Code Examples

### Perception Query: Messages (most complex query)
```lisp
;; Source: Derived from /opt/dpn-api/src/handlers/af64_perception.rs lines 33-56
;; Restructured per D-06: separate queries, Lisp-native result building

(defun perceive-messages (pool agent-id since)
  "Query unread messages for agent. Returns vector of hash-tables."
  (let* ((escaped-id (db-escape pool agent-id))
         (escaped-since (db-escape pool since))
         (sql (format nil
           "SELECT id, from_agent, LEFT(message, 500) as message, channel,
                   thread_id::text, metadata->>'source' as source
            FROM conversations
            WHERE (~a = ANY(to_agent) OR message ILIKE '%%@' || ~a || '%%')
              AND from_agent != ~a
              AND NOT (~a = ANY(read_by))
              AND (created_at > ~a::timestamptz
                   OR (metadata->>'source' = 'handoff'
                       AND NOT EXISTS (
                         SELECT 1 FROM conversations r
                         WHERE r.from_agent = ~a
                           AND r.metadata->>'responding_to' = conversations.id::text)))
            ORDER BY CASE WHEN metadata->>'source' = 'handoff' THEN 0 ELSE 1 END,
                     created_at DESC
            LIMIT 10"
           escaped-id escaped-id escaped-id escaped-id escaped-since escaped-id)))
    (db-query pool sql)))
```

### State Update: Energy Delta
```lisp
;; Source: Derived from /opt/dpn-api/src/handlers/af64_agents.rs line 134-141
;; Replaces: (api-patch "/api/agents/:id/state" (json-object :energy-delta delta))

(defun db-update-energy (pool agent-id delta)
  "Update agent energy by delta, clamped to [0, 100]. Returns new energy."
  (let* ((escaped-id (db-escape pool agent-id))
         (sql (format nil
           "UPDATE agent_state SET energy = GREATEST(0, LEAST(100, energy + ~a))
            WHERE agent_id = ~a RETURNING energy"
           delta escaped-id))
         (rows (db-query pool sql)))
    (if (> (length rows) 0)
        (let ((val (gethash :energy (aref rows 0))))
          (if (stringp val) (read-from-string val) val))
        50)))
```

### State Update: Full State Patch
```lisp
;; Source: Derived from /opt/dpn-api/src/handlers/af64_agents.rs lines 127-177
;; Replaces: (api-patch "/api/agents/:id/state" state-update) in phase-update-state

(defun db-update-agent-state (pool agent-id tier ticks-alive ticks-at-current-tier metadata-json)
  "Update agent state: tier, last_tick_at, ticks counters, metadata."
  (let* ((escaped-id (db-escape pool agent-id))
         (escaped-tier (db-escape pool tier))
         (escaped-meta (db-escape pool (encode-json metadata-json)))
         (sql (format nil
           "UPDATE agent_state SET
              tier = ~a,
              last_tick_at = now(),
              ticks_alive = ~a,
              ticks_at_current_tier = ~a,
              metadata = COALESCE(metadata, '{}'::jsonb) || ~a::jsonb
            WHERE agent_id = ~a"
           escaped-tier ticks-alive ticks-at-current-tier escaped-meta escaped-id)))
    (db-execute pool sql)))
```

### Fetch Active Agents
```lisp
;; Source: Derived from /opt/dpn-api/src/handlers/af64_agents.rs lines 15-49
;; Replaces: (api-get "/api/agents") in fetch-active-agents

(defun db-fetch-agents (pool)
  "Fetch all agents with their state. Returns vector of hash-tables."
  (db-query pool
    "SELECT a.id, a.full_name, a.role, a.department,
            array_to_json(a.reports_to) as reports_to,
            a.agent_tier, a.status, a.tool_scope IS NOT NULL AND array_length(a.tool_scope, 1) > 0 as has_tools,
            s.energy, s.tier, s.last_tick_at::text,
            s.ticks_at_current_tier, s.ticks_alive, s.metadata::text
     FROM agents a
     LEFT JOIN agent_state s ON s.agent_id = a.id
     ORDER BY a.id"))
```

### Fetch Fitness
```lisp
;; Source: Derived from /opt/dpn-api/src/handlers/af64_fitness.rs lines 28-30
;; Replaces: (api-get "/api/fitness/:id" ...) in fetch-fitness

(defun db-fetch-fitness (pool agent-id)
  "Get agent's 30-day fitness score."
  (let* ((escaped-id (db-escape pool agent-id))
         (sql (format nil
           "SELECT COALESCE(SUM(score), 0) as total
            FROM agent_fitness
            WHERE agent_id = ~a AND created_at > now() - interval '30 days'"
           escaped-id))
         (rows (db-query pool sql)))
    (if (> (length rows) 0)
        (let ((val (gethash :total (aref rows 0))))
          (if (stringp val) (parse-integer val :junk-allowed t) 0))
        0)))
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| curl subprocess per API call | libpq FFI (this phase) | Phase 21 | Eliminates process-spawn + HTTP overhead per query |
| JSON serialization round-trip | Direct row-to-hash-table | Phase 21 | Skips JSON encode in Rust + JSON decode in Lisp for simple types |
| Single perception mega-query | 6 focused queries per agent | Phase 21 (D-06) | Simpler Lisp code, easier to debug individual data types |

## Open Questions

1. **PQexecParams FFI complexity**
   - What we know: PQexecParams needs alien array allocation which is tricky with SB-ALIEN
   - What's unclear: Whether the extra safety of parameterized queries is worth the FFI complexity
   - Recommendation: Use PQexec + PQescapeLiteral. All parameters are either agent IDs (alphanumeric), timestamps, or integers. PQescapeLiteral provides SQL injection safety. This is a localhost-only, internal-only connection -- the security boundary is the PG auth itself.

2. **Perception queries in scope**
   - What we know: D-06/D-07 specify messages, tasks, projects, documents, team activity
   - What's unclear: The Rust perception endpoint also returns: proactive_eligible, responsibilities, relationships, requests, recent_memories, blocked_tasks, critical_issues
   - Recommendation: Include ALL fields the current perception endpoint returns. The downstream tick engine code accesses all of them. Missing fields would break existing ranking/classification logic. The extra fields (blocked_tasks, critical_issues, proactive_eligible, etc.) are used by ranking and action-planner.

3. **fetch-active-agents and fetch-fitness scope**
   - What we know: These are HTTP calls in the tick cycle outside the perception endpoint
   - What's unclear: Whether they should be converted to SQL in Phase 21 or left as HTTP
   - Recommendation: Convert them to SQL. They're called every tick, they're simple queries, and success criteria #4 says "full perceive-rank-classify cycle using SQL instead of HTTP." fetch-active-agents feeds perceive, fetch-fitness feeds classify.

4. **Tick reporting and mark-read**
   - What we know: api-post for tick-log, tick-reports, and mark-read happen every tick
   - Per CONTEXT.md deferred scope: conversations/mark-read is Phase 22
   - Recommendation: Keep tick-log/tick-reports as HTTP for Phase 21 (they're write-only operations, not in the perceive-rank-classify cycle). Mark-read is explicitly Phase 22.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| PostgreSQL | All queries | Yes | 16.9 | -- |
| libpq.so.5 | FFI bindings | Yes | 5.16 | -- |
| libpq-dev | Build headers (not needed at runtime) | Yes | 16.9 | -- |
| SBCL | Lisp runtime | Yes | 2.4.10 | -- |
| SB-ALIEN | FFI package | Yes | Built-in | -- |
| SB-BSD-SOCKETS | TCP (not needed -- libpq handles it) | Yes | Built-in | -- |
| OpenSSL | SCRAM-SHA-256 (handled by libpq internally) | Yes | 3.3.1 | -- |

**Missing dependencies with no fallback:** None.

**Missing dependencies with fallback:** None.

**PG Authentication:** Server uses `scram-sha-256` for host connections from 127.0.0.1 to master_chronicle. libpq handles this transparently.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual Lisp REPL validation (no test framework exists for AF64) |
| Config file | none -- Wave 0 gap |
| Quick run command | `sbcl --load /opt/project-noosphere-ghosts/lisp/tests/test-pg.lisp` |
| Full suite command | `sbcl --load /opt/project-noosphere-ghosts/lisp/tests/run-all.lisp` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DB-01 | Perception SQL returns same shape as HTTP endpoint | integration | Compare JSON output of db-perceive vs curl perception endpoint | No -- Wave 0 |
| DB-01 | Messages query returns unread messages for agent | integration | Query + verify against known test data | No -- Wave 0 |
| DB-01 | Tasks query respects role-based filtering (exec/staff/triage) | integration | Query for different agent types + verify | No -- Wave 0 |
| DB-02 | Energy delta update clamps to [0, 100] | unit | Update with extreme deltas, verify bounds | No -- Wave 0 |
| DB-02 | State update writes tier, last_tick_at, metadata | integration | Update + SELECT verify | No -- Wave 0 |
| DB-02 | Full tick cycle runs with SQL path | smoke | Run single tick, verify no HTTP errors | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** `sbcl --load tests/test-pg.lisp` (once test file exists)
- **Per wave merge:** Full comparison: run tick with SQL path, verify agent states updated correctly
- **Phase gate:** Run 3 consecutive ticks with SQL path, compare tick reports to baseline

### Wave 0 Gaps
- [ ] `/opt/project-noosphere-ghosts/lisp/tests/test-pg.lisp` -- covers DB-01, DB-02 unit/integration tests
- [ ] `/opt/project-noosphere-ghosts/lisp/tests/run-all.lisp` -- test runner
- [ ] Perception comparison script: curl endpoint vs db-perceive for same agent, diff output

## HTTP Call Inventory (Phase 21 Scope)

Complete inventory of HTTP calls that Phase 21 must convert to SQL:

### Must Convert (perception + state update cycle)
| File | Call | Replacement |
|------|------|-------------|
| `perception.lisp:22` | `(api-get (perception-path agent-id) ...)` | 6-8 separate SQL queries building perception hash-table |
| `tick-engine.lisp:28` | `(api-get "/api/agents")` | `SELECT a.*, s.* FROM agents a LEFT JOIN agent_state s ...` |
| `tick-engine.lisp:36` | `(api-get "/api/conversations" ...)` | `SELECT ... FROM conversations WHERE from_agent = 'nathan' AND ...` |
| `tick-engine.lisp:88` | `(api-get "/api/fitness/:id" ...)` | `SELECT COALESCE(SUM(score), 0) FROM agent_fitness ...` |
| `tick-engine.lisp:465` | `(api-patch "/api/agents/:id/state" ...)` | `UPDATE agent_state SET tier=..., last_tick_at=now() ...` |
| `energy.lisp:60` | `(api-patch "/api/agents/:id/state" :energy-delta)` | `UPDATE agent_state SET energy = GREATEST(0, LEAST(100, energy + delta)) ...` |
| `energy.lisp:66` | `(api-get "/api/agents/:id")` | `SELECT energy FROM agent_state WHERE agent_id = ...` |

### Keep as HTTP (Phase 22 or out of scope)
| File | Call | Reason |
|------|------|--------|
| `tick-engine.lisp:413` | `(api-post "/api/conversations/mark-read" ...)` | Phase 22 -- conversations scope |
| `tick-engine.lisp:478` | `(api-post "/api/tick-log/batch" ...)` | Write-only, not in perceive-rank-classify |
| `tick-reporting.lisp:6` | `(api-post "/api/tick-reports" ...)` | Write-only, not in perceive-rank-classify |
| `drive.lisp:*` | Drive tick/fulfill/query | Write-heavy, not in perceive-rank-classify |
| `action-executor.lisp:*` | All conversation/task mutations | Phase 22 scope |
| `action-planner.lisp:*` | Persona loading, context building | Phase 22 scope |
| `cognition-broker.lisp:*` | Telemetry, job persistence | Phase 22 scope |

## Perception Data Shape Reference

The perception hash-table MUST contain these keys (D-07):

```lisp
;; Keys from current perception.lisp + af64_perception.rs response
:messages       ;; vector of hash-tables: :id :from :message :channel :thread-id
:tasks          ;; vector of hash-tables: :id :text :status :assignee :assigned-to :department
                ;;   :stage :goal-id :stage-notes :project-id :source :context :parent-id
                ;;   :priority :scheduled-at :blocked-by
:projects       ;; vector of hash-tables: :id :name :status :description :goals :blockers
                ;;   :current-context :open-tasks :completed-tasks :schedule :lifestage :area-name
:documents      ;; vector of hash-tables: :id :title
:team-activity  ;; vector of hash-tables: :agent :action :energy

;; Additional fields from perception endpoint (used by ranking/action-planner):
:proactive-eligible  ;; boolean
:responsibilities    ;; hash-table (from agent frontmatter)
:relationships       ;; hash-table: :mentor :reports-to :collaborators :collaborator-activity
:requests            ;; vector (currently empty -- disabled in dpn-api)
:recent-memories     ;; vector of strings
:blocked-tasks       ;; vector of hash-tables (exec only)
:critical-issues     ;; vector of hash-tables (exec only)
```

## Sources

### Primary (HIGH confidence)
- `/opt/dpn-api/src/handlers/af64_perception.rs` -- Full perception SQL logic (514 lines), all JOINs and filters
- `/opt/dpn-api/src/handlers/af64_agents.rs` -- Agent list, state update, and drive endpoints
- `/opt/dpn-api/src/handlers/af64_fitness.rs` -- Fitness score query
- `/opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp` -- All HTTP calls in tick cycle (562 lines)
- `/opt/project-noosphere-ghosts/lisp/runtime/perception.lisp` -- Current perception interface
- `/opt/project-noosphere-ghosts/lisp/runtime/energy.lisp` -- Energy update via HTTP
- `/opt/project-noosphere-ghosts/lisp/util/json.lisp` -- JSON parser with underscore-to-hyphen conversion
- Verified libpq FFI on target system (SBCL 2.4.10, libpq 5.16, PG 16.9)

### Secondary (MEDIUM confidence)
- [PostgreSQL Wire Protocol Flow](https://www.postgresql.org/docs/current/protocol-flow.html) -- Official PG protocol docs
- [PostgreSQL Message Formats](https://www.postgresql.org/docs/current/protocol-message-formats.html) -- Byte-level format reference
- [Postmodern cl-postgres.asd](https://github.com/marijnh/Postmodern/blob/master/cl-postgres.asd) -- Dependency analysis for vendoring alternative

### Tertiary (LOW confidence)
- None -- all findings verified against source code or live system

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- libpq FFI verified working on target droplet with SCRAM-SHA-256 auth
- Architecture: HIGH -- All source files read, HTTP call inventory complete, SQL queries derived from Rust source
- Pitfalls: HIGH -- Based on direct experience with libpq C API behavior and AF64 JSON conventions

**Research date:** 2026-03-29
**Valid until:** 2026-04-28 (stable -- libpq API, PG wire protocol, and SBCL FFI are all mature and slow-moving)
