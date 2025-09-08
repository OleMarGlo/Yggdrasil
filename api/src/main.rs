mod handlers;
mod db;
mod router;
mod consts;
mod models;
mod functions;

use std::sync::Arc;

use serde::Deserialize;
use sqlx::PgPool;

use crate::{db::connection::connect_to_database, router::create_router};

#[derive(Deserialize)]
struct Config {
    database_url: String,
    port: String
}

pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info,tower_http=info")
        .with_writer(std::io::stdout)
        .with_ansi(false)
        .init();
    
    tracing::info!("Server initialized, ready to accept requests");

    let config = connect_to_database();
    let pool = match PgPool::connect(&config.database_url)
        .await
        {
            Ok(pool) => {
                println!("Successfully connected to database");
                pool
            }
            Err(err) => {
                panic!("Unable to connect to database err: {}", err);
            }
        };
        
    let app = create_router(Arc::new(AppState { db: pool.clone() }));
    
    let address: String = "0.0.0.0:".to_string() + &config.port.to_owned();

    println!("Starting server at port {}", address);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap();
    
    axum::serve(listener, app)
        .await
        .unwrap();
}
