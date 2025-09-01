use std::env;

use axum::{
    routing::get, Router
};
use serde::Deserialize;

use crate::handlers::posts::get_posts;

mod handlers;
mod db;

#[derive(Deserialize)]
struct Config {
    database_url: String,
    port: String
}

#[tokio::main]
async fn main() {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("POSTGRES_USER").unwrap(),
        env::var("POSTGRES_PASSWORD").unwrap(),
        env::var("POSTGRES_HOST").unwrap(),
        env::var("POSTGRES_PORT").unwrap(),
        env::var("POSTGRES_DB").unwrap()
    );
    println!("{}", url);

    let app = Router::new()
        .route("/root", get(get_posts));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
