use axum::response::{IntoResponse, Response};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use app::services::user::{create_user, search_users};
use app::state::AppState;
use models::orm::TryIntoModel;
use models::params::user::CreateUser;
use models::queries::user::UserQuery;
use models::schemas::user::{UserListSchema, UserSchema};

// TODO: use macro to simpify

pub async fn users_post(state: State<AppState>, Json(params): Json<CreateUser>) -> Response {
    let user = create_user(&state.conn, params).await;
    match user {
        Ok(user) => {
            let user = user.try_into_model().unwrap();
            return (StatusCode::CREATED, Json(UserSchema::from(user))).into_response();
        }
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn users_get(state: State<AppState>, query: Option<Query<UserQuery>>) -> Response {
    let Query(query) = query.unwrap_or_default();
    let user = search_users(&state.conn, query).await;
    match user {
        Ok(users) => {
            let user_schemas = UserListSchema::from(users);
            return (StatusCode::OK, Json(user_schemas)).into_response();
        }
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
