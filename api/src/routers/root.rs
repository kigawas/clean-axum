use axum::{extract::State, routing::get, Router};
use sea_orm::{ConnectionTrait, Statement};

use app::state::AppState;

use crate::error::ApiError;

#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "Hello world", body = String)
    )
)]
async fn root_get(state: State<AppState>) -> Result<String, ApiError> {
    let result = state
        .conn
        .query_one(Statement::from_string(
            state.conn.get_database_backend(),
            "SELECT 'Hello, World from DB!'",
        ))
        .await
        .map_err(ApiError::from)?;

    result.unwrap().try_get_by(0).map_err(|e| e.into())
}

pub fn create_root_router() -> Router<AppState> {
    Router::new().route("/", get(root_get))
}
