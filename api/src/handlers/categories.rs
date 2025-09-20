use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{db::categories::queries::create_categorie, functions::parse_id_handler, models::categorie_schema::{CreateCategorieSchema, PatchCategorie}, services::categorie::{delete_category_from_db, fetch_many_categories_from_db, fetch_one_categorie_from_db, format_category_response_many, format_category_response_one, patch_categorie_in_db}, AppState};



pub async fn get_categories(
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse,(StatusCode, Json<serde_json::Value>)> {
    let categories = fetch_many_categories_from_db(&data.db).await?;
    Ok(format_category_response_many(categories))
}

pub async fn get_one_categorie(
    State(data): State<Arc<AppState>>,
    Path(id_str): Path<String>, 
) -> Result<impl IntoResponse,(StatusCode, Json<serde_json::Value>)> {
    let id = parse_id_handler(&id_str)?;
    let categorie = fetch_one_categorie_from_db(&data.db, id).await?;
    Ok(format_category_response_one(categorie))
}

pub async fn post_categorie(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateCategorieSchema>
) -> Result <impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match create_categorie(&data.db, sqlx::types::Json(body)).await {
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
    let categorie = delete_category_from_db(&data.db, id).await?;
    Ok(format_category_response_one(categorie))
}

pub async fn patch_categorie_handler(
    State(data): State<Arc<AppState>>,
    Path(id_str): Path<String>,             // path first
    Json(payload): Json<PatchCategorie>,    // then body
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = parse_id_handler(&id_str)?;
    let categorie = patch_categorie_in_db(&data.db, id, payload).await?;
    Ok(format_category_response_one(categorie))
}
