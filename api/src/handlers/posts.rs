use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::serde;
use serde_json::json;

use crate::{db::{queries::{fetch_post, fetch_posts}, table::fetch_one_post}, handlers::posts, AppState};
use crate::models::{posts::{PostModel, PostModelResponse}, post_schema::FilterOptions};

fn to_post_response(post: &PostModel) ->  PostModelResponse {
    PostModelResponse { 
        id: (post.id), 
        title: (post.title.to_owned()), 
        slug: (post.slug.to_owned()), 
        content: (post.content.to_owned()), 
        created_at: (post.created_at), 
        updated_at: (post.updated_at) 
    }
}

pub async fn get_posts(
    Query(opts): Query<FilterOptions>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

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
