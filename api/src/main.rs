mod handlers;
mod db;
mod router;
mod consts;

use std::{env, sync::Arc};

use axum::{
    routing::get, Extension, Router
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{handlers::posts::get_posts, router::create_router};

#[derive(Deserialize)]
struct Config {
    database_url: String,
    port: String
}

pub struct State {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    let config = Config {
        database_url: format!(
            "postgres://{}:{}@{}:{}/{}",
            env::var("POSTGRES_USER").unwrap(),
            env::var("POSTGRES_PASSWORD").unwrap(),
            env::var("POSTGRES_HOST").unwrap(),
            env::var("POSTGRES_PORT").unwrap(),
            env::var("POSTGRES_DB").unwrap(),
        ),
        port: env::var("API_PORT").unwrap(),
    };

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
        


    let app = create_router(Arc::new(State { db: pool.clone() }));
    
    let address: String = "0.0.0.0:".to_string() + &config.port.to_owned();

    println!("Starting server at port {}", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
