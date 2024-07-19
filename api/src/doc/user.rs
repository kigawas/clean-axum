use utoipa::OpenApi;

use models::params::user::CreateUserParams;
use models::schemas::user::{UserListSchema, UserSchema};

use crate::models::{ApiErrorResponse, ParamsErrorResponse};
use crate::routers::user::*;

#[derive(OpenApi)]
#[openapi(
    paths(users_get, users_id_get, users_post),
    components(schemas(
        CreateUserParams,
        UserListSchema,
        UserSchema,
        ApiErrorResponse,
        ParamsErrorResponse,
    ))
)]
pub struct UserApi;
