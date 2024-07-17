use axum::{
    extract::FromRequest,
    response::{IntoResponse, Response},
};
use validator::Validate;

use crate::error::ApiError;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

impl<T> IntoResponse for Json<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

impl<T: Validate> Validate for Json<T> {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        self.0.validate()
    }
}
