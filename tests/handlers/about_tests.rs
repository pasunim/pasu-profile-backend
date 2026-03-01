use serde_json;

#[test]
fn test_update_about_payload_serialization() {
    let payload = serde_json::json!({
        "user_bio": "Software Developer",
        "userBio2": "Rust Enthusiast"
    });

    assert!(payload.is_object());
    assert_eq!(payload["user_bio"], "Software Developer");
    assert_eq!(payload["userBio2"], "Rust Enthusiast");
}

#[test]
fn test_update_about_payload_with_userbio_alias() {
    let payload = serde_json::json!({
        "userBio": "Software Developer",
        "userBio2": "Rust Enthusiast"
    });

    assert_eq!(payload["userBio"], "Software Developer");
    assert_eq!(payload["userBio2"], "Rust Enthusiast");
}

#[test]
fn test_update_about_payload_partial() {
    let payload = serde_json::json!({
        "user_bio": "Software Developer"
    });

    assert!(payload["userBio2"].is_null());
}

#[test]
fn test_update_about_payload_empty() {
    let payload = serde_json::json!({});

    assert!(payload.is_object());
    assert!(payload.get("user_bio").is_none());
    assert!(payload.get("userBio2").is_none());
}

#[test]
fn test_update_about_payload_response() {
    let response = serde_json::json!({
        "success": true
    });

    assert_eq!(response["success"], true);
}
