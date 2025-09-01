use std::{sync::Arc};

use axum::routing::get;
use axum::Router;

use crate::handlers::posts::get_posts;
use crate::{consts, State};



pub fn create_router(app_state: Arc<State>) -> Router {
    Router::new()
        .route(&format!("/api/{}/posts", consts::VERSION), get(get_posts))
        .with_state(app_state)
}