use sqlx::PgPool;

use crate::{db::table::{fetch_sql, Table}, models::posts::PostModel};

pub async fn fetch_posts(pool: &PgPool, limit: i32, offset: i32) -> Result<Vec<PostModel>, sqlx::Error> {
    let sql = fetch_sql(Table::Posts);

    sqlx::query_as::<_, PostModel>(sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}
