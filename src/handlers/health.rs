use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::db::DbPool;

/// Response for the liveness health check.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}

/// Response for the readiness health check.
#[derive(Serialize, Deserialize, ToSchema)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "ok".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("ok"));
        assert!(json.contains("2024-01-01T00:00:00Z"));
    }

    #[test]
    fn test_readiness_response_serialization() {
        let response = ReadinessResponse {
            status: "ok".to_string(),
            database: "connected".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("ok"));
        assert!(json.contains("connected"));
        assert!(json.contains("2024-01-01T00:00:00Z"));
    }

    #[test]
    fn test_readiness_response_error_serialization() {
        let response = ReadinessResponse {
            status: "error".to_string(),
            database: "disconnected".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("error"));
        assert!(json.contains("disconnected"));
    }

    #[test]
    fn test_readiness_response_deserialization() {
        let json = r#"{"status":"ok","database":"connected","timestamp":"2024-01-01T00:00:00Z"}"#;
        let response: ReadinessResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, "ok");
        assert_eq!(response.database, "connected");
        assert_eq!(response.timestamp, "2024-01-01T00:00:00Z");
    }
}
