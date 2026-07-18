use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Missing Authorization Headers")]
    MissingAuthorization,
    #[error("Invalid Credentials")]
    InvalidCredentials,
    #[error("Asset does not exists")]
    AssetDoesNotExist,
    #[error(transparent)]
    Database(#[from] sqlx::Error)
}

pub struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };

        let status = match self {
            Self::MissingAuthorization => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::AssetDoesNotExist => StatusCode::NOT_FOUND,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR
        };

        (status, Json(error_response)).0.into_response()
    }
}