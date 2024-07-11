use axum::{extract::rejection::JsonRejection, http::StatusCode};
use models::orm::DbErr;

pub trait HTTPError {
    fn to_status_code(&self) -> StatusCode;
}

impl HTTPError for JsonRejection {
    fn to_status_code(&self) -> StatusCode {
        match self {
            JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

impl HTTPError for DbErr {
    fn to_status_code(&self) -> StatusCode {
        match self {
            DbErr::ConnectionAcquire(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR, // TODO:: more granularity
        }
    }
}
