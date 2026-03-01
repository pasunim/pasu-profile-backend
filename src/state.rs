use sqlx::{Pool, Postgres};
use crate::cache::AppCache;
use crate::models::{About, Skill, ExperienceTimeline, Project, SocialLink, BlogCategory, BlogTag};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub about_cache: AppCache<String, About>,
    pub skills_cache: AppCache<String, Vec<Skill>>,
    pub experience_cache: AppCache<String, Vec<ExperienceTimeline>>,
    pub projects_cache: AppCache<String, Vec<Project>>,
    pub socials_cache: AppCache<String, Vec<SocialLink>>,
    pub categories_cache: AppCache<String, Vec<BlogCategory>>,
    pub tags_cache: AppCache<String, Vec<BlogTag>>,
}
