use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod mcp;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "noosphere=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment
    dotenvy::dotenv().ok();

    // Get database URL from config.json or environment
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://nebulab_user:nebulab_dev_password@localhost:5432/master_chronicle".to_string()
    });

    tracing::info!("Connecting to database...");
    let db = PgPool::connect(&database_url).await?;

    tracing::info!("Running migrations...");
    // sqlx::migrate!().run(&db).await?;

    let state = AppState { db: db.clone() };

    // Build the application router
    let app = Router::new()
        // Serve the UI dashboard
        .route("/", get(serve_dashboard))

        // API routes
        .route("/api/health", get(health_check))
        .route("/api/ghosts", get(api::ghosts::list_ghosts))
        .route("/api/ghosts/:id", get(api::ghosts::get_ghost))
        .route("/api/tasks", get(api::tasks::list_tasks))
        .route("/api/conversations", get(api::conversations::list_conversations))
        .route("/api/pipelines", get(api::pipelines::list_pipelines))
        .route("/api/system/stats", get(api::system::get_stats))

        // Serve static files
        .nest_service("/static", ServeDir::new("static"))

        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    tracing::info!("🌌 Noosphere server listening on http://{}", addr);
    tracing::info!("📊 Dashboard: http://localhost:8888");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn serve_dashboard() -> &'static str {
    // For now, return a simple redirect message
    "Noosphere Dashboard - Navigate to /static/noosphere-ops.html"
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "noosphere",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
