use crate::error::AppError;
use crate::models::user::{UnauthenticatedUser, User};
use crate::repository::Repository;
use askama::Template;
use axum::Form;
use axum::response::{Html, IntoResponse, Redirect};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use jwt_simple::prelude::Deserialize;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage {
    error_message: Option<String>,
}

pub async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage {
        error_message: None,
    }
    .render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}
pub async fn login(
    repository: Repository,
    jar: CookieJar,
    Form(LoginForm { username, password }): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let unauthenticated_user = UnauthenticatedUser::new(username, password);
    let mut is_admin: bool = false;
    let user = match unauthenticated_user.authenticate(&repository).await {
        Ok(user) => {
            is_admin = repository.is_user_admin(user.id()).await?;
            user
        }
        Err(AppError::UserDoesNotExists) => unauthenticated_user.register(repository).await?,
        Err(_other_err) => {
            let html = LoginPage {
                error_message: Some("Incorrect username or password.".to_string()),
            }
            .render()?;
            return Ok(Html(html).into_response());
        }
    };

    let token = user.clone().auth_token()?;

    let cookie = Cookie::build(("token", token)).http_only(true);
    let mut jar_with_cookie = jar.add(cookie);

    if is_admin {
        let admin_token = user.admin_token()?;
        let admin_cookie = Cookie::build(("admin_token", admin_token)).http_only(true);
        jar_with_cookie = jar_with_cookie.add(admin_cookie);
    }

    Ok((jar_with_cookie, Redirect::to("/")).into_response())
}

pub async fn index(maybe_user: Option<User>) -> Result<Redirect, AppError> {
    match maybe_user {
        Some(_user) => Ok(Redirect::to("/assets")),
        None => Ok(Redirect::to("/login")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::response::IntoResponse;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_login_invalid_credentials(db: PgPool) {
        let repo = Repository::from(db);

        // 1. Register a user "testuser" by calling login once (it registers user if not exists)
        let jar = CookieJar::new();
        let form = Form(LoginForm {
            username: "testuser".to_string(),
            password: "correctpassword".to_string(),
        });

        let response = login(repo.clone(), jar.clone(), form)
            .await
            .expect("login should register user successfully");
        let response = response.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::SEE_OTHER); // Redirects to "/" on success

        // 2. Try to login with wrong password
        let wrong_form = Form(LoginForm {
            username: "testuser".to_string(),
            password: "wrongpassword".to_string(),
        });

        let response = login(repo, jar, wrong_form).await.expect(
            "login with wrong password should succeed in returning the login page with error",
        );
        let response = response.into_response();

        // The user should stay on the login page normally, so status is OK (200)
        assert_eq!(response.status(), axum::http::StatusCode::OK);

        // Verify the HTML response contains the error message
        let body_bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("failed to read body");
        let body_str = String::from_utf8(body_bytes.to_vec()).expect("invalid utf-8");
        assert!(body_str.contains("Incorrect username or password."));
    }
}
