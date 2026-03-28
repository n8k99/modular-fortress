---
phase: 18-memories-rename
verified: 2026-03-28T23:00:00Z
status: passed
score: 7/7 must-haves verified
re_verification: false
---

# Phase 18: Memories Rename Verification Report

**Phase Goal:** The ghost memory substrate operates under its PARAT-native name with compression metadata and zero disruption to live ghost operations
**Verified:** 2026-03-28T23:00:00Z
**Status:** PASSED
**Re-verification:** No — initial verification

---

## Goal Achievement

### Observable Truths

| #  | Truth                                                              | Status     | Evidence                                                       |
|----|--------------------------------------------------------------------|------------|----------------------------------------------------------------|
| 1  | Table is named `memories` and `vault_notes` is a view             | VERIFIED   | `pg_class`: memories relkind='r', vault_notes relkind='v'      |
| 2  | SELECT/INSERT/UPDATE/DELETE all work through the vault_notes view  | VERIFIED   | Live INSERT returned id=61324; UPDATE 1 row; DELETE 1 row; SELECT returns 2832 rows |
| 3  | Every memory row has a compression_tier value                     | VERIFIED   | 0 NULL compression_tier rows across 2831 rows                  |
| 4  | compressed_from INTEGER[] column exists on memories               | VERIFIED   | `information_schema.columns` confirms column exists            |
| 5  | departments lookup table has 8 canonical entries                  | VERIFIED   | SELECT count(*) FROM departments = 8                           |
| 6  | Every agent has a department_id FK pointing to departments        | VERIFIED   | SELECT count(*) FROM agents WHERE department_id IS NULL = 0    |
| 7  | trg_sync_task_checkbox trigger fires on memories table            | VERIFIED   | pg_trigger confirms tgname='trg_sync_task_checkbox' on memories |

**Score:** 7/7 truths verified

---

### Required Artifacts

| Artifact                                      | Expected                                    | Status     | Details                                                                 |
|-----------------------------------------------|---------------------------------------------|------------|-------------------------------------------------------------------------|
| `master_chronicle: memories` (table)          | Primary memory storage, renamed from vault_notes | VERIFIED | relkind='r', 2831 rows, compression columns present                    |
| `master_chronicle: vault_notes` (view)        | Backward-compatible DML view                | VERIFIED   | relkind='v', all 3 INSTEAD OF triggers present (insert/update/delete)  |
| `master_chronicle: departments` (table)       | 8 canonical department entries              | VERIFIED   | 8 rows: Operations, Engineering, Content & Brand, Creative, Legal, Music, Strategy, Executive |
| `/root/dpn-core/src/db/memories.rs`           | Memory and MemoryLight structs, SQL queries | VERIFIED   | Structs Memory + MemoryLight, all SQL uses `FROM memories`             |
| `/root/dpn-core/src/db/mod.rs`                | pub mod memories declaration                | VERIFIED   | `pub mod memories` present                                             |
| `/root/dpn-core/src/lib.rs`                   | Re-exports of Memory, MemoryLight           | VERIFIED   | `pub use db::memories::{Memory, MemoryLight}` present                 |
| `/opt/dpn-api/src/handlers/documents.rs`      | Memory CRUD handlers using db::memories     | VERIFIED   | All calls use `dpn_core::db::memories::*`, raw SQL uses `FROM memories` |
| `/opt/dpn-api/src/handlers/af64_perception.rs`| Perception with memories table              | VERIFIED   | SQL: `FROM memories`, column queries against memories                   |
| `/opt/dpn-api/target/release/dpn-api`         | Built release binary                        | VERIFIED   | Binary exists (16MB, built 2026-03-28 22:48), strings include `FROM memories` x10 |

---

### Key Link Verification

| From                                           | To                                         | Via                              | Status     | Details                                                        |
|------------------------------------------------|--------------------------------------------|----------------------------------|------------|----------------------------------------------------------------|
| `vault_notes` view (INSERT)                    | `memories` table                           | INSTEAD OF trigger               | WIRED      | `vault_notes_insert` trigger → `INSERT INTO memories`, tested live with RETURNING id |
| `vault_notes` view (UPDATE)                    | `memories` table                           | INSTEAD OF trigger               | WIRED      | `vault_notes_update` trigger → `UPDATE memories`, tested live  |
| `vault_notes` view (DELETE)                    | `memories` table                           | INSTEAD OF trigger               | WIRED      | `vault_notes_delete` trigger → `DELETE FROM memories`, tested live |
| `agents.department_id`                         | `departments.id`                           | FK reference                     | WIRED      | All 64 agents have non-NULL department_id                      |
| `/root/dpn-core/src/lib.rs`                    | `/root/dpn-core/src/db/memories.rs`        | `pub use db::memories`           | WIRED      | `pub use db::memories::{Memory, MemoryLight}` confirmed        |
| `/opt/dpn-api/src/handlers/documents.rs`       | `/opt/dpn-core/src/db/memories.rs`         | `dpn_core::db::memories`         | WIRED      | 8+ call sites using `dpn_core::db::memories::*`                |
| `PM2 dpn-api process`                          | `/opt/dpn-api/target/release/dpn-api`      | pm2 restart                      | WIRED      | PM2 id=44, status=online, uptime=3m, serving 200s              |

---

### Data-Flow Trace (Level 4)

| Artifact                            | Data Variable    | Source                    | Produces Real Data | Status    |
|-------------------------------------|------------------|---------------------------|--------------------|-----------|
| `documents.rs` list endpoint        | memories list    | `db::memories::list_light` | Yes — queries `FROM memories` in live DB | FLOWING |
| `af64_perception.rs` memories fetch | recent_memories  | `SELECT FROM memories` raw SQL | Yes — queries agent column from memories table | FLOWING |

---

### Behavioral Spot-Checks

| Behavior                                         | Command                                                    | Result                          | Status  |
|--------------------------------------------------|------------------------------------------------------------|---------------------------------|---------|
| memories table has 2719+ rows                    | `SELECT count(*) FROM memories`                            | 2831 rows                       | PASS    |
| vault_notes view exists and is queryable         | `SELECT count(*) FROM vault_notes` as chronicle user       | 2832 rows (includes test row)   | PASS    |
| INSERT through view auto-generates id            | INSERT INTO vault_notes ... RETURNING id                   | id=61324 (non-NULL, sequential) | PASS    |
| UPDATE through view modifies underlying table    | UPDATE vault_notes; SELECT FROM memories                   | content='updated content'       | PASS    |
| DELETE through view removes from memories        | DELETE FROM vault_notes; SELECT count(*) FROM memories     | count=0 for test path           | PASS    |
| compression_tier populated with no NULLs         | `SELECT count(*) FROM memories WHERE compression_tier IS NULL` | 0                            | PASS    |
| departments has 8 rows                           | `SELECT count(*) FROM departments`                         | 8                               | PASS    |
| agents all have department_id                    | `SELECT count(*) FROM agents WHERE department_id IS NULL`  | 0                               | PASS    |
| dpn-api online and serving requests              | PM2 status + log check                                     | status=online, 200 responses on /api/tasks, /api/agents | PASS |
| Zero vault_notes SQL in dpn-api Rust source      | grep vault_notes /opt/dpn-api/src/**/*.rs                  | 0 matches                       | PASS    |
| Zero vault_notes SQL in dpn-core Rust source     | grep vault_notes /root/dpn-core/src/**/*.rs (non-comment)  | 0 production SQL matches (3 comment-only hits) | PASS |
| Zero vault_notes in dpn-api release binary       | strings dpn-api binary \| grep vault_notes                 | 0 matches                       | PASS    |

---

### Requirements Coverage

| Requirement | Source Plan | Description                                                                      | Status    | Evidence                                                                                |
|-------------|-------------|----------------------------------------------------------------------------------|-----------|-----------------------------------------------------------------------------------------|
| MEM-01      | 18-01       | vault_notes renamed to memories with VIEW bridge preserving backward compatibility | SATISFIED | memories table (relkind='r'), vault_notes view (relkind='v'), INSTEAD OF triggers verified live |
| MEM-02      | 18-01       | memories has compression_tier enum backfilled from note_type                      | SATISFIED | 2415 daily, 314 weekly, 68 monthly, 25 quarterly, 9 yearly — 0 NULLs                   |
| MEM-03      | 18-01       | memories has compressed_from INTEGER[] column                                     | SATISFIED | Column confirmed in information_schema.columns                                          |
| MEM-04      | 18-01       | Departments normalized via lookup table with FK from agents                       | SATISFIED | departments table: 8 rows, all 64 agents have department_id (0 NULLs)                  |
| MEM-05      | 18-03       | All dpn-api Rust endpoints updated from vault_notes to memories                   | SATISFIED | 0 vault_notes references in /opt/dpn-api/src/, documents.rs and af64_perception.rs confirmed |
| MEM-06      | 18-02       | All dpn-core Rust queries updated from vault_notes to memories                    | SATISFIED | 0 production SQL vault_notes references; memories.rs uses `FROM memories` throughout   |

All 6 requirements satisfied. No orphaned requirements found.

---

### Anti-Patterns Found

| File                                                    | Line | Pattern                                               | Severity | Impact                                                          |
|---------------------------------------------------------|------|-------------------------------------------------------|----------|-----------------------------------------------------------------|
| `/opt/project-noosphere-ghosts/tools/write_vault_memory.py` | 31 | `WHERE table_name = 'vault_notes'` in information_schema query | Info | Intentional — queries via vault_notes view (backward compat). information_schema returns vault_notes as VIEW. Column lookup returns 1 row correctly. |
| `/opt/project-noosphere-ghosts/tools/write_vault_memory.py` | 38 | `UPDATE vault_notes` | Info | Intentional — backward compat path through view. UPDATE routes to memories via INSTEAD OF trigger. |
| `/root/gotcha-workspace/tools/` (multiple Python scripts) | various | `vault_notes` in SQL strings | Info | Intentional — these tools use vault_notes view for backward compat. All DML passes through INSTEAD OF triggers to memories. |
| `/root/dpn-core/src/cache/sqlite.rs` | 63 | `vault_notes` in comment | Info | Comment only: `-- Memories (mirrors memories table, renamed from vault_notes)` — not a SQL reference |
| `/root/dpn-core/src/db/mod.rs` | 4 | `vault_notes` in module doc comment | Info | Comment only — not a SQL reference |

**Classification:** All findings are INFO-level. The vault_notes references in Lisp and Python tools are INTENTIONAL — they use the view for backward compatibility, which is exactly what MEM-01 required. No blockers or warnings.

---

### Human Verification Required

None. All phase goals are verifiable programmatically and have been confirmed against the live system.

---

### Gaps Summary

No gaps. Phase 18 achieved its goal completely.

The ghost memory substrate now operates under its PARAT-native name `memories`. The backward-compatible `vault_notes` VIEW with INSTEAD OF triggers ensures zero disruption to Lisp ghosts (action-executor.lisp) and Python tools (gotcha-workspace) that continue to reference vault_notes — their DML passes transparently through the view to the memories table. The dpn-api release binary was rebuilt and deployed with all vault_notes references eliminated from Rust source. Ghost perception reads from the memories table directly. Compression metadata (compression_tier, compressed_from) is fully populated. Departments are normalized with FK backfill complete.

---

_Verified: 2026-03-28T23:00:00Z_
_Verifier: Claude (gsd-verifier)_
