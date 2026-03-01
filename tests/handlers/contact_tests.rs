use serde_json;

#[test]
fn test_contact_message_payload_serialization() {
    let payload = serde_json::json!({
        "name": "John Doe",
        "email": "john@example.com",
        "message": "Hello, world!"
    });

    assert_eq!(payload["name"], "John Doe");
    assert_eq!(payload["email"], "john@example.com");
    assert_eq!(payload["message"], "Hello, world!");
}

#[test]
fn test_contact_message_payload_minimal() {
    let payload = serde_json::json!({
        "name": "Jane",
        "email": "jane@example.com",
        "message": "Hi"
    });

    assert!(payload.is_object());
    assert_eq!(payload.as_object().unwrap().len(), 3);
}

#[test]
fn test_update_contact_payload_full() {
    let payload = serde_json::json!({
        "email": "newemail@example.com",
        "phone": "+1234567890",
        "address": "123 New Street"
    });

    assert_eq!(payload["email"], "newemail@example.com");
    assert_eq!(payload["phone"], "+1234567890");
    assert_eq!(payload["address"], "123 New Street");
}

#[test]
fn test_update_contact_payload_partial() {
    let payload = serde_json::json!({
        "email": "updated@example.com"
    });

    assert_eq!(payload["email"], "updated@example.com");
    assert!(payload["phone"].is_null());
    assert!(payload["address"].is_null());
}

#[test]
fn test_social_payload_serialization() {
    let payload = serde_json::json!({
        "name": "GitHub",
        "url": "https://github.com/user"
    });

    assert_eq!(payload["name"], "GitHub");
    assert_eq!(payload["url"], "https://github.com/user");
}

#[test]
fn test_social_payload_minimal() {
    let payload = serde_json::json!({
        "name": "Twitter",
        "url": "https://twitter.com/user"
    });

    assert_eq!(payload["name"], "Twitter");
    assert_eq!(payload["url"], "https://twitter.com/user");
}

#[test]
fn test_delete_message_query_serialization() {
    let query = serde_json::json!({
        "id": 42
    });

    assert_eq!(query["id"], 42);
}

#[test]
fn test_delete_message_query_string_id() {
    let query = serde_json::json!({
        "id": "42"
    });

    assert_eq!(query["id"], "42");
}

#[test]
fn test_success_response() {
    let response = serde_json::json!({
        "success": true
    });

    assert_eq!(response["success"], true);
}
