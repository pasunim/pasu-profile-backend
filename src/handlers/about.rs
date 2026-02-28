use axum::{extract::State, Json};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use crate::models::About;
use crate::error::AppError;

#[utoipa::path(
    get,
    path = "/api/about",
    responses(
        (status = 200, description = "Get about info", body = About),
        (status = 404, description = "Not found")
    )
)]
pub async fn get_about(State(pool): State<Pool<Postgres>>) -> Result<Json<About>, AppError> {
    let about = sqlx::query_as::<_, About>(
        "SELECT id, user_bio, user_bio2, created_at, updated_at FROM about LIMIT 1"
    )
    .fetch_optional(&pool)
    .await?;

    match about {
        Some(a) => Ok(Json(a)),
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
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<UpdateAboutPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_bio = payload.user_bio.unwrap_or_default();
    let user_bio2 = payload.user_bio2.unwrap_or_default();

    sqlx::query(
        "UPDATE about SET user_bio = $1, user_bio2 = $2, updated_at = NOW() WHERE id = (SELECT id FROM about LIMIT 1)"
    )
    .bind(&user_bio)
    .bind(&user_bio2)
    .execute(&pool)
    .await?;

    Ok(Json(serde_json::json!({ "success": true })))
}
