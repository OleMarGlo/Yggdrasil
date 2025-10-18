pub enum Table {
    Posts,
    Categories,
}

pub fn fetch_all(table: Table, order_by: Option<&str>, sort_by: Option<&str>) -> String {

    let valid_direction = vec!["asc", "desc"];
    let dir = if valid_direction.contains(&sort_by.unwrap_or("asc")) {
        sort_by
    } else {
        Some("asc")
    };

    let valid_columns = match table {
        Table::Posts => vec![
            "id", "title", "slug", "content", "created_at", "updated_at", "category",
        ],
        Table::Categories => vec!["id", "category", "slug", "description"],
    };

    let column = if valid_columns.contains(&order_by.unwrap_or("id")) {
        order_by
    } else {
        Some("id")
    };

    match table {
        Table::Posts => format!(
        r#"SELECT 
            p.id, p.title, p.slug, 
            p.content, p.created_at, p.updated_at,
            categories.category as category
        FROM 
            posts AS p
        JOIN
            categories
        ON
            p.category_id = categories.id
        WHERE 
            (
                $1 = ''
                OR to_json(p)::text ILIKE '%' || $1 || '%'
            )
            AND (
                cardinality($2) = 0
                OR p.category_id = ANY($2)
            )
        ORDER BY p.{column} {direction}
        LIMIT $3 OFFSET $4;"#,
        column = column.unwrap_or("id"),
        direction = dir.unwrap_or("asc")
        ),
        Table::Categories => format!(
            "SELECT * FROM categories ORDER BY {column} {direction}",
            column = column.unwrap_or("id"),
            direction = dir.unwrap_or("asc")
        ),
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
            INSERT INTO posts(title, slug, content, category_id) 
            VALUES ($2, $3, $4, (SELECT id FROM categories WHERE category=$5));"#,
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
        Table::Posts => r#"
        WITH updated_post AS (
            UPDATE posts
            SET 
                title = COALESCE($1, title),   
                slug = COALESCE($2, slug),
                content = COALESCE($3, content),
                category_id = COALESCE($4, category_id),
                updated_at = NOW()
            WHERE id = $5
            RETURNING *
        )
        SELECT
            up.id,
            up.title,
            up.slug,
            up.content,
            up.updated_at,
            up.created_at,
            c.category AS category
        FROM updated_post up
        JOIN categories c ON up.category_id = c.id;
            "#,
        Table::Categories => r#"
            UPDATE categories
            SET 
                category = COALESCE($1, category),   
                slug = COALESCE($2, slug),
                description = COALESCE($3, description)
            WHERE id = $4
            RETURNING *;
        "#,
    }
}