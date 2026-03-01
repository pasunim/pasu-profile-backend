use axum::{extract::{State, Path, Query}, Json};
use serde::Deserialize;
use crate::models::{ContactInfo, SocialLink, ContactMessage};
use crate::error::AppError;
use crate::state::AppState;
use utoipa::ToSchema;

#[utoipa::path(
    get,
    path = "/api/contact",
    responses(
        (status = 200, description = "Get contact info", body = ContactInfo),
        (status = 404, description = "Not found")
    )
)]
pub async fn get_contact_info(State(state): State<AppState>) -> Result<Json<ContactInfo>, AppError> {
    let info = sqlx::query_as::<_, ContactInfo>(
        "SELECT id, email, phone, address, created_at, updated_at FROM contact_info LIMIT 1"
    )
    .fetch_optional(&state.pool)
    .await?;

    match info {
        Some(i) => Ok(Json(i)),
        None => Err(AppError::NotFound),
    }
}

#[utoipa::path(
    get,
    path = "/api/contact/socials",
    responses(
        (status = 200, description = "Get social links", body = [SocialLink])
    )
)]
pub async fn get_social_links(State(state): State<AppState>) -> Result<Json<Vec<SocialLink>>, AppError> {
    let cache_key = String::from("socials");
    
    if let Some(cached) = state.socials_cache.get(&cache_key).await {
        return Ok(Json(cached));
    }

    let links = sqlx::query_as::<_, SocialLink>(
        "SELECT id, name, url, created_at, updated_at FROM social_links ORDER BY id ASC"
    )
    .fetch_all(&state.pool)
    .await?;

    state.socials_cache.insert(cache_key, links.clone()).await;
    Ok(Json(links))
}

#[derive(Deserialize, ToSchema)]
pub struct ContactMessagePayload {
    name: String,
    email: String,
    message: String,
}

#[utoipa::path(
    post,
    path = "/api/contact",
    request_body = ContactMessagePayload,
    responses(
        (status = 200, description = "Submit contact message")
    )
)]
pub async fn submit_contact_message(
    State(state): State<AppState>,
    Json(payload): Json<ContactMessagePayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query(
        "INSERT INTO contact_messages (name, email, message) VALUES ($1, $2, $3)"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&payload.message)
    .execute(&state.pool)
    .await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

// Admin: Update contact info
#[derive(Deserialize)]
pub struct UpdateContactPayload {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

pub async fn update_contact_info(
    State(state): State<AppState>,
    Json(payload): Json<UpdateContactPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query(
        "UPDATE contact_info SET email = COALESCE($1, email), phone = COALESCE($2, phone), address = COALESCE($3, address), updated_at = NOW() WHERE id = (SELECT id FROM contact_info LIMIT 1)"
    )
    .bind(&payload.email)
    .bind(&payload.phone)
    .bind(&payload.address)
    .execute(&state.pool)
    .await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

// Admin: Social CRUD
#[derive(Deserialize)]
pub struct SocialPayload {
    pub name: String,
    pub url: String,
}

pub async fn create_social(
    State(state): State<AppState>,
    Json(payload): Json<SocialPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query("INSERT INTO social_links (name, url) VALUES ($1, $2)")
        .bind(&payload.name)
        .bind(&payload.url)
        .execute(&state.pool)
        .await?;

    state.socials_cache.invalidate(&String::from("socials")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn update_social(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<SocialPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("UPDATE social_links SET name = $1, url = $2, updated_at = NOW() WHERE id = $3")
        .bind(&payload.name)
        .bind(&payload.url)
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    state.socials_cache.invalidate(&String::from("socials")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn delete_social(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("DELETE FROM social_links WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    state.socials_cache.invalidate(&String::from("socials")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}

// Admin: Messages
pub async fn get_messages(State(state): State<AppState>) -> Result<Json<Vec<ContactMessage>>, AppError> {
    let messages = sqlx::query_as::<_, ContactMessage>(
        "SELECT id, name, email, message, created_at FROM contact_messages ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(messages))
}

#[derive(Deserialize)]
pub struct DeleteMessageQuery {
    pub id: i32,
}

pub async fn delete_message(
    Query(query): Query<DeleteMessageQuery>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("DELETE FROM contact_messages WHERE id = $1")
        .bind(query.id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(serde_json::json!({ "success": true })))
}
