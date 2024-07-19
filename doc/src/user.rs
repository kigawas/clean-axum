use utoipa::OpenApi;

use models::params::user::CreateUserParams;
use models::schemas::user::{UserListSchema, UserSchema};

use api::models::{ApiErrorResponse, ParamsErrorResponse};
use api::routers::user::*;

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
pub(super) struct UserApi;
