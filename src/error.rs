use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Environment error: {0}")]
    EnvError(#[from] std::env::VarError),

    #[error("Not found")]
    NotFound,

    #[error("Authentication failed")]
    AuthError,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Upload error: {0}")]
    UploadError(String),

    #[error("Internal server error: {0}")]
    InternalError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(err) => {
                tracing::error!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::AuthError => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::UploadError(msg) => {
                tracing::error!("Upload error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Upload error: {}", msg))
            }
            _ => {
                tracing::error!("Internal server error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
            }
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
