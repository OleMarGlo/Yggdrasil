use axum::{http::StatusCode, response::IntoResponse};
use sqlx::{types::Json, PgPool};

use crate::{db::table::{add_one_row, delete_row, fetch_all, fetch_one_row, Table}, functions::parse_id, models::{categorie_schema::CreateCategorieSchema, categories::CategorieModel}};

pub async fn fetch_categories(pool: &PgPool) 
-> Result<Vec<CategorieModel>, sqlx::Error> {
    let sql = fetch_all(Table::Categories);

    sqlx::query_as::<_, CategorieModel>(sql)
        .fetch_all(pool)
        .await
}

pub async fn fetch_one_categorie(pool: &PgPool, id_str: &str)
-> Result<CategorieModel, sqlx::Error> {
    let id = parse_id(id_str)?;
    let sql = fetch_one_row(Table::Categories);

    sqlx::query_as::<_, CategorieModel>(sql)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_categorie(
    pool: &PgPool,
    body: Json<CreateCategorieSchema>,
    id: i32
) -> Result<StatusCode, sqlx::Error> {
    let sql = add_one_row(Table::Categories);

    match sqlx::query(sql)
        .bind(&id)
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