use sqlx::PgPool;

use crate::{db::table::{fetch_one_post, fetch_sql, Table}, models::posts::PostModel};

pub async fn fetch_posts(pool: &PgPool, limit: i32, offset: i32) -> Result<Vec<PostModel>, sqlx::Error> {
    let sql = fetch_sql(Table::Posts);

    sqlx::query_as::<_, PostModel>(sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn fetch_post(pool: &PgPool, id_str: &str) -> Result<PostModel, sqlx::Error> {
    let id: i32 = match id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return Err(sqlx::Error::RowNotFound)
        }
    };

    let sql = fetch_one_post(Table::Posts);

    sqlx::query_as::<_, PostModel>(sql)
        .bind(id)
        .fetch_one(pool)
        .await
}
