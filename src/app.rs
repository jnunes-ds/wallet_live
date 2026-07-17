use std::collections::HashMap;
use std::sync::Arc;

use axum::{Router};

use tokio::net::TcpListener;
use tokio::sync::Mutex;

use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::routes::api;
use crate::models::{Asset, Id};

#[derive(Clone)]
pub struct AppState {
    pub(crate) assets: Arc<Mutex<HashMap<Id, Asset>>>
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
            .nest("/api", api::router())
            .with_state(AppState::new());


        axum::serve(listener, router).await?;

        Ok(())
    }
}