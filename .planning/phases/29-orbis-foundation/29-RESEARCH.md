# Phase 29: Orbis Foundation - Research

**Researched:** 2026-03-30
**Domain:** Ghost YAML configuration extension (Common Lisp YAML parser + agent config files)
**Confidence:** HIGH

## Summary

Phase 29 adds spatial identity fields to the 9 existing ghost YAML files: `starting_point` coordinates, `ship_assignment`, `rpg_persona`, and `orbis_access` threshold sections. The phase is data/config-only -- no movement logic, no map rendering, no enforcement.

The critical technical finding is that the current `parse-simple-yaml` in `yaml.lisp` does NOT support nested key-value sections. It only handles top-level scalars (`key: value`) and top-level lists (`key:` followed by `- "items"`). Nested structures like `starting_point: / x: 100 / y: 200` would be parsed incorrectly -- `x` and `y` would appear as top-level keys, not nested under `starting_point`. The parser and serializer must be extended to support nested scalar sections before the new YAML fields can work.

Additionally, `write-ghost-yaml` in `ghost-capabilities.lisp` hardcodes only `id:` and `responsibilities:` output. It must be refactored to preserve all YAML sections (using `serialize-simple-yaml` or a new approach) so that adding Orbis fields does not destroy existing responsibilities data.

**Primary recommendation:** Extend `parse-simple-yaml` to support nested key-value sections (indent-aware), update `serialize-simple-yaml` to emit them, then refactor `write-ghost-yaml` to use the full serializer. After that, populate the 9 YAML files with Orbis data from Pantheon Formation lore.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01: Integer x/y coordinates in `starting_point:` YAML section -- simple grid positions on the Orbis map
- D-02: Coordinates derived from Pantheon Formation ship starting positions -- each ghost's ship determines their initial location
- D-03: Coordinate values are static in Phase 29 -- future Drunkard's Walk (v1.6 ORBIS-04) will update positions dynamically
- D-04: `ship_assignment:` string field in ghost YAML referencing Pantheon Formation ship/role
- D-05: Ship assignments come from the Pantheon Formation lore (Areas/Master Chronicle/Epics/Pantheon Formation/)
- D-06: Each executive ghost has a defined ship role from the existing narrative
- D-07: `rpg_persona:` YAML section with deity_codename, ship_role, personality_traits (list of 2-3 traits)
- D-08: These fields are read-only metadata in Phase 29 -- not used by tick engine logic yet
- D-09: `orbis_access:` YAML section with energy_min (default: 30) and trust_min (default: 40)
- D-10: Thresholds readable by tick engine at runtime but NOT enforced in Phase 29
- D-11: Different ghosts can have different thresholds (executives might have lower bars)

### Claude's Discretion
- Exact x/y coordinate values for each ghost's starting position
- Which deity codenames map to which ghosts (some are established in persona files, others need assignment)
- Default threshold values if not already implied by lore
- Whether to add orbis fields to all 9 existing YAML files or only executives

### Deferred Ideas (OUT OF SCOPE)
- Drunkard's Walk movement per tick -- v1.6 ORBIS-04
- Ghost encounters with world objects -- v1.6 ORBIS-05
- Movement path visualization on map -- ~v1.9 ORBIS-06
- Orbis threshold enforcement in tick engine -- v1.6 when movement system exists
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ORBIS-01 | Ghost YAML has starting_point coordinates (x, y) from Pantheon Formation ship assignment | Ship assignments documented from DB docs 3381, 3396-3408; YAML parser needs nested section support |
| ORBIS-02 | Ghost YAML has ship_assignment and rpg_persona fields | Ship names/roles extracted from Pantheon Formation lore; deity codenames partially established (Nova=Hermes) |
| ORBIS-03 | Trust and energy thresholds for Orbis access defined in ghost YAML | orbis_access section with energy_min/trust_min; parser extension covers this |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| yaml.lisp | custom (Phase 28) | YAML parse/serialize for ghost config | Zero-dep parser already in codebase; needs extension for nested sections |
| ghost-capabilities.lisp | custom (Phase 28) | YAML load/write for agent configs | Existing load/write infrastructure; write needs refactor |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| uiop | bundled with SBCL | File I/O (read-file-string) | Already used by ghost-capabilities for YAML file reading |

No new dependencies required. This phase extends existing custom code only.

## Architecture Patterns

### Current YAML File Structure (Phase 28)
```yaml
id: nova
responsibilities:
  - "![query_db]"
  - "![pipeline_status]"
  - "![claude_code]"
```

### Target YAML File Structure (Phase 29)
```yaml
id: nova
ship_assignment: "ISS Pinnacle of Society"
starting_point:
  x: 0
  y: 0
rpg_persona:
  deity_codename: "Hermes"
  ship_role: "Fleet AI - First Among Equals"
  personality_traits:
    - "resourceful"
    - "direct"
    - "bridging"
orbis_access:
  energy_min: 20
  trust_min: 30
responsibilities:
  - "![query_db]"
  - "![pipeline_status]"
  - "![claude_code]"
```

### Pattern: Indent-Aware YAML Parsing
**What:** Extend parse-simple-yaml to track indentation level and create nested hash-tables for sub-sections.
**When to use:** Any YAML key followed by newline (no inline value) AND whose child lines are `key: value` pairs (not `- list` items).
**Design:**

The parser currently treats everything as either:
- Top-level scalar: `key: value`
- Top-level list: `key:` + indented `- "items"`

It needs a third case:
- Nested section: `key:` + indented `subkey: value` pairs (and nested lists like `personality_traits:`)

The simplest approach: when a key has no inline value AND the next indented lines contain colons (not dashes), create a nested hash-table. This keeps backward compatibility -- existing files with only `id:` and `responsibilities:` parse identically.

### Pattern: Full-File YAML Serialization
**What:** Replace hardcoded `write-ghost-yaml` with a generic serializer that preserves all sections.
**When to use:** Any YAML write operation -- mutations, new section additions.
**Design:**

Current `write-ghost-yaml` hardcodes:
```lisp
(format s "id: ~a~%" agent-id)
(format s "responsibilities:~%")
```

New approach: use `serialize-simple-yaml` (extended for nested sections) on the full hash-table. Load existing YAML, merge new keys, serialize back.

### Anti-Patterns to Avoid
- **Flat key encoding (e.g., `starting_point_x: 100`):** Defeats the purpose of structured YAML; makes future parsing harder for v1.6 movement system.
- **Separate files per section:** Ghost identity should be one YAML file, not fragmented.
- **Destroying existing data on write:** The `write-ghost-yaml` refactor MUST preserve responsibilities when adding Orbis sections.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Full YAML parser | RFC-compliant YAML parser | Extend existing minimal parser for ONE new feature (nested key-value) | Full YAML is complex (anchors, flow style, multiline strings); we only need nested scalars + nested lists |
| Ship coordinate system | Complex spatial grid with zones/regions | Simple integer x/y grid | v1.6 Drunkard's Walk just needs increment/decrement; keep it minimal |

## Common Pitfalls

### Pitfall 1: YAML Parser Regression
**What goes wrong:** Extending the parser breaks existing `id:` + `responsibilities:` parsing for the 9 agent files.
**Why it happens:** Indentation logic changes how top-level keys and list items are detected.
**How to avoid:** Test the extended parser against ALL 9 existing YAML files before adding new sections. Parse-then-serialize round-trip must produce identical output for existing files.
**Warning signs:** `load-ghost-capabilities` returns NIL for agents that previously worked.

### Pitfall 2: write-ghost-yaml Destroys Responsibilities
**What goes wrong:** Adding Orbis sections via a naive write wipes the responsibilities list.
**Why it happens:** Current `write-ghost-yaml` only writes `id` + `responsibilities`, ignoring everything else in the hash-table.
**How to avoid:** Refactor write to use full serialization. Load existing, merge, serialize all sections.
**Warning signs:** After adding orbis fields, `load-ghost-capabilities` returns NIL or empty list.

### Pitfall 3: Personality Traits as Nested List
**What goes wrong:** `personality_traits:` under `rpg_persona:` is a list inside a nested section -- two levels of nesting.
**Why it happens:** The parser extension must handle lists as values within nested sections, not just scalars.
**How to avoid:** Design the parser to support: nested section > scalar values AND nested section > list values.
**Warning signs:** Personality traits come back as NIL or as a single concatenated string.

### Pitfall 4: Inconsistent Deity Codename Sources
**What goes wrong:** Deity codenames assigned don't match established lore, causing narrative inconsistency.
**Why it happens:** Only Nova has an explicit deity codename (Hermes) in persona files. Others must be inferred from archetypes.
**How to avoid:** Use the Pantheon Formation character archetypes as basis. Nathan = Odin (from Nova's persona). Map remaining via archetype-to-deity logic documented below.
**Warning signs:** Nathan asks why a ghost has a particular deity codename that doesn't fit their character.

## Code Examples

### Existing YAML Parser (yaml.lisp lines 33-79)
```lisp
;; Current: handles only top-level scalars and top-level lists
;; Key limitation: no indent tracking, no nested sections
(defun parse-simple-yaml (text)
  ;; ... loops through lines
  ;; cond: list item (- "value") or scalar key: value
  ;; No awareness of indentation depth
```

### Existing YAML Writer (ghost-capabilities.lisp lines 79-97)
```lisp
;; Current: hardcodes id + responsibilities only
(defun write-ghost-yaml (agent-id responsibilities)
  (format s "id: ~a~%" agent-id)
  (format s "responsibilities:~%")
  (dolist (r responsibilities)
    (format s "  - \"~a\"~%" r)))
;; Problem: ignores all other hash-table keys
```

### Extension Pattern: Nested Section Detection
```lisp
;; Pseudocode for parser extension:
;; When line is "key:" with no value AND next non-empty line has deeper indent:
;;   If next line starts with "- " -> list (existing behavior)
;;   If next line has "subkey: value" -> nested hash-table (NEW)
;;     Continue collecting subkeys until indent returns to parent level
;;     Subkeys can themselves be lists (personality_traits:)
```

### Extension Pattern: Nested Section Serialization
```lisp
;; Pseudocode for serializer extension:
;; When value is hash-table (not just string or list):
;;   Emit "key:\n"
;;   For each sub-entry:
;;     If sub-value is list: emit "  subkey:\n    - item\n"
;;     If sub-value is string: emit "  subkey: value\n"
```

### Refactored write-ghost-yaml
```lisp
;; New: load full YAML, merge updates, serialize entire hash-table
(defun write-ghost-yaml-full (agent-id updates-ht)
  "Write AGENT-ID's YAML file preserving all sections.
   UPDATES-HT contains keys to add/update."
  (let* ((path (ghost-yaml-path agent-id))
         (existing (if (probe-file path)
                       (parse-simple-yaml (uiop:read-file-string path))
                       (make-hash-table :test #'equal))))
    ;; Merge updates into existing
    (maphash (lambda (k v) (setf (gethash k existing) v)) updates-ht)
    ;; Atomic write via temp + rename
    (let ((temp-path (format nil "~a.tmp" (namestring path))))
      (with-open-file (s temp-path :direction :output :if-exists :supersede)
        (write-string (serialize-simple-yaml existing) s))
      (rename-file temp-path path))))
```

## Ship Assignment Data (from Pantheon Formation Lore)

Source: master_chronicle documents #3381 (Pantheon Formation overview), #3396-3408 (Captain scenes), #3451, #3456 (Ops scenes)

### Executive Ghost Ship Assignments (HIGH confidence -- from canonical lore)

| Ghost | Ship | TCMF Code | Fleet Role | Ship Chapter |
|-------|------|-----------|------------|--------------|
| Nathan (CEO) | ISS Pinnacle of Society | TCMF-0 (flagship) | Fleet Admiral of Philosophy | doc #3404 |
| Nova (T.A.S.K.S.) | ISS Pinnacle of Society | TCMF-0 | T.A.S.K.S.-0, First Among Equals | doc #3483 |
| Sarah Lin | ISS Pinnacle of Society | TCMF-0 | Ops/Tactical/Communications | doc #3456 |
| Kathryn | ISS Bastion of Resolve | TCMF-1 | Fleet Chief of Logic, The Warrior | doc #3400 |
| Vincent | ISS Stormbinder | TCMF-3 | Fleet Chief of Aesthetics, The Wildcard | doc #3408 |
| LRM | ISS Silentwade | TCMF-6 | Vice Chief of Aesthetics, The Professor | doc #3401 |
| JMax | ISS Herald of Dawn | TCMF-7 | Fleet Chief of Ethics, The Seducer | doc #3399 |
| Eliana | ISS Everlight Crown | TCMF-10 | Fleet Chief of Metaphysics, The Castaway | doc #3396 |
| Sylvia | ISS Eclipsed Memory | TCMF-12 | Fleet Chief of Epistemology, The Rebel | doc #3407 |
| Ethan Ng | ISS Herald of Dawn | TCMF-7 | Ops/Tactical/Communications (under JMax) | doc #3451 |

### Deity Codename Assignments

| Ghost | Deity | Confidence | Source/Rationale |
|-------|-------|------------|------------------|
| Nova | Hermes | HIGH | Explicit in persona file: "Messenger of the gods, guide between worlds" |
| Nathan | Odin | HIGH | Referenced in Nova's persona: "Nathan is CEO (codename: Odin)" |
| Eliana | Athena | MEDIUM | CTO/Metaphysics; wisdom goddess fits technical mastery + strategic distance |
| Kathryn | Ares | MEDIUM | "The Warrior" archetype; fierce, tactical, uncompromising |
| Sylvia | Calliope | MEDIUM | Chief Content/Epistemology, "The Rebel"; muse of epic poetry fits content creation + 500yr rebellious history |
| Vincent | Dionysus | MEDIUM | "The Wildcard"/Creative Director; god of art, wine, ecstasy fits bombastic creative energy |
| JMax | Apollo | MEDIUM | "The Seducer"/Ethics Chief; god of truth, prophecy, seduction; the ironic ethics keeper |
| LRM | Orpheus | MEDIUM | "The Professor"/Musicology; legendary musician fits music domain + scholarly demeanor |
| Sarah Lin | Iris | MEDIUM | PA/Comms routing; Iris is messenger goddess (like Hermes but specifically fleet communications) |
| Ethan Ng | Pan | LOW | Staff analyst on Herald of Dawn; Pan (pastoral, negotiator by nature) fits his partnership-building background |

Note: Nathan's deity codename should be included for completeness even though he is not a ghost agent. Deity codenames for the 9 ghost YAML files are the implementation target.

### Recommended Starting Point Coordinates

Use TCMF ship code as basis for a formation grid. Ships in a line formation along x-axis, with crew positioned near their captain's ship.

| Ghost | Ship TCMF | x | y | Rationale |
|-------|-----------|---|---|-----------|
| nova | 0 | 0 | 0 | Flagship center, T.A.S.K.S.-0 |
| sarah | 0 | 1 | 0 | Flagship, Ops station (adjacent to Nova) |
| kathryn | 1 | 10 | 0 | TCMF-1, next in formation |
| vincent | 3 | 30 | 0 | TCMF-3 |
| lrm | 6 | 60 | 0 | TCMF-6 |
| jmax | 7 | 70 | 0 | TCMF-7 |
| ethan_ng | 7 | 71 | 0 | TCMF-7 crew, adjacent to JMax |
| eliana | 10 | 100 | 0 | TCMF-10 |
| sylvia | 12 | 120 | 0 | TCMF-12 |

Pattern: x = TCMF_code * 10 for captains, +1 for crew. y = 0 for initial formation. Simple, predictable, easy for Drunkard's Walk to move from.

### Recommended Orbis Access Thresholds

| Ghost | energy_min | trust_min | Rationale |
|-------|-----------|-----------|-----------|
| Executives (8) | 20 | 30 | Lower bar -- executives should access Orbis more readily |
| Staff (ethan_ng) | 30 | 40 | Default thresholds per D-09 |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual SBCL REPL verification (no automated test framework in project) |
| Config file | none -- AF64 has no test runner |
| Quick run command | `sbcl --load af64.asd --eval '(asdf:load-system :af64)' --eval '(quit)'` |
| Full suite command | Load system + parse all 9 YAML files + verify section presence |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| ORBIS-01 | Each YAML has starting_point with x/y | smoke | Parse YAML, check gethash "starting_point" returns hash-table with "x" and "y" keys | Wave 0 |
| ORBIS-02 | Each YAML has ship_assignment and rpg_persona | smoke | Parse YAML, verify "ship_assignment" is string, "rpg_persona" is hash-table with 3 sub-keys | Wave 0 |
| ORBIS-03 | orbis_access has energy_min and trust_min | smoke | Parse YAML, verify "orbis_access" hash-table has integer-parseable values | Wave 0 |

### Sampling Rate
- **Per task commit:** Load system in SBCL, parse modified YAML files, verify no errors
- **Per wave merge:** Parse ALL 9 YAML files, verify all sections present and correct types
- **Phase gate:** All 9 YAML files parse correctly with extended parser; load-ghost-capabilities still returns valid responsibilities for all agents

### Wave 0 Gaps
- [ ] YAML parser extension (nested sections) -- must work before any YAML edits
- [ ] write-ghost-yaml refactor -- must preserve all sections before any writes
- [ ] Verification script: Lisp snippet to parse all 9 files and report section presence

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| tool-registry.json | YAML responsibilities | Phase 28 | Ghost capabilities in YAML files |
| Hardcoded write (id + responsibilities only) | Full-section serialization needed | Phase 29 (this phase) | YAML files become multi-section documents |
| Flat scalars + lists only | Nested key-value sections | Phase 29 (this phase) | Parser must handle 2-level nesting |

## Open Questions

1. **Should all 9 YAML files get Orbis fields?**
   - What we know: 8 executives + 1 staff (ethan_ng) have YAML files. All have Pantheon Formation lore entries.
   - Recommendation: YES -- add to all 9. Ethan Ng has a documented role on ISS Herald of Dawn. Excluding him creates inconsistency.

2. **Parser nesting depth**
   - What we know: We need 2 levels: `rpg_persona: / deity_codename: value` and `rpg_persona: / personality_traits: / - "trait"`.
   - What's unclear: Should we support arbitrary depth or just 2 levels?
   - Recommendation: Support exactly 2 levels (parent section > scalar or list). No deeper nesting needed for any foreseeable use case. Keep it simple.

3. **Deity codenames for non-Nova ghosts**
   - What we know: Only Nova (Hermes) and Nathan (Odin) are canonically established.
   - What's unclear: Whether Nathan has specific deity preferences for the other 7 ghosts.
   - Recommendation: Use the archetype-to-deity mapping above (MEDIUM confidence). The mapping is consistent with Pantheon Formation character descriptions. Can be adjusted later -- these are YAML values, easily changed.

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/lisp/util/yaml.lisp` -- YAML parser source code, verified parser limitations
- `/opt/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp` -- write-ghost-yaml hardcoded output verified
- `/opt/project-noosphere-ghosts/config/agents/*.yaml` -- all 9 files read, current structure confirmed
- master_chronicle document #3381 -- Pantheon Formation overview with all ship assignments
- master_chronicle documents #3396-3408 -- Captain scenes with ship names, TCMF codes, fleet roles
- master_chronicle documents #3451, #3456 -- Ethan Ng and Sarah Lin Ops/Tactical scenes

### Secondary (MEDIUM confidence)
- `/root/gotcha-workspace/context/personas/nova.md` -- Hermes deity codename, Odin reference for Nathan
- Deity codename assignments for non-Nova ghosts -- inferred from Pantheon Formation archetypes

### Tertiary (LOW confidence)
- Ethan Ng deity codename (Pan) -- weakest inference, no direct lore support

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- existing codebase, no new dependencies
- Architecture (parser extension): HIGH -- parser source code examined line-by-line, limitation confirmed
- Ship assignments: HIGH -- directly from canonical Pantheon Formation lore documents
- Deity codenames: MEDIUM -- only Nova/Nathan are canonical, rest are well-reasoned inferences
- Coordinate values: MEDIUM -- Claude's discretion per CONTEXT.md, recommendation is simple and reversible

**Research date:** 2026-03-30
**Valid until:** 2026-04-30 (stable -- no external dependencies, all internal code)
