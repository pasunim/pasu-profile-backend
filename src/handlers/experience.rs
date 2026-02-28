use axum::{extract::{State, Path}, Json};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
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
        "SELECT id, title, company, period, description, skills, created_at, updated_at FROM experience_timeline ORDER BY id DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(exp))
}

#[derive(Deserialize)]
pub struct TimelinePayload {
    pub title: String,
    pub company: String,
    pub period: String,
    pub description: String,
    pub skills: Option<Vec<String>>,
}

pub async fn create_timeline(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<TimelinePayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query(
        "INSERT INTO experience_timeline (title, company, period, description, skills) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(&payload.title)
    .bind(&payload.company)
    .bind(&payload.period)
    .bind(&payload.description)
    .bind(&payload.skills)
    .execute(&pool)
    .await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn update_timeline(
    Path(id): Path<i32>,
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<TimelinePayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query(
        "UPDATE experience_timeline SET title = $1, company = $2, period = $3, description = $4, skills = $5, updated_at = NOW() WHERE id = $6"
    )
    .bind(&payload.title)
    .bind(&payload.company)
    .bind(&payload.period)
    .bind(&payload.description)
    .bind(&payload.skills)
    .bind(id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(serde_json::json!({ "success": true })))
}

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
