use std::{sync::Arc};

use axum::http::Method;
use axum::routing::{get};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::handlers::posts::get_posts;
use crate::{consts, AppState};



pub fn create_router(app_state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS]);


    Router::new()
        .route(&format!("/api/{}/posts", consts::VERSION), get(get_posts))
        .with_state(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
        )
}