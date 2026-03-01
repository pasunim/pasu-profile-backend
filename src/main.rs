mod db;
pub mod error;
mod handlers;
mod middleware;
pub mod models;

use axum::{
    routing::{get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
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

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let swagger_router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route_layer(axum::middleware::from_fn(middleware::auth_middleware));

    let app = Router::new()
        .merge(swagger_router)
        .route("/", get(|| async { "Welcome to PASU.APP" }))
        // Public: About
        .route("/api/about", get(handlers::about::get_about).post(handlers::about::update_about))
        // Public + Admin: Skills
        .route("/api/skills", get(handlers::skills::get_skills).post(handlers::skills::create_skill))
        .route("/api/skills/:id", put(handlers::skills::update_skill).delete(handlers::skills::delete_skill))
        // Public + Admin: Experience
        .route("/api/experience", get(handlers::experience::get_experience))
        .route("/api/experience/timeline", post(handlers::experience::create_timeline))
        .route("/api/experience/timeline/:id", put(handlers::experience::update_timeline).delete(handlers::experience::delete_timeline))
        // Public + Admin: Projects
        .route("/api/projects", get(handlers::projects::get_projects).post(handlers::projects::create_project))
        .route("/api/experience/projects", get(handlers::projects::get_projects).post(handlers::projects::create_project))
        .route("/api/experience/projects/:id", put(handlers::projects::update_project).delete(handlers::projects::delete_project))
        // Public + Admin: Contact
        .route("/api/contact", get(handlers::contact::get_contact_info).post(handlers::contact::submit_contact_message))
        .route("/api/contact/info", post(handlers::contact::update_contact_info))
        .route("/api/contact/socials", get(handlers::contact::get_social_links).post(handlers::contact::create_social))
        .route("/api/contact/socials/:id", put(handlers::contact::update_social).delete(handlers::contact::delete_social))
        .route("/api/contact/messages", get(handlers::contact::get_messages).delete(handlers::contact::delete_message))
        // Public + Admin: Blog
        .route("/api/blog/posts", get(handlers::blog::get_posts).post(handlers::blog::create_post))
        .route("/api/blog/posts/:slug", get(handlers::blog::get_post_by_slug))
        .route("/api/blog/admin/posts/:id", get(handlers::blog::get_post_by_id).put(handlers::blog::update_post).delete(handlers::blog::delete_post))
        .route("/api/blog/categories", get(handlers::blog::get_categories).post(handlers::blog::create_category))
        .route("/api/blog/categories/:id", put(handlers::blog::update_category).delete(handlers::blog::delete_category))
        .route("/api/blog/tags", get(handlers::blog::get_tags).post(handlers::blog::create_tag))
        .route("/api/blog/tags/:id", put(handlers::blog::update_tag).delete(handlers::blog::delete_tag))
        // Admin: Auth
        .route("/api/admin/login", post(handlers::admin::login))
        // Upload
        .route("/api/upload", post(handlers::upload::upload_image))
        // Health Checks
        .route("/health", get(handlers::health::health))
        .route("/health/ready", get(handlers::health::readiness))
        .layer(cors)
        .with_state(pool);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
