# Yggdrasil
Portfolio website i am working on
## Description
A portfolio website made to track my intrests, hobbies, and projects i am working on. 

### Backend
First version of basckend is up and working. In the future i plan to add more support for filtering and searching, aswell as sorting.
### Frontend
When i get to it im planning to write the frontend in nextjs

## Installation
So just clone it and create a .env file
```bash
git clone https://github.com/OleMarGlo/Yggdrasil/tree/main
cd yggdrasil
docker-compose up
```

## Usage
### API
The API has the following endpoints for posts
#### GET /posts
Returns all posts
```bash
curl -X GET http://localhost:8000/posts
```
returns
```json
{
    "status": "ok",
    "count": 2,
    "posts": [
        {
            "id": 1,
            "title": "Programming",
            "slug": "programming",
            "description": "Projects i am either working on or want to work on"
        },
        {
            "id": 2,
            "title": "Personal",
            "slug": "personal",
            "description": "Personal blogs about me, my life, hobbies, etc"
        }
    ]
}
```

#### GET /posts/:id
Returns a single post
```bash
curl -X GET http://localhost:8000/posts/1
```
returns
```json
{
    "status": "ok",
    "id": 1,
    "title": "Programming",
    "slug": "programming",
    "description": "Projects i am either working on or want to work on"
}
```

#### POST /posts
Creates a new post
```bash
curl -X POST http://localhost:8000/posts \
    -H 'Content-Type: application/json' \
    -d '{
        "title": "Hello World",
        "slug": "hello-world",
        "content": "Hello world!",
        "category": 1
    }'
```
returns 201 Created

#### PATCH /posts/:id
Updates a post
```bash
curl -X PATCH http://localhost:8000/posts/1 \
    -H 'Content-Type: application/json' \
    -d '{
        "title": "Hello World",
        "slug": "hello-world",
        "content": "Hello world!",
        "category": 1
    }'

```
return 
```json
{
    "status": "ok",
    "id": 1,
    "title": "Programming",
    "slug": "programming",
    "description": "Projects i am either working on or want to work on"
}
```

#### DELETE /posts/:id
Deletes a post
```bash
curl -X DELETE http://localhost:8000/posts/1
```
returns
```json
{
    "status": "ok",
    "id": 1,
    "title": "Programming",
    "slug": "programming",
    "description": "Projects i am either working on or want to work on"
}
```

The API has the following endpoints for categories
#### GET /categories
Returns all categories
```bash
curl -X GET http://localhost:8000/categories
```
returns
```json
{
    "status": "ok",
    "count": 2,
    "categories": [
        {
            "id": 1,
            "category": "Programming",
            "slug": "programming",
            "description": "Projects i am either working on or want to work on"
        },
        {
            "id": 2,
            "category": "Personal",
            "slug": "personal",
            "description": "Personal blogs about me, my life, hobbies, etc"
        }
    ]
}
```

#### GET /categories/:id
Returns a single category
```bash
curl -X GET http://localhost:8000/categories/1
```
returns
```json
{
    "status": "ok",
    "id": 1,
    "category": "Programming",
    "slug": "programming",
    "description": "Projects i am either working on or want to work on"
}
```

#### POST /categories
Creates a new category
```bash
curl -X POST http://localhost:8000/categories \
    -H 'Content-Type: application/json' \
    -d '{
        "category": "Hello World",
        "slug": "hello-world",
        "description": "Hello world!"
    }'
```
returns 201 Created

#### PATCH /categories/:id
Updates a category
```bash
curl -X PATCH http://localhost:8000/categories/1 \
    -H 'Content-Type: application/json' \
    -d '{
        "category": "Hello World",
        "slug": "hello-world",
        "description": "Hello world!"
    }'

```
return 
```json
{
    "status": "ok",
    "id": 1,
    "category": "Programming",
    "slug": "programming",
    "description": "Projects i am either working on or want to work on"
}
```

#### DELETE /categories/:id
Deletes a category
```bash
curl -X DELETE http://localhost:8000/categories/1
```
returns
```json
{
    "status": "ok",
    "id": 1,
    "category": "Programming",
    "slug": "programming",
    "description": "Projects i am either working on or want to work on"
}
```


### Frontend
I am still working on the frontend as of now, so i will get back to it later