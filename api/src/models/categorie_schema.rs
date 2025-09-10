use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]

pub struct CreateCategorieSchema {
    pub category: String,
    pub slug: String,
    pub description: String,
}

