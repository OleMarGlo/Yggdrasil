use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{db::categories::queries::{create_categorie, delete_categorie, fetch_categories, fetch_one_categorie, update_row}, functions::{get_highest_id, parse_id, parse_id_handler}, models::{categorie_schema::{CreateCategorieSchema, PatchCategorie}, categories::{CategorieModel, CategorieModelResponse}}, AppState};

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
) -> Result<impl IntoResponse,(StatusCode, Json<serde_json::Value>)> {
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

pub async fn post_categorie(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateCategorieSchema>
) -> Result <impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = match get_highest_id(&data.db).await {
        Ok(Some(highest_id)) => highest_id + 1,
        Ok(None) => 1,
        Err(_) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "unable to query DB"
            }))));        
        }
    };

    match create_categorie(&data.db, sqlx::types::Json(body), id).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(serde_json::json!({"message": "Post created successfully"})))),
        Err(_) => Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "unable to create post"
        })))),    
    }
}

pub async fn del_categorie(
    State(data): State<Arc<AppState>>,
    Path(id_str): Path<String>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = parse_id_handler(&id_str)?;
    let categorie = delete_categorie(&data.db, id)
        .await
        .map_err(|err| {
            let errpr_response = serde_json::json!({
                "status": "error",
                "message": format!("unable to delete id, err: {}", err),
            });
            (StatusCode::BAD_REQUEST, Json(errpr_response))
        })?;

        let categorie_response = to_category_response(&categorie);

        let json_response = json!({
            "status": "ok",
            "categorie": categorie_response
        });
        Ok(Json(json_response))
}

pub async fn patch_categorie_handler(
    State(data): State<Arc<AppState>>,
    Path(id_str): Path<String>,             // path first
    Json(payload): Json<PatchCategorie>,    // then body
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = parse_id_handler(&id_str)?;
    let categorie = update_row(&data.db, id, payload.category, payload.slug, payload.description)
        .await
        .map_err(|err| {
            let errpr_response = serde_json::json!({
                "status": "error",
                "message": format!("unable to delete id, err: {}", err),
            });
            (StatusCode::BAD_REQUEST, Json(errpr_response))
        })?;

        let categorie_response = to_category_response(&categorie);

        let json_response = json!({
            "status": "ok",
            "categorie": categorie_response
        });
        Ok(Json(json_response))
}
