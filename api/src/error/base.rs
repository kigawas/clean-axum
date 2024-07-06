use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use models::orm::DbErr;

use super::UserError;
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
        let (status, message) = if err.is::<DbErr>() {
            tracing::error!(%err, "error from db");
            // hide the detail
            (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string())
        } else if err.is::<UserError>() {
            let err = err.downcast_ref::<UserError>().unwrap();
            let status = match err {
                UserError::NotFound => StatusCode::NOT_FOUND,
            };
            (status, err.to_string())
        } else if err.is::<JsonRejection>() {
            let err = err.downcast_ref::<JsonRejection>().unwrap();
            (StatusCode::BAD_REQUEST, err.to_string())
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
