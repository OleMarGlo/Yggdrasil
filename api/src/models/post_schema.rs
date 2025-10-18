use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub order_by: Option<String>,
    pub sort: Option<String>
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreatePostSchema {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub category: String,
}

#[derive(Deserialize, Serialize)]
pub struct PatchPost {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub category: Option<i32>,
}