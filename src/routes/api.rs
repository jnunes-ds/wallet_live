mod admin;
mod assets;

use axum::Router;
use axum::routing::{get, post};

use crate::app::AppState;
use crate::routes::api;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/assets",
            get(api::assets::list_assets)
                .post(api::assets::create_asset)
                .patch(api::assets::update_asset),
        )
        .route("/admin", post(api::admin::turn_user_into_admin))
}
