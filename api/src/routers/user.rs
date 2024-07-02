use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};

use app::services::user::{create_user, get_user, search_users};
use app::state::AppState;
use models::orm::TryIntoModel;
use models::params::user::CreateUserParams;
use models::queries::user::UserQuery;
use models::schemas::user::{UserListSchema, UserSchema};

#[utoipa::path(
    post,
    path = "",
    request_body = CreateUserParams,
    responses(
        (status = 201, description = "User created", body = UserSchema),
    )
)]
pub async fn users_post(state: State<AppState>, Json(params): Json<CreateUserParams>) -> Response {
    let res = create_user(&state.conn, params).await;
    match res {
        Ok(user) => {
            let user = user.try_into_model().unwrap();
            return (StatusCode::CREATED, Json(UserSchema::from(user))).into_response();
        }
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "",
    params(
        UserQuery
    ),
    responses(
        (status = 200, description = "List users", body = UserListSchema)
    )
)]
pub async fn users_get(state: State<AppState>, query: Option<Query<UserQuery>>) -> Response {
    let Query(query) = query.unwrap_or_default();
    let res = search_users(&state.conn, query).await;
    match res {
        Ok(users) => {
            let user_schemas = UserListSchema::from(users);
            return (StatusCode::OK, Json(user_schemas)).into_response();
        }
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "Get user", body = UserSchema)
    )
)]
pub async fn users_id_get(state: State<AppState>, Path(id): Path<i32>) -> Response {
    let res: Result<Option<models::domains::user::Model>, models::orm::DbErr> =
        get_user(&state.conn, id).await;
    match res {
        Ok(user) => match user {
            Some(user) => {
                return (StatusCode::OK, Json(UserSchema::from(user))).into_response();
            }
            None => return (StatusCode::NOT_FOUND, "Not found").into_response(),
        },
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
