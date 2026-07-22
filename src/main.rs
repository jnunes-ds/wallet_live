mod app;
mod auth;
mod error;
mod models;
mod repository;
mod routes;

use crate::app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
