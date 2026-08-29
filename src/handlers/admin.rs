use axum::{extract::{ConnectInfo, State}, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use crate::error::AppError;
use crate::middleware::verify_admin_password;
use crate::state::AppState;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LoginPayload {
    password: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    success: bool,
}

/// Verifies the admin password. The session cookie itself is issued by the
/// Next.js proxy, which is the only party that holds the password — this
/// endpoint just answers whether a candidate password is correct.
#[utoipa::path(
    post,
    path = "/api/admin/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Unauthorized"),
        (status = 429, description = "Too many attempts")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, AppError> {
    let ip = addr.ip();
    if !state.login_limiter.check(ip).await {
        tracing::warn!("Login rate limit exceeded for {}", ip);
        return Err(AppError::RateLimited);
    }

    let password = payload
        .password
        .ok_or_else(|| AppError::ValidationError("กรุณากรอกรหัสผ่าน".to_string()))?;

    if verify_admin_password(&password) {
        state.login_limiter.reset(ip).await;
        Ok(Json(LoginResponse { success: true }))
    } else {
        Err(AppError::AuthError)
    }
}
