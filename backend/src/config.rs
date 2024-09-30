use std::env;

static DEFAULT_MODEL: &str = "llama3.2:3b";
static OLLAMA_URL: &str = "http://localhost:11434";
static LISTEN_ADDR: &str = "127.0.0.1:8080";
static ANALYSIS_PROMPT_TEMPLATE: &str = "You are an AI assistant specialized in analyzing diary entries. Please provide a thoughtful analysis of the following diary note, considering aspects such as mood, main themes, personal growth, and any potential insights or advice for the author. The note is as follows:\n\n{note_content}\n\nAnalysis:";

pub struct Config {
    pub ollama_url: String,
    pub listen_addr: String,
    pub default_model: String,
    pub analysis_prompt_template: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            ollama_url: env::var("OLLAMA_URL").unwrap_or_else(|_| OLLAMA_URL.to_string()),
            listen_addr: env::var("LISTEN_ADDR").unwrap_or_else(|_| LISTEN_ADDR.to_string()),
            default_model: env::var("DEFAULT_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string()),
            analysis_prompt_template: env::var("ANALYSIS_PROMPT_TEMPLATE")
                .unwrap_or_else(|_| ANALYSIS_PROMPT_TEMPLATE.to_string()),
        })
    }
}
