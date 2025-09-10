use sqlx::PgPool;

use crate::{db::table::{fetch_all, fetch_one_row, Table}, functions::parse_id, models::categories::CategorieModel};

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