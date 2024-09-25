use reqwest::Client;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Client>,
    pub ollama_url: String,
    pub default_model: String,
    pub pool: Arc<SqlitePool>,
    pub analysis_prompt_template: String,
}

#[derive(Deserialize)]
pub struct GenerateParams {
    pub prompt: String,
    pub model: Option<String>,
}
