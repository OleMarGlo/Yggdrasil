use std::{sync::Arc};

use axum::routing::get;
use axum::Router;

use crate::handlers::posts::posts_handler;
use crate::{consts, AppState};



pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(&format!("/api/{}/posts", consts::VERSION), get(posts_handler))
        .with_state(app_state)
}