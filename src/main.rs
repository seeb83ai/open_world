use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber;

mod config;
mod error;
mod models;
mod handlers;
mod services;
mod db;
mod middleware;
mod auth;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
        )
        .init();

    tracing::info!("Starting Open World server");

    // Load configuration
    let config = config::Config::from_env();
    tracing::debug!("Configuration loaded: {:?}", config);

    // Setup database
    let pool = db::init_pool(&config.database_url)
        .await
        .expect("Failed to initialize database pool");

    tracing::info!("Database initialized");

    // Run migrations
    db::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Migrations completed");

    // Build router
    let app = Router::new()
        .route("/", get(handlers::health_check))
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/login", post(handlers::auth::login))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
