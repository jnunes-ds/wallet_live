use std::collections::HashMap;
use std::sync::Arc;

use axum::{Router};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::models::asset::{Asset, Id};
use crate::routes::api;

#[derive(Clone)]
pub struct AppState {
    pub assets: Arc<Mutex<HashMap<Id, Asset>>>,
    pub db: PgPool,
}

impl AppState {
    async fn new() -> color_eyre::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let db = PgPool::connect(database_url.as_str()).await?;
        Ok(Self {
            assets: Default::default(),
            db
        })
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

        let state = AppState::new().await?;
        let listener = TcpListener::bind("127.0.0.1:3000").await?;
        let router = Router::new()
            .nest("/api", api::router())
            .with_state(state);


        axum::serve(listener, router).await?;

        Ok(())
    }
}