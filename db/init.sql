-- Create categories first
CREATE TABLE IF NOT EXISTS categories (
    id          SERIAL PRIMARY KEY,
    category    VARCHAR(255) NOT NULL,
    slug        VARCHAR(255) NOT NULL,
    description TEXT NOT NULL
);

-- Then posts referencing categories
CREATE TABLE IF NOT EXISTS posts (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    slug        VARCHAR(255) NOT NULL,
    content     TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    category_id INTEGER NOT NULL REFERENCES categories(id) ON UPDATE CASCADE ON DELETE CASCADE
);


INSERT INTO categories (id, category, slug, description) VALUES
    (1, 'Programming', 'programming', 'Projects i am either working on or want to work on')
ON CONFLICT (id) DO NOTHING;

INSERT INTO categories (id, category, slug, description) VALUES
    (2, 'Personal', 'personal', 'Personal blogs about me, my life, hobbies, etc')
ON CONFLICT (id) DO NOTHING;

INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (1, 'Hello World', 'hello-world', 'Hello world!', 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (2, 'Hello World 2', 'hello-world', 'Hello world!', 2)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (3, 'Hello World 3', 'hello-world', 'Hello world!', 1)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (4, 'Hello World 4', 'hello-world', 'Hello world!', 2)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (5, 'Hello World 5', 'hello-world', 'Hello world!', 1)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (6, 'Hello World 6', 'hello-world', 'Hello world!', 2)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (7, 'Hello World 7', 'hello-world', 'Hello world!', 1)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (8, 'Hello World 8', 'hello-world', 'Hello world!', 2)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (9, 'Hello World 9', 'hello-world', 'Hello world!', 1)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (10, 'Hello World 10', 'hello-world', 'Hello world!', 2)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (11, 'Hello World 11', 'hello-world', 'Hello world!', 1)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (12, 'Hello World 12', 'hello-world', 'Hello world!', 2)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (13, 'Hello World 13', 'hello-world', 'Hello world!', 1)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (14, 'Hello World 14', 'hello-world', 'Hello world!', 2)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (15, 'Hello World 15', 'hello-world', 'Hello world!', 1)
ON CONFLICT (id) DO NOTHING;
INSERT INTO posts (id, title, slug, content, category_id) VALUES
    (16, 'Hello World 16', 'hello-world', 'Hello world!', 2)
ON CONFLICT (id) DO NOTHING;