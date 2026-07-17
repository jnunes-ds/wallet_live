mod app;
mod models;
mod routes;

use crate::app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}

