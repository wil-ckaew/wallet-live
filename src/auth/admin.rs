// src/auth/admin.rs
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::header::AUTHORIZATION;

use crate::error::AppError;

const ADMIN_SECRET_KEY: &str = "im-the-admin";

pub struct Admin;

// Use tipo genérico S ao invés de AppState específico e adicione Sync
impl<S> FromRequestParts<S> for Admin
where
    S: Sync,  // Necessário para o Future ser Send
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Some(auth) = parts.headers.get(AUTHORIZATION) else {
            return Err(AppError::MissingAuthorization);
        };

        // Converte o header para string para comparação
        let auth_str = auth.to_str().map_err(|_| AppError::InvalidCredentials)?;
        
        if auth_str == ADMIN_SECRET_KEY {
            Ok(Admin)
        } else {
            Err(AppError::InvalidCredentials)
        }
    }
}