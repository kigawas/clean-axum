use axum::{
    routing::{get, post},
    Router,
};

pub mod root;
pub mod user;

use app::state::AppState;
use root::root;
use user::{users_get, users_id_get, users_post};

// TODO: middleware and error handling
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/users", create_user_router(state))
}

fn create_user_router(state: AppState) -> Router {
    Router::new()
        .route("/", post(users_post).get(users_get))
        .route("/:id", get(users_id_get))
        .with_state(state)
}
