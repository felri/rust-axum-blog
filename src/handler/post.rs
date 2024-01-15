use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;

use crate::{
    model::post::{CreatePostSchema, DeletePostSchema, Post, UpdatePostSchema},
    model::user::User,
    response::FilteredUser,
    AppState,
};

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

    // Access the request body using the `body` variable
    let post = Post {
        id: uuid::Uuid::new_v4(),
        title: body.title,
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
    if body.title.is_empty() || body.content.is_empty() || body.photo.is_empty() {
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

fn filter_user_record(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        email: user.email.to_owned(),
        name: user.name.to_owned(),
        photo: user.photo.to_owned(),
        role: user.role.to_owned(),
        verified: user.verified,
        created_at: user.created_at.unwrap(),
        updated_at: user.updated_at.unwrap(),
    }
}
