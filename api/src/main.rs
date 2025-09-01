use axum::{
    response::Json, routing::get, Router
};
use serde::Serialize;

#[derive(Serialize)]
struct Post {
    id: u32,
    title: String,
    content: String,
}

async fn get_posts() -> Json<Vec<Post>> {
    let posts = vec![
        Post { id: 1, title: "Hello World".into(), content: "This is my first post".into()},
        Post { id: 2, title: "Hello World again".into(), content: "This is my second post".into()},
    ];
    Json(posts)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/posts", get(get_posts));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
