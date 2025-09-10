use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct CategorieModel {
    pub id: i32,
    pub category: String,
    pub slug: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct CategorieModelResponse {
    pub id: i32,
    pub category: String,
    pub slug: String,
    pub description: String,
}