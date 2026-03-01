use axum::{extract::{State, Path}, Json};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;
use crate::models::ExperienceTimeline;
use crate::error::AppError;

#[utoipa::path(
    get,
    path = "/api/experience",
    responses(
        (status = 200, description = "Get experience timeline", body = [ExperienceTimeline])
    )
)]
pub async fn get_experience(State(pool): State<Pool<Postgres>>) -> Result<Json<Vec<ExperienceTimeline>>, AppError> {
    let exp = sqlx::query_as::<_, ExperienceTimeline>(
        "SELECT id, title, company, period, description, skills, categories, tags, details, created_at, updated_at FROM experience_timeline /* force_new_plan */ ORDER BY id DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(exp))
}

#[derive(Deserialize, ToSchema)]
pub struct TimelinePayload {
    pub title: String,
    pub company: String,
    pub period: String,
    pub description: String,
    pub skills: Option<Vec<String>>,
    #[serde(default)]
    pub categories: Option<serde_json::Value>,
    #[serde(default)]
    pub tags: Option<serde_json::Value>,
    #[serde(default)]
    pub details: Option<serde_json::Value>,
}

pub async fn create_timeline(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<TimelinePayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query(
        "INSERT INTO experience_timeline (title, company, period, description, skills, categories, tags, details) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(&payload.title)
    .bind(&payload.company)
    .bind(&payload.period)
    .bind(&payload.description)
    .bind(&payload.skills)
    .bind(&payload.categories)
    .bind(&payload.tags)
    .bind(&payload.details)
    .execute(&pool)
    .await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

#[utoipa::path(
    put,
    path = "/api/experience/{id}",
    params(
        ("id" = i32, Path, description = "Experience timeline ID")
    ),
    request_body = TimelinePayload,
    responses(
        (status = 200, description = "Update experience timeline", body = serde_json::Value)
    )
)]
pub async fn update_timeline(
    Path(id): Path<i32>,
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<TimelinePayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query(
        "UPDATE experience_timeline SET title = $1, company = $2, period = $3, description = $4, skills = $5, categories = $6, tags = $7, details = $8, updated_at = NOW() WHERE id = $9"
    )
    .bind(&payload.title)
    .bind(&payload.company)
    .bind(&payload.period)
    .bind(&payload.description)
    .bind(&payload.skills)
    .bind(&payload.categories)
    .bind(&payload.tags)
    .bind(&payload.details)
    .bind(id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(serde_json::json!({ "success": true })))
}

#[utoipa::path(
    delete,
    path = "/api/experience/{id}",
    params(
        ("id" = i32, Path, description = "Experience timeline ID")
    ),
    responses(
        (status = 200, description = "Delete experience timeline", body = serde_json::Value)
    )
)]
pub async fn delete_timeline(
    Path(id): Path<i32>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("DELETE FROM experience_timeline WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(serde_json::json!({ "success": true })))
}
