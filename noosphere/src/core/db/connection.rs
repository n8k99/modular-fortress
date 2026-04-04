//! Database connection management
//!
//! Expects PostgreSQL via SSH tunnel on port 5433:
//! ssh -L 5433:127.0.0.1:5432 root@144.126.251.126 -N -f

use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub type DbPool = PgPool;

/// Default connection string for the SSH-tunneled PostgreSQL
pub const DEFAULT_DATABASE_URL: &str = 
    "postgres://chronicle:chronicle2026@127.0.0.1:5433/master_chronicle";

/// Create a connection pool to the PostgreSQL database
pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    tracing::info!("Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .connect(database_url)
        .await?;

    tracing::info!("Database connected");
    Ok(pool)
}

/// Create a pool using the default connection URL
pub async fn create_default_pool() -> Result<DbPool> {
    create_pool(DEFAULT_DATABASE_URL).await
}

/// Test database connectivity
pub async fn test_connection(pool: &DbPool) -> Result<()> {
    let _: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await?;
    Ok(())
}
