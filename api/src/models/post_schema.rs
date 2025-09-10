use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>
}

#[derive(Deserialize, Serialize)]
pub struct PostSchema {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreatePostSchema {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub category: String,
}