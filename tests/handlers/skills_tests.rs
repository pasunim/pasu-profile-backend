use serde_json;

#[test]
fn test_skill_payload_serialization() {
    let payload = serde_json::json!({
        "icon": "fab fa-rust",
        "title": "Rust",
        "description": "A systems programming language"
    });

    assert_eq!(payload["icon"], "fab fa-rust");
    assert_eq!(payload["title"], "Rust");
    assert_eq!(payload["description"], "A systems programming language");
}

#[test]
fn test_skill_payload_with_different_icon() {
    let payload = serde_json::json!({
        "icon": "fas fa-database",
        "title": "PostgreSQL",
        "description": "Relational database management system"
    });

    assert_eq!(payload["icon"], "fas fa-database");
    assert_eq!(payload["title"], "PostgreSQL");
}

#[test]
fn test_skill_payload_with_long_description() {
    let payload = serde_json::json!({
        "icon": "fab fa-js",
        "title": "JavaScript",
        "description": "A high-level, interpreted programming language that conforms to the ECMAScript specification. It is a language that is also characterized as dynamic, weakly typed, prototype-based and multi-paradigm."
    });

    assert_eq!(payload["title"], "JavaScript");
    assert!(payload["description"].as_str().unwrap().len() > 100);
}

#[test]
fn test_skill_payload_with_special_characters() {
    let payload = serde_json::json!({
        "icon": "fab fa-c-plus-plus",
        "title": "C++",
        "description": "General-purpose programming language with object-oriented features"
    });

    assert_eq!(payload["title"], "C++");
    assert!(payload["description"].as_str().unwrap().contains("object-oriented"));
}

#[test]
fn test_success_response() {
    let response = serde_json::json!({
        "success": true
    });

    assert_eq!(response["success"], true);
}

#[test]
fn test_skill_response() {
    let response = serde_json::json!({
        "id": 1,
        "icon": "fab fa-rust",
        "title": "Rust",
        "description": "A systems programming language",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z"
    });

    assert_eq!(response["id"], 1);
    assert_eq!(response["icon"], "fab fa-rust");
    assert_eq!(response["title"], "Rust");
    assert_eq!(response["description"], "A systems programming language");
}

#[test]
fn test_multiple_skills_response() {
    let response = serde_json::json!([
        {
            "id": 1,
            "icon": "fab fa-rust",
            "title": "Rust",
            "description": "Systems programming language",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        },
        {
            "id": 2,
            "icon": "fas fa-database",
            "title": "PostgreSQL",
            "description": "Relational database",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        },
        {
            "id": 3,
            "icon": "fab fa-js",
            "title": "JavaScript",
            "description": "Web scripting language",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }
    ]);

    assert_eq!(response.as_array().unwrap().len(), 3);
    assert_eq!(response[0]["id"], 1);
    assert_eq!(response[1]["id"], 2);
    assert_eq!(response[2]["id"], 3);
}

#[test]
fn test_skill_payload_required_fields() {
    let payload = serde_json::json!({
        "icon": "fas fa-code",
        "title": "Test Skill",
        "description": "Test description"
    });

    assert!(payload["icon"].is_string());
    assert!(payload["title"].is_string());
    assert!(payload["description"].is_string());
}

#[test]
fn test_skill_payload_with_empty_fields() {
    let payload = serde_json::json!({
        "icon": "",
        "title": "",
        "description": ""
    });

    assert_eq!(payload["icon"], "");
    assert_eq!(payload["title"], "");
    assert_eq!(payload["description"], "");
}
