use crate::models::{AppState, GenerateParams};
use axum::{
    body::Body,
    extract::{Query, State},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
};
use tokio_stream::StreamExt;
use tracing::error;

pub async fn generate_handler(
    Query(params): Query<GenerateParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let model = params.model.as_deref().unwrap_or(&state.default_model);
    let ollama_response = match state
        .client
        .post(format!("{}/api/generate", state.ollama_url))
        .json(&serde_json::json!({
            "model": model,
            "prompt": params.prompt,
            "stream": true,
        }))
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            error!(%err, "request to Ollama failed");
            return (StatusCode::BAD_GATEWAY, HeaderMap::new(), Body::empty()).into_response();
        }
    };

    let status = ollama_response.status();
    if !status.is_success() {
        error!(status = ?status, "Ollama returned non-success status");
        return (StatusCode::BAD_GATEWAY, HeaderMap::new(), Body::empty()).into_response();
    }

    let mut headers = HeaderMap::new();
    // Manually convert headers from reqwest to axum
    for (name, value) in ollama_response.headers() {
        if let (Ok(header_name), Ok(header_value)) = (
            HeaderName::from_bytes(name.as_ref()),
            HeaderValue::from_bytes(value.as_bytes()),
        ) {
            headers.insert(header_name, header_value);
        }
    }
    // Ensure we set the content type for SSE
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/event-stream"),
    );

    let stream = ollama_response
        .bytes_stream()
        .map(|result| result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));

    (
        StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK),
        headers,
        Body::from_stream(stream),
    )
        .into_response()
}
