use utoipa::OpenApi;

use api::routers::root::*;

#[derive(OpenApi)]
#[openapi(paths(root_get))]
pub(super) struct RootApi;
