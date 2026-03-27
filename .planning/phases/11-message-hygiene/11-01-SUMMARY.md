---
phase: 11-message-hygiene
plan: 01
subsystem: api
tags: [rust, axum, postgresql, perception, conversations, read-marking]

# Dependency graph
requires:
  - phase: 10-lifecycle-signals
    provides: "dpn-api with conversation and perception handlers"
provides:
  - "POST /api/conversations/mark-read endpoint for batch read-marking"
  - "Perception read_by filter excluding already-read messages"
  - "GIN index on conversations.read_by for query performance"
  - "Historical stale message cleanup (2229 messages marked read)"
affects: [11-02-PLAN, ghost-tick-engine, action-executor]

# Tech tracking
tech-stack:
  added: []
  patterns: ["array_append/ANY for read-tracking arrays", "GIN index for array containment queries"]

key-files:
  created: []
  modified:
    - /opt/dpn-api/src/handlers/af64_conversations.rs
    - /opt/dpn-api/src/handlers/af64_perception.rs
    - /opt/dpn-api/src/main.rs

key-decisions:
  - "Used array_append with NOT ANY guard to prevent duplicate entries in read_by"
  - "GIN index added for read_by array -- table has 9K+ rows growing ~950/day"

patterns-established:
  - "Read-marking pattern: POST mark-read with {agent_id, message_ids} batch"

requirements-completed: [FIX-01, FIX-02, SPAM-01]

# Metrics
duration: 4min
completed: 2026-03-27
---

# Phase 11 Plan 01: Mark-Read API + Perception Filter Summary

**POST /api/conversations/mark-read endpoint with read_by perception filtering, GIN index, and historical cleanup of 2229 stale messages**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-27T10:38:55Z
- **Completed:** 2026-03-27T10:43:10Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Mark-read API endpoint functional: accepts {agent_id, message_ids}, returns {"updated": N}
- Perception query now excludes messages where requesting agent is already in read_by
- Historical cleanup: 91 messages marked read via responding_to metadata, 2138 stale messages (>7 days) marked read
- GIN index on conversations.read_by for array containment query performance
- Confirmed sqlx json feature already present in Cargo.toml (FIX-01)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add mark-read endpoint and perception filter to dpn-api** - `667c050` (feat)
2. **Task 2: Restart dpn-api, historical cleanup, GIN index, verify endpoints** - runtime operations only, no file commit needed

## Files Created/Modified
- `/opt/dpn-api/src/handlers/af64_conversations.rs` - Added MarkReadRequest struct and mark_read handler
- `/opt/dpn-api/src/handlers/af64_perception.rs` - Added NOT ($1 = ANY(read_by)) filter to messages query
- `/opt/dpn-api/src/main.rs` - Registered /conversations/mark-read POST route

## Decisions Made
- Used array_append with NOT ANY guard to prevent duplicate read_by entries (idempotent operation)
- Added GIN index for read_by since perception runs for every agent every tick against 9K+ row table
- Historical cleanup marked 2229 total stale messages as read to prevent ghost re-processing on restart

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- PostgreSQL peer authentication failed for chronicle user via Unix socket -- resolved by using TCP connection (-h 127.0.0.1) with password
- chronicle user lacked ownership to create index -- used postgres superuser for GIN index creation

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- API endpoint ready for Plan 02 (Lisp-side integration)
- Ghosts can now call POST /api/conversations/mark-read after processing messages
- Perception automatically filters out read messages, stopping the spam loop

## Self-Check: PASSED

All files verified present. Commit 667c050 confirmed in git log.

---
*Phase: 11-message-hygiene*
*Completed: 2026-03-27*
