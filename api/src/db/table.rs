pub enum Table {
    Posts,
    Categories,
}

pub fn fetch_sql(table: Table) -> &'static str {
    match table {
        Table::Posts => "SELECT * FROM posts ORDER BY id LIMIT $1 OFFSET $2",
        Table::Categories => "SELECT * FROM comments ORDER BY id LIMIT $1 OFFSET $2",
    }
}