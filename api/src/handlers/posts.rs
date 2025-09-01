use serde::Serialize;
use axum::Json;


#[derive(Serialize)]
pub struct Post {
    id: u32,
    title: String,
    content: String,
}


pub async fn get_posts() -> Json<Vec<Post>> {
    let posts = vec![
        Post {id:1,title:"Hello".to_string(), content: "Monkey".to_string() },
        Post { id: 2, title: "Foo".to_string(), content: "Bar".to_string() },
    ];

    Json(posts)
}