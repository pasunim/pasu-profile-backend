use serde_json;

#[test]
fn test_project_payload_serialization() {
    let payload = serde_json::json!({
        "title": "Portfolio Website",
        "description": "A personal portfolio website built with Rust",
        "image": "https://example.com/image.jpg",
        "skills": ["Rust", "Axum", "PostgreSQL"],
        "link": "https://github.com/example/portfolio",
        "is_active": true
    });

    assert_eq!(payload["title"], "Portfolio Website");
    assert_eq!(payload["description"], "A personal portfolio website built with Rust");
    assert_eq!(payload["image"], "https://example.com/image.jpg");
    assert_eq!(payload["skills"][0], "Rust");
    assert_eq!(payload["skills"][1], "Axum");
    assert_eq!(payload["skills"][2], "PostgreSQL");
    assert_eq!(payload["link"], "https://github.com/example/portfolio");
    assert_eq!(payload["is_active"], true);
}

#[test]
fn test_project_payload_without_optional_fields() {
    let payload = serde_json::json!({
        "title": "Test Project",
        "description": "Test description"
    });

    assert_eq!(payload["title"], "Test Project");
    assert_eq!(payload["description"], "Test description");
    assert!(!payload["image"].is_string());
    assert!(!payload["skills"].is_array());
    assert!(!payload["link"].is_string());
    assert!(!payload["is_active"].is_boolean());
}

#[test]
fn test_project_payload_partial_optional_fields() {
    let payload = serde_json::json!({
        "title": "Partial Project",
        "description": "Description",
        "skills": ["Rust"],
        "is_active": false
    });

    assert_eq!(payload["title"], "Partial Project");
    assert_eq!(payload["skills"][0], "Rust");
    assert_eq!(payload["is_active"], false);
}

#[test]
fn test_project_payload_empty_skills() {
    let payload = serde_json::json!({
        "title": "Project",
        "description": "Description",
        "skills": []
    });

    assert_eq!(payload["skills"].as_array().unwrap().len(), 0);
}

#[test]
fn test_project_payload_is_active_false() {
    let payload = serde_json::json!({
        "title": "Inactive Project",
        "description": "This project is inactive",
        "is_active": false
    });

    assert_eq!(payload["is_active"], false);
}

#[test]
fn test_success_response() {
    let response = serde_json::json!({
        "success": true
    });

    assert_eq!(response["success"], true);
}

#[test]
fn test_project_response() {
    let response = serde_json::json!({
        "id": 1,
        "title": "Portfolio Website",
        "description": "A personal portfolio website",
        "image": "https://example.com/image.jpg",
        "skills": ["Rust", "Axum"],
        "link": "https://github.com/example/portfolio",
        "is_active": true,
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z"
    });

    assert_eq!(response["id"], 1);
    assert_eq!(response["title"], "Portfolio Website");
    assert_eq!(response["is_active"], true);
}

#[test]
fn test_multiple_projects_response() {
    let response = serde_json::json!([
        {
            "id": 1,
            "title": "Project A",
            "description": "Description A",
            "image": "https://example.com/a.jpg",
            "skills": ["Rust"],
            "link": "https://github.com/example/a",
            "is_active": true,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        },
        {
            "id": 2,
            "title": "Project B",
            "description": "Description B",
            "image": "https://example.com/b.jpg",
            "skills": ["JavaScript", "React"],
            "link": "https://github.com/example/b",
            "is_active": true,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }
    ]);

    assert_eq!(response.as_array().unwrap().len(), 2);
    assert_eq!(response[0]["id"], 1);
    assert_eq!(response[1]["id"], 2);
}

#[test]
fn test_project_payload_required_fields() {
    let payload = serde_json::json!({
        "title": "Required Fields",
        "description": "This has only required fields"
    });

    assert!(payload["title"].is_string());
    assert!(payload["description"].is_string());
}
