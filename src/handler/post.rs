use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    model::post::{
        CreatePostSchema, DeletePostSchema, GetPostsPaginatedSchema, Post, UpdatePostSchema,
    },
    model::{post, user::User},
    response::FilteredUser,
    AppState,
};

pub async fn get_post_handler(
    State(data): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // get the post from the database and check if it exists and if the user is the owner
    let post = Post::get_by_id(id, &data.db).await;

    match post {
        Ok(post) => Ok((StatusCode::OK, Json(post))),
        Err(e) => {
            eprintln!("Error getting post: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Error getting post" })),
            ))
        }
    }
}

pub async fn get_posts_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<GetPostsPaginatedSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let page = match query.page {
        Some(page) => page,
        None => 1,
    };

    let per_page = match query.per_page {
        Some(per_page) => per_page,
        None => 10,
    };

    // Get the posts from the database
    let posts = Post::find_all(&data.db, page, per_page).await;

    // Check if the posts were found
    match posts {
        Ok(posts) => {
            let posts: Vec<Post> = posts
                .into_iter()
                .map(|mut post| {
                    post.user_id = uuid::Uuid::nil();
                    post
                })
                .collect();

            Ok((StatusCode::OK, Json(posts)))
        }
        Err(e) => {
            eprintln!("Error getting posts: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Error getting posts" })),
            ))
        }
    }
}

pub async fn create_post_handler(
    Extension(user): Extension<User>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreatePostSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // check if body is valid
    if body.title.is_empty() || body.content.is_empty() || body.photo.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Invalid body" })),
        ));
    }

    let slug = create_slug(&body.title);

    // Access the request body using the `body` variable
    let post = Post {
        id: uuid::Uuid::new_v4(),
        title: body.title,
        slug: Some(slug),
        content: body.content,
        photo: body.photo,
        user_id: user.id,
        created_at: None,
        updated_at: None,
    };

    // Insert the post into the database
    let result = Post::insert(post, &data.db).await;

    // Check if the post was inserted successfully
    match result {
        Ok(post) => Ok((StatusCode::CREATED, Json(post))),
        Err(e) => {
            eprintln!("Error creating post: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Error creating post" })),
            ))
        }
    }
}

pub async fn update_post_handler(
    Extension(user): Extension<User>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdatePostSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // check if body is valid
    if body.title.is_empty() || body.content.is_empty() || body.photo.is_empty() || body.id.is_nil()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Invalid body" })),
        ));
    }

    // get the post from the database and check if it exists and if the user is the owner
    let post = Post::get_by_id(body.id, &data.db).await;

    match post {
        Ok(post) => {
            if post.user_id != user.id {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "message": "Unauthorized" })),
                ));
            }
        }
        Err(e) => {
            eprintln!("Error getting post: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Error getting post" })),
            ));
        }
    }

    // Access the request body using the `body` variable
    let post = Post {
        id: body.id,
        title: body.title,
        slug: None,
        content: body.content,
        photo: body.photo,
        user_id: user.id,
        created_at: None,
        updated_at: None,
    };

    // Update the post in the database
    let result = Post::update(post, &data.db).await;

    // Check if the post was updated successfully
    match result {
        Ok(post) => Ok((StatusCode::OK, Json(post))),
        Err(e) => {
            eprintln!("Error updating post: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Error updating post" })),
            ))
        }
    }
}

pub fn create_slug(title: &str) -> String {
    let slug = title
        .to_lowercase()
        .replace(" ", "-")
        .replace("?", "")
        .replace("!", "")
        .replace(".", "")
        .replace(",", "")
        .replace(":", "")
        .replace(";", "")
        .replace("(", "")
        .replace(")", "")
        .replace("[", "")
        .replace("]", "")
        .replace("{", "")
        .replace("}", "")
        .replace("*", "")
        .replace("&", "")
        .replace("@", "")
        .replace("#", "")
        .replace("$", "")
        .replace("%", "")
        .replace("^", "")
        .replace("+", "")
        .replace("=", "")
        .replace("`", "")
        .replace("~", "")
        .replace("<", "")
        .replace(">", "")
        .replace("/", "")
        .replace("|", "");

    slug
}
