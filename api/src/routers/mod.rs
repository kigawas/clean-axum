use axum::{
    routing::{get, post},
    Router,
};

mod root;
mod user;

use app::state::AppState;
use root::root;
use user::{users_get, users_post};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(users_post).get(users_get))
        .with_state(state)
}
