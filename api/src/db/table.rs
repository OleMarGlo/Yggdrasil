pub enum Table {
    Posts,
    Categories,
}

pub fn fetch_all(table: Table) -> &'static str {
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
        Table::Categories => "SELECT * FROM categories ORDER BY id",
    }
}

pub fn fetch_one_row(table: Table) -> &'static str {
    match table {
        Table::Posts => r#"
        SELECT 
            posts.id, posts.title, posts.slug, 
            posts.content, posts.created_at, posts.updated_at,
            categories.category as category
        FROM posts
        JOIN categories ON posts.category_id = categories.id
        WHERE posts.id=$1;"#,
        Table::Categories => "SELECT * FROM categories WHERE id=$1",
    }
}

pub fn add_one_row(table: Table) -> &'static str {
    match table {
        Table::Posts => r#"
            INSERT INTO posts(id, title, slug, content, category_id) 
            VALUES ($1, $2, $3, $4, (SELECT id FROM categories WHERE category=$5));"#,
        Table::Categories => r#"
            INSERT INTO categories(category, slug, description)
            VALUES ($1, $2, $3)
        "#,
    }
}

pub fn delete_row(table: Table) -> &'static str {
    match table {
        Table::Posts => r#"
            WITH deleted_post AS (
                DELETE FROM posts
                WHERE id = $1
                RETURNING *
            )
            SELECT
                dp.id,
                dp.title,
                dp.slug,
                dp.content,
                dp.updated_at,
                dp.created_at,
                c.category as category
            FROM deleted_post dp
            JOIN categories c ON dp.category_id = c.id;
        "#,
        Table::Categories => r#"
            DELETE FROM categories
            WHERE id=$1
            RETURNING *
        "#,
    }
}

pub fn get_posts_with_categorie(table: Table) -> &'static str {
    match table {
        Table::Posts => r#"
            SELECT 
                posts.id, posts.title, posts.slug, 
                posts.content, posts.created_at, posts.updated_at,
                categories.category as category
            FROM posts
            JOIN categories ON posts.category_id = categories.id
            WHERE categories.id=$1;
        "#,
        _ => unimplemented!("Unimplemented"),
    }
}

pub fn update_one_row(table: Table) -> &'static str {
    match table {
        Table::Posts => unimplemented!("unimplemented"),
        Table::Categories => r#"
            UPDATE categories
            SET 
                category = COALESCE($1, category),   
                slug = COALESCE($2, slug),
                description = COALESCE($3, description)
            WHERE id = $4
            RETURNING *
        "#,
    }
}