use utoipa::OpenApi;

use api::routers::root::*;

#[derive(OpenApi)]
#[openapi(paths(root))]
pub(super) struct RootApi;
