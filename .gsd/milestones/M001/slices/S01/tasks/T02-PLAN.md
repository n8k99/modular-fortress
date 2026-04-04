---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T02: Config loading from .env

Implement config package that loads DATABASE_URL, HOST (default 0.0.0.0), PORT (default 8888) from .env file at repo root. Use godotenv for .env parsing. Config struct with Validate() method.

## Inputs

- `T01 output`
- `.env file`

## Expected Output

- `dragonpunk/internal/config/config.go with tests`

## Verification

go test ./internal/config/... passes
