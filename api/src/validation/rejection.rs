use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use validator::ValidationErrors;

use crate::models::ValidationErrorResponse;

#[derive(Debug)]
pub enum ValidationRejection<V, E> {
    Validator(V), // Validation errors
    Extractor(E), // Extraction errors, e.g. axum's JsonRejection
}

impl<V: std::fmt::Display, E: std::fmt::Display> std::fmt::Display for ValidationRejection<V, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationRejection::Validator(v) => write!(f, "{v}"),
            ValidationRejection::Extractor(e) => write!(f, "{e}"),
        }
    }
}

impl<V: std::error::Error + 'static, E: std::error::Error + 'static> std::error::Error
    for ValidationRejection<V, E>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ValidationRejection::Validator(v) => Some(v),
            ValidationRejection::Extractor(e) => Some(e),
        }
    }
}

impl<V: serde::Serialize + std::error::Error, E: IntoResponse> IntoResponse
    for ValidationRejection<V, E>
{
    fn into_response(self) -> Response {
        match self {
            ValidationRejection::Validator(v) => {
                tracing::error!("Validation error: {v}");
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    axum::Json(ValidationErrorResponse::from(v)),
                )
                    .into_response()
            }
            // logged by ApiError
            ValidationRejection::Extractor(e) => e.into_response(),
        }
    }
}

pub type ValidRejection<E> = ValidationRejection<ValidationErrors, E>;

impl<E> From<ValidationErrors> for ValidRejection<E> {
    fn from(v: ValidationErrors) -> Self {
        Self::Validator(v)
    }
}
