use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserSchema {
    pub name: String,
    pub email: String,
    pub photo: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePasswordSchema {
    pub password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordSchema {
    pub email: String,
    pub password: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordSchema {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailSchema {
    pub email: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenSchema {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteUserSchema {
    pub email: String,
    pub password: String,
}
