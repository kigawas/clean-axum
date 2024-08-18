use utoipa::OpenApi;

use models::params::blog::CreateBlogParams;
use models::schemas::blog::{BlogListSchema, BlogSchema};

use api::models::{ApiErrorResponse, ParamsErrorResponse};
use api::routers::blog::*;

#[derive(OpenApi)]
#[openapi(
    paths(blogs_get, blogs_post),
    components(schemas(
        CreateBlogParams,
        BlogListSchema,
        BlogSchema,
        ApiErrorResponse,
        ParamsErrorResponse,
    ))
)]
pub(super) struct BlogApi;
