use axum::Router;

pub mod blog;
pub mod root;
pub mod user;

use app::state::AppState;
use blog::create_blog_router;
use root::create_root_router;
use user::create_user_router;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/users", create_user_router())
        .nest("/blogs", create_blog_router())
        .nest("/", create_root_router())
        .with_state(state)
}
