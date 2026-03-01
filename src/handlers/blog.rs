use axum::{extract::{State, Path}, Json};
use serde::Deserialize;
use crate::models::{BlogPost, BlogCategory, BlogTag};
use crate::error::AppError;
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/blog/posts",
    responses(
        (status = 200, description = "Get blog posts", body = [BlogPost])
    )
)]
pub async fn get_posts(State(state): State<AppState>) -> Result<Json<Vec<BlogPost>>, AppError> {
    let posts = sqlx::query_as::<_, BlogPost>(
        r#"

SELECT p.id, p.uuid::text as uuid, p.title, p.slug, p.excerpt, p.content, p.content_markdown, p.featured_image, p.author, p.published, p.published_at, p.view_count, p.reading_time, p.meta_title, p.meta_description, p.meta_keywords, p.created_at, p.updated_at,
    (
        SELECT COALESCE(json_agg(jsonb_build_object(
        'id', c.id,
        'name', c.name,
        'slug', c.slug,
        'icon', c.icon,
        'color', c.color
        )), '[]'::json)
        FROM blog_post_categories pc
        JOIN blog_categories c ON pc.category_id = c.id
        WHERE pc.post_id = p.id
    ) as categories,
    (
        SELECT COALESCE(json_agg(jsonb_build_object(
        'id', t.id,
        'name', t.name,
        'slug', t.slug
        )), '[]'::json)
        FROM blog_post_tags pt
        JOIN blog_tags t ON pt.tag_id = t.id
        WHERE pt.post_id = p.id
    ) as tags
FROM blog_posts p
 WHERE p.published = true ORDER BY p.published_at DESC
"#
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(posts))
}

#[utoipa::path(
    get,
    path = "/api/blog/posts/{slug}",
    params(
        ("slug" = String, Path, description = "Post slug")
    ),
    responses(
        (status = 200, description = "Get blog post by slug", body = BlogPost),
        (status = 404, description = "Not found")
    )
)]
pub async fn get_post_by_slug(
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<BlogPost>, AppError> {
    let post = sqlx::query_as::<_, BlogPost>(
        r#"

SELECT p.id, p.uuid::text as uuid, p.title, p.slug, p.excerpt, p.content, p.content_markdown, p.featured_image, p.author, p.published, p.published_at, p.view_count, p.reading_time, p.meta_title, p.meta_description, p.meta_keywords, p.created_at, p.updated_at,
    (
        SELECT COALESCE(json_agg(jsonb_build_object(
        'id', c.id,
        'name', c.name,
        'slug', c.slug,
        'icon', c.icon,
        'color', c.color
        )), '[]'::json)
        FROM blog_post_categories pc
        JOIN blog_categories c ON pc.category_id = c.id
        WHERE pc.post_id = p.id
    ) as categories,
    (
        SELECT COALESCE(json_agg(jsonb_build_object(
        'id', t.id,
        'name', t.name,
        'slug', t.slug
        )), '[]'::json)
        FROM blog_post_tags pt
        JOIN blog_tags t ON pt.tag_id = t.id
        WHERE pt.post_id = p.id
    ) as tags
FROM blog_posts p
 WHERE (p.slug = $1 OR p.uuid::text = $1) AND p.published = true
"#
    )
    .bind(&slug)
    .fetch_optional(&state.pool)
    .await?;

    match post {
        Some(p) => Ok(Json(p)),
        None => Err(AppError::NotFound),
    }
}

#[utoipa::path(
    get,
    path = "/api/blog/categories",
    responses(
        (status = 200, description = "Get blog categories", body = [BlogCategory])
    )
)]
pub async fn get_categories(State(state): State<AppState>) -> Result<Json<Vec<BlogCategory>>, AppError> {
    let cache_key = String::from("categories");
    
    if let Some(cached) = state.categories_cache.get(&cache_key).await {
        return Ok(Json(cached));
    }

    let categories = sqlx::query_as::<_, BlogCategory>(
        "SELECT id, name, slug, description, icon, color, created_at, updated_at FROM blog_categories ORDER BY name ASC"
    )
    .fetch_all(&state.pool)
    .await?;

    state.categories_cache.insert(cache_key, categories.clone()).await;
    Ok(Json(categories))
}

#[utoipa::path(
    get,
    path = "/api/blog/tags",
    responses(
        (status = 200, description = "Get blog tags", body = [BlogTag])
    )
)]
pub async fn get_tags(State(state): State<AppState>) -> Result<Json<Vec<BlogTag>>, AppError> {
    let cache_key = String::from("tags");
    
    if let Some(cached) = state.tags_cache.get(&cache_key).await {
        return Ok(Json(cached));
    }

    let tags = sqlx::query_as::<_, BlogTag>(
        "SELECT id, name, slug, created_at, updated_at FROM blog_tags ORDER BY name ASC"
    )
    .fetch_all(&state.pool)
    .await?;

    state.tags_cache.insert(cache_key, tags.clone()).await;
    Ok(Json(tags))
}

// ========== Admin Blog Post CRUD ==========

#[derive(Deserialize)]
pub struct BlogPostPayload {
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: String,
    pub content_markdown: Option<String>,
    pub featured_image: Option<String>,
    pub author: Option<String>,
    pub published: Option<bool>,
    pub published_at: Option<String>,
    pub reading_time: Option<i32>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<Vec<String>>,
    pub categories: Option<Vec<i32>>,
    pub tags: Option<Vec<i32>>,
}

// GET /api/blog/admin/posts/:id - Get post by ID (admin, includes unpublished)
pub async fn get_post_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<BlogPost>, AppError> {
    let post = sqlx::query_as::<_, BlogPost>(
        "SELECT id, uuid::text as uuid, title, slug, excerpt, content, content_markdown, featured_image, author, published, published_at, view_count, reading_time, meta_title, meta_description, meta_keywords, created_at, updated_at FROM blog_posts WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?;

    match post {
        Some(p) => Ok(Json(p)),
        None => Err(AppError::NotFound),
    }
}

// POST /api/blog/posts - Create new blog post
pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<BlogPostPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query_scalar::<_, i32>(
        "INSERT INTO blog_posts (title, slug, excerpt, content, content_markdown, featured_image, author, published, published_at, reading_time, meta_title, meta_description, meta_keywords) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, COALESCE($9::timestamp, NOW()), $10, $11, $12, $13) RETURNING id"
    )
    .bind(&payload.title)
    .bind(&payload.slug)
    .bind(&payload.excerpt)
    .bind(&payload.content)
    .bind(&payload.content_markdown)
    .bind(&payload.featured_image)
    .bind(&payload.author)
    .bind(payload.published.unwrap_or(false))
    .bind(&payload.published_at)
    .bind(payload.reading_time.unwrap_or(1))
    .bind(&payload.meta_title)
    .bind(&payload.meta_description)
    .bind(&payload.meta_keywords)
    .fetch_one(&state.pool)
    .await?;

    let post_id = result;

    // Handle categories
    if let Some(categories) = &payload.categories {
        for cat_id in categories {
            let _ = sqlx::query("INSERT INTO blog_post_categories (post_id, category_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
                .bind(post_id)
                .bind(cat_id)
                .execute(&state.pool)
                .await;
        }
    }

    // Handle tags
    if let Some(tags) = &payload.tags {
        for tag_id in tags {
            let _ = sqlx::query("INSERT INTO blog_post_tags (post_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
                .bind(post_id)
                .bind(tag_id)
                .execute(&state.pool)
                .await;
        }
    }

    Ok(Json(serde_json::json!({ "success": true, "id": post_id })))
}

// PUT /api/blog/admin/posts/:id - Update blog post
pub async fn update_post(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<BlogPostPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query(
        "UPDATE blog_posts SET title = $1, slug = $2, excerpt = $3, content = $4, content_markdown = $5, featured_image = $6, author = $7, published = $8, published_at = COALESCE($9::timestamp, published_at), reading_time = $10, meta_title = $11, meta_description = $12, meta_keywords = $13, updated_at = NOW() WHERE id = $14"
    )
    .bind(&payload.title)
    .bind(&payload.slug)
    .bind(&payload.excerpt)
    .bind(&payload.content)
    .bind(&payload.content_markdown)
    .bind(&payload.featured_image)
    .bind(&payload.author)
    .bind(payload.published.unwrap_or(false))
    .bind(&payload.published_at)
    .bind(payload.reading_time.unwrap_or(1))
    .bind(&payload.meta_title)
    .bind(&payload.meta_description)
    .bind(&payload.meta_keywords)
    .bind(id)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    // Update categories: delete old, insert new
    sqlx::query("DELETE FROM blog_post_categories WHERE post_id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if let Some(categories) = &payload.categories {
        for cat_id in categories {
            let _ = sqlx::query("INSERT INTO blog_post_categories (post_id, category_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
                .bind(id)
                .bind(cat_id)
                .execute(&state.pool)
                .await;
        }
    }

    // Update tags: delete old, insert new
    sqlx::query("DELETE FROM blog_post_tags WHERE post_id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if let Some(tags) = &payload.tags {
        for tag_id in tags {
            let _ = sqlx::query("INSERT INTO blog_post_tags (post_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
                .bind(id)
                .bind(tag_id)
                .execute(&state.pool)
                .await;
        }
    }

    Ok(Json(serde_json::json!({ "success": true })))
}

// DELETE /api/blog/admin/posts/:id
pub async fn delete_post(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Delete related categories and tags first
    let _ = sqlx::query("DELETE FROM blog_post_categories WHERE post_id = $1").bind(id).execute(&state.pool).await;
    let _ = sqlx::query("DELETE FROM blog_post_tags WHERE post_id = $1").bind(id).execute(&state.pool).await;

    let result = sqlx::query("DELETE FROM blog_posts WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(serde_json::json!({ "success": true })))
}

// ========== Admin Blog Category CRUD ==========

#[derive(Deserialize)]
pub struct CategoryPayload {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CategoryPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query("INSERT INTO blog_categories (name, slug, description, icon, color) VALUES ($1, $2, $3, $4, $5)")
        .bind(&payload.name)
        .bind(&payload.slug)
        .bind(&payload.description)
        .bind(&payload.icon)
        .bind(&payload.color)
        .execute(&state.pool)
        .await?;

    state.categories_cache.invalidate(&String::from("categories")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn update_category(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<CategoryPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("UPDATE blog_categories SET name = $1, slug = $2, description = $3, icon = $4, color = $5, updated_at = NOW() WHERE id = $6")
        .bind(&payload.name)
        .bind(&payload.slug)
        .bind(&payload.description)
        .bind(&payload.icon)
        .bind(&payload.color)
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    state.categories_cache.invalidate(&String::from("categories")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn delete_category(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let _ = sqlx::query("DELETE FROM blog_post_categories WHERE category_id = $1").bind(id).execute(&state.pool).await;
    let result = sqlx::query("DELETE FROM blog_categories WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    state.categories_cache.invalidate(&String::from("categories")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}

// ========== Admin Blog Tag CRUD ==========

#[derive(Deserialize)]
pub struct TagPayload {
    pub name: String,
    pub slug: String,
}

pub async fn create_tag(
    State(state): State<AppState>,
    Json(payload): Json<TagPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query("INSERT INTO blog_tags (name, slug) VALUES ($1, $2)")
        .bind(&payload.name)
        .bind(&payload.slug)
        .execute(&state.pool)
        .await?;

    state.tags_cache.invalidate(&String::from("tags")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn update_tag(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<TagPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("UPDATE blog_tags SET name = $1, slug = $2, updated_at = NOW() WHERE id = $3")
        .bind(&payload.name)
        .bind(&payload.slug)
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    state.tags_cache.invalidate(&String::from("tags")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn delete_tag(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let _ = sqlx::query("DELETE FROM blog_post_tags WHERE tag_id = $1").bind(id).execute(&state.pool).await;
    let result = sqlx::query("DELETE FROM blog_tags WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    state.tags_cache.invalidate(&String::from("tags")).await;

    Ok(Json(serde_json::json!({ "success": true })))
}
