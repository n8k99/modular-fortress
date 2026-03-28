# Technology Stack

**Project:** PARAT Noosphere Schema (v1.3)
**Researched:** 2026-03-28

## Recommended Stack

No new technologies. PARAT is a schema restructuring within the existing stack.

### Core Framework
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| PostgreSQL | 14+ | Schema changes, new tables, views, rules | Existing substrate. All PARAT tables live here. |
| Rust (dpn-core) | 2021 ed. | New DB modules (areas, templates, archives, resources) | Existing shared library. All DB access goes through dpn-core. |
| Rust (dpn-api/Axum) | 0.7 | New REST endpoints for PARAT tables | Existing API gateway. New handlers follow existing patterns. |
| Common Lisp (SBCL) | -- | Tick engine updates for memories table name | Existing ghost runtime. Minimal changes (Python helper path). |
| Python 3 | 3.x | Migration scripts, temporal compression tool, Nexus import | Existing tool language. gotcha-workspace patterns. |

### Database Libraries
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| sqlx | 0.8 | Async PostgreSQL queries in dpn-core/dpn-api | Already in use. All new PARAT modules use sqlx. |
| pgvector | -- | VECTOR(768) columns on archives table | Already installed (vault_notes, documents use it). |

### Supporting Libraries
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| serde/serde_json | 1 | Serialize PARAT structs | All new Rust structs |
| chrono | 0.4 | Date handling for temporal compression | Compression date ranges |
| psycopg2 or pg (Python) | -- | Migration scripts direct DB access | Nexus import, backfill scripts |

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| Table rename | VIEW + RULES | Hard ALTER TABLE RENAME | Too many FK dependencies, breaks live system |
| Resources table | Overlay (source_table/source_id) | Full data migration | 48K documents, complex FKs, not worth the risk |
| Ghost memory columns | Keep wide-table (64 columns) | Normalize to join table | Wide-table is fast for per-agent reads, no JOINs needed at 64 agents |
| Temporal compression | Python tool via standing order | Rust compiled tool | Python is faster to iterate, compression is batch not real-time |
| Nexus import | Copy to archives (keep originals) | Move from documents | Preserves all existing document_id FKs |

## No New Dependencies

This milestone adds zero new crate dependencies. All functionality uses existing sqlx patterns, existing serde serialization, existing Axum routing.

```bash
# No new packages needed
# dpn-core: new .rs files only (areas.rs, templates.rs, archives.rs, resources.rs)
# dpn-api: new handler files only (areas.rs, templates.rs, archives.rs, resources.rs)
# Python: standard library + existing psycopg2/pg for migration scripts
```

## Sources

- dpn-core Cargo.toml (existing dependencies)
- dpn-api Cargo.toml (existing dependencies)
- Live database (pgvector already installed, HNSW indexes in use)
