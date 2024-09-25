use crate::models::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::query_as;
use tokio_stream::StreamExt;
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Default)]
#[sqlx(rename_all = "lowercase")]
pub enum Category {
    Personal,
    Work,
    Health,
    Travel,
    Family,
    Hobby,
    Finance,
    Goal,
    Memory,
    Reflection,
    #[default]
    Unspecified,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Note {
    pub id: Option<i64>,
    pub content: String,
    pub analyzed: bool,
    pub category: Category,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub analysis: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub content: String,
    pub analyzed: Option<bool>,
    pub category: Option<Category>,
    pub analysis: Option<String>,
}

pub async fn create_note(
    State(state): State<AppState>,
    Json(note): Json<CreateNoteRequest>,
) -> impl IntoResponse {
    let now = Utc::now();
    let analyzed = note.analyzed.unwrap_or(false);
    let category = note.category.unwrap_or_default();
    let analysis = note.analysis.unwrap_or_default();

    match sqlx::query_as::<_, Note>(
        "INSERT INTO notes (content, analyzed, category, created_at, updated_at, analysis) 
         VALUES (?, ?, ?, ?, ?, ?) 
         RETURNING id, content, analyzed, category, created_at, updated_at, analysis",
    )
    .bind(&note.content)
    .bind(analyzed)
    .bind(&category)
    .bind(now)
    .bind(now)
    .bind(analysis)
    .fetch_one(&*state.pool)
    .await
    {
        Ok(created_note) => (StatusCode::CREATED, Json(created_note)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create note: {}", e),
        )
            .into_response(),
    }
}

pub async fn list_notes(State(state): State<AppState>) -> impl IntoResponse {
    match query_as::<_, Note>(
        "SELECT id, content, analyzed, category, created_at, updated_at, analysis
         FROM notes 
         ORDER BY created_at DESC",
    )
    .fetch_all(&*state.pool)
    .await
    {
        Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch notes: {}", e),
        )
            .into_response(),
    }
}

pub async fn get_note(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match query_as::<_, Note>(
        "SELECT id, content, analyzed, category, created_at, updated_at, analysis
         FROM notes 
         WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&*state.pool)
    .await
    {
        Ok(Some(note)) => (StatusCode::OK, Json(note)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch note: {}", e),
        )
            .into_response(),
    }
}

pub async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(note): Json<CreateNoteRequest>,
) -> impl IntoResponse {
    let now = Utc::now();
    let analyzed = note.analyzed.unwrap_or(false);
    let category = note.category.unwrap_or_default();
    let analysis = note.analysis.unwrap_or_default();

    match sqlx::query_as::<_, Note>(
        "UPDATE notes 
         SET content = ?, analyzed = ?, category = ?, updated_at = ?, analysis = ?
         WHERE id = ? 
         RETURNING id, content, analyzed, category, created_at, updated_at, analysis",
    )
    .bind(&note.content)
    .bind(analyzed)
    .bind(&category)
    .bind(now)
    .bind(&analysis)
    .bind(id)
    .fetch_optional(&*state.pool)
    .await
    {
        Ok(Some(updated_note)) => (StatusCode::OK, Json(updated_note)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update note: {}", e),
        )
            .into_response(),
    }
}

pub async fn delete_note(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match sqlx::query("DELETE FROM notes WHERE id = ?")
        .bind(id)
        .execute(&*state.pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete note: {}", e),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
    model: String,
    created_at: String,
    total_duration: u64,
    load_duration: u64,
    prompt_eval_count: u64,
    prompt_eval_duration: u64,
    eval_count: u64,
    eval_duration: u64,
}

pub async fn analyze_note(Path(id): Path<i64>, State(state): State<AppState>) -> impl IntoResponse {
    // Fetch the note
    let note = sqlx::query_as::<_, Note>(
        "SELECT id, content, analyzed, category, created_at, updated_at, analysis 
         FROM notes 
         WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&*state.pool)
    .await;

    match note {
        Ok(Some(mut note)) => {
            if note.analyzed {
                return (StatusCode::OK, Json(note)).into_response();
            }

            // Prepare the prompt for analysis
            let prompt = state
                .analysis_prompt_template
                .replace("{note_content}", &note.content);

            // Use ollama::generate_handler to get the analysis
            let params = crate::models::GenerateParams {
                prompt,
                model: None, // Use default model
            };
            let query = axum::extract::Query(params);
            let response = crate::ollama::generate_handler(query, State(state.clone())).await;

            // Process the streaming response
            let (parts, body) = response.into_response().into_parts();
            let status = parts.status;

            if status != StatusCode::OK {
                return (status, "Failed to generate analysis").into_response();
            }

            let mut stream = body.into_data_stream();
            let mut full_response = String::new();
            let mut total_tokens = 0;

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(chunk) => {
                        if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                            full_response.push_str(&text);
                            // Try to parse each line as JSON
                            for line in text.lines() {
                                if let Ok(json) = serde_json::from_str::<OllamaResponse>(line) {
                                    total_tokens += json.prompt_eval_count + json.eval_count;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Stream error: {}", e);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Stream error: {}", e),
                        )
                            .into_response();
                    }
                }
            }

            // Extract the actual analysis content
            let analysis: String = full_response
                .lines()
                .filter_map(|line| {
                    serde_json::from_str::<Value>(line)
                        .ok()
                        .and_then(|v| v["response"].as_str().map(String::from))
                })
                .collect();

            info!(
                "Analysis generated for note {}. Total tokens used: {}",
                id, total_tokens
            );

            // Update the note with the analysis
            let result = sqlx::query_as::<_, Note>(
                "UPDATE notes 
                 SET analyzed = ?, analysis = ?, updated_at = ? 
                 WHERE id = ? 
                 RETURNING id, content, analyzed, category, created_at, updated_at, analysis",
            )
            .bind(true)
            .bind(&analysis)
            .bind(Utc::now())
            .bind(id)
            .fetch_one(&*state.pool)
            .await;

            match result {
                Ok(updated_note) => (StatusCode::OK, Json(updated_note)).into_response(),
                Err(e) => {
                    error!("Failed to update note with analysis: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to update note with analysis: {}", e),
                    )
                        .into_response()
                }
            }
        }
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            error!("Failed to fetch note: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch note: {}", e),
            )
                .into_response()
        }
    }
}
