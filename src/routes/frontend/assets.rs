use crate::error::AppError;
use crate::models::asset::Asset;
use crate::models::owned_assets::OwnedAsset;
use crate::models::user::User;
use crate::repository::Repository;
use crate::routes::frontend::filters;
use askama::Template;
use axum::Form;
use axum::response::{Html, Redirect};
use jwt_simple::prelude::Deserialize;
use tokio::try_join;

#[derive(Template)]
#[template(path = "assets.html")]
pub struct AssetsPage {
    owned_assets: Vec<OwnedAsset>,
    available_assets: Vec<Asset>,
    user: User,
    is_admin: bool,
}

pub async fn assets(repository: Repository, user: User) -> Result<Html<String>, AppError> {
    let (owned_assets, available_assets) = try_join!(
        repository.list_owned_assets(user.id()),
        repository.list_assets()
    )?;

    let is_admin = repository.is_user_admin(user.id()).await?;

    let html = AssetsPage {
        owned_assets,
        available_assets,
        user,
        is_admin,
    }
    .render()?;

    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct PurchaseAssetForm {
    #[serde(rename = "asset")]
    asset_id: i64,
    unit_value: f64,
    quantity: f64,
}

pub async fn purchase_asset(
    repository: Repository,
    user: User,
    Form(request): Form<PurchaseAssetForm>,
) -> Result<Redirect, AppError> {
    repository
        .insert_owned_asset(
            user.id(),
            request.asset_id,
            request.quantity,
            request.unit_value,
        )
        .await?;

    Ok(Redirect::to("/assets"))
}
