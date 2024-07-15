use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use models::orm::DbErr;

use super::core::HTTPError;
use super::user::UserError;
use crate::models::ErrorResponse;

pub struct ApiError(anyhow::Error);

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let err = self.0;
        let (status, message) = if let Some(err) = err.downcast_ref::<DbErr>() {
            tracing::error!(%err, "error from db");
            (err.to_status_code(), "DB error".to_string()) // hide the detail
        } else if let Some(err) = err.downcast_ref::<UserError>() {
            (err.to_status_code(), err.to_string())
        } else if let Some(err) = err.downcast_ref::<JsonRejection>() {
            (err.to_status_code(), err.to_string())
        } else {
            tracing::error!(%err, "error from other source");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown error".to_string(),
            )
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}
