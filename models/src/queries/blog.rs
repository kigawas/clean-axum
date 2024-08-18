use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, Default, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
pub struct BlogQuery {
    #[param(nullable = true)]
    pub title: String,
}
