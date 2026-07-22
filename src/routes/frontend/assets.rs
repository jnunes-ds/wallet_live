use crate::error::AppError;
use crate::models::asset::Asset;
use crate::models::owned_assets::{OwnedAsset, format_brl};
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
    total_invested: String,
    total_current_value: String,
    total_delta: String,
    is_delta_positive: bool,
}

pub async fn assets(repository: Repository, user: User) -> Result<Html<String>, AppError> {
    let (owned_assets, available_assets) = try_join!(
        repository.list_owned_assets(user.id()),
        repository.list_assets()
    )?;

    let is_admin = repository.is_user_admin(user.id()).await?;

    let mut total_current_value = 0.0;
    let mut total_delta = 0.0;

    for asset in &owned_assets {
        total_current_value += asset.quantity_owned * asset.unit_value;
        total_delta += asset.value_delta;
    }

    let total_invested = total_current_value - total_delta;
    let is_delta_positive = total_delta >= 0.0;

    let total_invested = format_brl(total_invested);
    let total_current_value = format_brl(total_current_value);
    let total_delta = format_brl(total_delta);

    let html = AssetsPage {
        owned_assets,
        available_assets,
        user,
        is_admin,
        total_invested,
        total_current_value,
        total_delta,
        is_delta_positive,
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
