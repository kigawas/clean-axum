use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use sea_orm::TryIntoModel;

use app::persistence::blog::{create_blog, search_blogs};
use app::state::AppState;
use models::params::blog::CreateBlogParams;
use models::queries::blog::BlogQuery;
use models::schemas::blog::{BlogListSchema, BlogSchema};

use crate::error::ApiError;
use crate::extractor::{Json, Valid};

#[utoipa::path(
    post,
    path = "",
    request_body = CreateBlogParams,
    responses(
        (status = 201, description = "Blog created", body = BlogSchema),
        (status = 400, description = "Bad request", body = ApiErrorResponse),
        (status = 422, description = "Validation error", body = ParamsErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn blogs_post(
    state: State<AppState>,
    Valid(Json(params)): Valid<Json<CreateBlogParams>>,
) -> Result<impl IntoResponse, ApiError> {
    let blog = create_blog(&state.conn, params)
        .await
        .map_err(ApiError::from)?;

    let blog = blog.try_into_model().unwrap();
    Ok((StatusCode::CREATED, Json(BlogSchema::from(blog))))
}

#[utoipa::path(
    get,
    path = "",
    params(
        BlogQuery
    ),
    responses(
        (status = 200, description = "List blogs", body = BlogListSchema),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn blogs_get(
    state: State<AppState>,
    query: Option<Query<BlogQuery>>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(query) = query.unwrap_or_default();

    let blogs = search_blogs(&state.conn, query)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(BlogListSchema::from(blogs)))
}

pub fn create_blog_router() -> Router<AppState> {
    Router::new().route("/", get(blogs_get).post(blogs_post))
}
