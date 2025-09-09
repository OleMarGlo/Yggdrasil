use std::{sync::Arc};

use axum::http::Method;
use axum::routing::{get};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

use crate::handlers::posts::{get_post, get_posts, post_posts};
use crate::{consts, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS]);


    Router::new()
        .route(
            &format!("{}/posts", consts::VERSION),
             get(get_posts)
            .post(post_posts))
        .route(&format!("{}/posts/{{id}}", consts::VERSION), get(get_post))
        .with_state(app_state)
        .layer (
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO))
                )       
                .layer(cors)        
        )
}