use axum::{Json, Router};
use axum::routing::{get, post};
use serde::Deserialize;

use crate::app::AppState;
use crate::auth::admin::Admin;
use crate::error::AppError;
use crate::models::asset::{Asset, Id};
use crate::repository::Repository;
use crate::routes::api;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/assets",
            get(api::list_assets)
                .post(api::create_asset)
                .patch(update_asset)
        ).route("/admin", post(api::turn_user_into_admin))
}
#[derive(Deserialize)]
struct TurnUserIntoAdminRequest {
    user_id: i64,
}

pub async fn turn_user_into_admin(repository: Repository, Json(request): Json<TurnUserIntoAdminRequest>) -> Result<(), AppError> {
    repository.turn_user_into_admin(request.user_id).await?;
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn list_assets(repository: Repository) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = repository.list_assets().await?;
    Ok(Json(assets))
}

#[derive(Deserialize)]
pub struct CreateAssetRequest {
    name: String,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
pub async fn create_asset(
    _admin: Admin,
    repository: Repository,
    Json(request): Json<CreateAssetRequest>
) -> Result<Json<Asset>, AppError> {
    let new_asset = repository.create_asset(request.name, request.unit_value).await?;

    Ok(Json(new_asset))
}

#[derive(Deserialize)]
pub struct UpdateAssetRequest {
    id: Id,
    name: Option<String>,
    unit_value: Option<f64>,
}

#[tracing::instrument(skip_all)]
pub async fn update_asset(
    _admin: Admin,
    repository: Repository,
    Json(request): Json<UpdateAssetRequest>
) -> Result<Json<Asset>, AppError> {
    match repository.update_asset(request.id, request.name, request.unit_value).await? {
        Some(asset) => Ok(Json(asset.clone())),
        None => Err(AppError::AssetDoesNotExist)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use super::*;

    #[sqlx::test]
    async fn test_create_asset(db: PgPool) {
        let request = CreateAssetRequest {
            name: "Bitcoin".to_string(),
            unit_value: 250_000.0
        };

        let Json(new_asset) = create_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(new_asset.id, 1);
        assert_eq!(new_asset.name, "Bitcoin");
        assert_eq!(new_asset.unit_value, 250_000.0);

        insta::assert_json_snapshot!(new_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_list_assets(db: PgPool) {
        let Json(assets) = list_assets(db.into()).await.expect("success");

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].name, "Bitcoin");

        insta::assert_json_snapshot!(assets);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_update_asset(db: PgPool) {
        let request = UpdateAssetRequest {
            id: 1,
            name: Some("Ethereum".to_string()),
            unit_value: Some(450_000.0)
        };

        let Json(new_asset) = update_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(new_asset.id, 1);
        assert_eq!(new_asset.name, "Ethereum");
        assert_eq!(new_asset.unit_value, 450_000.0);

        insta::assert_json_snapshot!(new_asset);
    }
}