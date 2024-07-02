use utoipa::OpenApi;

use crate::routers::user::*;
use models::params::user::CreateUserParams;
use models::schemas::user::{UserListSchema, UserSchema};

#[derive(OpenApi)]
#[openapi(
    paths(users_get, users_id_get, users_post),
    components(schemas(CreateUserParams, UserListSchema, UserSchema))
)]
pub struct UserApi;
