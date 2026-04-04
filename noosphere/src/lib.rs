// Noosphere - Ghosts in the Noosphere operations platform
// Unified codebase - minimal working version

pub mod api;
pub mod mcp;

// Common types
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}
