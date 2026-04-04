# S01: Go Membrane Scaffold + Health Endpoint

**Goal:** Minimal Go HTTP server with .env config, PostgreSQL connectivity to master_chronicle, and health endpoint
**Demo:** After this: go run ./membrane serves GET /api/health returning {"status":"ok","db_connected":true}

## Tasks
- [x] **T01: Initialized dragonpunk/ Go module with project structure, .env, and all source files** — Create dragonpunk/ directory with go.mod (module github.com/n8k99/modular-fortress/dragonpunk), main.go entry point, and internal package layout. Decide on router (chi vs stdlib) and DB driver (pgx). Create .env at repo root with DATABASE_URL, HOST, PORT defaults.
  - Estimate: 15min
  - Files: dragonpunk/go.mod, dragonpunk/main.go, dragonpunk/internal/config/config.go, .env
  - Verify: cd dragonpunk && go build ./... exits 0
- [x] **T02: Config loading from .env implemented with defaults and validation** — Implement config package that loads DATABASE_URL, HOST (default 0.0.0.0), PORT (default 8888) from .env file at repo root. Use godotenv for .env parsing. Config struct with Validate() method.
  - Estimate: 10min
  - Files: dragonpunk/internal/config/config.go, dragonpunk/go.mod
  - Verify: go test ./internal/config/... passes
- [x] **T03: PostgreSQL connection pool via pgx with ping verification** — Create db package with NewPool(databaseURL) returning *pgxpool.Pool. Ping on creation to verify connectivity. Connection pool size 10 to match existing config.json convention.
  - Estimate: 10min
  - Files: dragonpunk/internal/db/db.go, dragonpunk/go.mod
  - Verify: go test ./internal/db/... passes (requires live DB)
- [x] **T04: /api/health endpoint returns live DB status with table count, graceful shutdown on SIGINT** — Create /api/health handler that queries SELECT count(*) FROM information_schema.tables WHERE table_schema='public' and returns JSON: {status, db_connected, table_count, timestamp}. Wire up chi or stdlib router with structured request logging middleware. Graceful shutdown on SIGINT/SIGTERM.
  - Estimate: 15min
  - Files: dragonpunk/internal/api/health.go, dragonpunk/internal/api/router.go, dragonpunk/main.go
  - Verify: go run ./dragonpunk, then curl localhost:PORT/api/health returns valid JSON with db_connected:true and table_count:85
- [x] **T05: README with quickstart and integration verified end-to-end** — Full end-to-end test: build binary, start it, curl /api/health, verify JSON response. Add brief README.md or comment header in main.go with quickstart instructions.
  - Estimate: 10min
  - Files: dragonpunk/README.md
  - Verify: go build -o dragonpunk/bin/dragonpunk ./dragonpunk && ./dragonpunk/bin/dragonpunk & sleep 1 && curl -s localhost:PORT/api/health | jq . && kill %1
