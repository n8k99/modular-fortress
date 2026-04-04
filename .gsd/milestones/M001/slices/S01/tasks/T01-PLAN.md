---
estimated_steps: 1
estimated_files: 4
skills_used: []
---

# T01: Init dragonpunk/ Go module and project structure

Create dragonpunk/ directory with go.mod (module github.com/n8k99/modular-fortress/dragonpunk), main.go entry point, and internal package layout. Decide on router (chi vs stdlib) and DB driver (pgx). Create .env at repo root with DATABASE_URL, HOST, PORT defaults.

## Inputs

- `CLAUDE.md constraints`
- `Existing .env conventions from Rust codebase`

## Expected Output

- `dragonpunk/go.mod`
- `dragonpunk/main.go`
- `dragonpunk/internal/config/config.go`
- `.env`

## Verification

cd dragonpunk && go build ./... exits 0
