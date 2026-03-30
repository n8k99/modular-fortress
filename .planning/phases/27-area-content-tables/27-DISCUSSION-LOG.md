# Phase 27: Area Content Tables - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-30
**Phase:** 27-area-content-tables
**Areas discussed:** Table design, Content domain mapping, Resolver integration, Migration strategy
**Mode:** Auto (--auto flag, all defaults selected)

---

## Table Design

| Option | Description | Selected |
|--------|-------------|----------|
| Single area_content table with content_type discriminator | Simpler, works with resolver, avoids table proliferation | ✓ |
| Per-domain tables (podcast, blog, etc.) | More specific schemas but N tables to maintain | |
| Extend existing documents table with area_id FK | Minimal change but mixes concerns | |

**User's choice:** [auto] Single area_content table (recommended default)
**Notes:** JSONB metadata handles per-type schema differences without separate tables.

---

## Content Domain Mapping

| Option | Description | Selected |
|--------|-------------|----------|
| All existing EM content domains | Comprehensive — match existing path structure (10 types) | ✓ |
| Top 5 only (podcast, blog, branding, engineering, thought-police) | Most content, simpler | |
| Just podcast + blog | Minimal viable | |

**User's choice:** [auto] All existing (recommended default)
**Notes:** Path structure is clean, mapping is straightforward.

---

## Resolver Integration

| Option | Description | Selected |
|--------|-------------|----------|
| CLOS method on area_content scoped by area_id FK | Consistent with Phase 23 resolver pattern | ✓ |
| Extend existing document resolver with area filter | Less code but muddies document resolution | |

**User's choice:** [auto] New CLOS method (recommended default)
**Notes:** Clean separation — area_content is its own entity type in the resolver.

---

## Migration Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Populate from existing documents | Leverage 900+ existing EM docs | ✓ |
| Start fresh, manual population | Clean but loses existing content | |
| Lazy migration on first access | Complex, unpredictable | |

**User's choice:** [auto] Populate from existing documents (recommended default)
**Notes:** Documents table remains untouched — area_content is additive.

---

## Claude's Discretion

- JSONB metadata schema per content_type
- Index strategy
- dpn-api endpoints vs direct SQL only
- Migration batch size

## Deferred Ideas

- Area content for non-EM areas (Orbis, LRM, etc.)
- Full-text search on body
- Embedding vectors on area_content
