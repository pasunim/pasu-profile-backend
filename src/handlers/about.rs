use axum::{extract::State, Json};
use serde::Deserialize;
use crate::models::About;
use crate::error::AppError;
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/about",
    responses(
        (status = 200, description = "Get about info", body = About),
        (status = 404, description = "Not found")
    )
)]
pub async fn get_about(State(state): State<AppState>) -> Result<Json<About>, AppError> {
    let cache_key = String::from("about");
    
    if let Some(cached) = state.about_cache.get(&cache_key).await {
        return Ok(Json(cached));
    }

    let about = sqlx::query_as::<_, About>(
        "SELECT id, user_bio, user_bio2, created_at, updated_at FROM about LIMIT 1"
    )
    .fetch_optional(&state.pool)
    .await?;

    match about {
        Some(a) => {
            state.about_cache.insert(cache_key.clone(), a.clone()).await;
            Ok(Json(a))
        }
        None => Err(AppError::NotFound),
    }
}

#[derive(Deserialize)]
pub struct UpdateAboutPayload {
    #[serde(alias = "userBio")]
    pub user_bio: Option<String>,
    #[serde(alias = "userBio2")]
    pub user_bio2: Option<String>,
}

pub async fn update_about(
    State(state): State<AppState>,
    Json(payload): Json<UpdateAboutPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_bio = payload.user_bio.unwrap_or_default();
    let user_bio2 = payload.user_bio2.unwrap_or_default();

    sqlx::query(
        "UPDATE about SET user_bio = $1, user_bio2 = $2, updated_at = NOW() WHERE id = (SELECT id FROM about LIMIT 1)"
    )
    .bind(&user_bio)
    .bind(&user_bio2)
    .execute(&state.pool)
    .await?;

    state.about_cache.invalidate(&String::from("about")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}
