use axum::{http::StatusCode, Json};

pub fn parse_id_handler(id_string: &String) 
-> Result<i32, (StatusCode, Json<serde_json::Value>)> {
    match id_string.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err((
            StatusCode::BAD_REQUEST, 
            Json(serde_json::json!({
            "error": "unable to parse id, not an int"
            })
        ))),
    }
}