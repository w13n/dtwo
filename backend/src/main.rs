mod config;
mod error;
mod handlers;
mod routes;
mod settings;
mod sql;
mod traits;

use std::sync::Arc;

use axum::ServiceExt;
use axum::extract::Request;
use tower::Layer;
use tower_http::trace::TraceLayer;
use tower_http::{cors::CorsLayer, normalize_path::NormalizePathLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use handlers::AppState;
use routes::create_router;
use sql::SqliteSettingsRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "settings_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded: {:?}", config);

    // Initialize repository
    let repository = SqliteSettingsRepository::new(&config.database_path).await?;
    tracing::info!("Database initialized at: {}", config.database_path);

    let port = config.port; // save for later

    // Create application state
    let state = Arc::new(AppState {
        repository: Arc::new(repository),
        config,
    });

    tracing::info!("got here");
    // Build router
    let app = NormalizePathLayer::trim_trailing_slash().layer(
        create_router(state)
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::permissive()),
    );

    // Start server
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();

    Ok(())
}
