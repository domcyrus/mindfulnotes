mod config;
mod models;
mod notes;
mod ollama;

use anyhow::{Context, Result};
use axum::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use config::Config;
use models::AppState;
use sqlx::sqlite::SqlitePoolOptions;
use std::{path::Path, sync::Arc};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{debug, info, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("Starting up");

    // Load configuration from environment
    let config = Config::from_env().context("Failed to load configuration")?;

    // Enable CORS
    let cors = CorsLayer::new()
        // allow `GET`, `POST`, `PUT`, `DELETE` and `OPTIONS` methods
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        // allow requests from localhost
        .allow_origin(AllowOrigin::any())
        // allow headers `Content-Type` and `Authorization`
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    // Determine the database file path
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:notes.db".to_string());
    let db_path = db_url.trim_start_matches("sqlite:");

    // Ensure the database file exists
    if !Path::new(db_path).exists() {
        std::fs::File::create(db_path).context("Failed to create database file")?;
        info!("Created new database file: {}", db_path);
    }

    // Set up SQLite connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .context("Failed to connect to SQLite database")?;

    // Create notes table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            analyzed BOOLEAN NOT NULL DEFAULT 0,
            category TEXT NOT NULL DEFAULT 'unspecified',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            analysis TEXT
        )",
    )
    .execute(&pool)
    .await
    .context("Failed to create notes table")?;

    let state = AppState {
        client: Arc::new(reqwest::Client::new()),
        ollama_url: config.ollama_url.clone(),
        default_model: config.default_model.clone(),
        pool: Arc::new(pool),
        analysis_prompt_template: config.analysis_prompt_template.clone(),
    };

    let app = Router::new()
        .route("/generate", get(ollama::generate_handler))
        .route("/notes", post(notes::create_note))
        .route("/notes", get(notes::list_notes))
        .route("/notes/:id", get(notes::get_note))
        .route("/notes/:id", put(notes::update_note))
        .route("/notes/:id", delete(notes::delete_note))
        .route("/notes/:id/analyze", post(notes::analyze_note))
        .layer(TraceLayer::new_for_http().on_body_chunk(
            |chunk: &axum::body::Bytes, _latency: std::time::Duration, _span: &Span| {
                debug!("streaming {} bytes", chunk.len());
            },
        ))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.listen_addr).await?;
    info!("listening on {}", config.listen_addr);
    info!("ollama URL {}", config.ollama_url);
    info!("default model {}", config.default_model);

    axum::serve(listener, app).await?;
    Ok(())
}
