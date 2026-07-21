use askama::Template;
use axum::Form;
use axum::response::{Html, IntoResponse, Redirect};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use jwt_simple::prelude::Deserialize;
use crate::error::AppError;
use crate::models::user::{UnauthenticatedUser, User};
use crate::repository::Repository;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

pub async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String
}
pub async fn login(
    repository: Repository,
    jar: CookieJar,
    Form(LoginForm{username, password}): Form<LoginForm>
) -> Result<impl IntoResponse, AppError> {
    let unauthenticated_user = UnauthenticatedUser::new(username, password);
    let mut is_admin: bool = false;
    let user = match unauthenticated_user.authenticate(&repository).await {
        Ok(user) => {
            is_admin = repository.is_user_admin(user.id()).await?;
            user
        },
        Err(AppError::UserDoesNotExists) => unauthenticated_user.register(repository).await?,
        Err(other_err) => return Err(other_err)
    };

    let token = user.clone().auth_token()?;

    let cookie = Cookie::build(("token", token)).http_only(true);
    let mut jar_with_cookie = jar.add(cookie);

    if is_admin {
        let admin_token = user.admin_token()?;
        let admin_cookie = Cookie::build(("admin_token", admin_token)).http_only(true);
        jar_with_cookie = jar_with_cookie.add(admin_cookie);
    }

    Ok((jar_with_cookie, Redirect::to("/")))
}

pub async fn index(maybe_user: Option<User>) -> Result<Redirect, AppError> {
    match maybe_user {
        Some(_user) => Ok(Redirect::to("/assets")),
        None => Ok(Redirect::to("/login"))
    }
}