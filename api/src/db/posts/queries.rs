use axum::http::StatusCode;
use sqlx::{types::Json, PgPool};

use crate::{db::table::{add_one_row, delete_row, fetch_all, fetch_one_row, Table}, functions::parse_id, models::{post_schema::CreatePostSchema, posts::PostModel}};

// executes an SQL query based on filtering and paging
pub async fn fetch_posts(pool: &PgPool, limit: i32, offset: i32) -> Result<Vec<PostModel>, sqlx::Error> {
    let sql = fetch_all(Table::Posts);

    sqlx::query_as::<_, PostModel>(sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

// executes an SQL query based on an ID
pub async fn fetch_post(pool: &PgPool, id_str: &str) -> Result<PostModel, sqlx::Error> {
    let id = parse_id(id_str)?;
    let sql = fetch_one_row(Table::Posts);

    sqlx::query_as::<_, PostModel>(sql)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_post(
    pool: &PgPool, 
    body: Json<CreatePostSchema>, 
    id: i32
) -> Result<StatusCode, sqlx::Error> {
    let sql = add_one_row(Table::Posts);
    // Execute the query and handle the result
    match sqlx::query(sql)
        .bind(&id)
        .bind(&body.title)
        .bind(&body.slug)
        .bind(&body.content)
        .bind(&body.category)
        .execute(pool)
        .await {
            Ok(_) => Ok(StatusCode::CREATED),
            Err(err) => Err(err), // Return the error if execution fails
        }
}

pub async fn delete_post_sql(
    pool: &PgPool,
    id: i32
) -> Result<PostModel, sqlx::Error> {
    let sql = delete_row(Table::Posts);

    sqlx::query_as::<_, PostModel>(sql)
        .bind(id)
        .fetch_one(pool)
        .await
}