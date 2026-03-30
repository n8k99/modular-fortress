# Phase 29: Orbis Foundation - Context

**Gathered:** 2026-03-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Add spatial identity fields to ghost YAML files: starting_point coordinates, ship_assignment, rpg_persona, and trust/energy thresholds for Orbis access. This is a data/config phase — no movement system, no map visualization, no exploration logic. Just the YAML foundation that v1.6+ Orbis phases will build on.

</domain>

<decisions>
## Implementation Decisions

### Coordinate system
- **D-01:** Integer x/y coordinates in `starting_point:` YAML section — simple grid positions on the Orbis map
- **D-02:** Coordinates derived from Pantheon Formation ship starting positions — each ghost's ship determines their initial location
- **D-03:** Coordinate values are static in Phase 29 — future Drunkard's Walk (v1.6 ORBIS-04) will update positions dynamically

### Ship assignments
- **D-04:** `ship_assignment:` string field in ghost YAML referencing Pantheon Formation ship/role
- **D-05:** Ship assignments come from the Pantheon Formation lore (Areas/Master Chronicle/Epics/Pantheon Formation/)
- **D-06:** Each executive ghost has a defined ship role from the existing narrative

### RPG persona fields
- **D-07:** `rpg_persona:` YAML section with:
  - `deity_codename:` — Greek deity name from existing persona files (e.g., Nova = Hermes, Eliana = Athena)
  - `ship_role:` — role aboard the Pantheon Formation vessel
  - `personality_traits:` — list of 2-3 key traits from Pantheon Formation characterization
- **D-08:** These fields are read-only metadata in Phase 29 — not used by tick engine logic yet

### Trust/energy thresholds
- **D-09:** `orbis_access:` YAML section with:
  - `energy_min:` integer — minimum energy to participate in Orbis (default: 30)
  - `trust_min:` integer — minimum trust/fitness score for Orbis access (default: 40)
- **D-10:** Thresholds are readable by tick engine at runtime but NOT enforced in Phase 29 — enforcement comes with v1.6 Drunkard's Walk
- **D-11:** Different ghosts can have different thresholds (executives might have lower bars)

### Claude's Discretion
- Exact x/y coordinate values for each ghost's starting position
- Which deity codenames map to which ghosts (some are established in persona files, others need assignment)
- Default threshold values if not already implied by lore
- Whether to add orbis fields to all 9 existing YAML files or only executives

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Ghost YAML files (Phase 28)
- `/opt/project-noosphere-ghosts/config/agents/*.yaml` — 9 existing agent YAML files with responsibilities sections
- `/opt/project-noosphere-ghosts/lisp/util/yaml.lisp` — YAML parser (parse-simple-yaml, serialize-simple-yaml)
- `/opt/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp` — YAML load/write functions

### Pantheon Formation lore
- `Areas/Master Chronicle/Epics/Pantheon Formation/Pantheon Formation.md` — Overview document
- `Areas/Master Chronicle/Epics/Pantheon Formation/Scenes/Captain Eliana Riviera — The Castaway.md` — Eliana's ship role
- Persona files at `/root/gotcha-workspace/context/personas/` — deity codenames in existing personas

### Requirements
- `.planning/REQUIREMENTS.md` — ORBIS-01, ORBIS-02, ORBIS-03

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `yaml.lisp` — parse-simple-yaml and serialize-simple-yaml handle the YAML subset needed (key-value + string lists)
- `ghost-capabilities.lisp` — write-ghost-yaml does atomic file writes; can be reused or extended for new YAML sections
- 9 agent YAML files already exist — just need new sections added

### Established Patterns
- YAML files use double-quoted strings for special characters
- serialize-simple-yaml handles nested sections

### Integration Points
- Existing YAML files need new sections added without disrupting responsibilities
- yaml.lisp parser must handle nested key-value sections (starting_point: x: 100, y: 200)
- ghost-capabilities.lisp load functions may need to be aware of new sections

</code_context>

<specifics>
## Specific Ideas

- The Pantheon Formation is a rich narrative world with established character relationships — the YAML fields should reflect what's already written, not invent new lore
- 8 executives are the primary targets; EthanNg (staff) also has a YAML file from Phase 28
- The coordinate system should be simple enough that Drunkard's Walk can just increment/decrement x/y values

</specifics>

<deferred>
## Deferred Ideas

- Drunkard's Walk movement per tick — v1.6 ORBIS-04
- Ghost encounters with world objects — v1.6 ORBIS-05
- Movement path visualization on map — ~v1.9 ORBIS-06
- Orbis threshold enforcement in tick engine — v1.6 when movement system exists

</deferred>

---

*Phase: 29-orbis-foundation*
*Context gathered: 2026-03-30*
