use axum::{extract::{State, Path}, Json};
use serde::Deserialize;
use crate::models::Skill;
use crate::error::AppError;
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/skills",
    responses(
        (status = 200, description = "Get list of skills", body = [Skill])
    )
)]
pub async fn get_skills(State(state): State<AppState>) -> Result<Json<Vec<Skill>>, AppError> {
    let cache_key = String::from("skills");
    
    if let Some(cached) = state.skills_cache.get(&cache_key).await {
        return Ok(Json(cached));
    }

    let skills = sqlx::query_as::<_, Skill>(
        "SELECT id, icon, title, description, created_at, updated_at FROM skills ORDER BY id ASC"
    )
    .fetch_all(&state.pool)
    .await?;

    state.skills_cache.insert(cache_key, skills.clone()).await;
    Ok(Json(skills))
}

#[derive(Deserialize)]
pub struct SkillPayload {
    pub icon: String,
    pub title: String,
    pub description: String,
}

pub async fn create_skill(
    State(state): State<AppState>,
    Json(payload): Json<SkillPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query(
        "INSERT INTO skills (icon, title, description) VALUES ($1, $2, $3)"
    )
    .bind(&payload.icon)
    .bind(&payload.title)
    .bind(&payload.description)
    .execute(&state.pool)
    .await?;

    state.skills_cache.invalidate(&String::from("skills")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn update_skill(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<SkillPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query(
        "UPDATE skills SET icon = $1, title = $2, description = $3, updated_at = NOW() WHERE id = $4"
    )
    .bind(&payload.icon)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(id)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    state.skills_cache.invalidate(&String::from("skills")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn delete_skill(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("DELETE FROM skills WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    state.skills_cache.invalidate(&String::from("skills")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}
