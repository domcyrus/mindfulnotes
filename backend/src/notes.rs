use crate::models::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{query_as, sqlite::SqlitePool, FromRow};
use std::str::FromStr;
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
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
    Unspecified,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "personal" => Ok(Category::Personal),
            "work" => Ok(Category::Work),
            "health" => Ok(Category::Health),
            "travel" => Ok(Category::Travel),
            "family" => Ok(Category::Family),
            "hobby" => Ok(Category::Hobby),
            "finance" => Ok(Category::Finance),
            "goal" => Ok(Category::Goal),
            "memory" => Ok(Category::Memory),
            "reflection" => Ok(Category::Reflection),
            _ => Ok(Category::Unspecified),
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

async fn get_category_id(
    pool: &SqlitePool,
    category: &Category,
) -> Result<i64, (StatusCode, String)> {
    let category_str = category.to_string().to_lowercase();
    match sqlx::query!(
        "SELECT id FROM category_descriptions WHERE category = ?",
        category_str
    )
    .fetch_optional(pool)
    .await
    {
        Ok(Some(row)) => Ok(row.id),
        Ok(None) => Err((
            StatusCode::BAD_REQUEST,
            format!("Invalid category: {}", category_str),
        )),
        Err(e) => {
            error!("Database error when fetching category: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ))
        }
    }
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[sqlx(type_name = "CategoryDescription")]
pub struct CategoryDescription {
    pub id: i64,
    pub category: Category,
    pub explanation: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Note {
    pub id: i64,
    pub content: String,
    pub analyzed: bool,
    pub category_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub analysis: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub content: String,
    pub analyzed: Option<bool>,
    pub category: Category,
    pub analysis: Option<String>,
}

pub async fn create_note(
    State(state): State<AppState>,
    Json(note): Json<CreateNoteRequest>,
) -> impl IntoResponse {
    let now = Utc::now();
    let analyzed = note.analyzed.unwrap_or(false);
    let analysis = note.analysis.unwrap_or_default();

    // First, get the category_id
    let category_id = match get_category_id(&state.pool, &note.category).await {
        Ok(category_id) => category_id,
        Err(response) => return response.into_response(),
    };

    match sqlx::query_as!(
        Note,
        r#"
        INSERT INTO notes (
            content, 
            analyzed, 
            category_id, 
            created_at, 
            updated_at, 
            analysis
        ) 
        VALUES (?, ?, ?, ?, ?, ?) 
        RETURNING 
            id, 
            content, 
            analyzed, 
            category_id, 
            created_at as "created_at: DateTime<Utc>", 
            updated_at as "updated_at: DateTime<Utc>", 
            analysis
        "#,
        note.content,
        analyzed,
        category_id,
        now,
        now,
        analysis
    )
    .fetch_one(&*state.pool)
    .await
    {
        Ok(created_note) => (StatusCode::CREATED, Json(created_note)).into_response(),
        Err(e) => {
            error!("Failed to create note: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create note".to_string(),
            )
                .into_response()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteWithCategory {
    pub id: i64,
    pub content: String,
    pub analyzed: bool,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub analysis: Option<String>,
}

pub async fn list_notes(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query_as!(
        NoteWithCategory,
        r#"
        SELECT 
            n.id,
            n.content,
            n.analyzed,
            cd.category as "category!",
            n.created_at as "created_at: DateTime<Utc>",
            n.updated_at as "updated_at: DateTime<Utc>",
            n.analysis
        FROM 
            notes n
        JOIN 
            category_descriptions cd ON n.category_id = cd.id
        ORDER BY 
            n.created_at DESC
        "#
    )
    .fetch_all(&*state.pool)
    .await
    {
        Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
        Err(e) => {
            error!("Failed to fetch notes: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch notes".to_string(),
            )
                .into_response()
        }
    }
}

pub async fn get_note(
    State(state): State<AppState>,
    Path(note_id): Path<i64>,
) -> impl IntoResponse {
    match sqlx::query_as!(
        NoteWithCategory,
        r#"
        SELECT 
            n.id,
            n.content,
            n.analyzed,
            cd.category as "category!",
            n.created_at as "created_at: DateTime<Utc>",
            n.updated_at as "updated_at: DateTime<Utc>",
            n.analysis
        FROM 
            notes n
        JOIN 
            category_descriptions cd ON n.category_id = cd.id
        WHERE 
            n.id = ?
        "#,
        note_id
    )
    .fetch_optional(&*state.pool)
    .await
    {
        Ok(Some(note)) => (StatusCode::OK, Json(note)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            format!("Note with id {} not found", note_id),
        )
            .into_response(),
        Err(e) => {
            error!("Database error when fetching note {}: {}", note_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
                .into_response()
        }
    }
}

pub async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(note): Json<CreateNoteRequest>,
) -> impl IntoResponse {
    let now = Utc::now();
    let analyzed = note.analyzed.unwrap_or(false);
    let analysis = note.analysis.unwrap_or_default();

    // Get the category_id using the helper function
    let category_id = match get_category_id(&state.pool, &note.category).await {
        Ok(id) => id,
        Err(response) => return response.into_response(),
    };

    // Update the note and return the updated version
    match sqlx::query_as::<_, Note>(
        r#"
    UPDATE notes
    SET content = $1,
        analyzed = $2,
        category_id = $3,
        updated_at = $4,
        analysis = $5
    WHERE id = $6
    RETURNING id,
             content,
             analyzed,
             category_id,
             created_at,
             updated_at,
             analysis
    "#,
    )
    .bind(&note.content)
    .bind(analyzed)
    .bind(&category_id)
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

pub async fn delete_note(
    State(state): State<AppState>,
    Path(note_id): Path<i64>,
) -> impl IntoResponse {
    match sqlx::query!("DELETE FROM notes WHERE id = ?", note_id)
        .execute(&*state.pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                StatusCode::NO_CONTENT.into_response()
            } else {
                (StatusCode::NOT_FOUND, "Note not found").into_response()
            }
        }
        Err(e) => {
            error!("Failed to delete note {}: {}", note_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete note".to_string(),
            )
                .into_response()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LlmCategory {
    pub category: String,
    pub explanation: String,
}

pub async fn get_note_llm_categories(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match query_as::<_, LlmCategory>(
        "SELECT cd.category, cd.explanation
         FROM llm_categories as lc
         JOIN category_descriptions as cd ON lc.category_id = cd.id
         WHERE lc.note_id = ?",
    )
    .bind(id)
    .fetch_all(&*state.pool)
    .await
    {
        Ok(categories) => (StatusCode::OK, Json(categories)).into_response(),
        Err(e) => {
            error!("Failed to fetch categories for note {}: {}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch categories: {}", e),
            )
                .into_response()
        }
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
        "SELECT id, content, analyzed, category_id, created_at, updated_at, analysis 
         FROM notes 
         WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&*state.pool)
    .await;

    match note {
        Ok(Some(note)) => {
            if note.analyzed {
                return (StatusCode::OK, Json(note)).into_response();
            }

            // Prepare the prompt for analysis
            let prompt = state
                .detailed_diary_analysis_prompt
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
                 RETURNING id, content, analyzed, category_id, created_at, updated_at, analysis",
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

#[derive(Debug, Deserialize)]
struct CategoryResponse {
    categories: Vec<CategoryItem>,
}

#[derive(Debug, Deserialize)]
struct CategoryItem {
    name: String,
    explanation: String,
}

pub async fn categorize_note(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Fetch the note
    let note_result = sqlx::query_as::<_, Note>(
        "SELECT id, content, analyzed, category, created_at, updated_at, analysis 
         FROM notes 
         WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&*state.pool)
    .await;

    match note_result {
        Ok(Some(note)) => {
            // Check if the note already has LLM categories
            let existing_categories = sqlx::query_as::<_, LlmCategory>(
                "SELECT id, note_id, category, explanation, created_at 
                 FROM llm_categories 
                 WHERE note_id = ?",
            )
            .bind(note.id)
            .fetch_all(&*state.pool)
            .await;

            match existing_categories {
                Ok(categories) if !categories.is_empty() => {
                    // If categories exist, return the note with its categories
                    return (StatusCode::OK, Json((note, categories))).into_response();
                }
                Ok(_) | Err(_) => {
                    // If no categories or there was an error, proceed with categorization
                    // hmm maybe we should only categorize if there are no categories
                    // otherwise we might overwrite existing categories
                }
            }

            // Prepare the prompt for categorization
            let prompt = state
                .diary_categorization_prompt
                .replace("{note_content}", &note.content);

            // Use ollama::generate_handler to get the categorization
            let params = crate::models::GenerateParams {
                prompt: prompt.clone(),
                model: None, // Use default model
            };

            let query = axum::extract::Query(params);

            let mut attempts = 0;
            let max_attempts = 3;
            let mut categories = Vec::new();
            let mut total_tokens = 0;

            while attempts < max_attempts {
                attempts += 1;
                let response =
                    crate::ollama::generate_handler(query.clone(), State(state.clone())).await;

                // Process the streaming response
                let (parts, body) = response.into_response().into_parts();
                let status = parts.status;

                if status != StatusCode::OK {
                    if attempts == max_attempts {
                        return (status, "Failed to generate categorization").into_response();
                    }
                    continue;
                }

                let mut stream = body.into_data_stream();
                let mut full_response = String::new();

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
                            if attempts == max_attempts {
                                return (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    format!("Stream error: {}", e),
                                )
                                    .into_response();
                            }
                            continue;
                        }
                    }
                }

                // Extract the actual categorization content
                let categorization: String = full_response
                    .lines()
                    .filter_map(|line| {
                        serde_json::from_str::<Value>(line)
                            .ok()
                            .and_then(|v| v["response"].as_str().map(String::from))
                    })
                    .collect();

                // Try to parse the categorization as JSON
                match serde_json::from_str::<CategoryResponse>(&categorization) {
                    Ok(category_response) => {
                        categories = category_response.categories;
                        break;
                    }
                    Err(e) => {
                        error!("Failed to parse categorization JSON: {}", e);
                        if attempts == max_attempts {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Failed to generate valid categorization JSON".to_string(),
                            )
                                .into_response();
                        }
                    }
                }
            }

            info!(
                "Categorization generated for note {}. Total tokens used: {}",
                id, total_tokens
            );

            // Insert the new categories
            let mut inserted_categories = Vec::new();
            for category_item in categories {
                let category =
                    Category::from_str(&category_item.name).unwrap_or(Category::Unspecified);

                // we need to get the category_id from the category name string

                let result = sqlx::query_as::<_, LlmCategory>(
                    "INSERT INTO llm_categories (note_id, category, explanation) 
                     VALUES (?, ?, ?) 
                     RETURNING id, note_id, category, explanation, created_at",
                )
                .bind(note.id)
                .bind(category)
                .bind(&category_item.explanation)
                .fetch_one(&*state.pool)
                .await;

                match result {
                    Ok(inserted_category) => inserted_categories.push(inserted_category),
                    Err(e) => {
                        error!("Failed to insert category: {}", e);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to insert category: {}", e),
                        )
                            .into_response();
                    }
                }
            }

            // Return the note with its new categories
            (StatusCode::OK, Json((note, inserted_categories))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Note not found").into_response(),
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
