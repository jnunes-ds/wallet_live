use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::State;
use axum::routing::get;
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::models::Asset;

#[derive(Clone)]
pub struct AppState {
    assets: Arc<Mutex<Vec<Asset>>>
}

impl AppState {
    fn new() -> Self {
        Self {
            assets: Default::default()
        }
    }
}

pub struct App;

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();

        info!("Starting server...");

        let listener = TcpListener::bind("127.0.0.1:3000").await?;
        let router = Router::new()
            .route("/", get(list_assets).post(create_asset))
            .with_state(AppState::new());


        axum::serve(listener, router).await?;

        Ok(())
    }
}

#[tracing::instrument(skip_all)]
async fn list_assets(state: State<AppState>) -> Json<Vec<Asset>> {
    let assets = state.assets.lock().await;
    Json(assets.clone())
}

#[derive(Deserialize)]
struct CreateAssetRequest {
    name: String,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
async fn create_asset(state: State<AppState>, Json(request): Json<CreateAssetRequest>) -> Json<Asset> {
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