use axum::Router;

pub mod root;
pub mod user;

use app::state::AppState;
use root::create_root_router;
use user::create_user_router;

// TODO: middleware, testing, logging
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/users", create_user_router(state.clone()))
        .nest("/", create_root_router(state))
}
