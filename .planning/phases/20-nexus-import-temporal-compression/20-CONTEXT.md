# Phase 20: Nexus Import & Temporal Compression - Context

**Gathered:** 2026-03-29
**Status:** Ready for planning

<domain>
## Phase Boundary

Deduplicate ~1990 ChatGPT conversation imports across two archive paths (Retired Nebulab / backup-Nebulab), import the canonical set into the archives table, generate temporal compression summaries cascading from per-conversation through monthly/quarterly/yearly tiers in the memories table, inject synthesized perspectives into four executive ghost memory columns (Nova, LRM, Vincent, Sylvia), and link imported archives from daily/weekly notes using wikilinks.

</domain>

<decisions>
## Implementation Decisions

### Deduplication (IMPORT-01)
- **D-01:** Two source paths exist: `Archive/Retired Nebulab/04 Archives/01 Nexus AI Chat Imports/YYYY/MM/` (993 chats) and `Archive/backup-Nebulab/Eckenrode Muziekopname/Engineering/Nexus AI Chat Imports/YYYY/MM/` (997 chats). 938 titles overlap. Deduplication is by title match.
- **D-02:** For overlapping titles, prefer the backup-Nebulab copy (997 > 993, slightly more complete set). For unique-to-each-side entries (~55 Retired-only, ~59 Backup-only), include both. Final canonical set should be the union minus duplicates.
- **D-03:** Document the duplicate mapping (which paths were duplicates, which were unique) in an archive metadata record or report for audit.

### Archive Import (IMPORT-02)
- **D-04:** Import deduplicated conversations into the `archives` table with `source_type='chatgpt_import'`. Extract date from the filename pattern `YYYY-MM-DD - Title.md` into `period_start`/`period_end` (same date for single conversations). Extract topic from title.
- **D-05:** The `original_path` column stores the source document path from the documents table. The `source_id` references the document ID.
- **D-06:** Conversations span Dec 2023 through Oct 2025 (~18 months).

### Compression Approach (IMPORT-03)
- **D-07:** LLM summary per conversation first. Each non-trivial conversation gets an individual LLM summary call. This produces per-conversation archive metadata/summary.
- **D-08:** Filter by content size — conversations under a size threshold (Claude's discretion on exact threshold) are marked as 'trivial' and skipped for LLM summarization. They still get imported into archives but without a summary. This saves budget on DALL-E prompt chats, single-turn exchanges, etc.
- **D-09:** Cascade chain: per-conversation summaries -> monthly summaries -> quarterly summaries -> yearly summaries. Each tier summarizes the tier below via LLM. The `compressed_from` INTEGER[] column tracks source memory IDs at each level.
- **D-10:** Monthly summaries are created as memories rows with `compression_tier='monthly'`, quarterly with `compression_tier='quarterly'`, yearly with `compression_tier='yearly'`.

### Ghost Memory Injection (IMPORT-04)
- **D-11:** Four executives receive topic-routed memory injection, not just Nova:
  - **Nova** (operations, systems) — gets the full narrative perspective as primary memory ghost plus all ops-related content
  - **LRM** (music, audio) — music composition, audio production, podcast conversations
  - **Vincent** (visual, art) — DALL-E, image generation, visual design conversations
  - **Sylvia** (writing, content) — writing, blogging, editorial, narrative conversations
- **D-12:** One memory entry per tier per ghost — e.g., Nova gets one monthly memory per month, one quarterly per quarter, one yearly per year. Each ghost's `{name}_memories` column is populated with synthesized perspective specific to their domain.
- **D-13:** The LLM summary step must also classify each conversation by domain (operations/music/art/content/other) to enable routing. Conversations can route to multiple executives.
- **D-14:** Synthesis perspective is full narrative — rich retelling capturing tone, evolution of ideas, personal context. Not just operational bullet points.

### Daily/Weekly Note Linking (IMPORT-05)
- **D-15:** Append a `## Nexus Imports` section to matching daily notes with `[[wikilinks]]` to the archive entries. Matching is by date (conversation date = note date). Non-destructive — adds to end of note content.
- **D-16:** When a daily note doesn't exist for a date with Nexus imports, generate one using the Daily Note template from `Templates/Daily Note.md` in the documents table. Replace template variables (date, time, etc.) with actual values. This ensures every import date has a note with the Nexus import section.
- **D-17:** Weekly notes that span a week with imports also get a `## Nexus Imports` section summarizing that week's imported conversations.

### Claude's Discretion
- Content size threshold for trivial conversation filtering (D-08)
- Exact LLM prompt design for per-conversation summaries and tier synthesis
- Domain classification approach (keyword-based vs LLM-classified)
- Batch size for LLM calls (how many conversations per batch to manage rate limits/costs)
- Whether to use the dpn-api archives endpoint or direct SQL for the import
- Order of operations (dedup first, then import, then compress, then inject, then link — or interleaved)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Requirements
- `.planning/REQUIREMENTS.md` — IMPORT-01 through IMPORT-05 (acceptance criteria for this phase)

### Schema (live in DB)
- Archives table: `id, title, content, source_type, source_id, original_path, period_start, period_end, topic, tags, metadata, created_at, tsv`
- Memories table: all ghost memory columns (`nova_memories`, `lrm_memories`, `vincent_memories`, `sylvia_memories`, etc.), `compression_tier`, `compressed_from`
- Templates: `Templates/Daily Note.md` in documents table — template for generating missing daily notes

### Prior Phase Context
- `.planning/phases/16-foundation-tables-api/16-CONTEXT.md` — Archives table design decisions, immutability trigger
- `.planning/phases/18-memories-rename/18-CONTEXT.md` — Memories table rename, compression columns, department normalization

### Existing Reports
- Document `Archive/Reports/nexus-chat-image-linking.md` (id in documents table) — Prior processing report on 991 Nexus files, image linking analysis

### dpn-core Archives Module
- `/root/dpn-core/src/db/archives.rs` — Existing Rust CRUD for archives table

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `dpn-core/src/db/archives.rs` — Archive CRUD functions (create, list, get_by_id). Can be used for batch inserts.
- `dpn-core/src/db/memories.rs` — Memory CRUD with compression_tier and compressed_from support.
- `dpn-api` handlers for both archives and memories — REST endpoints available if scripted approach preferred.
- `gotcha-workspace/tools/` — Python tooling framework with DB access patterns.

### Established Patterns
- Phase 18 memories rename: migration pattern for batch operations on memories table.
- Phase 16 archives: immutability trigger prevents UPDATE on content — INSERT only.
- Document table stores source content; archives table is the curated, processed version.

### Integration Points
- `documents` table — source data for Nexus Chat AI conversations (993 Retired + 997 Backup paths)
- `archives` table — destination for deduplicated imports
- `memories` table — destination for temporal compression summaries with ghost memory columns
- `Templates/Daily Note.md` — template for generating missing daily notes
- Daily/weekly notes in memories table — append `## Nexus Imports` section

</code_context>

<specifics>
## Specific Ideas

- Nathan wants full narrative synthesis for ghost memories, not operational bullet points — captures tone, evolution of ideas, personal context
- Topic routing to 4 executives (Nova, LRM, Vincent, Sylvia) goes beyond the original scope of "Nova only" — this enriches the entire executive team's historical awareness
- Daily Note template from the documents table should be used to generate stub notes, maintaining consistency with the existing Innate expression format
- The nexus-chat-image-linking report (136K) shows prior processing work on these files — may contain useful metadata

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 20-nexus-import-temporal-compression*
*Context gathered: 2026-03-29*
