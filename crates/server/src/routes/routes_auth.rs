use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use lib_core::model::ModelManager;
use lib_web::{handlers::handlers_auth, middlewares};

pub async fn routes(mm: Arc<ModelManager>) -> Router {
    Router::new()
        .route("/register", post(handlers_auth::register))
        .route("/login", post(handlers_auth::login))
        .route("/logout", post(handlers_auth::logout))
        .route("/refresh", post(handlers_auth::refresh))
        .route(
            "/me",
            get(handlers_auth::auth_me).layer(middleware::from_fn(middlewares::require_auth)),
        )
        .with_state(mm)
}
