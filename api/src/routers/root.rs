#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "Hello world", body = &str)
    )
)]
pub async fn root() -> &'static str {
    "Hello, World!"
}
