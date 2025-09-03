use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PostModelResponse {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}