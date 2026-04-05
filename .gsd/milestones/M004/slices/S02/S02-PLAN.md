# S02: Bind Dragonpunk DB to Frontend

**Goal:** Expose existing Dragonpunk database functions to the TypeScript frontend via Wails bindings. The window displays live data from master_chronicle — table names, row counts, kind breakdowns.
**Demo:** After this: Window displays live table names, row counts, and kind breakdowns from master_chronicle

## Tasks
- [x] **T01: Created DbService with Health/ListTables/ListKinds bindings wrapping existing Dragonpunk db packages** — Create a new Go service struct that wraps the existing dragonpunk database code for Wails binding. The service needs to: (1) load config from .env using existing dragonpunk/internal/config, (2) connect to PostgreSQL using existing dragonpunk/internal/db, (3) expose methods callable from TypeScript: Health() returns db status, ListTables() returns table names with row counts, ListKinds(table) returns kind breakdown for a table. Import the existing packages from dragonpunk/internal/ — do NOT duplicate the database code.
  - Estimate: 30min
  - Files: dragonpunk-app/dbservice.go, dragonpunk-app/main.go, dragonpunk-app/go.mod
  - Verify: go build compiles with no errors, binding generation succeeds
- [x] **T02: Frontend wired to DbService bindings — displays health status, table list, and interactive kind breakdowns** — Update the TypeScript frontend to call DbService bindings and render the results. On load: call Health() and display connection status. Then call ListTables() and render a table list with row counts. Each table name is clickable — clicking calls ListKinds(table) and shows the kind breakdown below.
  - Estimate: 20min
  - Files: dragonpunk-app/frontend/src/main.ts, dragonpunk-app/frontend/public/style.css
  - Verify: wails3 build succeeds, app shows live table data from master_chronicle
- [x] **T03: End-to-end verified: Wails app connects to master_chronicle, displays 14 tables with live data through Go↔TypeScript binding** — Build the full app with wails3 build. Launch it. Verify the window shows live data from master_chronicle — table names, row counts, kind breakdowns. Confirm the vault-map aesthetic looks correct with real data. Check slog output for database query logging.
  - Estimate: 10min
  - Verify: App window displays all Nine Tables with correct row counts from master_chronicle
