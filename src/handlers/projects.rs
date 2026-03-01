use axum::{extract::{State, Path}, Json};
use serde::Deserialize;
use crate::models::Project;
use crate::error::AppError;
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/projects",
    responses(
        (status = 200, description = "Get all projects", body = [Project])
    )
)]
pub async fn get_projects(State(state): State<AppState>) -> Result<Json<Vec<Project>>, AppError> {
    let cache_key = String::from("projects");
    
    if let Some(cached) = state.projects_cache.get(&cache_key).await {
        return Ok(Json(cached));
    }

    let projects = sqlx::query_as::<_, Project>(
        "SELECT id, title, description, image, skills, link, is_active, created_at, updated_at FROM projects ORDER BY id DESC"
    )
    .fetch_all(&state.pool)
    .await?;

    state.projects_cache.insert(cache_key, projects.clone()).await;
    Ok(Json(projects))
}

#[derive(Deserialize)]
pub struct ProjectPayload {
    pub title: String,
    pub description: String,
    pub image: Option<String>,
    pub skills: Option<Vec<String>>,
    pub link: Option<String>,
    pub is_active: Option<bool>,
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(payload): Json<ProjectPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query(
        "INSERT INTO projects (title, description, image, skills, link, is_active) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.image.unwrap_or_default())
    .bind(&payload.skills)
    .bind(payload.link.unwrap_or_default())
    .bind(payload.is_active.unwrap_or(true))
    .execute(&state.pool)
    .await?;

    state.projects_cache.invalidate(&String::from("projects")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn update_project(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<ProjectPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query(
        "UPDATE projects SET title = $1, description = $2, image = $3, skills = $4, link = $5, is_active = $6, updated_at = NOW() WHERE id = $7"
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.image.unwrap_or_default())
    .bind(&payload.skills)
    .bind(payload.link.unwrap_or_default())
    .bind(payload.is_active.unwrap_or(true))
    .bind(id)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    state.projects_cache.invalidate(&String::from("projects")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn delete_project(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    state.projects_cache.invalidate(&String::from("projects")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}
