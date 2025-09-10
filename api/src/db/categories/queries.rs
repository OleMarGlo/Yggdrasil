use sqlx::PgPool;

use crate::{db::table::{fetch_all, Table}, models::categories::CategorieModel};

pub async fn fetch_categories(pool: &PgPool) 
-> Result<Vec<CategorieModel>, sqlx::Error> {
    let sql = fetch_all(Table::Categories);

    sqlx::query_as::<_, CategorieModel>(sql)
        .fetch_all(pool)
        .await
}