use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::db::DbPool;

/// Response for the liveness health check.
#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}

/// Response for the readiness health check.
#[derive(Serialize, ToSchema)]
pub struct ReadinessResponse {
    pub status: String,
    pub database: String,
    pub timestamp: String,
}

/// Liveness health check — confirms the service is running.
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is alive", body = HealthResponse)
    ),
    tag = "health"
)]
pub async fn health() -> impl IntoResponse {
    let now = chrono::Utc::now().to_rfc3339();
    Json(HealthResponse {
        status: "ok".to_string(),
        timestamp: now,
    })
}

/// Readiness health check — confirms the service and database are ready.
#[utoipa::path(
    get,
    path = "/health/ready",
    responses(
        (status = 200, description = "Service and database are ready", body = ReadinessResponse),
        (status = 503, description = "Service is not ready", body = ReadinessResponse)
    ),
    tag = "health"
)]
pub async fn readiness(State(pool): State<DbPool>) -> impl IntoResponse {
    let now = chrono::Utc::now().to_rfc3339();

    match sqlx::query_scalar::<_, i32>("SELECT 1").fetch_one(&pool).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ReadinessResponse {
                status: "ok".to_string(),
                database: "connected".to_string(),
                timestamp: now,
            }),
        ),
        Err(e) => {
            tracing::error!("Database health check failed: {:?}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ReadinessResponse {
                    status: "error".to_string(),
                    database: "disconnected".to_string(),
                    timestamp: now,
                }),
            )
        }
    }
}
