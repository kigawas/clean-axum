use utoipa::OpenApi;

use crate::routers::root::*;

#[derive(OpenApi)]
#[openapi(paths(root))]
pub struct RootApi;
