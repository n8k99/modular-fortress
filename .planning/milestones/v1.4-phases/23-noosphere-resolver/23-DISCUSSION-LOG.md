# Phase 23: Noosphere Resolver - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-29
**Phase:** 23-noosphere-resolver
**Areas discussed:** Resolver location, Entity-to-table mapping, Scope filters, Commission delivery, Error handling
**Mode:** Auto (--auto) — all decisions auto-selected

---

## Resolver Location

| Option | Description | Selected |
|--------|-------------|----------|
| innatescript repo | Keep resolver with the language | |
| noosphere-ghosts repo | Near the DB infrastructure | ✓ |

**User's choice:** [auto] noosphere-ghosts — needs db-client SQL infrastructure and PG connection pool
**Notes:** innatescript stays pure (no DB dependencies), noosphere-resolver is the bridge

---

## Entity-to-Table Mapping

| Option | Description | Selected |
|--------|-------------|----------|
| Single table per @ | @projects.name explicit table | |
| Priority search | agents→projects→areas→templates→resources | ✓ |

**User's choice:** [auto] Priority search with @table.name override
**Notes:** Agents referenced most, so they get highest priority

---

## Scope Filter Syntax

| Option | Description | Selected |
|--------|-------------|----------|
| {key=value} → SQL WHERE | Direct mapping to WHERE clauses | ✓ |
| Custom filter DSL | More expressive but more complex | |

**User's choice:** [auto] Direct SQL WHERE mapping
**Notes:** String comparison only, case-insensitive via ILIKE

---

## Commission Delivery

| Option | Description | Selected |
|--------|-------------|----------|
| db-insert-conversation | Reuse Phase 22 wrapper | ✓ |
| Custom commission table | Separate tracking table | |

**User's choice:** [auto] Reuse db-insert-conversation with channel="commission"
**Notes:** Fire-and-forget pattern matches resolver protocol

---

## Error Handling

| Option | Description | Selected |
|--------|-------------|----------|
| Resistance structs | Match stub-resolver pattern | ✓ |
| CL conditions | Use Lisp condition system | |

**User's choice:** [auto] Resistance structs — matches existing protocol
**Notes:** Priority order disambiguates, no resistance for ambiguous matches

---

## Claude's Discretion

- Exact SQL queries, caching strategy, NULL handling, ASDF wiring

## Deferred Ideas

- Template evaluation — Phase 24
- Ghost expression generation — Phase 25
- Entity caching across ticks
