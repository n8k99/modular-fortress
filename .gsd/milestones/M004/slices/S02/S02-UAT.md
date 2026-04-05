# S02: Bind Dragonpunk DB to Frontend — UAT

**Milestone:** M004
**Written:** 2026-04-05T06:06:34.486Z

## UAT: Bind Dragonpunk DB to Frontend\n\n1. Ensure PostgreSQL is running with master_chronicle database\n2. Run `wails3 build` in dragonpunk-app/ — completes without errors\n3. Run `bin/dragonpunk-app` — native window opens\n4. Window shows 'master_chronicle connected — 14 tables'\n5. Table list displays all Nine Tables with row counts\n6. Click any table — kind breakdown appears below\n7. slog output confirms DB queries on each interaction\n8. Close window — app exits cleanly, slog shows pool closed
