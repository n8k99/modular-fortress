# (.)(.) Noosphere

**Ghosts in the Noosphere** - Personal autonomous agent operations platform

A unified Rust codebase consolidating dpn-core, dpn-api, and dpn-mcp into a single modular system for managing your autonomous ghost agents, tasks, conversations, and pipelines.

## Architecture

```
noosphere/
├── src/
│   ├── main.rs              # Axum web server entry point
│   ├── lib.rs               # Library root
│   ├── api/                 # HTTP API handlers
│   │   ├── ghosts.rs        # Agent/ghost endpoints
│   │   ├── tasks.rs         # Task management
│   │   ├── conversations.rs # Chat history
│   │   ├── pipelines.rs     # Workflow status
│   │   └── system.rs        # System stats
│   ├── core/                # Business logic (from dpn-core)
│   │   ├── db/              # Database connections
│   │   ├── tasks/           # Task management
│   │   ├── conversations/   # Conversation tracking
│   │   ├── memory/          # Agent memory system
│   │   ├── pipeline/        # Workflow engine
│   │   ├── events/          # Event system
│   │   └── ...              # 20+ modules
│   └── mcp/                 # Model Context Protocol (TODO)
├── static/
│   └── noosphere-ops.html   # Dashboard UI
└── Cargo.toml
```

## Features

- **Web Dashboard**: Beautiful dark-themed ops interface at `http://localhost:8888/static/noosphere-ops.html`
- **REST API**: Query ghosts, tasks, conversations, pipelines
- **PostgreSQL Backend**: Connected to your local `master_chronicle` database
- **Real-time Stats**: Live system health, agent status, tick counts
- **Modular Core**: 20+ business logic modules from dpn-core

## Quick Start

### Prerequisites

- Rust 1.70+
- PostgreSQL 16+ running locally
- Database: `master_chronicle` with `nebulab_user` credentials

### Setup

```bash
# Clone/navigate to noosphere directory
cd noosphere

# Copy environment config
cp .env.example .env

# Build and run
cargo run

# Or build for release
cargo build --release
./target/release/noosphere
```

The server starts on `http://localhost:8888`

### Database Connection

Default connection string (configured in `.env`):
```
postgresql://nebulab_user:nebulab_dev_password@localhost:5432/master_chronicle
```

## API Endpoints

### System
- `GET /api/health` - Health check
- `GET /api/system/stats` - System statistics

### Ghosts
- `GET /api/ghosts` - List all active ghosts
- `GET /api/ghosts/:id` - Get specific ghost details

### Tasks
- `GET /api/tasks` - List pending/in-progress tasks

### Conversations
- `GET /api/conversations` - Recent conversation history

### Pipelines
- `GET /api/pipelines` - Active pipeline status

## Dashboard

Open `http://localhost:8888/static/noosphere-ops.html` to access the operations dashboard featuring:

- **Forge Workshop**: Your daily desk view with notes, inbox, and system health
- **Pipeline Workbench**: Visualize active workflows and agent chains
- **Orbis Map**: Ghost token positions in coordinate space
- **Nova's Bridge**: T.A.S.K.S. command center
- **Domain View**: Browse the nine divisions of master_chronicle

## Development

### Adding New API Endpoints

1. Create handler in `src/api/`
2. Register route in `src/main.rs`
3. Wire up database queries

### Core Modules

All dpn-core functionality is available under `noosphere::core::*`:

```rust
use noosphere::core::db::create_pool;
use noosphere::core::memory::MemoryScope;
use noosphere::core::tasks::Task;
```

## Configuration

Edit `.env` to customize:

```bash
DATABASE_URL=postgresql://user:pass@host:port/database
RUST_LOG=noosphere=debug,tower_http=debug
HOST=0.0.0.0
PORT=8888
```

## Migration from dpn-api + dpn-core

This codebase consolidates:
- `dpn-api` → `noosphere/src/api` + `noosphere/src/main.rs`
- `dpn-core` → `noosphere/src/core/*`
- `dpn-mcp` → `noosphere/src/mcp` (stub, TODO)

All existing dpn-core modules are preserved and functional.

## What's Next

- [ ] Implement MCP server for Claude Code integration
- [ ] Add WebSocket support for real-time updates
- [ ] Expand pipeline visualization
- [ ] Add agent command/control interface
- [ ] Implement tick engine monitoring

## License

Personal project - All rights reserved

---

*"The ghosts are operational. The noosphere breathes."*
