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

pub fn fetch_one_post(table: Table) -> &'static str {
    match table {
        Table::Posts => "SELECT * FROM posts WHERE id=$1",
        _ => unimplemented!("Unimplemented")
    }
}

pub fn add_one_post(table: Table) -> &'static str {
    match table {
        Table::Posts => "INSERT INTO posts(id, title, slug, content, category_id) VALUES ($1, $2, $3, $4, $5)",
        _ => unimplemented!("Unimplemented"),
    }
}