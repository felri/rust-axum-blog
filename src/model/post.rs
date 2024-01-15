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

impl Post {
    pub async fn insert(post: Post, db: &sqlx::PgPool) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (id, title, content, photo, user_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, content, photo, user_id, created_at, updated_at
            "#,
            post.id,
            post.title,
            post.content,
            post.photo,
            post.user_id,
        )
        .fetch_one(db)
        .await?;

        Ok(post)
    }

    pub async fn get_by_id(id: uuid::Uuid, db: &sqlx::PgPool) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT id,
                     title,
                     content,
                     photo,
                     user_id,
                     created_at,
                     updated_at
            FROM posts
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(db)
        .await?;

        Ok(post)
    }

    pub async fn update(post: Post, db: &sqlx::PgPool) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET title = $1, content = $2, photo = $3, user_id = $4, updated_at = $5
            WHERE id = $6
            RETURNING id, title, content, photo, user_id, created_at, updated_at
            "#,
            post.title,
            post.content,
            post.photo,
            post.user_id,
            Utc::now(),
            post.id,
        )
        .fetch_one(db)
        .await?;

        Ok(post)
    }

    pub async fn delete(id: uuid::Uuid, db: &sqlx::PgPool) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            DELETE FROM posts
            WHERE id = $1
            RETURNING id, title, content, photo, user_id, created_at, updated_at
            "#,
            id,
        )
        .fetch_one(db)
        .await?;

        Ok(post)
    }

    pub async fn find_all(db: &sqlx::PgPool) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id,
                     title,
                     content,
                     photo,
                     user_id,
                     created_at,
                     updated_at
            FROM posts
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(db)
        .await?;

        Ok(posts)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePostSchema {
    pub title: String,
    pub content: String,
    pub photo: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostSchema {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub photo: String,
}

#[derive(Debug, Deserialize)]
pub struct DeletePostSchema {
    pub id: uuid::Uuid,
}
