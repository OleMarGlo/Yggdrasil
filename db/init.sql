CREATE TABLE if NOT EXISTS posts (
    id INTEGER PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    category_id INTEGER NOT NULL,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE TABLE if NOT EXISTS categories (
    id INTEGER PRIMARY KEY,
    categorie VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
);

INSERT INTO categories (id, categorie, slug, description) 
VALUES (1, 'Programming', 'programming', 'Projects i am either working on or want to work on');
INSERT INTO categories (id, categorie, slug, description) 
VALUES (2, 'Personal', 'personal', 'Personal blogs about me, my life, hobbies, etc');
