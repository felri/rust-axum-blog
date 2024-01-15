use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub photo: String,
    pub user_id: uuid::Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostSchema {
    pub title: String,
    pub content: String,
    pub photo: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostSchema {
    pub title: String,
    pub content: String,
    pub photo: String,
}

#[derive(Debug, Deserialize)]
pub struct DeletePostSchema {
    pub id: uuid::Uuid,
}
