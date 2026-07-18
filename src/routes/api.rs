use axum::{Json, Router};
use axum::routing::get;
use serde::Deserialize;

use crate::app::AppState;
use crate::auth::admin::Admin;
use crate::error::AppError;
use crate::models::asset::{Asset, Id};
use crate::repository::Repository;
use crate::routes::api;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/assets",
            get(api::list_assets)
                .post(api::create_asset)
                .patch(update_asset)
        )
}

#[tracing::instrument(skip_all)]
pub async fn list_assets(repository: Repository) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = repository.list_assets().await?;
    Ok(Json(assets))
}

#[derive(Deserialize)]
pub struct CreateAssetRequest {
    name: String,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
pub async fn create_asset(
    _admin: Admin,
    repository: Repository,
    Json(request): Json<CreateAssetRequest>
) -> Result<Json<Asset>, AppError> {
    let new_asset = repository.create_asset(request.name, request.unit_value).await?;

    Ok(Json(new_asset))
}

#[derive(Deserialize)]
pub struct UpdateAssetRequest {
    id: Id,
    name: Option<String>,
    unit_value: Option<f64>,
}

#[tracing::instrument(skip_all)]
pub async fn update_asset(
    _admin: Admin,
    repository: Repository,
    Json(request): Json<UpdateAssetRequest>
) -> Result<Json<Asset>, AppError> {
    match repository.update_asset(request.id, request.name, request.unit_value).await? {
        Some(asset) => Ok(Json(asset.clone())),
        None => Err(AppError::AssetDoesNotExist)
    }
}