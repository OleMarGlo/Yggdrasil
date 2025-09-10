pub enum Table {
    Posts,
    Categories,
}

pub fn fetch_sql(table: Table) -> &'static str {
    match table {
        Table::Posts => 
        r#"SELECT 
            posts.id, posts.title, posts.slug, 
            posts.content, posts.created_at, posts.updated_at,
            categories.category as category
        FROM 
            posts
        JOIN
            categories
        ON
            posts.category_id = categories.id
        ORDER BY posts.id 
        LIMIT $1 OFFSET $2;"#,
        Table::Categories => "SELECT * FROM categories ORDER BY id LIMIT $1 OFFSET $2",
    }
}

pub fn fetch_one_post(table: Table) -> &'static str {
    match table {
        Table::Posts => r#"
        SELECT 
            posts.id, posts.title, posts.slug, 
            posts.content, posts.created_at, posts.updated_at,
            categories.category as category
        FROM posts
        JOIN categories ON posts.category_id = categories.id
        WHERE posts.id=$1;"#,
        _ => unimplemented!("Unimplemented")
    }
}

pub fn add_one_post(table: Table) -> &'static str {
    match table {
        Table::Posts => r#"
            INSERT INTO posts(id, title, slug, content, category_id) 
            VALUES ($1, $2, $3, $4, (SELECT id FROM categories WHERE category=$5));"#,
        _ => unimplemented!("Unimplemented"),
    }
}