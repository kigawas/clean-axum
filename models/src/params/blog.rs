use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateBlogParams {
    pub author_id: u32,

    #[validate(length(min = 2))]
    pub title: String,

    #[validate(length(min = 2))]
    pub content: String,
}
