use std::{sync::Arc};

use axum::http::Method;
use axum::routing::{get};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

use crate::handlers::categories::{del_categorie, get_categories, get_one_categorie, patch_categorie_handler, post_categorie};
use crate::handlers::posts::{delete_post, get_post, get_posts, get_posts_in_categorie, patch_post, post_posts};
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
        .route(&format!("{}/posts/{{id}}", consts::VERSION), 
        get(get_post)
        .delete(delete_post)
        .patch(patch_post)
    )
        .route(&format!("{}/posts/categories/{{id}}", consts::VERSION), get(get_posts_in_categorie))
        .route(
            &format!("{}/categories", consts::VERSION),
            get(get_categories)
            .post(post_categorie)
        )
        .route(
            &format!("{}/categories/{{id}}", consts::VERSION), 
        get(get_one_categorie)
            .delete(del_categorie)
            .patch(patch_categorie_handler)
        )
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