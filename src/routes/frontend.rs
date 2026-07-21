use askama::Template;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::{Form, Router};
use axum::routing::get;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use tokio::try_join;
use crate::app::AppState;
use crate::error::AppError;
use crate::models::asset::Asset;
use crate::models::owned_assets::OwnedAsset;
use crate::models::user::{UnauthenticatedUser, User};
use crate::repository::Repository;

pub fn router() ->  Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
        .route("/logout", get(logout))
        .route("/assets", get(assets).post(purchase_asset))
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

async fn index(maybe_user: Option<User>) -> Result<Redirect, AppError> {
    match maybe_user {
        Some(_user) => Ok(Redirect::to("/assets")),
        None => Ok(Redirect::to("/login"))
    }
}

pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    (jar.remove("token"), Redirect::to("/login"))
}

#[derive(Template)]
#[template(path = "assets.html")]
pub struct AssetsPage {
    owned_assets: Vec<OwnedAsset>,
    available_assets: Vec<Asset>,
    user: User
}

pub async fn assets(repository: Repository, user: User) -> Result<Html<String>, AppError> {
    let (owned_assets, available_assets) = try_join!(
        repository.list_owned_assets(user.id()),
        repository.list_assets()
    )?;
    
    let html = AssetsPage {
        owned_assets,
        available_assets,
        user
    }.render()?;

    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct PurchaseAssetForm {
    #[serde(rename = "asset")]
    asset_id: i64,
    unit_value: f64,
    quantity: f64
}

pub async fn purchase_asset(
    repository: Repository,
    user: User,
    Form(request): Form<PurchaseAssetForm>
) -> Result<Redirect, AppError> {
    repository
        .insert_owned_asset(
            user.id(),
            request.asset_id,
            request.quantity,
            request.unit_value
        ).await?;

    Ok(Redirect::to("/assets"))
}

pub mod filters {
    use askama;
    use time::{
        OffsetDateTime, format_description::StaticFormatDescription, macros::format_description
    };

    #[askama::filter_fn]
    pub fn human_datetime(
        datetime: &OffsetDateTime,
        _env: &dyn askama::Values
    ) -> askama::Result<String> {
        const HUMAN_READABLE_FORMAT: StaticFormatDescription =
            format_description!(version =2, "[year]-[month]-[day] [hour]:[minute]");

        datetime
            .format(HUMAN_READABLE_FORMAT)
            .map_err(askama::Error::custom)
    }
}