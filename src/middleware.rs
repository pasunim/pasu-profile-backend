use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use base64::{engine::general_purpose, Engine as _};
use std::env;

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, Response> {
    let unauth_response = || -> Response {
        let mut res = StatusCode::UNAUTHORIZED.into_response();
        res.headers_mut()
            .insert(header::WWW_AUTHENTICATE, "Basic realm=\"Admin Access\"".parse().unwrap());
        res
    };

    let auth_header = match req.headers().get(header::AUTHORIZATION) {
        Some(header) => header,
        None => return Err(unauth_response()),
    };

    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(_) => return Err(unauth_response()),
    };

    if !auth_str.starts_with("Basic ") {
        return Err(unauth_response());
    }

    let b64_credentials = &auth_str[6..];
    let decoded_bytes = match general_purpose::STANDARD.decode(b64_credentials) {
        Ok(bytes) => bytes,
        Err(_) => return Err(unauth_response()),
    };

    let credentials = match String::from_utf8(decoded_bytes) {
        Ok(s) => s,
        Err(_) => return Err(unauth_response()),
    };

    let parts: Vec<&str> = credentials.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(unauth_response());
    }

    let password = parts[1];
    
    // Check against ADMIN_PASSWORD from .env
    let admin_password = env::var("ADMIN_PASSWORD").unwrap_or_default();
    
    // Make sure we actually have an admin password configured,
    // otherwise reject everything to fail closed.
    if password != admin_password || admin_password.is_empty() {
        return Err(unauth_response());
    }

    Ok(next.run(req).await)
}
