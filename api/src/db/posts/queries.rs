use axum::http::StatusCode;
use sqlx::{types::Json, PgPool};

use crate::{db::table::{add_one_row, delete_row, fetch_all, fetch_one_row, get_posts_with_categorie, update_one_row, Table}, models::{post_schema::CreatePostSchema, posts::PostModel}};

// executes an SQL query based on filtering and paging
pub async fn fetch_posts(
    pool: &PgPool, 
    limit: i32, 
    offset: i32, 
    order_by:&str, 
    sort_by: &str,
    search: &str,
    categories: Vec<i32>
) -> Result<Vec<PostModel>, sqlx::Error> {
    let sql = fetch_all(Table::Posts, std::option::Option::Some(order_by), std::option::Option::Some(sort_by));

    sqlx::query_as::<_, PostModel>(&sql)
        .bind(search)
        .bind(categories)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

// executes an SQL query based on an ID
pub async fn fetch_post(pool: &PgPool, id: i32) 
-> Result<PostModel, sqlx::Error> {
    let sql = fetch_one_row(Table::Posts);

    sqlx::query_as::<_, PostModel>(sql)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_post(
    pool: &PgPool, 
    body: Json<CreatePostSchema>, 
) -> Result<StatusCode, sqlx::Error> {
    let sql = add_one_row(Table::Posts);
    // Execute the query and handle the result
    match sqlx::query(sql)
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

pub async fn get_posts_in_categies_sql(
    pool: &PgPool,
    id: i32
) -> Result<Vec<PostModel>, sqlx::Error> {
    let sql = get_posts_with_categorie(Table::Posts);

    sqlx::query_as::<_, PostModel>(sql)
        .bind(id)
        .fetch_all(pool)
        .await
}

pub async fn update_post(
    pool: &PgPool,
    id: i32,
    title: Option<String>,
    slug: Option<String>,
    categorie_id: Option<i32>,
    content: Option<String>,
) -> Result<PostModel, sqlx::Error> {
    let sql = update_one_row(Table::Posts);

    sqlx::query_as::<_, PostModel>(sql)
        .bind(title)
        .bind(slug)
        .bind(content)
        .bind(categorie_id)
        .bind(id)
        .fetch_one(pool)
        .await
}