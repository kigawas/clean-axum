use axum::{extract::rejection::JsonRejection, http::StatusCode};
use sea_orm::DbErr;

use app::error::UserError;

use super::traits::HTTPError;

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

impl HTTPError for UserError {
    fn to_status_code(&self) -> StatusCode {
        match self {
            UserError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
