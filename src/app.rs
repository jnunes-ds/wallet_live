use axum::{Router};
use sqlx::PgPool;
use tokio::net::TcpListener;

use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::routes;
use crate::routes::{api, frontend};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    async fn new() -> color_eyre::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let db = PgPool::connect(database_url.as_str()).await?;
        Ok(Self {
            db
        })
    }
}

pub struct App;

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let filter = tracing::level_filters::LevelFilter::INFO;
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW);

        tracing_subscriber::registry().with(layer.with_filter(filter)).init();

        dotenvy::dotenv()?;
        info!("Starting server...");

        let state = AppState::new().await?;
        let listener = TcpListener::bind("0.0.0.0:3000").await?;
        let router = Router::new()
            .nest("/api", api::router())
            .merge(frontend::router())
            .with_state(state);


        axum::serve(listener, router).await?;

        Ok(())
    }
}