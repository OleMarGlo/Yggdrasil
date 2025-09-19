use sqlx::PgPool;

pub fn parse_id(id_str: &str) -> Result<i32, sqlx::Error> {
    match id_str.parse() {
        Ok(id) => Ok(id),
        Err(_) => Err(sqlx::Error::RowNotFound)
    }
}

pub async fn get_highest_id(pool: &PgPool) -> Result<Option<i32>, sqlx::Error> {
    let row: (i32,) = sqlx::query_as("SELECT MAX(id) FROM posts")
        .fetch_one(pool)
        .await?;
    Ok(Some(row.0))
}