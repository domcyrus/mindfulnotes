use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Client>,
    pub ollama_url: String,
    pub default_model: String,
    pub pool: Arc<SqlitePool>,
    pub detailed_diary_analysis_prompt: String,
    pub diary_categorization_prompt: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct GenerateParams {
    pub prompt: String,
    pub model: Option<String>,
}
