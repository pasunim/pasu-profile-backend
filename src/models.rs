use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::NaiveDateTime;

// Formats for uuid, etc. Although schema uses SERIAL (int) for IDs.

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct About {
    pub id: i32,
    pub user_bio: String,
    pub user_bio2: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(default)]
    #[sqlx(default)]
    pub categories: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(default)]
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Skill {
    pub id: i32,
    pub icon: String,
    pub title: String,
    pub description: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(default)]
    #[sqlx(default)]
    pub categories: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(default)]
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ExperienceTimeline {
    pub id: i32,
    pub title: String,
    pub company: String,
    pub period: String,
    pub description: String,
    pub skills: Option<Vec<String>>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(default)]
    #[sqlx(default)]
    pub categories: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(default)]
    pub tags: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(default)]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub image: String,
    pub skills: Option<Vec<String>>,
    pub link: String,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(default)]
    #[sqlx(default)]
    pub categories: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(default)]
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ContactInfo {
    pub id: i32,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(default)]
    #[sqlx(default)]
    pub categories: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(default)]
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SocialLink {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(default)]
    #[sqlx(default)]
    pub categories: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(default)]
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ContactMessage {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub message: String,
    pub created_at: Option<NaiveDateTime>,
}

// Blog
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BlogCategory {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(default)]
    #[sqlx(default)]
    pub categories: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(default)]
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BlogTag {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(default)]
    #[sqlx(default)]
    pub categories: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(default)]
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BlogPost {
    pub id: i32,
    pub uuid: Option<String>,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: String,
    pub content_markdown: Option<String>,
    pub featured_image: Option<String>,
    pub author: Option<String>,
    pub published: Option<bool>,
    pub published_at: Option<NaiveDateTime>,
    pub view_count: Option<i32>,
    pub reading_time: Option<i32>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<Vec<String>>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(default)]
    #[sqlx(default)]
    pub categories: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(default)]
    pub tags: Option<serde_json::Value>,
}
