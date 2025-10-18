use axum::{http::StatusCode, Json};
use sqlx::{PgPool};

use crate::{db::categories::queries::{delete_categorie, fetch_all_active_categories, fetch_categories, fetch_one_categorie, update_row}, models::{categorie_schema::PatchCategorie, categories::{CategorieModel, CategorieModelResponse}}};


fn to_category_response(cat: &CategorieModel) -> CategorieModelResponse {
    CategorieModelResponse { 
        id: (cat.id), 
        category: (cat.category.to_owned()), 
        slug: (cat.slug.to_owned()), 
        description: (cat.description.to_owned()) }
}


pub async fn delete_category_from_db(
    db: &PgPool,
    id: i32,
) -> Result<CategorieModel, (StatusCode, Json<serde_json::Value>)> {
    delete_categorie(&db, id)
        .await
        .map_err(|err| {
            let errpr_response = serde_json::json!({
                "status": "error",
                "message": format!("unable to delete id, err: {}", err),
            });
            (StatusCode::BAD_REQUEST, Json(errpr_response))
        })
}

pub async fn patch_categorie_in_db(
    db: &PgPool,
    id: i32,
    payload: PatchCategorie
) -> Result<CategorieModel, (StatusCode, Json<serde_json::Value>)> {
    update_row(&db, id, payload.category, payload.slug, payload.description)
    .await
        .map_err(|err| {
            let error_response = serde_json::json!({
            "status": "error",
            "message": format!("unable to delete id, err: {}", err),
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })
}

pub fn format_category_response_one(categorie: CategorieModel) -> Json<serde_json::Value> {
    let categorie_response = to_category_response(&categorie);
    let json_response = serde_json::json!({
        "categorie": categorie_response
    });
    Json(json_response)
}

pub fn format_category_response_many(categories: Vec<CategorieModel>) -> Json<serde_json::Value> {
    let categorie_response = categories
        .into_iter()
        .map(| cat: CategorieModel | to_category_response(&cat))
        .collect::<Vec<CategorieModelResponse>>();

    let json_response = serde_json::json!({
        "count": categorie_response.len(),
        "categories": categorie_response
    });
    Json(json_response)
}

pub async fn fetch_one_categorie_from_db(
    db: &PgPool,
    id: i32
) -> Result<CategorieModel, (StatusCode, Json<serde_json::Value>)> {
    fetch_one_categorie(db, id)
        .await
        .map_err(|_| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "unable to find id"
            });
            (StatusCode::BAD_REQUEST, Json(error_response))
        })
}

pub async fn fetch_many_categories_from_db(
    db: &PgPool,
) -> Result <Vec<CategorieModel>, (StatusCode, Json<serde_json::Value>)> {
    fetch_categories(&db)
        .await
        .map_err(|err| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", err)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
}

pub async fn fetch_active_categories_from_db(
    db: &PgPool,
) -> Result<Vec<CategorieModel>, (StatusCode, Json<serde_json::Value>)> {
    fetch_all_active_categories(db)
        .await
        .map_err(|err| {
            let error_respose = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", err)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_respose))
        })
}