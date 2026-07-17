use axum::extract::State;
use axum::{Json, Router};
use axum::routing::get;
use serde::Deserialize;
use crate::app::AppState;
use crate::models::Asset;
use crate::routes::api;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assets", get(api::list_assets).post(api::create_asset))
}

#[tracing::instrument(skip_all)]
pub async fn list_assets(state: State<AppState>) -> Json<Vec<Asset>> {
    let assets = state.assets.lock().await;
    Json(assets.clone())
}

#[derive(Deserialize)]
pub struct CreateAssetRequest {
    name: String,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
pub async fn create_asset(state: State<AppState>, Json(request): Json<CreateAssetRequest>) -> Json<Asset> {
    let mut assets = state.assets.lock().await;

    let id = assets.iter().map(|asset| asset.id).max().unwrap_or_default() + 1;

    let asset = Asset {
        id,
        name: request.name,
        unit_value: request.unit_value,
    };

    assets.push(asset.clone());

    Json(asset)
}