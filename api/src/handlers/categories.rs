use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{db::{categories::queries::fetch_categories, posts}, models::categories::{CategorieModel, CategorieModelResponse}, AppState};

fn to_category_response(cat: &CategorieModel) -> CategorieModelResponse {
    CategorieModelResponse { 
        id: (cat.id), 
        category: (cat.category.to_owned()), 
        slug: (cat.slug.to_owned()), 
        description: (cat.description.to_owned()) }
}

pub async fn get_categories(
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse,(StatusCode, Json<serde_json::Value>)> {
    let categories = fetch_categories(&data.db)
        .await
        .map_err(|err| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", err)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

        let category_response = categories
            .into_iter()
            .map(|cat: CategorieModel| to_category_response(&cat))
            .collect::<Vec<CategorieModelResponse>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "count": category_response.len(),
        "categories": category_response,
    });

    Ok(Json(json_response))
}