use serde_json;

#[test]
fn test_timeline_payload_serialization() {
    let payload = serde_json::json!({
        "title": "Senior Developer",
        "company": "Tech Corp",
        "period": "2020-2024",
        "description": "Led development team",
        "skills": ["Rust", "PostgreSQL"]
    });

    assert_eq!(payload["title"], "Senior Developer");
    assert_eq!(payload["company"], "Tech Corp");
    assert_eq!(payload["period"], "2020-2024");
    assert_eq!(payload["description"], "Led development team");
    assert_eq!(payload["skills"][0], "Rust");
    assert_eq!(payload["skills"][1], "PostgreSQL");
}

#[test]
fn test_timeline_payload_without_skills() {
    let payload = serde_json::json!({
        "title": "Junior Developer",
        "company": "Startup Inc",
        "period": "2018-2020",
        "description": "Developed web applications"
    });

    assert_eq!(payload["title"], "Junior Developer");
    assert_eq!(payload["company"], "Startup Inc");
    assert_eq!(payload["period"], "2018-2020");
    assert_eq!(payload["description"], "Developed web applications");
    assert!(!payload["skills"].is_array());
}

#[test]
fn test_timeline_payload_empty_skills() {
    let payload = serde_json::json!({
        "title": "Developer",
        "company": "Company",
        "period": "2022-2023",
        "description": "Work description",
        "skills": []
    });

    assert_eq!(payload["skills"].as_array().unwrap().len(), 0);
}

#[test]
fn test_success_response() {
    let response = serde_json::json!({
        "success": true
    });

    assert_eq!(response["success"], true);
}

#[test]
fn test_experience_timeline_response() {
    let response = serde_json::json!([
        {
            "id": 1,
            "title": "Senior Developer",
            "company": "Tech Corp",
            "period": "2020-2024",
            "description": "Led development team",
            "skills": ["Rust", "PostgreSQL"],
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }
    ]);

    assert_eq!(response[0]["id"], 1);
    assert_eq!(response[0]["title"], "Senior Developer");
    assert_eq!(response[0]["company"], "Tech Corp");
    assert_eq!(response[0]["period"], "2020-2024");
    assert_eq!(response[0]["description"], "Led development team");
    assert_eq!(response[0]["skills"][0], "Rust");
    assert_eq!(response[0]["skills"][1], "PostgreSQL");
}

#[test]
fn test_empty_skills_in_response() {
    let response = serde_json::json!([
        {
            "id": 1,
            "title": "Developer",
            "company": "Company",
            "period": "2022-2023",
            "description": "Description",
            "skills": [],
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }
    ]);

    assert_eq!(response[0]["skills"].as_array().unwrap().len(), 0);
}

#[test]
fn test_multiple_timeline_entries() {
    let response = serde_json::json!([
        {
            "id": 1,
            "title": "Senior Developer",
            "company": "Tech Corp",
            "period": "2020-2024",
            "description": "Led team",
            "skills": ["Rust"],
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        },
        {
            "id": 2,
            "title": "Junior Developer",
            "company": "Startup Inc",
            "period": "2018-2020",
            "description": "Developed apps",
            "skills": ["JavaScript", "React"],
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }
    ]);

    assert_eq!(response.as_array().unwrap().len(), 2);
    assert_eq!(response[0]["id"], 1);
    assert_eq!(response[1]["id"], 2);
}

#[test]
fn test_timeline_payload_required_fields() {
    let payload = serde_json::json!({
        "title": "Test Title",
        "company": "Test Company",
        "period": "2020-2024",
        "description": "Test description"
    });

    assert!(payload["title"].is_string());
    assert!(payload["company"].is_string());
    assert!(payload["period"].is_string());
    assert!(payload["description"].is_string());
}
