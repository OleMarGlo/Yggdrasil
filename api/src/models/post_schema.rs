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
