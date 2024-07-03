use utoipa::OpenApi;

mod root;
mod user;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/", api = root::RootApi),
        (path = "/users", api = user::UserApi),
    ),
    tags(
        (name = "root", description = "Root API"),
        (name = "user", description = "User API")

    )
)]
pub struct ApiDoc;
