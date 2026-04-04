# Noosphere - Quick Start

## What We Built

A unified **Rust web server** consolidating dpn-api, dpn-core, and dpn-mcp into a single `noosphere` codebase for your personal autonomous agent operations.

## Current Status

✅ **Server Running**: `http://localhost:8888`
✅ **PostgreSQL Connected**: master_chronicle database (83 tables, 2,554 tasks, 9,846 conversations)
✅ **API Endpoints Live**: Health, system stats, ghosts, tasks, conversations, pipelines
✅ **UI Dashboard**: Static file serving at `/static/noosphere-ops.html`

## What's Working Right Now

```bash
# Server is running on port 8888
curl http://localhost:8888/api/health
# {"service":"noosphere","status":"ok","version":"0.1.0"}

curl http://localhost:8888/api/system/stats
# {"active_ghosts":0,"idle_ghosts":0,"dormant_ghosts":0,
#  "total_tasks":2554,"total_conversations":9846,
#  "tick_number":4847,"uptime_days":31}
```

## Next Steps

### 1. Wire Up the UI to Real API Data

The `noosphere-ops.html` file currently has mock data. Connect it to the live endpoints:

```javascript
// Replace mock data in noosphere-ops.html with:
fetch('/api/system/stats')
  .then(r => r.json())
  .then(data => {
    // Update dashboard with real data
  });
```

### 2. Fix Agent Schema Mapping

The `/api/ghosts` endpoint expects these columns in the `agents` table:
- `trust_level`, `energy_level`, `state`, `current_location`, `tier`, `area_id`

Either add these columns or adjust the query in `src/api/ghosts.rs` to match your schema.

### 3. Add WebSocket Support (Optional)

For real-time updates, add WebSocket support for live tick updates and agent status changes.

### 4. Integrate dpn-core Modules (Later)

The core business logic modules from dpn-core are available in `src/core/` but not yet integrated. These provide:
- Memory system
- Pipeline engine
- Event tracking
- Task management
- Context injection
- Embeddings

## Directory Structure

```
noosphere/
├── src/
│   ├── main.rs              # Server entry point ✅
│   ├── lib.rs               # Library root ✅
│   ├── api/                 # HTTP handlers ✅
│   │   ├── ghosts.rs
│   │   ├── tasks.rs
│   │   ├── conversations.rs
│   │   ├── pipelines.rs
│   │   └── system.rs
│   ├── core/                # dpn-core modules (not yet integrated)
│   └── mcp/                 # MCP server stub (TODO)
├── static/
│   └── noosphere-ops.html   # Dashboard UI ✅
├── target/release/
│   └── noosphere            # Compiled binary ✅
└── README.md                # Full documentation

```

## Running the Server

```bash
# Development mode (with hot reload via cargo-watch)
cargo watch -x run

# Production mode
cargo build --release
./target/release/noosphere

# Background daemon
./target/release/noosphere > /tmp/noosphere.log 2>&1 &
```

## Configuration

Edit `.env`:
```bash
DATABASE_URL=postgresql://nebulab_user:nebulab_dev_password@localhost:5432/master_chronicle
RUST_LOG=noosphere=debug,tower_http=debug
```

## Database Connection

Already configured for your local PostgreSQL:
- Host: localhost:5432
- Database: master_chronicle
- User: nebulab_user
- 83 tables loaded ✅
- 2,554 tasks ✅
- 9,846 conversations ✅

## What Happened to dpn-api and dpn-core?

**Merged into noosphere**:
- `dpn-api/src/handlers/` → `noosphere/src/api/`
- `dpn-core/src/` → `noosphere/src/core/` (preserved but not yet integrated)
- Single binary, single codebase, no more inter-project dependencies

## Known Issues

1. **Agent endpoint returns empty**: The schema mapping needs adjustment for your `agents` table structure
2. **Core modules not integrated**: The 20+ dpn-core business logic modules are copied but not yet wired up
3. **UI still has mock data**: Need to replace JavaScript mock data with API calls

---

**Status**: Server is running and serving real data from your database! 🌌

*"The noosphere breathes. The ghosts are operational."*
