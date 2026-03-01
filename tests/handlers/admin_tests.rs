use serde_json;

#[test]
fn test_login_payload_with_password() {
    let payload = serde_json::json!({
        "password": "admin123"
    });

    assert_eq!(payload["password"], "admin123");
}

#[test]
fn test_login_payload_without_password() {
    let payload = serde_json::json!({});

    assert!(!payload["password"].is_string());
}

#[test]
fn test_login_payload_empty_password() {
    let payload = serde_json::json!({
        "password": ""
    });

    assert_eq!(payload["password"], "");
}

#[test]
fn test_login_payload_long_password() {
    let payload = serde_json::json!({
        "password": "very-long-password-with-special-characters-123!@#$%^&*()"
    });

    assert_eq!(payload["password"], "very-long-password-with-special-characters-123!@#$%^&*()");
}

#[test]
fn test_login_payload_with_special_characters() {
    let payload = serde_json::json!({
        "password": "p@ssw0rd!#$%"
    });

    assert_eq!(payload["password"], "p@ssw0rd!#$%");
}

#[test]
fn test_login_response_success() {
    let response = serde_json::json!({
        "success": true
    });

    assert_eq!(response["success"], true);
}

#[test]
fn test_login_response_failure() {
    let response = serde_json::json!({
        "success": false
    });

    assert_eq!(response["success"], false);
}

#[test]
fn test_login_payload_is_string() {
    let payload = serde_json::json!({
        "password": "test123"
    });

    assert!(payload["password"].is_string());
}

#[test]
fn test_login_response_is_boolean() {
    let response = serde_json::json!({
        "success": true
    });

    assert!(response["success"].is_boolean());
}
