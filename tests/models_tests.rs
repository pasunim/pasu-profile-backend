use pasu_profile_backend::models::*;

#[test]
fn test_about_serialization() {
    let about = About {
        id: 1,
        user_bio: "Software Developer".to_string(),
        user_bio2: "Rust Enthusiast".to_string(),
        created_at: None,
        updated_at: None,
        categories: Some(serde_json::json!(["programming", "rust"])),
        tags: Some(serde_json::json!(["developer", "backend"])),
    };

    let json = serde_json::to_string(&about).unwrap();
    assert!(json.contains("Software Developer"));
    assert!(json.contains("Rust Enthusiast"));
}

#[test]
fn test_about_deserialization() {
    let json = r#"{
        "id": 1,
        "user_bio": "Software Developer",
        "user_bio2": "Rust Enthusiast",
        "created_at": null,
        "updated_at": null,
        "categories": ["programming", "rust"],
        "tags": ["developer", "backend"]
    }"#;

    let about: About = serde_json::from_str(json).unwrap();
    assert_eq!(about.id, 1);
    assert_eq!(about.user_bio, "Software Developer");
    assert_eq!(about.user_bio2, "Rust Enthusiast");
}

#[test]
fn test_skill_serialization() {
    let skill = Skill {
        id: 1,
        icon: "🦀".to_string(),
        title: "Rust".to_string(),
        description: "Systems programming language".to_string(),
        created_at: None,
        updated_at: None,
        categories: Some(serde_json::json!(["programming"])),
        tags: Some(serde_json::json!(["language"])),
    };

    let json = serde_json::to_string(&skill).unwrap();
    assert!(json.contains("Rust"));
    assert!(json.contains("🦀"));
}

#[test]
fn test_skill_deserialization() {
    let json = r#"{
        "id": 1,
        "icon": "🦀",
        "title": "Rust",
        "description": "Systems programming language",
        "created_at": null,
        "updated_at": null
    }"#;

    let skill: Skill = serde_json::from_str(json).unwrap();
    assert_eq!(skill.id, 1);
    assert_eq!(skill.title, "Rust");
    assert_eq!(skill.icon, "🦀");
}

#[test]
fn test_experience_timeline_serialization() {
    let experience = ExperienceTimeline {
        id: 1,
        title: "Senior Developer".to_string(),
        company: "Tech Corp".to_string(),
        period: "2020-2024".to_string(),
        description: "Building awesome things".to_string(),
        skills: Some(vec!["Rust".to_string(), "PostgreSQL".to_string()]),
        created_at: None,
        updated_at: None,
        categories: None,
        tags: None,
        details: None,
    };

    let json = serde_json::to_string(&experience).unwrap();
    assert!(json.contains("Senior Developer"));
    assert!(json.contains("Tech Corp"));
}

#[test]
fn test_experience_timeline_with_details() {
    let experience = ExperienceTimeline {
        id: 1,
        title: "Senior Developer".to_string(),
        company: "Tech Corp".to_string(),
        period: "2020-2024".to_string(),
        description: "Building awesome things".to_string(),
        skills: Some(vec!["Rust".to_string(), "PostgreSQL".to_string()]),
        created_at: None,
        updated_at: None,
        categories: Some(serde_json::json!(["engineering"])),
        tags: Some(serde_json::json!(["backend", "senior"])),
        details: Some(serde_json::json!(["Led team of 5 developers", "Implemented CI/CD"])),
    };

    let json = serde_json::to_string(&experience).unwrap();
    let parsed: ExperienceTimeline = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.title, "Senior Developer");
    assert!(parsed.categories.is_some());
    assert!(parsed.tags.is_some());
    assert!(parsed.details.is_some());
}

#[test]
fn test_project_serialization() {
    let project = Project {
        id: 1,
        title: "Awesome Project".to_string(),
        description: "A great project".to_string(),
        image: "https://example.com/image.png".to_string(),
        skills: Some(vec!["Rust".to_string(), "Axum".to_string()]),
        link: "https://github.com/user/project".to_string(),
        is_active: Some(true),
        created_at: None,
        updated_at: None,
        categories: None,
        tags: None,
    };

    let json = serde_json::to_string(&project).unwrap();
    assert!(json.contains("Awesome Project"));
    assert!(json.contains("https://github.com"));
}

#[test]
fn test_contact_info_serialization() {
    let contact = ContactInfo {
        id: 1,
        email: "user@example.com".to_string(),
        phone: "+1234567890".to_string(),
        address: "123 Main St".to_string(),
        created_at: None,
        updated_at: None,
        categories: None,
        tags: None,
    };

    let json = serde_json::to_string(&contact).unwrap();
    assert!(json.contains("user@example.com"));
    assert!(json.contains("+1234567890"));
}

#[test]
fn test_social_link_serialization() {
    let social = SocialLink {
        id: 1,
        name: "GitHub".to_string(),
        url: "https://github.com/user".to_string(),
        created_at: None,
        updated_at: None,
        categories: None,
        tags: None,
    };

    let json = serde_json::to_string(&social).unwrap();
    assert!(json.contains("GitHub"));
    assert!(json.contains("github.com"));
}

#[test]
fn test_contact_message_serialization() {
    let message = ContactMessage {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        message: "Hello, world!".to_string(),
        created_at: None,
    };

    let json = serde_json::to_string(&message).unwrap();
    assert!(json.contains("John Doe"));
    assert!(json.contains("Hello, world!"));
}

#[test]
fn test_blog_category_serialization() {
    let category = BlogCategory {
        id: 1,
        name: "Programming".to_string(),
        slug: "programming".to_string(),
        description: Some("Programming related posts".to_string()),
        icon: Some("💻".to_string()),
        color: Some("#FF5733".to_string()),
        created_at: None,
        updated_at: None,
        categories: None,
        tags: None,
    };

    let json = serde_json::to_string(&category).unwrap();
    assert!(json.contains("Programming"));
    assert!(json.contains("programming"));
}

#[test]
fn test_blog_tag_serialization() {
    let tag = BlogTag {
        id: 1,
        name: "rust".to_string(),
        slug: "rust".to_string(),
        created_at: None,
        updated_at: None,
        categories: None,
        tags: None,
    };

    let json = serde_json::to_string(&tag).unwrap();
    assert!(json.contains("rust"));
}

#[test]
fn test_blog_post_serialization() {
    let post = BlogPost {
        id: 1,
        uuid: Some("123e4567-e89b-12d3-a456-426614174000".to_string()),
        title: "Hello World".to_string(),
        slug: "hello-world".to_string(),
        excerpt: "This is an excerpt".to_string(),
        content: "Full content here".to_string(),
        content_markdown: Some("# Hello".to_string()),
        featured_image: Some("https://example.com/image.png".to_string()),
        author: Some("Pasu".to_string()),
        published: Some(true),
        published_at: None,
        view_count: Some(100),
        reading_time: Some(5),
        meta_title: Some("Meta Title".to_string()),
        meta_description: Some("Meta Description".to_string()),
        meta_keywords: Some(vec!["rust".to_string(), "programming".to_string()]),
        created_at: None,
        updated_at: None,
        categories: None,
        tags: None,
    };

    let json = serde_json::to_string(&post).unwrap();
    assert!(json.contains("Hello World"));
    assert!(json.contains("hello-world"));
}

#[test]
fn test_skill_with_optional_fields() {
    let skill = Skill {
        id: 1,
        icon: "🦀".to_string(),
        title: "Rust".to_string(),
        description: "Systems programming language".to_string(),
        created_at: None,
        updated_at: None,
        categories: None,
        tags: None,
    };

    let json = serde_json::to_string(&skill).unwrap();
    let parsed: Skill = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.id, 1);
    assert_eq!(parsed.title, "Rust");
    assert!(parsed.categories.is_none());
    assert!(parsed.tags.is_none());
}

#[test]
fn test_project_with_defaults() {
    let project = Project {
        id: 1,
        title: "Test Project".to_string(),
        description: "Description".to_string(),
        image: "image.png".to_string(),
        skills: None,
        link: "https://example.com".to_string(),
        is_active: None,
        created_at: None,
        updated_at: None,
        categories: None,
        tags: None,
    };

    let json = serde_json::to_string(&project).unwrap();
    let parsed: Project = serde_json::from_str(&json).unwrap();
    assert!(parsed.is_active.is_none());
    assert!(parsed.skills.is_none());
}
