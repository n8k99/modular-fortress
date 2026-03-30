# Phase 30: Team Pipelines - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions captured in CONTEXT.md — this log preserves the analysis.

**Date:** 2026-03-30
**Phase:** 30-team-pipelines
**Mode:** discuss (interactive)
**Areas analyzed:** Pipeline Location, Fork/Join Support, Stage Name Disambiguation

## Gray Areas Presented

### 1. Pipeline YAML Location
- **Claude's assumption:** Separate pipeline YAML files at `config/pipelines/`
- **Options presented:** (A) Separate pipeline files, (B) Department YAML files, (C) Embed in ghost YAML
- **User correction:** None of the above — pipelines live in the DB, not filesystem. The noosphere is the substrate. Department pipelines stored as area_content entries (Assignments.md per department). Project-specific pipelines as InnateScipt on the project itself.

### 2. Fork/Join Support
- **Claude's assumption:** Forks only, defer joins
- **User response:** Confirmed. Forks yes, joins deferred.

### 3. Stage Name Disambiguation
- **Claude's assumption:** Needed explicit decision
- **User response:** Pipeline-scoped names are natural since pipelines live on projects/departments. `(pipeline, stage)` tuple.

## Key User Input (Verbatim Themes)

- "I want to avoid creating files outside of the DB so that I can more easily look at edit configure them"
- "The Eckenrode Muziekopname table should have virtual directories for each Department/Team which has an entry (we can call it Assignments.md because my brain is still thinking in Obsidian vault metaphors)"
- "The whole idea all along has been to recursively build the system by building the system. Ghosts are in the Noosphere, not just as an entry but their instructions, their backstories, their behaviors. The noosphere is the substrate and all things pull from it."
- "I acknowledge that there are outside tools which will be required to be coded into either their own directory or rolled into DPN or Noosphere, in order that the InnateScipt has hands to operate their tasks."

## Corrections Applied

| Area | Original Assumption | User's Direction |
|------|---------------------|-----------------|
| Pipeline location | Filesystem YAML files | DB-native in master_chronicle (area_content + project metadata) |
| Assignments format | Structured YAML | Markdown with embedded InnateScipt (Obsidian vault metaphor) |
| Project hierarchy | Implicit | Expressed through noosphere ownership + department delegation |
