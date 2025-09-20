use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{db::posts::queries::create_post, functions::parse_id_handler, models::post_schema::{CreatePostSchema, FilterOptions, PatchPost}, services::post::{delete_post_from_db, format_post_response_many, format_post_response_one, get_post_from_db, get_posts_from_db, get_posts_in_categorie_from_db, patch_post_in_db}, AppState};


// fetches all posts in DB
pub async fn get_posts(
    Query(opts): Query<FilterOptions>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let limit = opts.limit.unwrap_or(10);       //limit the amount fetched
    let offset = (opts.page.unwrap_or(1) - 1) * limit;          //used for paging
    
    // fetches posts from DB
    let posts = get_posts_from_db(&data.db, limit as i32, offset as i32).await?;
    Ok(format_post_response_many(posts))
}

pub async fn get_post(
    State(data): State<Arc<AppState>>,
    Path(id_str): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = parse_id_handler(&id_str)?;
    let post = get_post_from_db(&data.db, id).await?;
    Ok(format_post_response_one(post))
}

pub async fn post_posts(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreatePostSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {    
    match create_post(&data.db, sqlx::types::Json(body)).await {
        Ok(_) => Ok((StatusCode::OK, Json(serde_json::json!({"message": "Post created successfully"})))),
        Err(_) => Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "unable to create post"
        })))),
    }
}

pub async fn get_posts_in_categorie(
    State(data): State<Arc<AppState>>,
    Path(id_string): Path<String>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = parse_id_handler(&id_string)?;
    let posts = get_posts_in_categorie_from_db(&data.db, id).await?;

    match posts.is_empty() {
        true => Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "stauts": "error",
            "message": "categorie is empty",
        })))),
        _ => Ok(format_post_response_many(posts))
    }
}

pub async fn delete_post(
    State(data): State<Arc<AppState>>,
    Path(id_string): Path<String>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = parse_id_handler(&id_string)?;
    let post = delete_post_from_db(&data.db, id).await?;
    Ok(format_post_response_one(post))
}

pub async fn patch_post(
    State(data): State<Arc<AppState>>,
    Path(id_str): Path<String>,             // path first
    Json(payload): Json<PatchPost>, 
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = parse_id_handler(&id_str)?;
    let post = patch_post_in_db(&data.db, id, payload).await?;
    Ok(format_post_response_one(post))
}