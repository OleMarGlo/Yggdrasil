use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::PgPool;

use crate::{db::{queries::{create_post, fetch_post, fetch_posts}, table::fetch_one_post}, handlers::posts, models::post_schema::CreatePostSchema, AppState};
use crate::models::{posts::{PostModel, PostModelResponse}, post_schema::FilterOptions};


// used to map from a PostModel in DB to a response
fn to_post_response(post: &PostModel) ->  PostModelResponse {
    PostModelResponse { 
        id: (post.id), 
        title: (post.title.to_owned()), 
        slug: (post.slug.to_owned()), 
        content: (post.content.to_owned()), 
        category: (post.category.to_owned()),
        created_at: (post.created_at), 
        updated_at: (post.updated_at) 
    }
}

// fetches all posts in DB
pub async fn get_posts(
    Query(opts): Query<FilterOptions>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let limit = opts.limit.unwrap_or(10);       //limit the amount fetched
    let offset = (opts.page.unwrap_or(1) - 1) * limit;          //used for paging

    // fetches posts from DB
    let posts = fetch_posts(&data.db, limit as i32, offset as i32)
        .await
        .map_err(|err| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", err),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let post_responses = posts
        .into_iter()
        .map(|post: PostModel| to_post_response(&post))
        .collect::<Vec<PostModelResponse>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "count": post_responses.len(),
        "posts": post_responses,
    });

    Ok(Json(json_response))
}

pub async fn get_post(
    State(data): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let post = fetch_post(&data.db, &id)
        .await
        .map_err(|err| {
            let errpr_response = serde_json::json!({
                "status": "error",
                "message": format!("unable to fetch id, err: {}", err),
            });
            (StatusCode::BAD_REQUEST, Json(errpr_response))
        })?;

        let post_response = to_post_response(&post);

        let json_response = json!({
            "status": "ok",
            "post": post_response
        });
        Ok(Json(json_response))
}

pub async fn post_posts(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreatePostSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = match get_highest_id(&data.db).await {
        Ok(Some(highest_id)) => highest_id + 1,
        Ok(None) => 1,
        Err(_) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "unable to query DB"
            }))));
        }
    };
    
    match create_post(&data.db, sqlx::types::Json(body), id).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(serde_json::json!({"message": "Post created successfully"})))),
        Err(_) => Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "unable to create post"
        })))),
    }
}

async fn get_highest_id(pool: &PgPool) -> Result<Option<i32>, sqlx::Error> {
    let row: (i32,) = sqlx::query_as("SELECT MAX(id) FROM posts")
        .fetch_one(pool)
        .await?;
    Ok(Some(row.0))
}