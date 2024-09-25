// config.rs
use std::env;

pub struct Config {
    pub ollama_url: String,
    pub listen_addr: String,
    pub default_model: String,
    pub analysis_prompt_template: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            ollama_url: env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
            listen_addr: env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
            default_model: env::var("DEFAULT_MODEL").unwrap_or_else(|_| "phi3.5".to_string()),
            analysis_prompt_template: env::var("ANALYSIS_PROMPT_TEMPLATE").unwrap_or_else(|_| {
                "You are an AI assistant specialized in analyzing diary entries. Please provide a thoughtful analysis of the following diary note, considering aspects such as mood, main themes, personal growth, and any potential insights or advice for the author. The note is as follows:\n\n{note_content}\n\nAnalysis:".to_string()
            }),
        })
    }
}
