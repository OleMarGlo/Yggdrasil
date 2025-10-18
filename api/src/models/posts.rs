use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub category: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub total_count: i64
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PostModelResponse {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub category: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}