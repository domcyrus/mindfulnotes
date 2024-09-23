mod config;
mod models;
mod routes;

use anyhow::Result;
use axum::{routing::get, Router};
use clap::Parser;
use config::Args;
use models::AppState;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::{debug, info, Span};

#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();
    info!("Starting up");

    let args = Args::parse();
    let state = AppState {
        client: Arc::new(reqwest::Client::new()),
        ollama_url: args.ollama_url.clone(),
        default_model: args.default_model.clone(),
    };

    let app = Router::new()
        .route("/generate", get(routes::generate_handler))
        .layer(TraceLayer::new_for_http().on_body_chunk(
            |chunk: &axum::body::Bytes, _latency: std::time::Duration, _span: &Span| {
                debug!("streaming {} bytes", chunk.len());
            },
        ))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&args.listen_addr).await?;
    info!("listening on {}", args.listen_addr);
    info!("ollama URL {}", args.ollama_url);
    info!("default model {}", args.default_model);

    axum::serve(listener, app).await?;
    Ok(())
}
