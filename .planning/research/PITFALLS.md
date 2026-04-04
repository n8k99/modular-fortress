# Domain Pitfalls

**Domain:** Rust→Go API Rewrite + 83→9 Table Database Migration
**Researched:** 2026-04-04
**Confidence:** HIGH

## Executive Summary

This is a **dual simultaneous migration**: rewriting a working Rust API in Go while collapsing 83 PostgreSQL tables into 9 polymorphic tables. Each migration alone is high-risk. Together, they create compound failure modes that require rigorous separation of concerns, incremental validation, and explicit prevention of "second-system syndrome."

**Critical insight from research:** 83% of data migrations fail. The "rewrite from scratch" is called "the single worst strategic mistake any software company can make." This project is doing both simultaneously.

---

## CRITICAL PITFALLS — REWRITE-SPECIFIC

### Pitfall 1: The Netscape Trap (Complete Rewrite Paralysis)

**What goes wrong:**
You throw away 15 years of accumulated bugfixes, edge cases, and production hardening baked into the Rust codebase. The Go rewrite takes 3x longer than estimated because you're re-discovering problems the Rust code already solved. Meanwhile, the old system keeps adding features that need to be reimplemented in the new system—you're chasing a moving target.

**Why it happens:**
"New code is better than old code" fallacy. The belief that starting fresh eliminates technical debt, when in reality old code has been used, tested, and debugged through years of real-world usage. Developers underestimate the hidden complexity that exists in working systems.

**How to avoid:**
1. **Feature freeze the Rust API** — No new features until Go reaches parity
2. **Extract implicit business logic** — Document every quirk, workaround, and edge case in the Rust code before rewriting
3. **Behavior-driven migration** — Write characterization tests that capture Rust behavior, then ensure Go matches
4. **Phase gate:** No Go code written until Rust behavior is fully documented

**Warning signs:**
- "The Go version will be cleaner" without explaining what specific problems are being solved
- Discovering Rust features that "nobody remembers why they exist"
- Go implementation differs from Rust "because it's better this way"
- Timeline estimates based on lines of code rather than behavior preservation

**Phase to address:**
**Phase 0 (Pre-Development):** Full Rust API behavioral audit with documentation of every endpoint, edge case, error condition, and quirk before any Go code is written.

---

### Pitfall 2: Second-System Syndrome (Feature Creep During Rewrite)

**What goes wrong:**
The Go rewrite becomes "the version we always wanted to build" with improved error handling, better logging, cleaner abstractions, comprehensive metrics, rate limiting, and "just one more feature." You're now building a superset of the Rust system, not a replacement. Completion date slides indefinitely.

**Why it happens:**
Fred Brooks: Developers become overconfident and pack Version 2 with every feature they wished they had before, because "we're rewriting anyway, might as well add X." The rewrite becomes both a migration AND a feature upgrade, doubling the scope.

**How to avoid:**
1. **Strict parity requirement** — Go must match Rust endpoint-for-endpoint, no additions
2. **"Improvements Later" backlog** — Log all "better ways" for post-parity implementation
3. **No architectural changes** — Same REST structure, same response formats, same error codes
4. **Definition of Done:** "Go behaves identically to Rust" not "Go is better than Rust"

**Warning signs:**
- "While we're at it, let's add..."
- Redesigning API contracts instead of copying them
- Adding features that don't exist in Rust
- Architecture discussions that diverge from current design

**Phase to address:**
**Every Phase:** Explicit parity checks with automated comparison testing. Post-migration phases can improve; migration phases must only replicate.

---

### Pitfall 3: Running Two Systems Forever (Parallel Operation Hell)

**What goes wrong:**
You plan to run Rust and Go in parallel "temporarily" during migration. 18 months later both are still running. Every bug fix must be implemented twice. Every schema change must work with both. You now maintain two complete codebases instead of one. Cost doubles. Team burns out.

**Why it happens:**
Migration is "slower (18-24 months instead of 12), but dramatically less risky" so teams choose parallel operation without defining a hard cutover date. The "temporary" state becomes permanent because there's always "one more thing" before the old system can be retired.

**How to avoid:**
1. **Hard cutover date set at project start** — Non-negotiable deadline to retire Rust
2. **No dual maintenance after cutover** — One bug, one codebase, Go only
3. **Phased rollout with gates** — Week 1: 1% traffic, Week 2: 10%, Week 4: 50%, Week 6: 100%
4. **Automated cutover criteria** — Error rates, latency, feature parity tests must pass before progressing

**Warning signs:**
- "We'll switch when we're ready" without a date
- Bug fixes going into Rust after Go reaches parity
- "Keep Rust as a backup" discussions
- Inability to answer "when do we delete the Rust code?"

**Phase to address:**
**Phase -1 (Planning):** Cutover date and criteria defined before Go development starts. Every phase ends with "percentage of traffic migrated" metric.

---

### Pitfall 4: Technology Stack Shift Without Experience

**What goes wrong:**
The team rewrites Rust→Go while simultaneously changing HTTP frameworks, ORMs, connection pooling libraries, logging systems, and monitoring tools. Nobody has production experience with Go's concurrency model. Goroutine leaks cause memory exhaustion. Channels deadlock. Error handling patterns from Rust don't translate to Go's explicit error returns—every function returns `(result, error)` and 137 unwraps/expects in Rust become 200+ unchecked errors in Go.

**Why it happens:**
"Let's modernize everything" mentality. Developers change technologies for a rewrite—frontend frameworks, backend frameworks, message queues, databases—without deep knowledge of the new stack. Go looks simple but has concurrency footguns that Rust's borrow checker prevented.

**How to avoid:**
1. **Minimize technology changes** — Same PostgreSQL, same authentication, same deployment
2. **Go concurrency training** — Every developer completes goroutine/channel/errgroup exercises before production code
3. **Code review gates** — Every PR must use sync.WaitGroup or errgroup, no bare goroutines
4. **Rust→Go translation guide** — Document how every Rust pattern translates to idiomatic Go

**Warning signs:**
- Goroutines launched without coordination (`go someFunc()` with no WaitGroup)
- `_ = someFunc()` discarding errors (Go equivalent of `.unwrap()`)
- Channels with no receivers/senders (deadlock risk)
- Porting Rust's Result<T, E> pattern instead of using Go's (T, error)

**Phase to address:**
**Phase 0:** Go best practices training and concurrency sandbox testing before API development.

---

### Pitfall 5: Throwing Away Institutional Knowledge

**What goes wrong:**
The Rust codebase contains years of learned behavior: "this endpoint sleeps 200ms because X API is rate-limited," "this WHERE clause exists because Y table has duplicates," "this retry logic handles Z vendor's flaky service." None of this is documented. The Go rewrite removes these "unnecessary" pieces. Production breaks in subtle ways.

**Why it happens:**
Working code looks arbitrary when you don't know its history. Developers assume old code is "just bad" rather than "solving a real problem." Comments don't explain WHY things exist, only WHAT they do.

**How to avoid:**
1. **Archaeology phase** — Interview original Rust developers, extract tribal knowledge
2. **Annotated Rust code** — Add inline comments explaining WHY each quirk exists before rewriting
3. **Behavioral tests for quirks** — "This endpoint retries 3x with exponential backoff" becomes a test
4. **Knowledge transfer sessions** — Rust developers pair with Go developers during rewrite

**Warning signs:**
- "I don't know why this code exists, let's remove it"
- Simplifying logic without understanding its purpose
- No documentation of removed features
- Assumption that "cleaner" means "better"

**Phase to address:**
**Phase 0 (Pre-Development):** Knowledge extraction and documentation before any Go code is written.

---

## CRITICAL PITFALLS — DATABASE MIGRATION

### Pitfall 6: Data Loss During 83→9 Table Consolidation

**What goes wrong:**
You migrate 83 tables into 9 polymorphic tables with `kind` fields and JSONB content. 83% of database migrations fail. You lose data during consolidation because: foreign keys can't exist across polymorphic associations, orphaned records slip through, JSONB fields are misspelled and silently drop data, or row counts don't match but nobody notices until production.

**Why it happens:**
No foreign key enforcement on polymorphic associations—database can't validate integrity. JSONB is flexible which means typos become data loss. Reconciliation testing skipped because "the migration script ran successfully."

**How to avoid:**
1. **Pre-migration snapshot** — Full database dump with checksums before any migration
2. **Row count reconciliation** — Every old table → new table mapping verified with exact counts
3. **Checksum validation** — Hash critical columns, compare source vs. destination
4. **Dry-run migrations** — Test migration on production snapshot, verify no data loss
5. **Rollback plan** — Tested procedure to restore from backup within 1 hour

**Warning signs:**
- Migration script shows "X rows migrated" without verification
- Different row counts between old and new schema ("probably duplicates")
- Missing NULL checks in JSONB field extraction
- No automated reconciliation tests

**Phase to address:**
**Phase 1 (Schema Design):** Define reconciliation tests BEFORE writing migration scripts. Every migration script must produce a reconciliation report.

---

### Pitfall 7: JSONB Performance Cliff (PostgreSQL Query Optimizer Blindness)

**What goes wrong:**
You collapse 83 tables into 9 polymorphic tables using JSONB for flexibility. Queries that were fast on structured columns (indexed, with statistics) become 2000x slower because PostgreSQL can't maintain statistics on JSONB values. The query planner relies on hardcoded estimates. Production grinds to a halt. One team reported queries degrading from 0.3s to 584s.

**Why it happens:**
PostgreSQL doesn't know field value distributions inside JSONB, can't optimize joins, and JIT compilation thinks JSONB queries are "massive enough for JITing" spending 150-1500ms of overhead. GIN indexes help but don't solve the statistics problem. Updating large JSONB documents is inefficient—even small changes require copying the entire value if it exceeds 2KB (TOAST threshold).

**How to avoid:**
1. **Hybrid schema** — Use traditional columns for frequently-queried fields, JSONB for variable data
2. **Denormalize hot paths** — Pull critical JSONB fields into indexed columns (e.g., agent.name, task.status)
3. **Query budget** — Test every common query against production-size data, set max latency thresholds
4. **GIN index strategy** — Index specific JSONB paths, not entire documents
5. **Size limits** — Keep JSONB documents under 2KB to avoid TOAST performance penalty

**Warning signs:**
- "JSONB is flexible, we'll figure out indexing later"
- Queries that worked on 1000 rows fail at 10,000 rows
- EXPLAIN ANALYZE shows sequential scans on large JSONB columns
- Update latency grows with JSONB document size

**Phase to address:**
**Phase 1 (Schema Design):** Identify high-frequency queries, benchmark against production data, establish hybrid column strategy before implementing polymorphic schema.

---

### Pitfall 8: Losing Foreign Key Integrity (Polymorphic Association Trap)

**What goes wrong:**
83-table schema had foreign keys enforcing referential integrity: `tasks.agent_id REFERENCES agents(id)`. Nine-table polymorphic schema can't do this—`the_work.entity_id` could point to ANY table based on `entity_kind` string. Database can't enforce foreign keys across polymorphic associations. "Wrong" kind of object gets saved. Orphaned records accumulate. Data integrity degrades silently.

**Why it happens:**
GitLab engineering documentation explicitly warns: "Always use separate tables instead of polymorphic associations" because databases can't enforce referential integrity on polymorphic references. Rails/Django polymorphic features trade data integrity for flexibility.

**How to avoid:**
1. **Application-level validation** — Check referential integrity in code before every insert
2. **Batch integrity audits** — Cron job verifies all `entity_id` references exist in target tables
3. **Separate junction tables** — Use explicit many-to-many tables for relationships instead of polymorphic references
4. **Consider separate tables** — For core relationships (agent↔task), use dedicated tables with real foreign keys

**Warning signs:**
- "We'll validate in the application layer" without written validation rules
- No automated integrity checks
- Polymorphic references used for critical relationships
- Missing backfill scripts for invalid references

**Phase to address:**
**Phase 1 (Schema Design):** Define which relationships MUST have database-level integrity (use separate tables) vs. which can be polymorphic (use application validation).

---

### Pitfall 9: Migration Without Rollback (One-Way Door)

**What goes wrong:**
You migrate 83 tables → 9 tables, discover critical data loss or performance issues in production, and have no way to roll back. The migration is one-way—you can't reconstruct structured columns from JSONB. Business halts while you debug. Team works 72-hour shifts trying to recover.

**Why it happens:**
Rollback procedures are "nice to have" that get skipped under deadline pressure. Teams test the forward migration but not the reverse. Database backups exist but restoring loses hours of production data.

**How to avoid:**
1. **Bidirectional migration scripts** — Forward (83→9) and reverse (9→83) both tested
2. **Shadow writing period** — Write to both old and new schemas simultaneously, verify parity
3. **Gradual cutover** — Read-only on new schema (week 1), then writes (week 2), then retire old (week 4)
4. **Documented rollback procedure** — Written steps, tested in staging, timed for execution speed

**Warning signs:**
- "We can restore from backup if needed" without testing restore
- No reverse migration script
- All-or-nothing cutover ("flip the switch")
- Rollback requires manual data manipulation

**Phase to address:**
**Phase 2 (Migration Implementation):** Write and test reverse migration BEFORE running forward migration. Shadow-write period is non-negotiable.

---

### Pitfall 10: Ignoring the 9,846 Conversations + 2,554 Tasks (Production Data Scale)

**What goes wrong:**
Migration scripts tested against 100 sample rows run successfully in 2 seconds. Production has 9,846 conversations + 2,554 tasks across 83 tables. Migration takes 4 hours. Locks the database. Times out. Partial data migrated. Application crashes because schema is half-old, half-new.

**Why it happens:**
Testing against unrealistic data volumes. Development database has 100 rows, production has 10,000. Migration locks tables, blocking all queries. Long-running transactions hit timeouts. Network failures during multi-hour migrations corrupt state.

**How to avoid:**
1. **Test against production snapshot** — Copy production database, run migration locally
2. **Batch migration strategy** — Migrate 1000 rows at a time, commit between batches
3. **Online migration tools** — Use tools that allow queries during migration (pg_repack, gh-ost)
4. **Downtime window** — If migration requires locks, schedule maintenance window
5. **Progress monitoring** — Migration script reports percentage complete, ETA, row counts

**Warning signs:**
- "Works on my machine" with 10 test rows
- No estimation of production migration time
- Single transaction for entire migration
- No progress indicators or logging during migration

**Phase to address:**
**Phase 2 (Migration Implementation):** Load testing with production-scale data before any production migration attempt.

---

## CRITICAL PITFALLS — COMPOUND (DUAL MIGRATION)

### Pitfall 11: Blame Ambiguity (Which Migration Broke It?)

**What goes wrong:**
Production breaks. Is it the Go rewrite? The schema migration? Both? Debugging becomes exponential—every bug could be caused by language differences, schema differences, or their interaction. Team spends weeks isolating root cause instead of fixing bugs.

**Why it happens:**
Two simultaneous changes with overlapping failure modes. Rust→Go error handling differences look like data integrity issues. Schema migration bugs look like Go serialization problems. No isolation between changes.

**How to avoid:**
1. **Sequential migration** — Finish Go rewrite with old schema FIRST, then migrate schema
2. **Compatibility layer** — Go reads from 83-table schema initially, schema migration happens separately
3. **Isolated testing** — Test Go against old schema, test new schema with Rust, before combining
4. **Change freeze** — Never deploy Go rewrite and schema migration in same release

**Warning signs:**
- "We'll do both at once to save time"
- Production bugs with unclear root cause
- Team arguing whether issue is code or data
- Inability to isolate variables

**Phase to address:**
**Phase -1 (Architecture Decision):** Decide migration sequence (Go-first or schema-first) before development begins. Explicitly forbid simultaneous deployment.

---

### Pitfall 12: Generalization Without Understanding Current State

**What goes wrong:**
Modular Fortress spec says: "The generalization requirement—produce a binary that knows none of Nathan's specific infrastructure." You remove Nathan-specific assumptions without documenting what they are. The Go rewrite breaks because it's missing implicit dependencies: SSH tunnel configuration, pm2 setup, specific file paths, droplet-specific environment variables, hardcoded database credentials (`chronicle:chronicle2026`).

**Why it happens:**
"Nathan-specific" infrastructure is invisible—it just works. Developers generalize without mapping current dependencies. Fresh droplet install fails because 20 implicit assumptions weren't documented.

**How to avoid:**
1. **Dependency audit** — Map every environment variable, file path, external service, SSH tunnel
2. **Fresh droplet test early** — Attempt install on clean system in Phase 1, document every failure
3. **Configuration extraction** — Move all Nathan-specific values to config file, document defaults
4. **"Works nowhere" milestone** — System deliberately fails on fresh install until all dependencies are explicit

**Warning signs:**
- "We'll generalize as we go"
- Hardcoded paths in Go rewrite
- Fresh droplet test delayed to "later"
- Assumptions that "everyone has X"

**Phase to address:**
**Phase 0 (Pre-Development):** Nathan-specific dependency audit with documentation before any generalization code is written.

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Skip rollback scripts | Ship migration faster | Cannot recover from failures, 72-hour emergency fixes | Never—rollback is non-negotiable |
| Use JSONB for everything | Flexible schema, easy to add fields | 2000x query slowdown, no statistics, index hell | Only for truly variable data, not core fields |
| Port Rust patterns to Go | Familiar code structure | Unidiomatic Go, goroutine leaks, missed error handling | Never—Go wants different patterns |
| Skip behavioral tests | Faster rewrite | Re-introduce bugs the old system solved | Never—tests capture institutional knowledge |
| No shadow-write period | Faster cutover | Blind faith migration worked correctly | Never—verification is mandatory |
| Parallel operation "temporarily" | Lower risk migration | Maintain two systems forever, double cost | Only with hard cutover date and automated criteria |

---

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| PostgreSQL JSONB | Using JSONB for all fields "for flexibility" | Hybrid: structured columns for queries, JSONB for truly variable data |
| Go error handling | Discarding errors with `_ = someFunc()` | Always check errors: `if err != nil { return err }` |
| Goroutine management | `go someFunc()` with no coordination | Use sync.WaitGroup or errgroup for all goroutines |
| Schema migration | All-or-nothing cutover | Shadow-write period → gradual rollout → verified cutover |
| Lisp ↔ Go communication | Assuming Rust HTTP client behavior matches Go | Test Common Lisp → Go HTTP integration explicitly |

---

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| JSONB sequential scans | Queries take 2000x longer than structured columns | Hybrid schema: index hot fields as columns | At 10K+ rows with complex queries |
| TOAST overhead | Small JSONB updates take 100ms+ | Keep JSONB documents under 2KB | When JSONB exceeds 2KB (TOAST threshold) |
| Goroutine leaks | Memory usage grows unbounded | Use sync.WaitGroup, never bare `go` | At 7+ concurrent ghosts with tick engine |
| Connection pool exhaustion | "Too many connections" errors | Configure connection pools explicitly (10-20 connections) | At 5+ concurrent API requests |
| Migration locks | Database unresponsive during migration | Batch migrations, online migration tools | At 10K+ rows in single transaction |

---

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Hardcoded credentials in Go | `chronicle:chronicle2026` visible in source code | Environment variables only, no defaults |
| JSONB injection | User input in JSONB field names → SQL injection | Parameterized queries, validate field names |
| No authentication in rewrite | "TODO: Implement auth" comment remains | Auth implementation is Phase 0 requirement |
| Secrets in config.json | API keys committed to repository | `.gitignore` for config.json, rotate all keys |
| Running two systems with different auth | Security policy divergence | Shared authentication layer, single source of truth |

---

## "Looks Done But Isn't" Checklist

- [ ] **Go API parity:** Automated comparison tests verify identical responses to Rust (not "similar")
- [ ] **Schema migration:** Row count reconciliation for ALL 83→9 table mappings with checksums
- [ ] **Rollback procedure:** Reverse migration tested in staging and timed (not just "we have backups")
- [ ] **Performance testing:** Production-scale load testing (9846 conversations, 2554 tasks) before cutover
- [ ] **Error handling:** Every Go function that returns `error` is checked (no `_` discard)
- [ ] **Foreign key integrity:** Application-level validation for ALL polymorphic references + batch audit cron
- [ ] **Fresh droplet test:** Install script succeeds on clean system with zero Nathan-specific assumptions
- [ ] **Cutover criteria:** Automated gates (error rate, latency, feature parity) define when to switch

---

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Data loss during migration | HIGH (72-hour emergency) | Restore from pre-migration snapshot, investigate loss cause, fix migration script, re-run |
| JSONB performance cliff | MEDIUM (1-week refactor) | Identify slow queries, extract hot fields to indexed columns, rebuild queries |
| Goroutine leaks | LOW (1-day hotfix) | Add sync.WaitGroup to leaking code path, deploy, monitor memory |
| Lost institutional knowledge | HIGH (weeks of debugging) | Interview Rust developers post-incident, document discovered edge cases |
| Running two systems forever | VERY HIGH (months of dual maintenance) | Set hard cutover deadline, automated traffic migration, delete Rust code |
| No rollback plan | CRITICAL (business halt) | Emergency database restore, accept data loss since last backup, postmortem |

---

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Netscape Trap | Phase 0: Rust behavioral audit | Characterization test suite passes |
| Second-System Syndrome | Every Phase: Parity gates | Automated endpoint comparison: Go === Rust |
| Running Two Systems Forever | Phase -1: Cutover date defined | Calendar date set, non-negotiable |
| Technology Stack Shift | Phase 0: Go training + sandbox | Code review checklist enforces patterns |
| Throwing Away Knowledge | Phase 0: Knowledge extraction | Documentation reviewed by Rust developers |
| Data Loss | Phase 1: Schema design with reconciliation | Checksum validation on test migration |
| JSONB Performance Cliff | Phase 1: Schema design with benchmarks | Query latency tests pass on production data |
| Losing Foreign Key Integrity | Phase 1: Schema design + validation rules | Integrity audit cron job passes |
| Migration Without Rollback | Phase 2: Reverse migration tested | Rollback procedure timed under 1 hour |
| Production Data Scale | Phase 2: Load testing | Migration completes on full snapshot |
| Blame Ambiguity | Phase -1: Sequential migration plan | Go tested on old schema first |
| Generalization Without Understanding | Phase 0: Dependency audit | Fresh droplet test passes |

---

## Sources

**Language Rewrite:**
- Joel Spolsky: "Things You Should Never Do, Part I" (2000) — Classic rewrite anti-pattern
- Medium: "We Rewrote It in Rust. Now No One Knows How It Works" (2025)
- Medium: "I Tried Rewriting Our Go Service in Rust — Here's the Truth No One Tells You" (2025)
- DaedTech: "The Myth of the Software Rewrite"
- AmazingCTO: "Software Rewrite Strategy: Why 90% Fail" (2025)

**Second-System Syndrome:**
- Fred Brooks: "The Mythical Man-Month" (original source)
- Systems Approach: "Second-System Syndrome" (December 2025)
- Albright Labs: "Avoiding Second System Syndrome in Code Rewrites" (November 2025)
- Wikipedia: "Second-system effect"

**Database Migration:**
- DoltHub: "Choosing a Database Schema for Polymorphic Data" (June 2024)
- GitLab Docs: "Polymorphic Associations" — recommends avoiding them
- Heap.io: "When To Avoid JSONB In A PostgreSQL Schema"
- pganalyze: "Postgres performance cliffs with large JSONB values and TOAST" (2025)
- BrowserStack: "A Complete Guide to Data Migration Testing"
- DataGaps: "Data Migration Testing: Challenges, Best Practices and 7 Types"

**Go Concurrency:**
- Medium: "7 Go Concurrency Mistakes That Even Experienced Devs Make" (September 2025)
- DEV Community: "5 Common Go Concurrency Mistakes That'll Trip You Up" (2025)
- JetBrains Guide: "Common Mistakes to Avoid When Handling Errors in Go"

**Parallel Operation:**
- Medium: "Parallel Run & Gradual System Migration Plan" (2025)
- ShiftAsia: "Legacy System Migration Strategies: The Complete Guide"
- Datafold: "The Anatomy of a Data Migration"

**Codebase Context:**
- `/Volumes/Elements/Modular Fortress/.planning/codebase/CONCERNS.md` (692 lines of technical debt)
- `/Volumes/Elements/Modular Fortress/.planning/PROJECT.md` (dual migration context)
- `/Volumes/Elements/Modular Fortress/Modular Fortress.md` (generalization requirement)

---

*Pitfalls research for: Modular Fortress v2.0 (Rust→Go + 83→9 Tables)*
*Researched: 2026-04-04*
*Confidence: HIGH — Based on multiple authoritative sources (Joel Spolsky, Fred Brooks, GitLab, PostgreSQL experts) + 2025 rewrite postmortems*
