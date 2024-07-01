use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, Default, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
pub struct UserQuery {
    #[param(nullable = true)]
    pub username: String,
}
