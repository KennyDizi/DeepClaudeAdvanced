//! DeepClaude - A high-performance LLM inference API and Chat UI that integrates DeepSeek R1's CoT reasoning traces with Anthropic Claude models..
//!
//! This application provides a REST API for chat interactions that:
//! - Processes messages through DeepSeek R1 for reasoning
//! - Uses Anthropic's Claude for final responses
//! - Supports both streaming and non-streaming responses
//! - Tracks token usage and costs
//! - Provides detailed usage statistics
//!
//! The API requires authentication tokens for both services and
//! supports custom configuration through a TOML config file.

mod clients;
mod config;
mod error;
mod handlers;
mod models;

use crate::{config::Config, handlers::AppState};
use axum::routing::{get, post, Router};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use chrono::Utc;

/// Application entry point.
///
/// Sets up logging, loads configuration, and starts the HTTP server
/// with the configured routes and middleware.
///
/// # Returns
///
/// * `anyhow::Result<()>` - Ok if server starts successfully, Err otherwise
///
/// # Errors
///
/// Returns an error if:
/// - Logging setup fails
/// - Server address binding fails
/// - Server encounters a fatal error while running
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "deepclaude=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::load().unwrap_or_else(|_| {
        tracing::warn!("Failed to load config.toml, using default configuration");
        Config::default()
    });

    // Create application state
    // Clone config for AppState
    let config_clone = config.clone();
    let state = Arc::new(AppState { config: config_clone });

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    // Build router
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/chat/completions", post(handlers::handle_chat))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state);

    // Get host and port from config
    let host = config.server.host.clone();
    let port = config.server.port;
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .unwrap_or_else(|_| {
            tracing::error!("Invalid host/port configuration: {}:{}", host, port);
            std::process::exit(1);
        });

    tracing::info!("Binding to {}:{}", host, port);

    tracing::info!("Starting server on {}", addr);

    // Start server
    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app.into_make_service(),
    )
    .await?;

    Ok(())
}

async fn health_handler() -> axum::response::Html<&'static str> {
    // Log new request with formatted datetime
    println!("New request coming at {}", Utc::now().format("%d:%m:%Y %H:%M:%S"));
    axum::response::Html("Hello DeepClaude Advanced.")
}
