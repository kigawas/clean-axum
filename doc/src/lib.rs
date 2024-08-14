use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

mod blog;
mod root;
mod user;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/", api = root::RootApi),
        (path = "/users", api = user::UserApi),
        (path = "/blogs", api = blog::BlogApi),
    ),
    tags(
        (name = "root", description = "Root API"),
        (name = "user", description = "User API"),
        (name = "blog", description = "Blog API"),

    )
)]
struct _ApiDoc;

pub trait ApiDoc {
    fn attach_doc(self) -> Self;
}

impl ApiDoc for Router {
    fn attach_doc(self) -> Self {
        self.merge(SwaggerUi::new("/docs").url("/openapi.json", _ApiDoc::openapi()))
            .merge(Scalar::with_url("/scalar", _ApiDoc::openapi()))
    }
}
