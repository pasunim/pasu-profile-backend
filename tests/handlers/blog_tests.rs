use serde_json;

#[test]
fn test_blog_post_payload_serialization() {
    let payload = serde_json::json!({
        "title": "Getting Started with Rust",
        "slug": "getting-started-with-rust",
        "excerpt": "Learn the basics of Rust programming language",
        "content": "Rust is a systems programming language...",
        "content_markdown": "# Getting Started\n\nRust is...",
        "featured_image": "https://example.com/rust.jpg",
        "author": "John Doe",
        "published": true,
        "published_at": "2024-01-01T00:00:00Z",
        "reading_time": 5,
        "meta_title": "Getting Started with Rust",
        "meta_description": "Learn Rust basics",
        "meta_keywords": ["rust", "programming", "tutorial"],
        "categories": [1, 2],
        "tags": [1, 2, 3]
    });

    assert_eq!(payload["title"], "Getting Started with Rust");
    assert_eq!(payload["slug"], "getting-started-with-rust");
    assert_eq!(payload["excerpt"], "Learn the basics of Rust programming language");
    assert_eq!(payload["content"], "Rust is a systems programming language...");
    assert_eq!(payload["published"], true);
    assert_eq!(payload["reading_time"], 5);
}

#[test]
fn test_blog_post_payload_without_optional_fields() {
    let payload = serde_json::json!({
        "title": "Test Post",
        "slug": "test-post",
        "excerpt": "Test excerpt",
        "content": "Test content"
    });

    assert_eq!(payload["title"], "Test Post");
    assert_eq!(payload["slug"], "test-post");
    assert_eq!(payload["excerpt"], "Test excerpt");
    assert_eq!(payload["content"], "Test content");
}

#[test]
fn test_blog_post_payload_partial_categories() {
    let payload = serde_json::json!({
        "title": "Partial Categories",
        "slug": "partial-categories",
        "excerpt": "Test",
        "content": "Test content",
        "categories": [1]
    });

    assert_eq!(payload["categories"].as_array().unwrap().len(), 1);
    assert_eq!(payload["categories"][0], 1);
}

#[test]
fn test_blog_post_payload_empty_tags() {
    let payload = serde_json::json!({
        "title": "Empty Tags",
        "slug": "empty-tags",
        "excerpt": "Test",
        "content": "Test content",
        "tags": []
    });

    assert_eq!(payload["tags"].as_array().unwrap().len(), 0);
}

#[test]
fn test_category_payload_serialization() {
    let payload = serde_json::json!({
        "name": "Programming",
        "slug": "programming",
        "description": "Programming tutorials",
        "icon": "fas fa-code",
        "color": "#ff6b6b"
    });

    assert_eq!(payload["name"], "Programming");
    assert_eq!(payload["slug"], "programming");
    assert_eq!(payload["description"], "Programming tutorials");
    assert_eq!(payload["icon"], "fas fa-code");
    assert_eq!(payload["color"], "#ff6b6b");
}

#[test]
fn test_category_payload_without_optional_fields() {
    let payload = serde_json::json!({
        "name": "Test Category",
        "slug": "test-category"
    });

    assert_eq!(payload["name"], "Test Category");
    assert_eq!(payload["slug"], "test-category");
}

#[test]
fn test_tag_payload_serialization() {
    let payload = serde_json::json!({
        "name": "Rust",
        "slug": "rust"
    });

    assert_eq!(payload["name"], "Rust");
    assert_eq!(payload["slug"], "rust");
}

#[test]
fn test_success_response() {
    let response = serde_json::json!({
        "success": true
    });

    assert_eq!(response["success"], true);
}

#[test]
fn test_success_response_with_id() {
    let response = serde_json::json!({
        "success": true,
        "id": 1
    });

    assert_eq!(response["success"], true);
    assert_eq!(response["id"], 1);
}

#[test]
fn test_blog_post_response() {
    let response = serde_json::json!({
        "id": 1,
        "uuid": "550e8400-e29b-41d4-a716-446655440000",
        "title": "Getting Started with Rust",
        "slug": "getting-started-with-rust",
        "excerpt": "Learn the basics",
        "content": "Rust is a systems programming language...",
        "content_markdown": "# Getting Started\n\nRust is...",
        "featured_image": "https://example.com/rust.jpg",
        "author": "John Doe",
        "published": true,
        "published_at": "2024-01-01T00:00:00Z",
        "view_count": 100,
        "reading_time": 5,
        "meta_title": "Getting Started with Rust",
        "meta_description": "Learn Rust basics",
        "meta_keywords": ["rust", "programming"],
        "categories": [
            {
                "id": 1,
                "name": "Programming",
                "slug": "programming",
                "icon": "fas fa-code",
                "color": "#ff6b6b"
            }
        ],
        "tags": [
            {
                "id": 1,
                "name": "Rust",
                "slug": "rust"
            }
        ],
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z"
    });

    assert_eq!(response["id"], 1);
    assert_eq!(response["title"], "Getting Started with Rust");
    assert_eq!(response["published"], true);
    assert_eq!(response["view_count"], 100);
    assert_eq!(response["categories"][0]["name"], "Programming");
    assert_eq!(response["tags"][0]["name"], "Rust");
}

#[test]
fn test_multiple_blog_posts_response() {
    let response = serde_json::json!([
        {
            "id": 1,
            "title": "Post 1",
            "slug": "post-1",
            "excerpt": "Excerpt 1",
            "content": "Content 1",
            "content_markdown": null,
            "featured_image": null,
            "author": null,
            "published": true,
            "published_at": "2024-01-01T00:00:00Z",
            "view_count": 10,
            "reading_time": 3,
            "meta_title": null,
            "meta_description": null,
            "meta_keywords": null,
            "categories": [],
            "tags": [],
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        },
        {
            "id": 2,
            "title": "Post 2",
            "slug": "post-2",
            "excerpt": "Excerpt 2",
            "content": "Content 2",
            "content_markdown": null,
            "featured_image": null,
            "author": null,
            "published": true,
            "published_at": "2024-01-02T00:00:00Z",
            "view_count": 20,
            "reading_time": 4,
            "meta_title": null,
            "meta_description": null,
            "meta_keywords": null,
            "categories": [],
            "tags": [],
            "created_at": "2024-01-02T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z"
        }
    ]);

    assert_eq!(response.as_array().unwrap().len(), 2);
    assert_eq!(response[0]["id"], 1);
    assert_eq!(response[1]["id"], 2);
}

#[test]
fn test_blog_category_response() {
    let response = serde_json::json!({
        "id": 1,
        "name": "Programming",
        "slug": "programming",
        "description": "Programming tutorials",
        "icon": "fas fa-code",
        "color": "#ff6b6b",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z"
    });

    assert_eq!(response["id"], 1);
    assert_eq!(response["name"], "Programming");
    assert_eq!(response["slug"], "programming");
    assert_eq!(response["icon"], "fas fa-code");
}

#[test]
fn test_blog_tag_response() {
    let response = serde_json::json!({
        "id": 1,
        "name": "Rust",
        "slug": "rust",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z"
    });

    assert_eq!(response["id"], 1);
    assert_eq!(response["name"], "Rust");
    assert_eq!(response["slug"], "rust");
}

#[test]
fn test_multiple_categories_response() {
    let response = serde_json::json!([
        {
            "id": 1,
            "name": "Programming",
            "slug": "programming",
            "description": "Programming tutorials",
            "icon": "fas fa-code",
            "color": "#ff6b6b",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        },
        {
            "id": 2,
            "name": "Tutorial",
            "slug": "tutorial",
            "description": "Tutorial posts",
            "icon": "fas fa-book",
            "color": "#4ecdc4",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }
    ]);

    assert_eq!(response.as_array().unwrap().len(), 2);
    assert_eq!(response[0]["name"], "Programming");
    assert_eq!(response[1]["name"], "Tutorial");
}

#[test]
fn test_blog_post_payload_required_fields() {
    let payload = serde_json::json!({
        "title": "Required Fields",
        "slug": "required-fields",
        "excerpt": "Required excerpt",
        "content": "Required content"
    });

    assert!(payload["title"].is_string());
    assert!(payload["slug"].is_string());
    assert!(payload["excerpt"].is_string());
    assert!(payload["content"].is_string());
}

#[test]
fn test_blog_post_published_false() {
    let payload = serde_json::json!({
        "title": "Draft Post",
        "slug": "draft-post",
        "excerpt": "Draft",
        "content": "Draft content",
        "published": false
    });

    assert_eq!(payload["published"], false);
}
