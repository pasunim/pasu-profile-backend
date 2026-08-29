mod cache;
mod db;
pub mod error;
mod handlers;
mod middleware;
mod ratelimit;
pub mod models;
mod state;

use state::AppState;

use axum::{
    routing::{get, post, put},
    Router,
};
use axum::http::{header, Method};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::about::get_about,
        handlers::skills::get_skills,
        handlers::experience::get_experience,
        handlers::projects::get_projects,
        handlers::contact::get_contact_info,
        handlers::contact::get_social_links,
        handlers::contact::submit_contact_message,
        handlers::blog::get_posts,
        handlers::blog::get_post_by_slug,
        handlers::blog::get_categories,
        handlers::blog::get_tags,
        handlers::admin::login,
        handlers::upload::upload_image,
        handlers::health::health,
        handlers::health::readiness
    ),
    components(
        schemas(
            models::About,
            models::Skill,
            models::ExperienceTimeline,
            models::Project,
            models::ContactInfo,
            models::SocialLink,
            models::ContactMessage,
            models::BlogCategory,
            models::BlogTag,
            models::BlogPost,
            handlers::contact::ContactMessagePayload,
            handlers::admin::LoginPayload,
            handlers::admin::LoginResponse,
            handlers::upload::UploadResponse,
            handlers::health::HealthResponse,
            handlers::health::ReadinessResponse
        )
    ),
    tags(
        (name = "pasu-profile", description = "Pasu Profile Backend API"),
        (name = "health", description = "Health Check Endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pasu_profile_backend=debug,tower_http=debug,axum=warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db::init_pool().await?;

    let state = AppState {
        pool: pool.clone(),
        about_cache: crate::cache::AppCache::new(100, 300),
        skills_cache: crate::cache::AppCache::new(100, 300),
        experience_cache: crate::cache::AppCache::new(100, 300),
        projects_cache: crate::cache::AppCache::new(100, 300),
        socials_cache: crate::cache::AppCache::new(100, 300),
        categories_cache: crate::cache::AppCache::new(100, 300),
        tags_cache: crate::cache::AppCache::new(100, 300),
        // 10 login attempts per IP per 5 minutes.
        login_limiter: crate::ratelimit::LoginRateLimiter::new(10, 300),
    };

    // Swagger UI is itself behind admin auth so the API surface is not public.
    let swagger_router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route_layer(axum::middleware::from_fn(middleware::auth_middleware));

    // Restrict CORS to configured origins. `Any` cannot be combined with
    // credentials, and a wildcard would let any site call the API directly.
    let allowed_origins: Vec<axum::http::HeaderValue> =
        std::env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
            .split(',')
            .map(str::trim)
            .filter(|o| !o.is_empty())
            .filter_map(|o| match o.parse::<axum::http::HeaderValue>() {
                Ok(v) => Some(v),
                Err(_) => {
                    tracing::warn!("Ignoring invalid origin in ALLOWED_ORIGINS: {}", o);
                    None
                }
            })
            .collect();

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true);

    // Public, read-only endpoints.
    let public_router = Router::new()
        .route("/", get(|| async { "Welcome to PASU.APP" }))
        .route("/api/about", get(handlers::about::get_about))
        .route("/api/skills", get(handlers::skills::get_skills))
        .route("/api/experience", get(handlers::experience::get_experience))
        .route("/api/projects", get(handlers::projects::get_projects))
        .route("/api/experience/projects", get(handlers::projects::get_projects))
        .route("/api/contact", get(handlers::contact::get_contact_info))
        .route("/api/contact/socials", get(handlers::contact::get_social_links))
        .route("/api/blog/posts", get(handlers::blog::get_posts))
        .route("/api/blog/posts/:slug", get(handlers::blog::get_post_by_slug))
        .route("/api/blog/categories", get(handlers::blog::get_categories))
        .route("/api/blog/tags", get(handlers::blog::get_tags))
        // Submitting a contact message is intentionally public.
        .route("/api/contact", post(handlers::contact::submit_contact_message))
        .route("/api/admin/login", post(handlers::admin::login))
        .route("/health", get(handlers::health::health))
        .route("/health/ready", get(handlers::health::readiness));

    // Everything that mutates data, or exposes non-public data, requires auth.
    let admin_router = Router::new()
        .route("/api/about", post(handlers::about::update_about))
        .route("/api/skills", post(handlers::skills::create_skill))
        .route("/api/skills/:id", put(handlers::skills::update_skill).delete(handlers::skills::delete_skill))
        .route("/api/experience/timeline", post(handlers::experience::create_timeline))
        .route("/api/experience/timeline/:id", put(handlers::experience::update_timeline).delete(handlers::experience::delete_timeline))
        .route("/api/projects", post(handlers::projects::create_project))
        .route("/api/projects/:id", put(handlers::projects::update_project).delete(handlers::projects::delete_project))
        .route("/api/experience/projects", post(handlers::projects::create_project))
        .route("/api/experience/projects/:id", put(handlers::projects::update_project).delete(handlers::projects::delete_project))
        .route("/api/contact/info", post(handlers::contact::update_contact_info))
        .route("/api/contact/socials", post(handlers::contact::create_social))
        .route("/api/contact/socials/:id", put(handlers::contact::update_social).delete(handlers::contact::delete_social))
        .route("/api/contact/messages", get(handlers::contact::get_messages).delete(handlers::contact::delete_message))
        .route("/api/blog/posts", post(handlers::blog::create_post))
        .route("/api/blog/admin/posts", get(handlers::blog::get_admin_posts))
        .route("/api/blog/admin/posts/:id", get(handlers::blog::get_post_by_id).put(handlers::blog::update_post).delete(handlers::blog::delete_post))
        .route("/api/blog/categories", post(handlers::blog::create_category))
        .route("/api/blog/categories/:id", put(handlers::blog::update_category).delete(handlers::blog::delete_category))
        .route("/api/blog/tags", post(handlers::blog::create_tag))
        .route("/api/blog/tags/:id", put(handlers::blog::update_tag).delete(handlers::blog::delete_tag))
        .route("/api/upload", post(handlers::upload::upload_image))
        .route_layer(axum::middleware::from_fn(middleware::auth_middleware));

    let app = Router::new()
        .merge(swagger_router)
        .merge(public_router)
        .merge(admin_router)
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
        )
        .layer(cors)
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
