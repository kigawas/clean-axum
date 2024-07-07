use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use app::services::user::{create_user, get_user, search_users};
use app::state::AppState;
use models::orm::TryIntoModel;
use models::params::user::CreateUserParams;
use models::queries::user::UserQuery;
use models::schemas::user::{UserListSchema, UserSchema};

use crate::error::{ApiError, UserError};
use crate::models::Json;

#[utoipa::path(
    post,
    path = "",
    request_body = CreateUserParams,
    responses(
        (status = 201, description = "User created", body = UserSchema),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
async fn users_post(
    state: State<AppState>,
    Json(params): Json<CreateUserParams>,
) -> Result<impl IntoResponse, ApiError> {
    let res = create_user(&state.conn, params).await;
    match res {
        Ok(user) => {
            let user = user.try_into_model().unwrap();
            Ok((StatusCode::CREATED, Json(UserSchema::from(user))))
        }
        Err(err) => Err(err.into()),
    }
}

#[utoipa::path(
    get,
    path = "",
    params(
        UserQuery
    ),
    responses(
        (status = 200, description = "List users", body = UserListSchema),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
async fn users_get(
    state: State<AppState>,
    query: Option<Query<UserQuery>>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(query) = query.unwrap_or_default();
    let res = search_users(&state.conn, query).await;
    match res {
        Ok(users) => Ok(Json(UserListSchema::from(users))),
        Err(err) => Err(err.into()),
    }
}
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "Get user", body = UserSchema),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
async fn users_id_get(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let res = get_user(&state.conn, id).await;
    match res {
        Ok(user) => match user {
            Some(user) => Ok(Json(UserSchema::from(user))),
            None => Err(UserError::NotFound.into()),
        },
        Err(err) => Err(err.into()),
    }
}

pub fn create_user_router(state: AppState) -> Router {
    Router::new()
        .route("/", post(users_post).get(users_get))
        .route("/:id", get(users_id_get))
        .with_state(state)
}
