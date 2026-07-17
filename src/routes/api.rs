use std::collections::HashMap;
use axum::extract::State;
use axum::{Json, Router};
use axum::routing::get;
use serde::Deserialize;
use crate::app::AppState;
use crate::auth::admin::Admin;
use crate::error::AppError;
use crate::models::asset::{Asset, Id};
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
pub async fn list_assets(state: State<AppState>) -> Json<HashMap<Id, Asset>> {
    let assets = state.assets.lock().await;
    Json(assets.clone())
}

#[derive(Deserialize)]
pub struct CreateAssetRequest {
    name: String,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
pub async fn create_asset(
    _admin: Admin,
    state: State<AppState>,
    Json(request): Json<CreateAssetRequest>
) -> Json<Asset> {
    let mut assets = state.assets.lock().await;

    let id = assets.values().map(|asset| asset.id).max().unwrap_or_default() + 1;

    let asset = Asset {
        id,
        name: request.name,
        unit_value: request.unit_value,
    };

    assets.insert(id, asset.clone());

    Json(asset)
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
    state: State<AppState>,
    Json(request): Json<UpdateAssetRequest>
) -> Result<Json<Asset>, AppError> {
    let mut assets = state.assets.lock().await;
    if let Some(existing_asset) = assets.get_mut(&request.id) {
        existing_asset.name = request.name.unwrap_or(existing_asset.name.clone());
        existing_asset.unit_value = request.unit_value.unwrap_or(existing_asset.unit_value);
        Ok(Json(existing_asset.clone()))
    } else {
        Err(AppError::AssetDoesNotExist)
    }
}