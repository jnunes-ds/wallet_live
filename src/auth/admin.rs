use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use crate::app::AppState;
use crate::error::AppError;

pub struct Admin;

impl FromRequestParts<AppState> for Admin {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _state: &AppState) -> Result<
        Self, Self::Rejection
    > {
        let admin_secret_key = std::env::var("ADMIN_SECRET_KEY").unwrap();
        if let Some(auth) = parts.headers.get(AUTHORIZATION) {
            if auth == admin_secret_key.as_str() {
                 Ok(Admin)
            } else {
                 Err(AppError::MissingAuthorization)
            }
        } else {
             Err(AppError::InvalidCredentials)
        }
    }
}