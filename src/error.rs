// src/error.rs
use axum::{
    response::{IntoResponse, Json},
    http::StatusCode,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Missing authorization header")]
    MissingAuthorization,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Asset does not exist")]
    AssetDoesNotExist,
    #[error("User dows not exist")]
    UserDoesNotExist,
    #[error("This username is already registered")]
    UsernameTaken,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Template(#[from] askama::Error),
    #[error(transparent)]
    Jwt(#[from] jwt_simple::Error),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };
        let status = match self {
            Self::UsernameTaken | Self::MissingAuthorization => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::AssetDoesNotExist | Self::UserDoesNotExist => StatusCode::NOT_FOUND,
            Self::Database(_) | Self::Template(_) | Self::Jwt(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(error_response)).into_response()
    }
}