use axum::{http::StatusCode, Json};
use sqlx::PgPool;

use crate::{db::posts::queries::{delete_post_sql, fetch_post, fetch_posts, update_post}, models::{post_schema::PatchPost, posts::{PostModel, PostModelResponse}}};

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

pub fn format_post_response_one(post: PostModel) -> Json<serde_json::Value> {
    let post_response = to_post_response(&post);
    let json_response = serde_json::json!({
        "post": post_response
    });
    Json(json_response)
}

pub fn format_post_response_many(posts: Vec<PostModel>) -> Json<serde_json::Value> {
    let total = posts.first()
        .map(|p| p.total_count).unwrap_or(0);
    
    let post_response = posts
        .into_iter()
        .map(| post: PostModel | to_post_response(&post))
        .collect::<Vec<PostModelResponse>>();


    let json_response = serde_json::json!({
        "total_count": total,
        "posts": post_response
    });
    Json(json_response)
}

pub async fn delete_post_from_db(
    db: &PgPool,
    id: i32,
) -> Result<PostModel, (StatusCode, Json<serde_json::Value>)> {
    delete_post_sql(&db, id)
        .await
        .map_err(|err| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("unable to delete id, err: {}", err),
            });
            (StatusCode::BAD_REQUEST, Json(error_response))
        })
}

pub async fn get_post_from_db(
    db: &PgPool,
    id: i32,
) -> Result<PostModel, (StatusCode, Json<serde_json::Value>)> {
    fetch_post(&db, id)
        .await
        .map_err(|_| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "unable to find id"
            });
            (StatusCode::BAD_REQUEST, Json(error_response))
        })
}

pub async fn get_posts_from_db(
    db: &PgPool,
    limit: i32,
    offset: i32,
    order_by: String,
    sort: String,
    search: String,
    categories_str: String,
) -> Result<Vec<PostModel>, (StatusCode, Json<serde_json::Value>)> {
    let categories = categories_str
        .split(",")
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect::<Vec<_>>();

    fetch_posts(&db, limit, offset, &order_by, &sort, &search, categories)
        .await
        .map_err(|err| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", err),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })
}

pub async fn patch_post_in_db(
    db: &PgPool,
    id: i32,
    payload: PatchPost
) -> Result<PostModel,(StatusCode, Json<serde_json::Value>)> {
    update_post(db, id, payload.title, payload.slug, payload.category, payload.content)
        .await
        .map_err(|err| {
            let error_response = serde_json::json!({
            "status": "error",
            "message": format!("unable to patch, err: {}", err),
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })
}