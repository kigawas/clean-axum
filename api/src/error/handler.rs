use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbErr;

use app::error::UserError;

use super::{ApiError, HTTPError};
use crate::models::ApiErrorResponse;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let err = self.0;

        let (status, message) = if let Some(err) = err.downcast_ref::<DbErr>() {
            tracing::error!(%err, "error from db:");
            (err.to_status_code(), "DB error".to_string()) // hide the detail
        } else if let Some(err) = err.downcast_ref::<UserError>() {
            (err.to_status_code(), err.to_string())
        } else if let Some(err) = err.downcast_ref::<JsonRejection>() {
            tracing::error!(%err, "error from extractor:");
            (err.to_status_code(), err.to_string())
        } else {
            tracing::error!(%err, "error from other source:");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown error".to_string(),
            )
        };

        (status, Json(ApiErrorResponse { message })).into_response()
    }
}
