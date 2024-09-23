use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Client>,
    pub ollama_url: String,
    pub default_model: String,
}

#[derive(Deserialize)]
pub struct GenerateParams {
    pub prompt: String,
    pub model: Option<String>,
}
