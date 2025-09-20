use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateCategorieSchema {
    pub category: String,
    pub slug: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct PatchCategorie {
    pub category: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>
}