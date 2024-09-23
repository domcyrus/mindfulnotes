use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "http://localhost:11434")]
    pub ollama_url: String,
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    pub listen_addr: String,
    #[arg(short, long, default_value = "phi3.5")]
    pub default_model: String,
}
