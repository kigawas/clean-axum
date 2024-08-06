use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use sea_orm::TryIntoModel;

use app::error::UserError;
use app::persistence::user::{create_user, get_user, search_users};
use app::state::AppState;
use models::params::user::CreateUserParams;
use models::queries::user::UserQuery;
use models::schemas::user::{UserListSchema, UserSchema};

use crate::error::ApiError;
use crate::extractor::{Json, Valid};

#[utoipa::path(
    post,
    path = "",
    request_body = CreateUserParams,
    responses(
        (status = 201, description = "User created", body = UserSchema),
        (status = 400, description = "Bad request", body = ApiErrorResponse),
        (status = 422, description = "Validation error", body = ParamsErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn users_post(
    state: State<AppState>,
    Valid(Json(params)): Valid<Json<CreateUserParams>>,
) -> Result<impl IntoResponse, ApiError> {
    let user = create_user(&state.conn, params)
        .await
        .map_err(ApiError::from)?;

    let user = user.try_into_model().unwrap();
    Ok((StatusCode::CREATED, Json(UserSchema::from(user))))
}

#[utoipa::path(
    get,
    path = "",
    params(
        UserQuery
    ),
    responses(
        (status = 200, description = "List users", body = UserListSchema),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn users_get(
    state: State<AppState>,
    query: Option<Query<UserQuery>>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(query) = query.unwrap_or_default();

    let users = search_users(&state.conn, query)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(UserListSchema::from(users)))
}
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "Get user", body = UserSchema),
        (status = 404, description = "Not found", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn users_id_get(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let user = get_user(&state.conn, id).await.map_err(ApiError::from)?;

    user.map(|user| Json(UserSchema::from(user)))
        .ok_or_else(|| UserError::NotFound.into())
}

pub fn create_user_router() -> Router<AppState> {
    Router::new()
        .route("/", post(users_post).get(users_get))
        .route("/:id", get(users_id_get))
}
