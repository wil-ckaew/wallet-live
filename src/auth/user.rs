// src/auth/user.rs
use password_auth::VerifyError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::cookie::{CookieJar};
use jwt_simple::{
    claims::Claims,
    prelude::{Duration, HS256Key, MACLike},
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

use crate::{error::AppError, repository::Repository, app::AppState};

const SECRET_KEY: &[u8] = b"im-so-secret";

pub struct UnauthenticatedUser {
    username: String,
    password: String,
}

impl UnauthenticatedUser {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub async fn authenticate(&self, repository: &Repository) -> Result<User, AppError> {
        let user_record = match repository.get_user_by_username(&self.username).await? {
            Some(user_record) => user_record,
            None => return Err(AppError::UserDoesNotExist),
        };

        match password_auth::verify_password(&self.password, &user_record.password_hash) {
            Ok(_) => Ok(User::new(user_record.id, user_record.username)), 
            Err(VerifyError::PasswordInvalid) => Err(AppError::InvalidCredentials),
            Err(VerifyError::Parse(err)) => panic!("Hashing algorithm failed: {err}"),
        }
    }
    
    pub async fn register(self, repository: &Repository) -> Result<User, AppError> {
        let password_hash = password_auth::generate_hash(&self.password);
        let user_record = match repository.add_user(&self.username, &password_hash).await {
            Ok(user_record) => user_record,
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                return Err(AppError::UsernameTaken);
            }
            Err(err) => return Err(AppError::Database(err)),
        };
        Ok(User::new(user_record.id, user_record.username))
    }
}

#[derive(Clone)]  // Adicione Clone aqui
pub struct User {
    id: i64,
    username: String,
}

impl User {
    pub fn new(id: i64, username: String) -> Self {
        Self { id, username }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn auth_token(&self) -> Result<String, AppError> {
        let key = HS256Key::from_bytes(SECRET_KEY);
        // Crie uma instância de UserClaims diretamente
        let user_claims = UserClaims {
            id: self.id,
            username: self.username.clone(),
        };
        let claims = Claims::with_custom_claims(user_claims, Duration::from_mins(10));
        let token = key.authenticate(claims)?;
        Ok(token)
    }

    pub fn from_auth_token(token: &str) -> Result<Self, AppError> {
        let key = HS256Key::from_bytes(SECRET_KEY);
        let claims: UserClaims = key.verify_token(token, None)?.custom;
        Ok(Self::new(claims.id, claims.username))
    }
}

impl FromRequestParts<AppState> for User {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        
        let token = match jar.get("token") {
            Some(token) => token.value(),
            None => return Err(AppError::MissingAuthorization),
        };
        
        let user = User::from_auth_token(token)?;
        
        let repository = Repository::from_request_parts(parts, state).await
            .map_err(|_| AppError::Database(sqlx::Error::RowNotFound))?;
        
        let user_record = repository.get_user_by_id(user.id()).await?
            .ok_or(AppError::UserDoesNotExist)?;
        
        Ok(User::new(user_record.id, user_record.username))
    }
}

impl FromRequestParts<AppState> for Option<User> {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(User::from_request_parts(parts, state).await.ok())
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct UserClaims {
    id: i64,
    username: String,
}