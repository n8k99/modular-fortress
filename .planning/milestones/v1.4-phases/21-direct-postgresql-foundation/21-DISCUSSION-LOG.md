# Phase 21: Direct PostgreSQL Foundation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-29
**Phase:** 21-direct-postgresql-foundation
**Areas discussed:** PostgreSQL client approach, Migration strategy, Connection management, Query architecture

---

## PostgreSQL Client Approach

| Option | Description | Selected |
|--------|-------------|----------|
| Vendor cl-postgres | Copy cl-postgres source into AF64 codebase. Pure CL, ~3000 LOC. | |
| Minimal wire protocol | Hand-roll ~500-800 LOC PG client. Follows AF64 zero-deps perfectly. | |
| libpq FFI | Use SBCL FFI to call C libpq. Fast but C dependency. | |
| You decide | Claude picks based on AF64 conventions and tradeoffs. | ✓ |

**User's choice:** You decide
**Notes:** Claude has discretion to pick the best approach.

---

## Migration Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Incremental with fallback | Replace one call at a time, keep HTTP toggle | |
| Big bang replacement | Replace all HTTP calls in one shot, no dual paths | ✓ |
| Shadow mode first | Run both in parallel, compare, then cut over | |

**User's choice:** Big bang replacement
**Notes:** Simpler code, no conditional branching.

---

## Connection Management

| Option | Description | Selected |
|--------|-------------|----------|
| Single persistent | One long-lived connection, reconnect on failure | |
| Connection pool | Small pool (2-3 connections) for parallel queries | ✓ |
| Per-tick connection | Open/close each tick cycle | |

**User's choice:** Connection pool
**Notes:** Enables parallel perception queries for multiple agents.

---

## Query Architecture

| Option | Description | Selected |
|--------|-------------|----------|
| Mirror API queries | Extract exact SQL from af64_perception.rs | |
| Restructure for Lisp | Rewrite queries optimized for Lisp data structures | ✓ |
| You decide | Claude picks | |

**User's choice:** Restructure for Lisp
**Notes:** Query each data type separately, build hash-table directly from rows.

---

## Claude's Discretion

- PG client implementation choice
- Exact connection pool size
- Prepared vs simple queries
- Connection failure error handling
- Startup health-check query

## Deferred Ideas

- Conversations and task mutations — Phase 22
- Removing dpn-api — out of scope
- LISTEN/NOTIFY — future
