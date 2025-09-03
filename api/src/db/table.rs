pub enum Table {
    Posts,
    Categories,
}

impl Table {
    fn as_str(&self) -> &'static str {
        match self {
            Table::Posts => "posts",
            Table::Categories => "categories",
        }
    }
}

pub fn fetch_sql(table: Table) -> &'static str {
    match table {
        Table::Posts => "SELECT * FROM posts ORDER BY id LIMIT $1 OFFSET $2",
        Table::Categories => "SELECT * FROM comments ORDER BY id LIMIT $1 OFFSET $2",
    }
}