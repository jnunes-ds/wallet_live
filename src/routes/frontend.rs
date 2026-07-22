mod assets;
mod login;
mod logout;

use crate::app::AppState;
use crate::routes::frontend;
use axum::Router;
use axum::routing::get;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(frontend::login::index))
        .route(
            "/login",
            get(frontend::login::login_page).post(frontend::login::login),
        )
        .route("/logout", get(frontend::logout::logout))
        .route(
            "/assets",
            get(frontend::assets::assets).post(frontend::assets::purchase_asset),
        )
}

pub mod filters {
    use askama;
    use time::{
        OffsetDateTime, format_description::StaticFormatDescription, macros::format_description,
    };

    #[askama::filter_fn]
    pub fn human_datetime(
        datetime: &OffsetDateTime,
        _env: &dyn askama::Values,
    ) -> askama::Result<String> {
        const HUMAN_READABLE_FORMAT: StaticFormatDescription =
            format_description!(version = 2, "[year]-[month]-[day] [hour]:[minute]");

        datetime
            .format(HUMAN_READABLE_FORMAT)
            .map_err(askama::Error::custom)
    }
}
