# Dragonpunk

The Go membrane for Modular Fortress. Replaces all Rust user-facing I/O
(dpn-core, dpn-api, noosphere) as the v2.0 API server.

## Quickstart

```bash
# From the dragonpunk/ directory:
go run ./cmd/dragonpunk/

# Requires .env at repo root with:
#   DATABASE_URL=postgresql://user:pass@localhost:5432/master_chronicle
#   HOST=0.0.0.0
#   PORT=8888
```

## Verify

```bash
curl localhost:8888/api/health
# {"status":"ok","db_connected":true,"table_count":85,"timestamp":"..."}
```

## Architecture

Dragonpunk is one of three pillars in Modular Fortress:

| Pillar | Language | Role |
|--------|----------|------|
| **Dragonpunk** | Go | API server, auth, readers, graph — all user-facing I/O |
| **Noosphere Ghosts** | Common Lisp | Ghost tick engine, cognition, perception, action |
| **InnateScipt** | Common Lisp | Scripting language for ghost routines |

All three connect to `master_chronicle` PostgreSQL (Nine Tables schema).
