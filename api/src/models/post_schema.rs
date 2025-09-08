use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>
}

#[derive(Deserialize)]
pub struct PostSchema {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CreatePostSchema {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub category: i32,
}