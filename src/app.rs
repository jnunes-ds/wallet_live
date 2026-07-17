use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub struct App;

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();

        info!("Starting server...");

        let listener = TcpListener::bind("127.0.0.1:3000").await?;
        let router = Router::new().route("/", get(hello_world));


        axum::serve(listener, router).await?;

        Ok(())
    }
}

#[tracing::instrument]
async fn hello_world() -> &'static str {
    "Hello, world!"
}