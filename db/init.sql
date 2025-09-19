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

-- Seed
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
