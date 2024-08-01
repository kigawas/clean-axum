use axum::Router;

pub mod root;
pub mod user;

use app::state::AppState;
use root::create_root_router;
use user::create_user_router;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/users", create_user_router())
        .nest("/", create_root_router())
        .with_state(state)
}
