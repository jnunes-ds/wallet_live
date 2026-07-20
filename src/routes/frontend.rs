use askama::Template;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::{Form, Router};
use axum::routing::get;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use crate::app::AppState;
use crate::error::AppError;
use crate::models::user::{UnauthenticatedUser, User};
use crate::repository::Repository;

pub fn router() ->  Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String
}
async fn login(
    repository: Repository,
    jar: CookieJar,
    Form(LoginForm{username, password}): Form<LoginForm>
) -> Result<impl IntoResponse, AppError> {
    let unauthenticated_user = UnauthenticatedUser::new(username, password);
    let user = match unauthenticated_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExists) => unauthenticated_user.register(repository).await?,
        Err(other_err) => return Err(other_err)
    };

    let token = user.auth_token()?;

    let cookie = Cookie::build(("token", token)).http_only(true);

    Ok((jar.add(cookie), Redirect::to("/")))
}

async fn index(maybe_user: Option<User>) -> Result<Response, AppError> {
    match maybe_user {
        Some(user) => Ok(Html(format!("Hello, {}", user.username())).into_response()),
        None => Ok(Redirect::to("/login").into_response())
    }
}