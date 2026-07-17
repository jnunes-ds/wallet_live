use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use crate::app::AppState;

const ADMIN_SECRET_KEY: &str = "im-the-admin";

pub struct Admin;

impl FromRequestParts<AppState> for Admin {
    type Rejection = &'static str;
    async fn from_request_parts(parts: &mut Parts, _state: &AppState) -> Result<
        Self, Self::Rejection
    > {
        if let Some(auth) = parts.headers.get(AUTHORIZATION) {
            if auth == ADMIN_SECRET_KEY {
                 Ok(Admin)
            } else {
                 Err("Invalid Credentials")
            }
        } else {
             Err("Missing Authorization Header")
        }
    }
}