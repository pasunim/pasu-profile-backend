use axum::{extract::{State, Path}, Json};
use serde::Deserialize;
use crate::models::{BlogPost, BlogCategory, BlogTag};
use crate::error::AppError;
use crate::state::AppState;

/// Generate a URL-friendly slug from text.
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

/// Resolve the slug to store: an explicit non-empty slug wins, otherwise one
/// derived from the title. Titles made purely of punctuation slugify to an
/// empty string, which would be unreachable as a URL, so fall back to a
/// timestamp-based slug.
fn resolve_slug(explicit: &Option<String>, title: &str) -> String {
    if let Some(s) = explicit {
        let trimmed = s.trim();
        if !trimmed.is_empty() {
            return slugify(trimmed);
        }
    }
    let derived = slugify(title);
    if derived.is_empty() {
        format!("post-{}", chrono::Utc::now().timestamp())
    } else {
        derived
    }
}

/// Parse an optional client-supplied timestamp. Accepts RFC 3339 (what
/// JavaScript's `toISOString()` produces) as well as a plain naive datetime,
/// so a timezone-qualified value no longer fails the SQL cast.
fn parse_published_at(raw: &Option<String>) -> Result<Option<chrono::NaiveDateTime>, AppError> {
    let Some(value) = raw.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) else {
        return Ok(None);
    };

    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(value) {
        return Ok(Some(dt.naive_utc()));
    }
    for fmt in ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%d %H:%M:%S%.f"] {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(value, fmt) {
            return Ok(Some(dt));
        }
    }
    // Date-only input pins to midnight.
    if let Ok(d) = chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        return Ok(Some(d.and_hms_opt(0, 0, 0).unwrap()));
    }
    Err(AppError::ValidationError(format!(
        "รูปแบบวันที่เผยแพร่ไม่ถูกต้อง: {}",
        value
    )))
}

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
        Some(p) => {
            // Best-effort: a failed counter bump must not fail the read.
            if let Err(e) = sqlx::query("UPDATE blog_posts SET view_count = COALESCE(view_count, 0) + 1 WHERE id = $1")
                .bind(p.id)
                .execute(&state.pool)
                .await
            {
                tracing::warn!("Failed to increment view_count for post {}: {:?}", p.id, e);
            }
            Ok(Json(p))
        }
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
    pub slug: Option<String>,
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

// GET /api/blog/admin/posts - List every post, drafts included.
// The public listing filters to published posts, so the admin panel needs its
// own endpoint or drafts would be invisible there.
pub async fn get_admin_posts(State(state): State<AppState>) -> Result<Json<Vec<BlogPost>>, AppError> {
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
 ORDER BY COALESCE(p.published_at, p.created_at) DESC, p.id DESC
"#
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(posts))
}

// GET /api/blog/admin/posts/:id - Get post by ID (admin, includes unpublished)
pub async fn get_post_by_id(
    Path(id): Path<i32>,
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
 WHERE p.id = $1
"#
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
    let slug = resolve_slug(&payload.slug, &payload.title);
    let published_at = parse_published_at(&payload.published_at)?;

    // One transaction so a post never ends up half-linked to its taxonomy.
    let mut tx = state.pool.begin().await?;

    let post_id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO blog_posts (title, slug, excerpt, content, content_markdown, featured_image, author, published, published_at, reading_time, meta_title, meta_description, meta_keywords) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, COALESCE($9, NOW()), $10, $11, $12, $13) RETURNING id"
    )
    .bind(&payload.title)
    .bind(&slug)
    .bind(&payload.excerpt)
    .bind(&payload.content)
    .bind(&payload.content_markdown)
    .bind(&payload.featured_image)
    .bind(&payload.author)
    .bind(payload.published.unwrap_or(false))
    .bind(published_at)
    .bind(payload.reading_time.unwrap_or(1))
    .bind(&payload.meta_title)
    .bind(&payload.meta_description)
    .bind(&payload.meta_keywords)
    .fetch_one(&mut *tx)
    .await
    .map_err(map_slug_conflict)?;

    link_taxonomy(&mut tx, post_id, &payload.categories, &payload.tags).await?;

    tx.commit().await?;

    Ok(Json(serde_json::json!({ "success": true, "id": post_id })))
}

/// Translate a unique-violation on `slug` into a 400 the admin UI can show,
/// instead of a bare 500.
fn map_slug_conflict(err: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(ref db_err) = err {
        if db_err.code().as_deref() == Some("23505") {
            return AppError::ValidationError(
                "slug นี้ถูกใช้ไปแล้ว กรุณาเปลี่ยน slug หรือชื่อบทความ".to_string(),
            );
        }
    }
    AppError::DatabaseError(err)
}

/// Replace a post's category/tag links. Uses `UNNEST` so each side is a single
/// round trip rather than one query per id, and reports invalid ids instead of
/// silently dropping them.
async fn link_taxonomy(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    post_id: i32,
    categories: &Option<Vec<i32>>,
    tags: &Option<Vec<i32>>,
) -> Result<(), AppError> {
    if let Some(category_ids) = categories {
        if !category_ids.is_empty() {
            sqlx::query(
                "INSERT INTO blog_post_categories (post_id, category_id) SELECT $1, id FROM UNNEST($2::int[]) AS id ON CONFLICT DO NOTHING",
            )
            .bind(post_id)
            .bind(category_ids)
            .execute(&mut **tx)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(ref db) if db.code().as_deref() == Some("23503") => {
                    AppError::ValidationError("มีหมวดหมู่ที่ไม่มีอยู่จริง".to_string())
                }
                other => AppError::DatabaseError(other),
            })?;
        }
    }

    if let Some(tag_ids) = tags {
        if !tag_ids.is_empty() {
            sqlx::query(
                "INSERT INTO blog_post_tags (post_id, tag_id) SELECT $1, id FROM UNNEST($2::int[]) AS id ON CONFLICT DO NOTHING",
            )
            .bind(post_id)
            .bind(tag_ids)
            .execute(&mut **tx)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(ref db) if db.code().as_deref() == Some("23503") => {
                    AppError::ValidationError("มีแท็กที่ไม่มีอยู่จริง".to_string())
                }
                other => AppError::DatabaseError(other),
            })?;
        }
    }

    Ok(())
}

// PUT /api/blog/admin/posts/:id - Update blog post
pub async fn update_post(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<BlogPostPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let slug = resolve_slug(&payload.slug, &payload.title);
    let published_at = parse_published_at(&payload.published_at)?;

    let mut tx = state.pool.begin().await?;

    // published_at precedence: an explicit value wins; otherwise keep the
    // existing one, except when a draft is being published for the first time,
    // where it is stamped now so the post sorts correctly in the public feed.
    let result = sqlx::query(
        "UPDATE blog_posts SET title = $1, slug = $2, excerpt = $3, content = $4, content_markdown = $5, featured_image = $6, author = $7, published = $8,          published_at = CASE              WHEN $9::timestamp IS NOT NULL THEN $9::timestamp              WHEN $8 AND published_at IS NULL THEN NOW()              WHEN $8 AND NOT published THEN NOW()              ELSE published_at          END,          reading_time = $10, meta_title = $11, meta_description = $12, meta_keywords = $13, updated_at = NOW() WHERE id = $14"
    )
    .bind(&payload.title)
    .bind(&slug)
    .bind(&payload.excerpt)
    .bind(&payload.content)
    .bind(&payload.content_markdown)
    .bind(&payload.featured_image)
    .bind(&payload.author)
    .bind(payload.published.unwrap_or(false))
    .bind(published_at)
    .bind(payload.reading_time.unwrap_or(1))
    .bind(&payload.meta_title)
    .bind(&payload.meta_description)
    .bind(&payload.meta_keywords)
    .bind(id)
    .execute(&mut *tx)
    .await
    .map_err(map_slug_conflict)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    // Only rewrite the links the caller actually sent, so a payload that omits
    // `categories` keeps the existing ones instead of clearing them.
    if payload.categories.is_some() {
        sqlx::query("DELETE FROM blog_post_categories WHERE post_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }
    if payload.tags.is_some() {
        sqlx::query("DELETE FROM blog_post_tags WHERE post_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }

    link_taxonomy(&mut tx, id, &payload.categories, &payload.tags).await?;

    tx.commit().await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

// DELETE /api/blog/admin/posts/:id
pub async fn delete_post(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut tx = state.pool.begin().await?;

    // Remove the taxonomy links before the post they reference.
    sqlx::query("DELETE FROM blog_post_categories WHERE post_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM blog_post_tags WHERE post_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    let result = sqlx::query("DELETE FROM blog_posts WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    tx.commit().await?;

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
