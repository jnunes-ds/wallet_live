use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::CookieJar;

pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    (
        jar.remove("token").remove("admin_token"),
        Redirect::to("/login"),
    )
}
