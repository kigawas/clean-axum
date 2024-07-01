use utoipa::OpenApi;

mod user;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/users", api = user::UserApi)
    ),
    tags(
        (name = "user", description = "User API")
    )
)]
pub struct ApiDoc;
