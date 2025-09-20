use std::env;

use crate::Config;

pub fn connect_to_database() -> Config {
    Config {
        database_url: format!(
            "postgres://{}:{}@{}:{}/{}",
            env::var("POSTGRES_USER").unwrap(),
            env::var("POSTGRES_PASSWORD").unwrap(),
            env::var("POSTGRES_HOST").unwrap(),
            env::var("POSTGRES_PORT").unwrap(),
            env::var("POSTGRES_DB").unwrap(),
        ),
        port: env::var("API_PORT").unwrap(),
    }
}