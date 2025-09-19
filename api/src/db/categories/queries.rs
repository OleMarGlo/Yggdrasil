use axum::http::StatusCode;
use sqlx::{types::Json, PgPool};

use crate::{db::table::{add_one_row, delete_row, fetch_all, fetch_one_row, update_one_row, Table}, models::{categorie_schema::CreateCategorieSchema, categories::CategorieModel}};

pub async fn fetch_categories(pool: &PgPool) 
-> Result<Vec<CategorieModel>, sqlx::Error> {
    let sql = fetch_all(Table::Categories);

    sqlx::query_as::<_, CategorieModel>(sql)
        .fetch_all(pool)
        .await
}

pub async fn fetch_one_categorie(pool: &PgPool, id: i32)
-> Result<CategorieModel, sqlx::Error> {
    let sql = fetch_one_row(Table::Categories);

    sqlx::query_as::<_, CategorieModel>(sql)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_categorie(
    pool: &PgPool,
    body: Json<CreateCategorieSchema>,
) -> Result<StatusCode, sqlx::Error> {
    let sql = add_one_row(Table::Categories);

    match sqlx::query(sql)
        .bind(&body.category)
        .bind(&body.slug)
        .bind(&body.description)
        .execute(pool)
        .await {
            Ok(_) => Ok(StatusCode::CREATED),
            Err(err) => Err(err), 
        }
}

pub async fn delete_categorie(
    pool: &PgPool,
    id: i32
) -> Result<CategorieModel, sqlx::Error> {
    let sql = delete_row(Table::Categories);

    sqlx::query_as::<_, CategorieModel>(sql)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn update_row(
    pool: &PgPool,
    id: i32,
    category: Option<String>,
    slug: Option<String>,
    desc: Option<String>
) -> Result<CategorieModel, sqlx::Error> {
    let sql = update_one_row(Table::Categories);

    sqlx::query_as::<_, CategorieModel>(sql)
        .bind(category)
        .bind(slug)
        .bind(desc)
        .bind(id)
        .fetch_one(pool)
        .await
}