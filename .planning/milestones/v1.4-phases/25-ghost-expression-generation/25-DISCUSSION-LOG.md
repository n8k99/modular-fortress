# Phase 25: Ghost Expression Generation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md -- this log preserves the alternatives considered.

**Date:** 2026-03-30
**Phase:** 25-ghost-expression-generation
**Areas discussed:** Expression Syntax Generation, Template CRUD from Lisp, Validation Strategy, LLM Output Parsing

---

## Expression Syntax Generation

| Option | Description | Selected |
|--------|-------------|----------|
| String concatenation with builder functions | Lisp helpers that produce valid syntax strings from validated parts | ✓ |
| AST-to-string serializer | Build a full serializer that converts node structs back to Innate syntax | |
| Raw LLM output only | Let the LLM produce expression strings with no Lisp-side helpers | |

**User's choice:** [auto] String concatenation with builder functions (recommended default)
**Notes:** Innate syntax is simple enough that string builders work. Full AST serializer would be over-engineering. LLM can also emit raw syntax for natural language tasks.

---

## Template CRUD from Lisp

| Option | Description | Selected |
|--------|-------------|----------|
| Direct SQL INSERT/UPDATE via db-client | Follow Phase 21/22 convention, use db-execute | ✓ |
| HTTP POST/PUT to dpn-api | Use existing CRUD endpoints in templates.rs | |

**User's choice:** [auto] Direct SQL INSERT/UPDATE via db-client (recommended default)
**Notes:** Consistent with the entire v1.4 milestone theme of direct PostgreSQL access from Lisp.

---

## Validation Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Parse-round-trip validation | tokenize + parse the expression; if no error, it's valid | ✓ |
| Regex pattern matching | Validate against known syntax patterns | |
| No validation (trust LLM) | Persist whatever the LLM produces | |

**User's choice:** [auto] Parse-round-trip validation (recommended default)
**Notes:** The parser is the authoritative validator (175/176 tests). Regex would be fragile. Trusting LLM output blindly risks invalid templates that break downstream evaluation.

---

## LLM Output Parsing

| Option | Description | Selected |
|--------|-------------|----------|
| Structured JSON field in LLM response | innate_expressions array in JSON output | ✓ |
| Special text delimiters in LLM response | Parse between markers like ```innate ... ``` | |
| Separate LLM call for expression generation | Two-pass: plan, then generate | |

**User's choice:** [auto] Structured JSON field in LLM response (recommended default)
**Notes:** Follows existing --output-format json convention. Action-executor already parses JSON. Adding a field is minimal.

---

## Claude's Discretion

- Exact builder function signatures and file organization
- Exact JSON schema for innate_expressions field
- Tool registry vs inline action-executor handling
- System prompt wording for template generation tasks
- Ghost-generated template categorization

## Deferred Ideas

- Template versioning UI -- future frontend milestone
- Template sharing/access control -- future milestone
- Innate language v2 features -- out of scope
