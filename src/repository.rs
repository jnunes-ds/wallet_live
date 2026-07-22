use crate::app::AppState;
use crate::models::asset::Asset;
use crate::models::owned_assets::{OwnedAsset, PurchaseHistory};
use crate::models::user::UserRecord;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use sqlx::{PgPool, Row};
use std::convert::Infallible;

#[derive(Clone)]
pub struct Repository {
    db: PgPool,
}

impl Repository {
    pub async fn list_assets(&self) -> sqlx::Result<Vec<Asset>> {
        sqlx::query_as!(Asset, "SELECT id, name, assets.unit_value FROM assets;")
            .fetch_all(&self.db)
            .await
    }

    pub async fn create_asset(&self, name: String, unit_value: f64) -> sqlx::Result<Asset> {
        sqlx::query_as!(
            Asset,
            "INSERT INTO assets (name, unit_value)
            VALUES ($1, $2)
            RETURNING id, name, unit_value;",
            name,
            unit_value
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn update_asset(
        &self,
        asset_id: i64,
        name: Option<String>,
        unit_value: Option<f64>,
    ) -> sqlx::Result<Option<Asset>> {
        sqlx::query_as!(
            Asset,
            "UPDATE assets
            SET name=COALESCE($2, name),
                unit_value=COALESCE($3, unit_value)
            WHERE id=$1
            RETURNING id, name, unit_value;",
            asset_id,
            name,
            unit_value
        )
        .fetch_optional(&self.db)
        .await
    }

    pub async fn get_asset_by_id(&self, asset_id: i64) -> sqlx::Result<Asset> {
        sqlx::query_as!(
            Asset,
            "SELECT id, name, unit_value FROM assets WHERE id = $1",
            asset_id
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn add_user(&self, username: &str, password_hash: &str) -> sqlx::Result<UserRecord> {
        sqlx::query_as!(
            UserRecord,
            "INSERT INTO users (username, password_hash)
            VALUES ($1, $2)
            RETURNING id, username, password_hash;",
            username,
            password_hash
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn get_user_by_username(&self, username: &str) -> sqlx::Result<Option<UserRecord>> {
        sqlx::query_as!(
            UserRecord,
            "SELECT id, username, password_hash
            FROM users
            WHERE username = $1;",
            username
        )
        .fetch_optional(&self.db)
        .await
    }

    pub async fn turn_user_into_admin(&self, user_id: i64) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO admins
            (user_id)
            VALUES ($1)",
            user_id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn is_user_admin(&self, user_id: i64) -> sqlx::Result<bool> {
        sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM admins WHERE user_id = $1)",
            user_id
        )
        .fetch_one(&self.db)
        .await
        .map(|row| row.exists.unwrap_or(false))
    }

    pub async fn insert_owned_asset(
        &self,
        user_id: i64,
        asset_id: i64,
        quantity: f64,
        unit_value: f64,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO owned_assets
            (user_id, asset_id, quantity_owned, bought_for)
            VALUES ($1, $2, $3, $4)",
            user_id,
            asset_id,
            quantity,
            unit_value
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn list_owned_assets(&self, user_id: i64) -> sqlx::Result<Vec<OwnedAsset>> {
        sqlx::query(
            r#"
            SELECT
                a.id,
                a.name,
                a.unit_value,
                COALESCE(SUM((a.unit_value - o.bought_for) * o.quantity_owned), 0.0) as value_delta,
                COALESCE(SUM(o.quantity_owned), 0.0) AS quantity_owned,
                COALESCE(
                    JSON_AGG(
                        JSON_BUILD_OBJECT(
                            'bought_at', o.timestamp,
                            'bought_for', o.bought_for,
                            'quantity_bought', o.quantity_owned,
                            'value_delta', (a.unit_value - o.bought_for) * o.quantity_owned
                        )
                    ),
                    '[]'::json
                ) AS purchase_history
            FROM assets AS a
            JOIN owned_assets AS o
                ON o.asset_id = a.id
            WHERE o.user_id = $1
            GROUP BY a.id
            "#,
        )
        .bind(user_id)
        .map(|row: sqlx::postgres::PgRow| {
            let purchase_history: sqlx::types::Json<Vec<PurchaseHistory>> =
                row.get("purchase_history");
            OwnedAsset {
                id: row.get("id"),
                name: row.get("name"),
                unit_value: row.get("unit_value"),
                value_delta: row.get("value_delta"),
                quantity_owned: row.get("quantity_owned"),
                purchase_history,
            }
        })
        .fetch_all(&self.db)
        .await
    }
}

impl FromRequestParts<AppState> for Repository {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            db: state.db.clone(),
        })
    }
}

#[cfg(test)]
impl From<PgPool> for Repository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
