// src/routes/api.rs
use axum::{
    Router, 
    routing::{get, post, patch},
    Json,
};
use serde::Deserialize;
use crate::{app::AppState, models::Asset};
use crate::repository::Repository;
use crate::error::AppError;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/assets", get(list_assets))  // Adicionado /api/
        .route("/api/assets", post(create_asset))  // Adicionado /api/
        .route("/api/assets", patch(update_asset))  // Adicionado /api/
}

#[tracing::instrument(skip_all)]
async fn list_assets(repository: Repository) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = repository.list_assets().await?;
    Ok(Json(assets))
}

#[derive(Debug, Deserialize)]
struct CreateAssetRequest {
    name: String,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
async fn create_asset(
    repository: Repository,
    Json(request): Json<CreateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    let new_asset = repository
        .create_asset(request.name, request.unit_value)
        .await?;

    Ok(Json(new_asset))
}

#[derive(Debug, Deserialize)]
struct UpdateAssetRequest {
    id: i64,
    name: Option<String>,
    unit_value: Option<f64>,
}

#[tracing::instrument(skip_all)]
async fn update_asset(
    repository: Repository,
    Json(request): Json<UpdateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    match repository
        .update_asset(request.id, request.name, request.unit_value)
        .await
    {
        Ok(Some(updated_asset)) => Ok(Json(updated_asset)),
        Ok(None) => Err(AppError::AssetDoesNotExist),
        Err(e) => Err(AppError::Database(e)),
    }
}

#[cfg(test)]
mod tests { 
    use sqlx::PgPool;
    use super::*;
    use crate::repository::Repository;

    #[sqlx::test]
    async fn test_create_asset(db: PgPool) {
        let request = CreateAssetRequest {
            name: "Bitcoin".to_string(),
            unit_value: 10.0,
        };

        let repository: Repository = db.into();
        
        let Json(new_asset) = create_asset(repository, Json(request))
            .await
            .expect("success");

        assert_eq!(new_asset.id, 1);
        assert_eq!(new_asset.name, "Bitcoin");
        assert_eq!(new_asset.unit_value, 10.0);
    }
}