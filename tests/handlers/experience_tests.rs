use serde_json;

#[test]
fn test_timeline_payload_serialization() {
    let payload = serde_json::json!({
        "title": "Senior Developer",
        "company": "Tech Corp",
        "period": "2020-2024",
        "description": "Led development team",
        "skills": ["Rust", "PostgreSQL"],
        "categories": ["engineering", "backend"],
        "tags": ["senior", "leadership"],
        "details": ["Led team of 5 developers", "Implemented CI/CD pipeline"]
    });

    assert_eq!(payload["title"], "Senior Developer");
    assert_eq!(payload["company"], "Tech Corp");
    assert_eq!(payload["period"], "2020-2024");
    assert_eq!(payload["description"], "Led development team");
    assert_eq!(payload["skills"][0], "Rust");
    assert_eq!(payload["skills"][1], "PostgreSQL");
    assert_eq!(payload["categories"][0], "engineering");
    assert_eq!(payload["tags"][0], "senior");
    assert_eq!(payload["details"][0], "Led team of 5 developers");
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
        "skills": [],
        "categories": null,
        "tags": null,
        "details": null
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
            "categories": ["engineering"],
            "tags": ["backend"],
            "details": ["Built scalable microservices"],
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
    assert_eq!(response[0]["categories"][0], "engineering");
    assert_eq!(response[0]["tags"][0], "backend");
    assert_eq!(response[0]["details"][0], "Built scalable microservices");
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
            "categories": null,
            "tags": null,
            "details": null,
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
            "categories": ["engineering"],
            "tags": ["senior"],
            "details": ["Led team of 5 developers"],
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
            "categories": ["frontend"],
            "tags": ["junior"],
            "details": ["Built responsive web applications"],
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

#[test]
fn test_timeline_payload_with_optional_fields() {
    let payload = serde_json::json!({
        "title": "Test Title",
        "company": "Test Company",
        "period": "2020-2024",
        "description": "Test description",
        "categories": ["category1", "category2"],
        "tags": ["tag1", "tag2"],
        "details": ["detail1", "detail2"]
    });

    assert_eq!(payload["categories"].as_array().unwrap().len(), 2);
    assert_eq!(payload["tags"].as_array().unwrap().len(), 2);
    assert_eq!(payload["details"].as_array().unwrap().len(), 2);
}

#[test]
fn test_timeline_payload_with_null_optional_fields() {
    let payload = serde_json::json!({
        "title": "Test Title",
        "company": "Test Company",
        "period": "2020-2024",
        "description": "Test description",
        "categories": null,
        "tags": null,
        "details": null
    });

    assert!(payload["categories"].is_null());
    assert!(payload["tags"].is_null());
    assert!(payload["details"].is_null());
}
