pub fn parse_id(id_str: &str) -> Result<i32, sqlx::Error> {
    match id_str.parse() {
        Ok(id) => Ok(id),
        Err(_) => Err(sqlx::Error::RowNotFound)
    }
}