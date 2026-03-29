# Phase 20: Nexus Import & Temporal Compression - Research

**Researched:** 2026-03-29
**Domain:** Data migration, LLM-driven summarization, PostgreSQL batch operations
**Confidence:** HIGH

## Summary

This phase imports ~994 deduplicated ChatGPT conversations from the `documents` table into the `archives` table, generates LLM-summarized temporal compression cascades in the `memories` table, injects synthesized perspectives into four executive ghost memory columns, and links imported archives from daily/weekly notes. The infrastructure is mature -- archives table, memories table with compression columns, Python DB tooling, and Claude CLI are all available.

The key complexity is the LLM summarization pipeline: ~822 non-trivial conversations totaling 43MB of content require per-conversation summaries, then cascading monthly/quarterly/yearly rollups, then domain-routed ghost perspective synthesis. At $0.50/request budget, this requires careful batching and a chunked approach. The deduplication is straightforward -- 990 conversations overlap by relative path between the two archive sets, with only 3 Retired-only and 1 Backup-only unique entries.

**Primary recommendation:** Build a Python pipeline tool in `gotcha-workspace/tools/` using `psycopg2` for DB access and `claude -p` CLI for LLM calls. Process in 5 sequential stages: dedup analysis, archive import, per-conversation summarization, temporal cascade compression, and note linking.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Two source paths. Retired Nebulab (993) and backup-Nebulab (991). 938 title overlaps (actual: 990 by relative path). Deduplication by title match.
- **D-02:** Prefer backup-Nebulab copy for overlaps. Include unique-to-each-side entries. Final = union minus duplicates.
- **D-03:** Document duplicate mapping in audit report.
- **D-04:** Import into archives with source_type='chatgpt_import'. Extract date from filename `YYYY-MM-DD - Title.md`. period_start = period_end = extracted date.
- **D-05:** original_path = document path, source_id = document ID.
- **D-06:** Date range Dec 2023 - Jun 2025 (corrected from Oct 2025; actual data shows 2023-12-21 to 2025-06-10, 19 months).
- **D-07:** LLM summary per non-trivial conversation first.
- **D-08:** Size threshold filters trivial conversations (Claude's discretion on threshold).
- **D-09:** Cascade: per-conversation -> monthly -> quarterly -> yearly. compressed_from tracks source IDs.
- **D-10:** Monthly/quarterly/yearly as compression_tier values in memories.
- **D-11:** Four executives: Nova (ops), LRM (music), Vincent (visual), Sylvia (writing/content).
- **D-12:** One memory per tier per ghost per period.
- **D-13:** LLM summary classifies each conversation by domain for routing.
- **D-14:** Full narrative synthesis, not bullet points.
- **D-15:** Append `## Nexus Imports` section to daily notes with wikilinks.
- **D-16:** Generate missing daily notes from template.
- **D-17:** Weekly notes also get `## Nexus Imports` section.

### Claude's Discretion
- Content size threshold for trivial conversation filtering (D-08)
- Exact LLM prompt design for per-conversation summaries and tier synthesis
- Domain classification approach (keyword-based vs LLM-classified)
- Batch size for LLM calls
- Whether to use dpn-api archives endpoint or direct SQL
- Order of operations

### Deferred Ideas (OUT OF SCOPE)
None
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| IMPORT-01 | Nexus Chat AI documents deduplicated across Archive paths, canonical set identified | Dedup by relative path yields 990 overlaps + 3 Retired-only + 1 Backup-only; direct SQL query on documents table |
| IMPORT-02 | Deduplicated conversations imported into archives with source_type='chatgpt_import', dates, topics | Archives table ready with immutability trigger; date extraction via regex on filename; ~994 canonical entries |
| IMPORT-03 | Temporal cascade: monthly/quarterly/yearly summaries in memories from grouped conversations | Memories table has compression_tier and compressed_from columns; existing monthly/quarterly/yearly path patterns available |
| IMPORT-04 | Nova + 3 executive ghost memory columns populated with synthesized perspectives | 316 daily notes already have Nexus Import markers in nova_memories; four target columns: nova_memories, lrm_memories, vincent_memories, sylvia_memories |
| IMPORT-05 | Daily/weekly notes with markdown links to imported archives, no data corruption | 701 daily notes exist in Nexus date range; Daily Note template available; weekly notes follow YYYY-WNN pattern |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

- **Stack**: Python (gotcha-workspace tools), PostgreSQL -- no new languages
- **DB is the OS**: All state in master_chronicle
- **Workspace portability**: Tools use `_config.py` PATHS, never hardcoded absolute paths
- **New tools must be added** to `tools/manifest.md`
- **Ghost LLM**: Claude Code CLI (`claude -p`) with `--output-format json`, $0.50/request budget
- **Guardrails**: Never mix character positions with byte indices in Rust

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| psycopg2 | (installed) | PostgreSQL access from Python | Already used by gotcha-workspace _db.py |
| claude CLI | 2.1.86 | LLM summarization calls | Ghost LLM provider, already available |
| Python 3.12 | 3.12.7 | Script runtime | gotcha-workspace venv |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| json (stdlib) | built-in | Metadata/tags serialization | Archive metadata, LLM output parsing |
| re (stdlib) | built-in | Date/title extraction from paths | Filename parsing |
| subprocess (stdlib) | built-in | Claude CLI invocation | LLM summary calls |
| datetime (stdlib) | built-in | Date arithmetic for monthly/quarterly/yearly grouping | Temporal cascade |
| logging (stdlib) | built-in | Progress reporting | Batch operation tracking |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Direct SQL via psycopg2 | dpn-api REST endpoints | Direct SQL is faster for bulk operations, no HTTP overhead; API is good for single-record ops |
| claude CLI | Direct Anthropic API | CLI is the established pattern; direct API would need API key management |
| Python script | Rust tool | Python is right for data pipeline scripts; Rust is for the API/core library layer |

## Architecture Patterns

### Recommended Project Structure
```
gotcha-workspace/tools/nexus-import/
  __init__.py
  dedup.py              # Stage 1: Dedup analysis and canonical set
  import_archives.py    # Stage 2: Insert into archives table
  summarize.py          # Stage 3: Per-conversation LLM summaries
  compress.py           # Stage 4: Temporal cascade compression
  link_notes.py         # Stage 5: Daily/weekly note linking
  run_pipeline.py       # Orchestrator: runs all stages in order
  _prompts.py           # LLM prompt templates
```

### Pattern 1: Stage-Based Pipeline
**What:** Each stage is an idempotent script that can be rerun independently
**When to use:** Always -- this pipeline will take hours due to LLM calls
**Example:**
```python
# Each stage checks what's already done and skips completed work
def import_archives(dry_run=False):
    """Stage 2: Import canonical set into archives table."""
    with get_cursor(dict_cursor=True) as cursor:
        # Check what's already imported
        cursor.execute("SELECT source_id FROM archives WHERE source_type='chatgpt_import'")
        already_imported = {row['source_id'] for row in cursor.fetchall()}

        # Get canonical set minus already imported
        cursor.execute(canonical_query)
        for doc in cursor.fetchall():
            if doc['id'] in already_imported:
                continue
            # Insert into archives
```

### Pattern 2: LLM Call with Retry and Budget Tracking
**What:** Wrap Claude CLI calls with retry logic and cost tracking
**When to use:** All LLM summarization calls
**Example:**
```python
import subprocess
import json

def call_claude(prompt: str, max_retries: int = 3) -> str:
    """Call Claude CLI for summarization."""
    for attempt in range(max_retries):
        result = subprocess.run(
            ['claude', '-p', prompt, '--output-format', 'json'],
            capture_output=True, text=True, timeout=120
        )
        if result.returncode == 0:
            response = json.loads(result.stdout)
            return response.get('result', '')
        time.sleep(2 ** attempt)
    raise RuntimeError(f"Claude CLI failed after {max_retries} attempts")
```

### Anti-Patterns to Avoid
- **Loading all 43MB into memory at once:** Process conversations in batches
- **No progress tracking:** Use a progress table or metadata to track which conversations are summarized
- **Destructive daily note updates:** Always append `## Nexus Imports` section, never replace content
- **Single monolithic script:** Break into stages so partial failures are resumable

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| PostgreSQL connectivity | Custom DB driver | psycopg2 via `_db.py` | Already available, tested, portable |
| LLM API integration | Direct HTTP to Anthropic | `claude -p` CLI | Project standard, handles auth/model selection |
| Date parsing from filenames | Complex manual parsing | `re.match(r'(\d{4}-\d{2}-\d{2})')` | Simple regex, well-defined format |
| Quarterly date grouping | Manual month-to-quarter mapping | `(month - 1) // 3 + 1` | Standard arithmetic |

## Common Pitfalls

### Pitfall 1: Archive Immutability Trigger
**What goes wrong:** Trying to UPDATE archives after INSERT fails silently or raises error
**Why it happens:** `trg_archive_immutability` blocks updates to content, title, source_type, source_id, original_path, period_start, period_end
**How to avoid:** Get everything right on INSERT. Only `topic`, `tags`, and `metadata` can be updated after creation.
**Warning signs:** "Archives are immutable: content fields cannot be updated" error

### Pitfall 2: Memories Path Uniqueness Constraint
**What goes wrong:** Inserting a memory with a duplicate path fails
**Why it happens:** `memories_path_key` UNIQUE constraint on `path` column
**How to avoid:** Use a deterministic path pattern like `Nexus/Monthly/YYYY-MM.md` and check existence before insert
**Warning signs:** Unique constraint violation on memories INSERT

### Pitfall 3: Existing Nova Memories Data
**What goes wrong:** Overwriting existing `nova_memories` content on daily notes
**Why it happens:** 316 daily notes already have `[Nexus Import]` markers in nova_memories; 433 total have nova_memories populated
**How to avoid:** For daily notes: check if nova_memories already contains Nexus Import data, append or skip. For new temporal entries: these are new rows, no conflict.
**Warning signs:** Lost ghost perspective data on existing daily notes

### Pitfall 4: LLM Budget Exhaustion
**What goes wrong:** 822 non-trivial conversations * $0.50/call = $411 just for per-conversation summaries
**Why it happens:** Large content (median 6.8KB, max 1.8MB) sent to LLM
**How to avoid:** Truncate very large conversations (>50K chars) to first+last sections. Batch monthly summaries by combining per-conversation summaries (much smaller). Use Claude Haiku for classification, Sonnet for synthesis.
**Warning signs:** Cost exceeding budget, rate limiting

### Pitfall 5: Generic "New chat" Titles
**What goes wrong:** 16 conversations in backup have title "New chat" or "New chat (N)" -- no meaningful topic extraction
**Why it happens:** ChatGPT default title when user didn't rename
**How to avoid:** For these entries, extract topic from first 500 chars of content via LLM or keyword extraction
**Warning signs:** Archives with topic = "New chat" provide no value

### Pitfall 6: Date Extraction Edge Cases
**What goes wrong:** 1 document has no date in path: `Nexus AI Chat Imports/Nexus AI Chat Imports.md` (index file, not a conversation)
**Why it happens:** Index/README files in the import directory
**How to avoid:** Filter to only files matching `YYYY-MM-DD` pattern in filename. Skip index files.
**Warning signs:** NULL period_start on archives

### Pitfall 7: Daily Note Template Variable Substitution
**What goes wrong:** Generated daily notes have unresolved `{{date:YYYY-MM-DD}}` placeholders
**Why it happens:** Template uses Innate expression syntax that isn't evaluated by Python
**How to avoid:** Simple string replacement: `{{date:YYYY-MM-DD}}` -> actual date, `{{date:YYYY}}` -> year, etc.
**Warning signs:** Template variables visible in generated notes

## Code Examples

### Deduplication Query (Stage 1)
```sql
-- Canonical set: backup-Nebulab preferred, plus Retired-only unique entries
WITH backup AS (
  SELECT id, path, title, content,
    regexp_replace(path, '^.*/(\d{4}/\d{2}/)', '\1') AS rel_path
  FROM documents
  WHERE path LIKE 'Archive/backup-Nebulab/Eckenrode Muziekopname/Engineering/Nexus AI Chat Imports/%'
    AND path ~ '\d{4}-\d{2}-\d{2}'
),
retired AS (
  SELECT id, path, title, content,
    regexp_replace(path, '^.*/(\d{4}/\d{2}/)', '\1') AS rel_path
  FROM documents
  WHERE path LIKE 'Archive/Retired Nebulab/04 Archives/01 Nexus AI Chat Imports/%'
    AND path ~ '\d{4}-\d{2}-\d{2}'
),
canonical AS (
  -- All backup entries (preferred)
  SELECT id, path, title, 'backup' as source FROM backup
  UNION ALL
  -- Retired-only entries (not in backup)
  SELECT r.id, r.path, r.title, 'retired' as source
  FROM retired r
  LEFT JOIN backup b ON r.rel_path = b.rel_path
  WHERE b.id IS NULL
)
SELECT * FROM canonical ORDER BY path;
-- Expected: ~994 rows (991 backup + 3 retired-only)
```

### Archive Import Pattern (Stage 2)
```python
def import_conversation(cursor, doc_id, doc_path, content):
    """Insert one conversation into archives table."""
    # Extract date from filename
    date_match = re.search(r'(\d{4}-\d{2}-\d{2})', doc_path)
    conv_date = date_match.group(1) if date_match else None

    # Extract title from filename
    title_match = re.search(r'\d{4}-\d{2}-\d{2} - (.+)\.md$', doc_path)
    title = title_match.group(1) if title_match else os.path.basename(doc_path)

    cursor.execute("""
        INSERT INTO archives (title, content, source_type, source_id,
                              original_path, period_start, period_end, topic, tags, metadata)
        VALUES (%s, %s, 'chatgpt_import', %s, %s, %s, %s, %s, %s::jsonb, %s::jsonb)
        RETURNING id
    """, (title, content, doc_id, doc_path, conv_date, conv_date,
          title,  # topic defaults to title, refined by LLM later
          '[]', json.dumps({'source': 'nexus_import', 'trivial': len(content) < 2000})))
    return cursor.fetchone()[0]
```

### Temporal Cascade Path Patterns
```python
# Path patterns for new memories rows (must be unique)
NEXUS_MONTHLY_PATH = "Nexus/Monthly/{year}-{month:02d}.md"
NEXUS_QUARTERLY_PATH = "Nexus/Quarterly/{year}-Q{quarter}.md"
NEXUS_YEARLY_PATH = "Nexus/Yearly/{year}.md"

# Example: "Nexus/Monthly/2024-01.md"
# These are separate from existing "Areas/N8K99Notes/Monthly Notes/2024-01.md"
```

### LLM Summarization Prompt Pattern
```python
CONVERSATION_SUMMARY_PROMPT = """You are analyzing a ChatGPT conversation from Nathan Eckenrode's archive.

Date: {date}
Title: {title}

Provide a JSON response with:
1. "summary": A 2-4 sentence narrative summary capturing key ideas, decisions, and context
2. "domains": Array of applicable domains from ["operations", "music", "art", "content", "other"]
3. "key_topics": Array of 2-5 topic tags

Conversation content:
{content}
"""
```

## Data Profile (Empirically Verified)

| Metric | Value | Source |
|--------|-------|--------|
| Backup-Nebulab documents | 991 | `SELECT count(*) FROM documents WHERE path LIKE 'Archive/backup-Nebulab/...'` |
| Retired-Nebulab documents | 993 | `SELECT count(*) FROM documents WHERE path LIKE 'Archive/Retired Nebulab/...'` |
| Overlap by relative path | 990 | JOIN query on rel_path |
| Retired-only entries | 3 (1 index file + 2 Oct 2025 chats) | LEFT JOIN exclusion |
| Backup-only entries | 1 (index file) | LEFT JOIN exclusion |
| Canonical conversations | ~994 (991 backup + 3 retired, minus 2 index files = ~992) | Calculated |
| Date range | 2023-12-21 to 2025-06-10 | `min/max(extracted_date)` |
| Months spanned | 19 (Dec 2023 - Jun 2025, but Jun 2025 has only 1 chat) | `SELECT DISTINCT month` |
| Unique conversation dates | 318 | COUNT DISTINCT extracted dates |
| Content size: median | 6,796 chars | percentile_cont(0.50) |
| Content size: avg | 45,350 chars | avg(length(content)) |
| Content size: max | 1,831,927 chars (~1.8MB) | max(length(content)) |
| Under 2K chars (trivial) | 169 (17%) | COUNT with threshold |
| Over 50K chars (large) | 164 (17%) | COUNT with threshold |
| Over 100K chars (very large) | 100 (10%) | COUNT with threshold |
| Existing daily notes in range | 701 | COUNT WHERE note_date in range |
| Daily notes already with Nexus markers | 316 | nova_memories LIKE '%Nexus Import%' |
| Monthly notes needed | ~19 | Months with conversations |
| Quarterly notes needed | ~7 | Q4-2023 through Q2-2025 |
| Yearly notes needed | 3 | 2023, 2024, 2025 |
| Archives table current rows | 2 (test data) | SELECT count(*) |

## Discretion Recommendations

### Content Size Threshold (D-08)
**Recommendation: 2,000 characters.** This filters 169 conversations (17%) as trivial, which includes DALL-E prompt-only chats, single-turn "New chat" exchanges, and very short interactions. Conversations at 2K+ chars have enough substance for meaningful summarization. This saves ~$85 in LLM costs.

### Domain Classification Approach (D-13)
**Recommendation: LLM-classified during per-conversation summary.** Include domain classification in the same LLM call as the summary (see prompt pattern above). Keyword-based classification would miss nuanced multi-domain conversations. Since we already need an LLM call for summary, adding domain classification is zero extra cost.

### Batch Size for LLM Calls
**Recommendation: Process 20 conversations per batch with 2-second delays between calls.** The Claude CLI has no explicit rate limit but is constrained by the $0.50/request budget. Processing in batches of 20 with progress logging allows monitoring and resumability. Save progress after each batch.

### Direct SQL vs dpn-api
**Recommendation: Direct SQL via psycopg2.** This is a data migration pipeline, not a runtime operation. Direct SQL is faster, supports transactions, and allows batch inserts. The dpn-api endpoints are single-record oriented and add HTTP overhead.

### Order of Operations
**Recommendation: Strictly sequential stages.** Each stage depends on the output of the previous:
1. Dedup analysis -> produces canonical set + audit report
2. Archive import -> inserts into archives, records archive IDs
3. Per-conversation LLM summarization -> populates archive metadata + produces summaries
4. Temporal cascade -> groups summaries into monthly/quarterly/yearly memories
5. Note linking -> appends wikilinks to daily/weekly notes

### Large Conversation Handling
**Recommendation: For conversations over 50K chars (164 conversations), truncate to first 10K + last 5K chars for LLM summarization.** The full conversation is still stored in archives (immutable content), but the LLM summary uses a representative sample. This prevents token limit issues and reduces cost.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| nova_memories only (original spec) | 4 executives with domain routing | D-11 (CONTEXT.md) | Richer ghost awareness |
| Bullet-point summaries | Full narrative synthesis | D-14 (CONTEXT.md) | Ghost memories capture tone and evolution |
| vault_notes table | memories table | Phase 18 | New path and column names |
| No compression columns | compression_tier + compressed_from | Phase 18 | Native temporal cascade support |

## Open Questions

1. **Existing nova_memories Nexus markers**
   - What we know: 316 daily notes already have `[Nexus Import] N ChatGPT conversations on this date:` in nova_memories
   - What's unclear: Should we overwrite these with richer narrative synthesis, or preserve and append?
   - Recommendation: Overwrite. The existing markers are placeholder bullet lists. D-14 calls for full narrative synthesis. The new content subsumes the old.

2. **Ghost memory column update for temporal entries**
   - What we know: Monthly/quarterly/yearly memories need ghost columns populated
   - What's unclear: Are these new memory rows or updates to existing monthly/quarterly notes?
   - Recommendation: Create NEW memory rows at `Nexus/Monthly/...` paths, separate from existing `Areas/N8K99Notes/Monthly Notes/...`. This avoids conflicting with operational monthly summaries.

3. **LLM cost estimate**
   - What we know: ~822 non-trivial conversations need summarization + ~19 monthly + ~7 quarterly + 3 yearly rollups + ~4 ghost perspectives per tier = ~900 LLM calls minimum
   - What's unclear: Exact token cost per call at current Anthropic pricing
   - Recommendation: Use `claude -p` with Haiku model for per-conversation summaries (cheaper), Sonnet for temporal cascade synthesis (higher quality needed)

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | pytest (gotcha-workspace) |
| Config file | None -- standard pytest discovery |
| Quick run command | `cd /root/gotcha-workspace && .venv/bin/pytest tests/ -x -q` |
| Full suite command | `cd /root/gotcha-workspace && .venv/bin/pytest tests/ -v` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| IMPORT-01 | Dedup produces canonical set with documented mapping | smoke | `psql -c "SELECT count(*) FROM archives WHERE source_type='chatgpt_import'"` | No -- Wave 0 |
| IMPORT-02 | Archives have correct source_type, dates, topics | SQL verification | `psql -c "SELECT count(*) FROM archives WHERE source_type='chatgpt_import' AND period_start IS NOT NULL"` | No -- Wave 0 |
| IMPORT-03 | Monthly/quarterly/yearly memories exist with compressed_from | SQL verification | `psql -c "SELECT compression_tier, count(*) FROM memories WHERE path LIKE 'Nexus/%' GROUP BY compression_tier"` | No -- Wave 0 |
| IMPORT-04 | Ghost memory columns populated on temporal entries | SQL verification | `psql -c "SELECT count(*) FROM memories WHERE path LIKE 'Nexus/%' AND nova_memories IS NOT NULL"` | No -- Wave 0 |
| IMPORT-05 | Daily notes have ## Nexus Imports section | SQL verification | `psql -c "SELECT count(*) FROM memories WHERE content LIKE '%## Nexus Imports%'"` | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** Run verification SQL queries after each stage
- **Per wave merge:** Full SQL verification suite
- **Phase gate:** All 5 verification queries pass

### Wave 0 Gaps
- [ ] `gotcha-workspace/tools/nexus-import/verify.py` -- SQL verification queries for all 5 requirements
- [ ] Dedup audit report generation

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| PostgreSQL | All stages | Yes | 16 | -- |
| Python 3 | Pipeline scripts | Yes | 3.12.7 | -- |
| psycopg2 | DB access | Yes | (installed in venv) | -- |
| Claude CLI | LLM summarization | Yes | 2.1.86 | -- |
| gotcha-workspace venv | Runtime | Yes | /root/gotcha-workspace/.venv | -- |

**Missing dependencies:** None. All required tools are available.

## Sources

### Primary (HIGH confidence)
- Live database queries on master_chronicle -- all data profile numbers verified empirically
- `/root/dpn-core/src/db/archives.rs` -- Archive struct and CRUD functions
- `/root/dpn-core/src/db/memories.rs` -- Memory struct, create, update functions
- `/root/gotcha-workspace/tools/_db.py` -- Python DB access patterns
- `/root/gotcha-workspace/tools/_config.py` -- Workspace configuration

### Secondary (MEDIUM confidence)
- CONTEXT.md decisions D-01 through D-17 -- user-confirmed implementation choices
- Archive immutability trigger source (verified via pg_proc)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - all tools verified as installed and working
- Architecture: HIGH - pipeline pattern matches existing gotcha-workspace conventions
- Data profile: HIGH - all numbers from live database queries
- Pitfalls: HIGH - verified via schema inspection and live data

**Research date:** 2026-03-29
**Valid until:** 2026-04-28 (stable infrastructure, no version changes expected)
