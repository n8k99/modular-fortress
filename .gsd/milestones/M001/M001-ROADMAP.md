# M001: Dragonpunk Bootstrap — Health + DB Connectivity

## Vision
Stand up the Dragonpunk Go server connecting to Nathan's master_chronicle PostgreSQL. First code in the Go pillar that will replace all Rust. Binary starts, loads .env, connects to the existing database, and serves a health endpoint with live DB status.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Go Membrane Scaffold + Health Endpoint | low — standard go http server setup | — | ✅ | go run ./membrane serves GET /api/health returning {"status":"ok","db_connected":true} |
| S02 | Nine Tables Schema Validation + Migration Runner | medium — existing sql may have hidden dependencies | S01 | ✅ | Schema SQL scripts run on fresh empty PostgreSQL database; Go binary confirms all 12 tables exist |
