use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use base64::{engine::general_purpose, Engine as _};
use std::env;

/// Constant-time comparison so a wrong password cannot be recovered by
/// measuring how long the rejection takes.
fn secure_compare(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// Reads the configured admin password, treating "unset" and "empty" alike so
/// a missing env var fails closed instead of accepting a blank password.
fn admin_password() -> Option<String> {
    match env::var("ADMIN_PASSWORD") {
        Ok(p) if !p.is_empty() => Some(p),
        _ => None,
    }
}

/// Extracts the password from a `Basic base64(user:pass)` header value.
/// RFC 7617 makes the scheme name case-insensitive, so accept any casing.
fn password_from_basic(auth_str: &str) -> Option<String> {
    let (scheme, b64) = auth_str.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("Basic") {
        return None;
    }
    let decoded = general_purpose::STANDARD.decode(b64.trim()).ok()?;
    let credentials = String::from_utf8(decoded).ok()?;
    let (_user, password) = credentials.split_once(':')?;
    Some(password.to_string())
}

fn unauthorized() -> Response {
    let mut res = StatusCode::UNAUTHORIZED.into_response();
    res.headers_mut().insert(
        header::WWW_AUTHENTICATE,
        header::HeaderValue::from_static("Basic realm=\"Admin Access\""),
    );
    res
}

/// Checks a candidate password against the configured admin password in
/// constant time. Returns false when no password is configured.
pub fn verify_admin_password(candidate: &str) -> bool {
    match admin_password() {
        Some(expected) => secure_compare(candidate, &expected),
        None => {
            tracing::error!("ADMIN_PASSWORD is not configured; rejecting login");
            false
        }
    }
}

/// Guards every mutating/admin endpoint. The frontend proxies these requests
/// server-side and attaches the Basic header, so the secret never reaches the
/// browser.
pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, Response> {
    let Some(expected) = admin_password() else {
        tracing::error!("ADMIN_PASSWORD is not configured; rejecting admin request");
        return Err(unauthorized());
    };

    let supplied = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(password_from_basic);

    match supplied {
        Some(password) if secure_compare(&password, &expected) => Ok(next.run(req).await),
        _ => Err(unauthorized()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secure_compare_matches_only_identical_strings() {
        assert!(secure_compare("hunter2", "hunter2"));
        assert!(!secure_compare("hunter2", "hunter3"));
        assert!(!secure_compare("hunter2", "hunter22"));
        assert!(!secure_compare("", "x"));
    }

    #[test]
    fn password_is_extracted_from_basic_header() {
        // base64("admin:s3cret")
        let header = format!("Basic {}", general_purpose::STANDARD.encode("admin:s3cret"));
        assert_eq!(password_from_basic(&header).as_deref(), Some("s3cret"));
    }

    #[test]
    fn password_may_contain_colons() {
        let header = format!("Basic {}", general_purpose::STANDARD.encode("admin:a:b:c"));
        assert_eq!(password_from_basic(&header).as_deref(), Some("a:b:c"));
    }

    #[test]
    fn scheme_name_is_case_insensitive() {
        let encoded = general_purpose::STANDARD.encode("admin:s3cret");
        for scheme in ["Basic", "basic", "BASIC", "BaSiC"] {
            let header = format!("{} {}", scheme, encoded);
            assert_eq!(password_from_basic(&header).as_deref(), Some("s3cret"));
        }
    }

    #[test]
    fn malformed_headers_are_rejected() {
        assert!(password_from_basic("Bearer abc").is_none());
        assert!(password_from_basic("Basic !!!not-base64!!!").is_none());
        // No colon separator at all.
        let header = format!("Basic {}", general_purpose::STANDARD.encode("nocolon"));
        assert!(password_from_basic(&header).is_none());
    }
}
