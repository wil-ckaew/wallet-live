// src/repository.rs
use axum::extract::FromRequestParts;
use crate::app::AppState;
use crate::models::{Asset, UserRecord, OwnedAsset, PurchaseHistory};
use sqlx::PgPool;
use std::convert::Infallible;

pub struct Repository {
    pub db: PgPool,  // Tornando público para acesso
}

impl Repository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn list_assets(&self) -> Result<Vec<Asset>, sqlx::Error> {
        sqlx::query_as!(
            Asset, 
            "SELECT id, name, unit_value FROM assets"
        )
        .fetch_all(&self.db)
        .await
    }

    pub async fn create_asset(&self, name: String, unit_value: f64) -> Result<Asset, sqlx::Error> {
        sqlx::query_as!(
            Asset,
            "INSERT INTO assets (name, unit_value) VALUES ($1, $2) RETURNING id, name, unit_value",
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
        unit_value: Option<f64>
    ) -> Result<Option<Asset>, sqlx::Error> {
        let result = sqlx::query_as!(
            Asset,
            "UPDATE assets SET 
                name = COALESCE($1, name), 
                unit_value = COALESCE($2, unit_value) 
             WHERE id = $3 
             RETURNING id, name, unit_value",
            name,
            unit_value,
            asset_id
        )
        .fetch_optional(&self.db)
        .await?;
        
        Ok(result)
    }

    pub async fn delete_asset(&self, asset_id: i64) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM assets WHERE id = $1", asset_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    pub async fn add_user(&self, username: &str, password_hash: &str) -> sqlx::Result<UserRecord> {
        sqlx::query_as!(
            UserRecord,
            "INSERT INTO users (username, password_hash) 
            VALUES ($1, $2) 
            RETURNING id, username, password_hash",
            username,
            password_hash       
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn get_user_by_id(&self, id: i64) -> Result<Option<UserRecord>, sqlx::Error> {
        sqlx::query_as!(
            UserRecord,
            "SELECT id, username, password_hash 
            FROM users 
            WHERE id = $1",
            id
        )
        .fetch_optional(&self.db)
        .await
    }

    pub async fn get_user_by_username(&self, username: &str) -> sqlx::Result<Option<UserRecord>> {
        sqlx::query_as!(
            UserRecord,
            "SELECT id, username, password_hash 
            FROM users 
            WHERE username = $1",
            username
        )
        .fetch_optional(&self.db)
        .await
    }

    pub async fn list_owned_assets(&self, user_id: i64) -> sqlx::Result<Vec<OwnedAsset>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                a.id,
                a.name,
                a.unit_value,
                COALESCE(SUM((a.unit_value - o.bought_for) * o.quantity_owned), 0) AS "value_delta!",
                COALESCE(SUM(o.quantity_owned), 0) AS "quantity_owned!",
                COALESCE(
                    json_agg(
                        json_build_object(
                            'bought_at', o.bought_at,
                            'bought_for', o.bought_for,
                            'quantity_bought', o.quantity_owned,
                            'value_delta', (a.unit_value - o.bought_for) * o.quantity_owned
                        )
                        ORDER BY o.bought_at DESC
                    ) FILTER (WHERE o.id IS NOT NULL),
                    '[]'::json
                ) AS "purchase_history"
            FROM assets AS a
            LEFT JOIN owned_assets AS o ON o.asset_id = a.id AND o.user_id = $1
            WHERE o.user_id = $1 OR EXISTS (
                SELECT 1 FROM owned_assets WHERE user_id = $1 AND asset_id = a.id
            )
            GROUP BY a.id
            "#,
            user_id
        )
        .fetch_all(&self.db)
        .await?;

        let mut owned_assets = Vec::new();
        for row in rows {
            let purchase_history: Vec<PurchaseHistory> = 
                row.purchase_history
                    .map(|v| serde_json::from_value(v).unwrap_or_default())
                    .unwrap_or_default();
            
            owned_assets.push(OwnedAsset {
                id: row.id,
                name: row.name,
                unit_value: row.unit_value,
                value_delta: row.value_delta,
                quantity_owned: row.quantity_owned,
                purchase_history,
            });
        }
        
        Ok(owned_assets)
    }

    pub async fn insert_owned_asset(
        &self,
        user_id: i64,
        asset_id: i64,
        quantity: f64,
        bought_for: f64,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO owned_assets (user_id, asset_id, quantity_owned, bought_for, bought_at)
             VALUES ($1, $2, $3, $4, NOW())",
            user_id,
            asset_id,
            quantity,
            bought_for,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}

impl FromRequestParts<AppState> for Repository {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
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