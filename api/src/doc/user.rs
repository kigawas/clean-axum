use utoipa::OpenApi;

use models::params::user::CreateUserParams;
use models::schemas::user::{UserListSchema, UserSchema};

use crate::models::ErrorResponse;
use crate::routers::user::*;

#[derive(OpenApi)]
#[openapi(
    paths(users_get, users_id_get, users_post),
    components(schemas(CreateUserParams, UserListSchema, UserSchema, ErrorResponse))
)]
pub struct UserApi;
