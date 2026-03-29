---
phase: 21-direct-postgresql-foundation
plan: 02
subsystem: database
tags: [postgresql, perception, sql, common-lisp, sbcl, tick-engine, direct-db]

# Dependency graph
requires:
  - phase: 21-01 (libpq FFI Foundation)
    provides: pg-query, pg-execute, pg-escape, db-query, db-execute, db-escape, db-coerce-row, connection pool
provides:
  - db-perceive: full perception via SQL matching HTTP endpoint shape (12 keys)
  - db-perceive-messages, db-perceive-tasks, db-perceive-projects, db-perceive-documents
  - db-perceive-team-activity, db-perceive-proactive, db-perceive-responsibilities
  - db-perceive-relationships, db-perceive-recent-memories
  - db-perceive-blocked-tasks, db-perceive-critical-issues
  - db-fetch-agents, db-fetch-fitness, db-fetch-nathan-messages
  - perception.lisp rewired from HTTP to SQL
  - tick-engine.lisp fetch-active-agents and fetch-fitness rewired from HTTP to SQL
affects: [21-03 (state update migration), phase-22 (action executor migration)]

# Tech tracking
tech-stack:
  added: []
  patterns: [handler-case per sub-query for error isolation, array_to_json for PG arrays, db-coerce-row for type coercion, SQL column aliases matching HTTP JSON keys]

key-files:
  created: []
  modified:
    - /opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp
    - /opt/project-noosphere-ghosts/lisp/runtime/perception.lisp
    - /opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp
    - /opt/project-noosphere-ghosts/lisp/packages.lisp
    - /opt/project-noosphere-ghosts/launch.sh

key-decisions:
  - "Used handler-case per sub-query in db-perceive so individual query failures don't break entire perception"
  - "Let PostgreSQL compute time differences in proactive eligibility (EXTRACT EPOCH) rather than parsing ISO timestamps in Lisp"
  - "Used array_to_json() for all PG array columns to ensure proper JSON parsing on the Lisp side"
  - "Used SQL column aliases (agent_id as agent, action_taken as action) to match HTTP endpoint JSON key naming"

patterns-established:
  - "Error-isolated sub-queries: each perception data type queried separately with handler-case fallback to empty default"
  - "Role-based task routing in SQL: triage/exec/staff/toolless agent types get different queries"
  - "Frontmatter parsing from documents table for agent responsibilities and relationships"

requirements-completed: [DB-01]

# Metrics
duration: 9min
completed: 2026-03-29
---

# Phase 21 Plan 02: Perception SQL Migration Summary

**11 perception SQL query functions in db-client.lisp, perception.lisp and tick-engine.lisp rewired from HTTP to direct PostgreSQL -- ghosts now perceive the noosphere via SQL**

## Performance

- **Duration:** 9 min
- **Started:** 2026-03-29T16:58:28Z
- **Completed:** 2026-03-29T17:07:34Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Implemented all 11 perception sub-query functions matching the Rust af64_perception.rs endpoint exactly
- db-perceive orchestrator assembles full perception hash-table with 12 keys (messages, tasks, projects, documents, team-activity, proactive-eligible, responsibilities, relationships, requests, recent-memories, blocked-tasks, critical-issues)
- db-fetch-agents returns 64 agents matching /api/agents output shape
- db-fetch-fitness returns 30-day fitness score via SQL
- db-fetch-nathan-messages enables dormant agent wake-up without HTTP
- Zero api-get calls remain in perception.lisp, fetch-active-agents, or fetch-fitness

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement perception SQL queries in db-client.lisp** - `33b9d77` (feat)
2. **Task 2: Rewire perception.lisp and tick-engine.lisp to use SQL** - `62f5a34` (feat)

## Files Created/Modified
- `lisp/runtime/db-client.lisp` - Added 603 lines: 11 perception query functions + db-perceive + db-fetch-agents/fitness/nathan-messages
- `lisp/runtime/perception.lisp` - Replaced api-get call with db-perceive via SQL
- `lisp/runtime/tick-engine.lisp` - Replaced api-get in fetch-active-agents and fetch-fitness with SQL
- `lisp/packages.lisp` - Updated af64.runtime.perception and af64.runtime.tick-engine imports from af64.runtime.db, exported new functions
- `launch.sh` - Added init-db-pool call during system startup

## Decisions Made
- Used handler-case per sub-query in db-perceive for error isolation (one failing query doesn't break all perception)
- Let PostgreSQL compute EXTRACT(EPOCH) for proactive cooldown check instead of parsing ISO timestamps in Lisp (avoids circular dependency with cognition-types package)
- Used array_to_json() in SQL for all PG array columns (reports_to, assigned_to, blocked_by, to_agent) for reliable JSON parsing
- SQL column aliases match HTTP endpoint JSON keys directly (agent_id as agent, action_taken as action)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed tilde characters in SQL string literals**
- **Found during:** Task 1 (db-fetch-agents and db-fetch-nathan-messages)
- **Issue:** Used `~` line continuation in plain string literals (not format calls), producing literal tilde characters in SQL
- **Fix:** Wrapped all multi-line SQL strings in (format nil ...) for proper line continuation
- **Files modified:** lisp/runtime/db-client.lisp
- **Committed in:** 33b9d77

**2. [Rule 1 - Bug] Fixed unmatched close parenthesis in format nil**
- **Found during:** Task 1 (db-fetch-agents)
- **Issue:** Extra closing paren in format nil expression broke let* binding structure
- **Fix:** Removed extra paren
- **Files modified:** lisp/runtime/db-client.lisp
- **Committed in:** 33b9d77

**3. [Rule 3 - Blocking] Replaced #\Bullet character name**
- **Found during:** Task 1 (db-perceive-relationships)
- **Issue:** #\Bullet is not a standard CL character name, caused READ error during compilation
- **Fix:** Used (code-char #x2022) for the bullet character in string-left-trim
- **Files modified:** lisp/runtime/db-client.lisp
- **Committed in:** 33b9d77

**4. [Rule 3 - Blocking] Avoided circular package dependency**
- **Found during:** Task 1 (db-perceive-proactive)
- **Issue:** parse-iso8601 is in af64.runtime.cognition-types which is defined after af64.runtime.db in packages.lisp
- **Fix:** Used EXTRACT(EPOCH) in SQL to compute minutes-ago instead of parsing timestamps in Lisp
- **Files modified:** lisp/runtime/db-client.lisp
- **Committed in:** 33b9d77

---

**Total deviations:** 4 auto-fixed (2 bugs, 2 blocking)
**Impact on plan:** All fixes necessary for compilation and correct SQL generation. No scope creep.

## Issues Encountered
- ASDF system load requires DPN_API_URL and DPN_API_KEY environment variables (pre-existing requirement from api-client.lisp)
- Pre-existing SBCL warnings (openclaw gateway, forward references) remain unchanged -- not from this plan's changes

## User Setup Required

None - no external service configuration required.

## Known Stubs

None - all functions are fully implemented and tested against live master_chronicle.

## Next Phase Readiness
- All perception and agent-fetch paths now use direct SQL
- api-post/api-patch calls remain in tick-engine (mark-read, state updates, tick-log) -- these are Plan 03 scope
- action-executor.lisp and cognition-broker.lisp HTTP calls remain -- Phase 22 scope
- The tick engine can now run with init-db-pool initialized at startup via launch.sh

## Self-Check: PASSED

All 5 modified files exist. Both commit hashes (33b9d77, 62f5a34) verified in git log.

---
*Phase: 21-direct-postgresql-foundation*
*Completed: 2026-03-29*
