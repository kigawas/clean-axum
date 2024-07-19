use axum::http::StatusCode;

pub trait HTTPError {
    fn to_status_code(&self) -> StatusCode;
}
