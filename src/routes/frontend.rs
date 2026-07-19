use askama::Template;
use axum::response::Html;
use axum::{Form, Router};
use axum::routing::get;
use serde::Deserialize;
use crate::app::AppState;
use crate::error::AppError;
use crate::models::user::UnauthenticatedUser;
use crate::repository::Repository;

pub fn router() ->  Router<AppState> {
    Router::new()
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
    Form(LoginForm{username, password}): Form<LoginForm>
) -> Result<Html<String>, AppError> {
    let unauth_user = UnauthenticatedUser::new(username, password);
    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExists) => unauth_user.register(repository).await?,
        Err(other_err) => return Err(other_err)
    };
    Ok(Html(user.username().to_string()))
}