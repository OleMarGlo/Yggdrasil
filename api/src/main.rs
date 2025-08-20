use axum::{
    routing::get,
    Router,
    response::Json,
};

use serde::Serialize;
use std::net::SocketAddr;

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
fn main() {
    let app = Router::new()
        .route("/posts", get(get_posts()));
    
    let addr = SocketAddr::from(([127,0,0,1], 3000));
    println!("Server running at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind()
}
