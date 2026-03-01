use axum::Json;
use serde::{Deserialize, Serialize};
use std::env;
use crate::error::AppError;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LoginPayload {
    password: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    success: bool,
}

#[utoipa::path(
    post,
    path = "/api/admin/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn login(
    jar: CookieJar,
    Json(payload): Json<LoginPayload>,
) -> Result<(CookieJar, Json<LoginResponse>), AppError> {
    let password = payload.password.ok_or_else(|| AppError::ValidationError("กรุณากรอกรหัสผ่าน".to_string()))?;
    let admin_password = env::var("ADMIN_PASSWORD").map_err(|_| AppError::InternalError(anyhow::anyhow!("ADMIN_PASSWORD not set")))?;

    if password == admin_password {
        let token = format!("{}-{}", chrono::Utc::now().timestamp_millis(), rand::random::<u64>());
        let token_base64 = {
            use base64::{Engine as _, engine::general_purpose};
            general_purpose::STANDARD.encode(token)
        };

        let cookie = Cookie::build(("admin_token", token_base64))
            .path("/")
            .http_only(true)
            .secure(env::var("NODE_ENV").unwrap_or_default() == "production")
            .same_site(axum_extra::extract::cookie::SameSite::Lax)
            .max_age(time::Duration::days(7))
            .build();

        Ok((jar.add(cookie), Json(LoginResponse { success: true })))
    } else {
        Err(AppError::AuthError)
    }
}
