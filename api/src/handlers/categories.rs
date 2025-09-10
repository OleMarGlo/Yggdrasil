use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{db::{categories::queries::{fetch_categories, fetch_one_categorie}, posts, table::fetch_one_row}, models::categories::{CategorieModel, CategorieModelResponse}, AppState};

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

pub async fn get_one_categorie(
    State(data): State<Arc<AppState>>,
    Path(id): Path<String>, 
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let categorie = fetch_one_categorie(&data.db, &id)
        .await
        .map_err(|_| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "unable to find id"
            });
            (StatusCode::BAD_REQUEST, Json(error_response))
        })?;
    let json_response = serde_json::json!({
        "status": "ok",
        "categorie": categorie,
    });

    Ok(Json(json_response))
}