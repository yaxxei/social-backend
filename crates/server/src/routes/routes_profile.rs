use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use lib_core::model::ModelManager;
use lib_web::{handlers::handlers_profile, middlewares};

pub async fn routes(mm: Arc<ModelManager>) -> Router {
    Router::new()
        .route("/saves", get(handlers_profile::get_saves))
        .route("/saves", post(handlers_profile::create_save))
        .route("/saves", delete(handlers_profile::delete_save))
        .with_state(mm)
        .layer(middleware::from_fn(middlewares::require_auth))
}
