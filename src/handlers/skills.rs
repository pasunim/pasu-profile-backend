use axum::{extract::{State, Path}, Json};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use crate::models::Skill;
use crate::error::AppError;

#[utoipa::path(
    get,
    path = "/api/skills",
    responses(
        (status = 200, description = "Get list of skills", body = [Skill])
    )
)]
pub async fn get_skills(State(pool): State<Pool<Postgres>>) -> Result<Json<Vec<Skill>>, AppError> {
    let skills = sqlx::query_as::<_, Skill>(
        "SELECT id, icon, title, description, created_at, updated_at FROM skills ORDER BY id ASC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(skills))
}

#[derive(Deserialize)]
pub struct SkillPayload {
    pub icon: String,
    pub title: String,
    pub description: String,
}

pub async fn create_skill(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<SkillPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query(
        "INSERT INTO skills (icon, title, description) VALUES ($1, $2, $3)"
    )
    .bind(&payload.icon)
    .bind(&payload.title)
    .bind(&payload.description)
    .execute(&pool)
    .await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn update_skill(
    Path(id): Path<i32>,
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<SkillPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query(
        "UPDATE skills SET icon = $1, title = $2, description = $3, updated_at = NOW() WHERE id = $4"
    )
    .bind(&payload.icon)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn delete_skill(
    Path(id): Path<i32>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("DELETE FROM skills WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(serde_json::json!({ "success": true })))
}
